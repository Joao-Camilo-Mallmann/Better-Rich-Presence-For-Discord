//! # Discord RPC Manager Module
//!
//! Manages the Discord Rich Presence IPC connection on a **dedicated `std::thread`**.
//!
//! The `discord-presence` crate is fully synchronous — calling any of its methods
//! from an async context would block the Tokio runtime. This module isolates all
//! RPC interactions on a background OS thread that communicates with the rest of
//! the app via `std::sync::mpsc`.
//!
//! ## Architecture
//!
//! ```text
//!  Engine (tokio) ──► DiscordHandle::send(cmd) ──► std::sync::mpsc ──► RPC Thread
//!                                                                       │
//!                                                        discord_presence::Client
//! ```
//!
//! The RPC thread owns the `Client`, processes `EngineCommand`s, and implements
//! an exponential-backoff watchdog for automatic reconnection.

use crate::models::types::{EngineCommand, PresenceData, DEFAULT_CLIENT_ID};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use log::{error, info, warn};

/// Discord Application ID constant imported from types.
const DISCORD_APP_ID: u64 = DEFAULT_CLIENT_ID; 

// ---------------------------------------------------------------------------
// DiscordHandle — send-side interface for the rest of the app
// ---------------------------------------------------------------------------

/// A cloneable handle used by the engine to send commands to the Discord thread.
#[derive(Clone)]
pub struct DiscordHandle {
    tx: Sender<EngineCommand>,
    connected: Arc<AtomicBool>,
}

impl DiscordHandle {
    /// Send a command to the Discord RPC thread.
    /// Returns `false` if the channel is disconnected (thread has stopped).
    pub fn send(&self, cmd: EngineCommand) -> bool {
        match self.tx.send(cmd) {
            Ok(()) => true,
            Err(e) => {
                error!("[Discord] Failed to send command: {}", e);
                false
            }
        }
    }

    /// Check whether the Discord client reports itself as connected.
    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::Relaxed)
    }
}

// ---------------------------------------------------------------------------
// DiscordManager — spawns and owns the background RPC thread
// ---------------------------------------------------------------------------

/// Manages the lifecycle of the Discord RPC background thread.
pub struct DiscordManager {
    _handle: thread::JoinHandle<()>,
}

impl DiscordManager {
    /// Spawn the Discord RPC thread and return `(DiscordManager, DiscordHandle)`.
    ///
    /// The thread will:
    /// 1. Create a `discord_presence::Client`.
    /// 2. Register `on_ready` and `on_error` callbacks.
    /// 3. Call `client.start()` to initiate IPC.
    /// 4. Loop, receiving `EngineCommand`s and applying them.
    /// 5. On disconnection, attempt reconnect with exponential backoff.
    pub fn new(app_handle: tauri::AppHandle) -> (Self, DiscordHandle) {
        let (tx, rx): (Sender<EngineCommand>, Receiver<EngineCommand>) = std::sync::mpsc::channel();
        let connected = Arc::new(AtomicBool::new(false));
        let connected_clone = Arc::clone(&connected);

        let handle = thread::Builder::new()
            .name("discord-rpc".into())
            .spawn(move || {
                Self::rpc_thread_main(rx, connected_clone, app_handle);
            })
            .expect("Failed to spawn Discord RPC thread");

        let discord_handle = DiscordHandle { tx, connected };
        (DiscordManager { _handle: handle }, discord_handle)
    }

