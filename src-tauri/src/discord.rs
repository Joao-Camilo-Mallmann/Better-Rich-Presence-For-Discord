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

use crate::types::{EngineCommand, PresenceData};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use log::{error, info, warn};

/// Discord Application ID placeholder.
/// Replace with your actual Discord Application ID (as u64).
const DISCORD_APP_ID: u64 = 1517170930764480552; // DISCORD_APP_ID_PLACEHOLDER

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
    pub fn new() -> (Self, DiscordHandle) {
        let (tx, rx): (Sender<EngineCommand>, Receiver<EngineCommand>) = std::sync::mpsc::channel();
        let connected = Arc::new(AtomicBool::new(false));
        let connected_clone = Arc::clone(&connected);

        let handle = thread::Builder::new()
            .name("discord-rpc".into())
            .spawn(move || {
                Self::rpc_thread_main(rx, connected_clone);
            })
            .expect("Failed to spawn Discord RPC thread");

        let discord_handle = DiscordHandle { tx, connected };
        (DiscordManager { _handle: handle }, discord_handle)
    }

    /// Main loop for the RPC thread. Owns the `discord_presence::Client`.
    fn rpc_thread_main(rx: Receiver<EngineCommand>, connected: Arc<AtomicBool>) {
        // Exponential backoff delays (seconds).
        let backoff_steps: &[u64] = &[3, 5, 10, 30, 60];
        let mut backoff_idx: usize = 0;

        // Create the discord client
        let mut client = discord_presence::Client::new(DISCORD_APP_ID);

        // Register callbacks — these are called internally by the client on
        // its own reader thread. We use the `connected` flag to communicate
        // connection state back to the handle.
        let connected_on_ready = Arc::clone(&connected);
        client.on_ready(move |_ctx| {
            info!("[Discord] RPC client is READY");
            connected_on_ready.store(true, Ordering::Relaxed);
        });

        let connected_on_error = Arc::clone(&connected);
        client.on_error(move |_ctx| {
            error!("[Discord] RPC client encountered an error");
            connected_on_error.store(false, Ordering::Relaxed);
        });

        // Attempt to start the IPC connection
        info!("[Discord] Starting IPC client (app_id={})", DISCORD_APP_ID);
        client.start();

        // Give the handshake a moment
        thread::sleep(Duration::from_secs(2));

        // Main command loop
        loop {
            // Try to receive with a timeout so we can periodically check health
            match rx.recv_timeout(Duration::from_secs(1)) {
                Ok(cmd) => {
                    match cmd {
                        EngineCommand::SetActivity(data) => {
                            Self::apply_activity(&mut client, &data, &connected);
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
                            "[Discord] Connection lost — reconnecting in {}s (attempt #{})",
                            delay,
                            backoff_idx + 1
                        );
                        thread::sleep(Duration::from_secs(delay));

                        // Re-create the client for a fresh connection attempt
                        client = discord_presence::Client::new(DISCORD_APP_ID);

                        let connected_ready = Arc::clone(&connected);
                        client.on_ready(move |_ctx| {
                            info!("[Discord] RPC client reconnected (READY)");
                            connected_ready.store(true, Ordering::Relaxed);
                        });

                        let connected_err = Arc::clone(&connected);
                        client.on_error(move |_ctx| {
                            error!("[Discord] RPC client error during reconnect");
                            connected_err.store(false, Ordering::Relaxed);
                        });

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
        data: &PresenceData,
        connected: &Arc<AtomicBool>,
    ) {
        let details = data.details.clone();
        let state = data.state.clone();
        let large_image = data.large_image.clone();
        let large_text = data.large_text.clone();
        let timestamp = data.timestamp;

        info!(
            "[Discord] Setting activity: details='{}', state='{}', image='{}'",
            details, state, large_image
        );

        match client.set_activity(|act| {
            act.details(&details)
                .state(&state)
                .assets(|assets| assets.large_image(&large_image).large_text(&large_text))
                .timestamps(|ts| ts.start(timestamp))
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