    /// Helper to register Event callbacks for client
    fn register_callbacks(
        client: &mut discord_presence::Client,
        client_id: u64,
        connected: Arc<AtomicBool>,
        app_handle: tauri::AppHandle,
    ) {
        use crate::models::types::DiscordUser;
        use tauri::{Emitter, Manager};

        let connected_ready = Arc::clone(&connected);
        let app_ready = app_handle.clone();
        client
            .on_ready(move |ctx| {
                info!(
                    "[Discord] RPC client is READY (client_id={})",
                    client_id
                );
                connected_ready.store(true, Ordering::Relaxed);

                if let discord_presence::models::EventData::Ready(ref ready_event) = ctx.event {
                    if let Some(ref user) = ready_event.user {
                        let user_id = user.id.clone().unwrap_or_default();
                        let username = user.username.clone().unwrap_or_default();
                        let discriminator = user.discriminator.clone().unwrap_or_default();
                        let avatar = user.avatar.clone().unwrap_or_default();

                        if !user_id.is_empty() && !username.is_empty() {
                            let avatar_url = if avatar.is_empty() {
                                let disc = discriminator.parse::<u32>().unwrap_or(0);
                                format!("https://cdn.discordapp.com/embed/avatars/{}.png", if disc > 0 { disc % 5 } else { 0 })
                            } else {
                                format!("https://cdn.discordapp.com/avatars/{}/{}.png", user_id, avatar)
                            };

                            let discord_user = DiscordUser {
                                id: user_id,
                                username,
                                avatar_url,
                            };

                            if let Some(state) = app_ready.try_state::<crate::models::types::AppState>() {
                                let mut inner = tauri::async_runtime::block_on(state.inner.write());
                                inner.discord_user = Some(discord_user.clone());
                            }

                            let _ = app_ready.emit("discord-user-updated", &discord_user);
                        }
                    }
                }
            })
            .persist();

        let connected_err = Arc::clone(&connected);
        let app_err = app_handle.clone();
        client
            .on_error(move |_ctx| {
                error!(
                    "[Discord] RPC client encountered an error (client_id={})",
                    client_id
                );
                connected_err.store(false, Ordering::Relaxed);

                if let Some(state) = app_err.try_state::<crate::models::types::AppState>() {
                    let mut inner = tauri::async_runtime::block_on(state.inner.write());
                    inner.discord_user = None;
                }

                let _ = app_err.emit("discord-user-updated", Option::<DiscordUser>::None);
            })
            .persist();
    }

    /// Main loop for the RPC thread. Owns the `discord_presence::Client`.
    fn rpc_thread_main(rx: Receiver<EngineCommand>, connected: Arc<AtomicBool>, app_handle: tauri::AppHandle) {
        // Exponential backoff delays (seconds).
        let backoff_steps: &[u64] = &[3, 5, 10, 30, 60];
        let mut backoff_idx: usize = 0;
        let mut current_client_id = DISCORD_APP_ID;

        // Create the discord client
        let mut client = discord_presence::Client::new(current_client_id);

        // Register callbacks
        Self::register_callbacks(&mut client, current_client_id, Arc::clone(&connected), app_handle.clone());

        // Attempt to start the IPC connection
        info!(
            "[Discord] Starting IPC client (app_id={})",
            current_client_id
        );
        client.start();

        // Give the handshake a moment
        thread::sleep(Duration::from_secs(2));

        // Main command loop
        loop {
            // Try to receive with a timeout so we can periodically check health
            match rx.recv_timeout(Duration::from_secs(1)) {
                Ok(cmd) => {
                    match cmd {
                        EngineCommand::SetActivity { client_id, data } => {
                            if client_id != current_client_id {
                                info!(
                                    "[Discord] Switching Discord Client ID from {} to {}",
                                    current_client_id, client_id
                                );
                                let _ = client.clear_activity();
                                connected.store(false, Ordering::Relaxed);
                                thread::sleep(Duration::from_millis(500));

                                client = discord_presence::Client::new(client_id);
                                current_client_id = client_id;

                                Self::register_callbacks(&mut client, current_client_id, Arc::clone(&connected), app_handle.clone());

                                client.start();
                                thread::sleep(Duration::from_secs(2));
                            }

                            Self::apply_activity(&mut client, current_client_id, &data, &connected);
                            // Reset backoff on successful interaction
                            backoff_idx = 0;
                        }
                        EngineCommand::ClearActivity => {
                            info!("[Discord] Clearing activity");
                            if let Err(e) = client.clear_activity() {
                                warn!("[Discord] Failed to clear activity: {}", e);
                            }
                        }
                        EngineCommand::Disconnect => {
                            info!("[Discord] Disconnect requested — shutting down");
                            let _ = client.clear_activity();
                            // The client will be dropped, closing the connection.
                            return;
                        }
                    }
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                    // Watchdog: if we believe we should be connected but aren't,
                    // attempt a reconnect with backoff.
                    if !connected.load(Ordering::Relaxed) {
                        let delay = backoff_steps[backoff_idx.min(backoff_steps.len() - 1)];
                        warn!(
                            "[Discord] Connection lost — reconnecting in {}s (attempt #{}) (client_id={})",
                            delay,
                            backoff_idx + 1,
                            current_client_id
                        );
                        thread::sleep(Duration::from_secs(delay));

                        // Re-create the client for a fresh connection attempt
                        client = discord_presence::Client::new(current_client_id);

                        Self::register_callbacks(&mut client, current_client_id, Arc::clone(&connected), app_handle.clone());

                        client.start();
                        thread::sleep(Duration::from_secs(2));

                        if backoff_idx < backoff_steps.len() - 1 {
                            backoff_idx += 1;
                        }
                    }
                }
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    info!("[Discord] Command channel closed — shutting down RPC thread");
                    let _ = client.clear_activity();
                    return;
                }
            }
        }
    }

    /// Apply a `PresenceData` as a Discord activity.
    fn apply_activity(
        client: &mut discord_presence::Client,
        _client_id: u64,
        data: &PresenceData,
        connected: &Arc<AtomicBool>,
    ) {
        // Helper to enforce Discord string limits (2 - 128 bytes)
        let format_string = |s: &str| -> String {
            let mut trimmed = s.trim().to_string();
            if trimmed.is_empty() {
                return String::new();
            }
            if trimmed.len() < 2 {
                trimmed.push_str("  ");
            }
            if trimmed.len() > 128 {
                let mut idx = 125;
                while !trimmed.is_char_boundary(idx) {
                    idx -= 1;
                }
                trimmed.truncate(idx);
                trimmed.push_str("...");
            }
            trimmed
        };

        let details = format_string(&data.details);
        let state = format_string(&data.state);
        let mut large_image = data.large_image.trim().to_string();
        let large_text = format_string(&data.large_text);
        let timestamp = data.timestamp;

        // Handle HTTP/HTTPS URLs for Discord RPC external assets
        if large_image.starts_with("http://") || large_image.starts_with("https://") {
            // Convert SVG URLs to PNG URLs since Discord cannot render raw SVGs
            if large_image.ends_with(".svg") {
                large_image = format!("{}.png", large_image.trim_end_matches(".svg"));
            }
        } else if large_image.is_empty() || large_image == "auto" {
            large_image = "default".to_string();
        }

        println!("Rich Presence image URL: {}", large_image);
        info!(
            "[Discord] Setting activity: name='{}', details='{}', state='{}', image='{}'",
            large_text, details, state, large_image
        );

        match client.set_activity(|mut act| {
            act = act.name(&large_text);
            if !details.is_empty() {
                act = act.details(&details);
            }
            if !state.is_empty() {
                act = act.state(&state);
            }

            act = act.assets(|mut assets| {
                if !large_image.is_empty() && large_image != "auto" {
                    assets = assets.large_image(&large_image);
                }
                if !large_text.is_empty() {
                    assets = assets.large_text(&large_text);
                }
                assets
            });

            if timestamp > 0 {
                act = act.timestamps(|ts| ts.start(timestamp as u64));
            }

            act
        }) {
            Ok(_) => {
                connected.store(true, Ordering::Relaxed);
            }
            Err(e) => {
                error!("[Discord] Failed to set activity: {}", e);
                connected.store(false, Ordering::Relaxed);
            }
        }
    }
}
