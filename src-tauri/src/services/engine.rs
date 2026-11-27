//! # Presence Engine Module
//!
//! The core brain of the application. Processes incoming events, applies priority rules,
//! handles debounce and anti-flicker, and sends updates to the Discord RPC thread.

use crate::domain::claude_code::{
    claude_code_takes_precedence, ClaudeSession, ClaudeSessionTracker, CLAUDE_CODE_CATALOG_ID,
};
use crate::domain::editor::{parse_editor_title, EditorTitleContext};
use crate::domain::language::{detect_language, language_icon_url};
use crate::domain::presence::{
    activity_name, ide_details, ide_state, resolve_large_image, resolve_small_image, rule_details,
    rule_has_default_text,
};
use crate::models::types::{
    AppRule, AppState, ClaudeSessionSummary, EngineCommand, EngineEvent, PresenceData,
    PresenceState, PriorityInfo, Settings, DEFAULT_CLIENT_ID,
};
use crate::models::IdeContext;
use crate::services::app_registry::{format_app_presence, AppEntry, AppRegistry};
use log::{debug, info};
use std::collections::HashMap;
use tauri::Emitter;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration, Instant};

async fn fetch_discord_app_details(client_id: u64) -> Option<(String, String)> {
    let url = format!(
        "https://discord.com/api/v9/oauth2/applications/{}/rpc",
        client_id
    );
    let client = reqwest::Client::builder()
        .user_agent("Better-Rich-Presence/0.1.0")
        .timeout(Duration::from_secs(3))
        .build()
        .ok()?;
    let response = client.get(&url).send().await.ok()?;
    if response.status().is_success() {
        #[derive(serde::Deserialize)]
        struct DiscordAppResponse {
            name: String,
            #[serde(default)]
            icon: String,
        }
        let app: DiscordAppResponse = response.json().await.ok()?;
        Some((app.name, app.icon))
    } else {
        None
    }
}

#[derive(Debug, Clone)]
struct WindowSnapshot {
    process_name: String,
    exe_path: Option<String>,
    aliases: Vec<String>,
    window_title: String,
    is_prioritized: bool,
    foreground_app: Option<String>,
}

pub struct PresenceEngine {
    app_state: AppState,

    current_state: PresenceState,
    current_data: PresenceData,
    is_idle: bool,
    current_client_id: u64,
    last_update_time: Option<Instant>,

    // For storing the last non-idle presence
    pre_idle_data: Option<PresenceData>,

    // Cache for Discord application details (client_id -> (name, icon_hash))
    client_cache: HashMap<u64, (String, String)>,

    app_registry: AppRegistry,

    // Session tracking for IDEs
    current_ide_workspace: Option<String>,
    ide_workspace_start_time: Option<i64>,

    // Live Claude Code session tracking & precedence
    claude_tracker: ClaudeSessionTracker,
    claude_override_active: bool,
    last_window_snapshot: Option<WindowSnapshot>,
    pending_presence: Option<(u64, PresenceData)>,
    has_committed_presence: bool,
}

impl PresenceEngine {
    pub fn new(app_state: AppState) -> Self {
        Self {
            app_state,
            current_state: PresenceState::Disconnected,
            current_data: PresenceData::default(),
            is_idle: false,
            current_client_id: DEFAULT_CLIENT_ID,
            last_update_time: None,
            pre_idle_data: None,
            client_cache: HashMap::new(),
            app_registry: AppRegistry::new(),
            current_ide_workspace: None,
            ide_workspace_start_time: None,
            claude_tracker: ClaudeSessionTracker::new(),
            claude_override_active: false,
            last_window_snapshot: None,
            pending_presence: None,
            has_committed_presence: false,
        }
    }

    pub async fn run(mut self, mut rx: mpsc::Receiver<EngineEvent>) {
        info!("Presence Engine started");

        let mut settle_task: Option<tauri::async_runtime::JoinHandle<()>> = None;
        let (settle_tx, mut settle_rx) = mpsc::channel::<(u64, PresenceData)>(10);
        let mut rate_limit_task: Option<tauri::async_runtime::JoinHandle<()>> = None;
        let (flush_tx, mut flush_rx) = mpsc::channel::<()>(4);
        let mut connection_check_interval = tokio::time::interval(Duration::from_secs(1));

        // Initial sync of state to frontend
        self.sync_state().await;

        loop {
            tokio::select! {
                _ = connection_check_interval.tick() => {
                    let settings = self.app_state.get_settings().await;
                    let waiting_timeout_secs =
                        i64::from(settings.claude_code_timeout_minutes).saturating_mul(60);
                    if self
                        .claude_tracker
                        .prune_expired(chrono::Utc::now().timestamp(), waiting_timeout_secs)
                    {
                        self.sync_claude_status().await;
                        if settings.claude_code_enabled && self.claude_override_active {
                            if let Some(session) = self.claude_tracker.active_session().cloned() {
                                let data = build_claude_code_presence(
                                    &session,
                                    self.claude_tracker.active_count(),
                                    settings.claude_code_show_chat_title,
                                );
                                if !self.is_idle {
                                    self.schedule_or_apply_presence(
                                        DEFAULT_CLIENT_ID,
                                        data,
                                        &mut rate_limit_task,
                                        &flush_tx,
                                    )
                                    .await;
                                } else {
                                    self.pre_idle_data = Some(data);
                                }
                            } else {
                                self.claude_override_active = false;
                                info!("Claude Code ended → restoring window presence");
                                if !self.is_idle {
                                    self.restore_window_presence(
                                        &mut settle_task,
                                        &mut rate_limit_task,
                                        &flush_tx,
                                    )
                                    .await;
                                }
                            }
                        }
                    }

                    self.sync_state().await;
                }
                Some(event) = rx.recv() => {
                    match event {
                        EngineEvent::WindowChanged { process_name, exe_path, aliases, window_title, is_prioritized, foreground_app } => {
                            let snapshot = WindowSnapshot {
                                process_name,
                                exe_path,
                                aliases,
                                window_title,
                                is_prioritized,
                                foreground_app,
                            };
                            self.last_window_snapshot = Some(snapshot.clone());

                            let settings = self.app_state.get_settings().await;
                            let rules = self.app_state.get_app_rules().await;
                            let window_label =
                                resolve_window_label(&snapshot, &rules, &self.app_registry);

                            if let Some(session) = self.shown_claude_session(&settings).await {
                                // Claude Code outranks this app in the Apps list.
                                if !self.claude_override_active {
                                    let project_or_session = session
                                        .project_name
                                        .as_deref()
                                        .unwrap_or(session.session_id.as_str());
                                    info!(
                                        "Claude Code took precedence over {} (project: {})",
                                        window_label, project_or_session
                                    );
                                    self.claude_override_active = true;
                                    if let Some(task) = settle_task.take() {
                                        task.abort();
                                    }
                                    let claude_data = build_claude_code_presence(
                                        &session,
                                        self.claude_tracker.active_count(),
                                        settings.claude_code_show_chat_title,
                                    );
                                    if !self.is_idle {
                                        self.schedule_or_apply_presence(
                                            DEFAULT_CLIENT_ID,
                                            claude_data,
                                            &mut rate_limit_task,
                                            &flush_tx,
                                        )
                                        .await;
                                    } else {
                                        self.pre_idle_data = Some(claude_data);
                                    }
                                } else {
                                    debug!(
                                        "Window {} ignored (Claude Code has higher priority)",
                                        window_label
                                    );
                                }
                                continue;
                            }
                            // This app outranks Claude Code (or no session): show the window.
                            self.claude_override_active = false;

                            match self.resolve_window_presence(&snapshot, &rules).await {
                                None => {
                                    info!("Clearing presence (no active app)");
                                    if let Some(task) = settle_task.take() {
                                        task.abort();
                                    }
                                    self.clear_pending(&mut rate_limit_task);
                                    self.app_state.discord_handle.send(EngineCommand::ClearActivity);
                                }
                                Some((client_id, new_data)) => {
                                    if !self.is_idle {
                                        if let Some(task) = settle_task.take() {
                                            task.abort();
                                        }

                                        let delay = settings.settle_delay_seconds;
                                        if delay > 0 {
                                            debug!(
                                                "Settling {} ({}s)",
                                                presence_app_label(&new_data),
                                                delay
                                            );
                                        }
                                        let tx = settle_tx.clone();
                                        let data_clone = new_data.clone();

                                        settle_task = Some(tauri::async_runtime::spawn(async move {
                                            sleep(Duration::from_secs(delay)).await;
                                            let _ = tx.send((client_id, data_clone)).await;
                                        }));
                                    } else {
                                        self.pre_idle_data = Some(new_data);
                                    }
                                }
                            }
                        }
                        EngineEvent::ClaudeCodeEvent(payload) => {
                            let now = chrono::Utc::now().timestamp();
                            if !self.claude_tracker.apply_event(&payload, now) {
                                continue;
                            }

                            let sid_key = if payload.session_id.trim().is_empty() {
                                "default".to_string()
                            } else {
                                payload.session_id.trim().to_string()
                            };
                            if let Some(session_meta_target) = self.claude_tracker.get_session(&sid_key) {
                                let transcript_path = session_meta_target.transcript_path.clone();
                                let cwd = session_meta_target.cwd.clone();
                                if transcript_path.is_some() || cwd.is_some() {
                                    if let Ok(meta) = tokio::task::spawn_blocking(move || {
                                        crate::services::claude_code_metadata::read_session_metadata(
                                            transcript_path.as_deref(),
                                            cwd.as_deref(),
                                        )
                                    })
                                    .await
                                    {
                                        self.claude_tracker.apply_metadata(&sid_key, &meta);
                                    }
                                }
                            }

                            self.sync_claude_status().await;

                            let settings = self.app_state.get_settings().await;
                            if !settings.claude_code_enabled {
                                continue;
                            }

                            if let Some(session) = self.shown_claude_session(&settings).await {
                                if let Some(task) = settle_task.take() {
                                    task.abort();
                                }
                                let _ = self
                                    .app_state
                                    .app_handle
                                    .emit("priority-info", &PriorityInfo::default());
                                if !self.claude_override_active {
                                    info!("Claude Code took precedence ({})", session.details_text());
                                }
                                self.claude_override_active = true;
                                let claude_data = build_claude_code_presence(
                                    &session,
                                    self.claude_tracker.active_count(),
                                    settings.claude_code_show_chat_title,
                                );
                                if !self.is_idle {
                                    self.schedule_or_apply_presence(
                                        DEFAULT_CLIENT_ID,
                                        claude_data,
                                        &mut rate_limit_task,
                                        &flush_tx,
                                    )
                                    .await;
                                } else {
                                    self.pre_idle_data = Some(claude_data);
                                }
                            } else if self.claude_override_active {
                                self.claude_override_active = false;
                                info!("Claude Code ended → restoring window presence");
                                if !self.is_idle {
                                    self.restore_window_presence(
                                        &mut settle_task,
                                        &mut rate_limit_task,
                                        &flush_tx,
                                    )
                                    .await;
                                }
                            }
                        }
                        EngineEvent::SelectClaudeCodeSession(session_id) => {
                            debug!(
                                "Claude Code session pin updated: {}",
                                session_id.as_deref().unwrap_or("auto")
                            );
                            if !self.claude_tracker.select_session(session_id) {
                                continue;
                            }
                            self.sync_claude_status().await;

                            let settings = self.app_state.get_settings().await;
                            if settings.claude_code_enabled {
                                if let Some(session) = self.shown_claude_session(&settings).await {
                                    if let Some(task) = settle_task.take() {
                                        task.abort();
                                    }
                                    let _ = self
                                        .app_state
                                        .app_handle
                                        .emit("priority-info", &PriorityInfo::default());
                                    if !self.claude_override_active {
                                        info!(
                                            "Claude Code took precedence ({})",
                                            session.details_text()
                                        );
                                    }
                                    self.claude_override_active = true;
                                    let claude_data = build_claude_code_presence(
                                        &session,
                                        self.claude_tracker.active_count(),
                                        settings.claude_code_show_chat_title,
                                    );
                                    if !self.is_idle {
                                        self.schedule_or_apply_presence(
                                            DEFAULT_CLIENT_ID,
                                            claude_data,
                                            &mut rate_limit_task,
                                            &flush_tx,
                                        )
                                        .await;
                                    } else {
                                        self.pre_idle_data = Some(claude_data);
                                    }
                                }
                            }
                        }
                        EngineEvent::ClaudeCodeDisconnected => {
                            self.claude_tracker.clear();
                            self.sync_claude_status().await;
                            if self.claude_override_active {
                                self.claude_override_active = false;
                                info!("Claude Code ended → restoring window presence");
                                if !self.is_idle {
                                    self.restore_window_presence(
                                        &mut settle_task,
                                        &mut rate_limit_task,
                                        &flush_tx,
                                    )
                                    .await;
                                }
                            }
                        }
                        EngineEvent::ClaudeCodeConfigChanged => {
                            let settings = self.app_state.get_settings().await;
                            if settings.claude_code_enabled {
                                if let Some(session) = self.shown_claude_session(&settings).await {
                                    if let Some(task) = settle_task.take() {
                                        task.abort();
                                    }
                                    let _ = self
                                        .app_state
                                        .app_handle
                                        .emit("priority-info", &PriorityInfo::default());
                                    if !self.claude_override_active {
                                        info!(
                                            "Claude Code took precedence ({})",
                                            session.details_text()
                                        );
                                    }
                                    self.claude_override_active = true;
                                    let claude_data = build_claude_code_presence(
                                        &session,
                                        self.claude_tracker.active_count(),
                                        settings.claude_code_show_chat_title,
                                    );
                                    if !self.is_idle {
                                        self.schedule_or_apply_presence(
                                            DEFAULT_CLIENT_ID,
                                            claude_data,
                                            &mut rate_limit_task,
                                            &flush_tx,
                                        )
                                        .await;
                                    } else {
                                        self.pre_idle_data = Some(claude_data);
                                    }
                                    continue;
                                }
                            }
                            if self.claude_override_active {
                                self.claude_override_active = false;
                                info!("Claude Code ended → restoring window presence");
                                if !self.is_idle {
                                    self.restore_window_presence(
                                        &mut settle_task,
                                        &mut rate_limit_task,
                                        &flush_tx,
                                    )
                                    .await;
                                }
                            }
                        }
                        EngineEvent::ClearPresence => {
                            if let Some(task) = settle_task.take() {
                                task.abort();
                            }
                            self.clear_pending(&mut rate_limit_task);
                            self.app_state.discord_handle.send(EngineCommand::ClearActivity);
                        }
                        EngineEvent::IdleChanged { idle, idle_minutes } => {
                            let settings = self.app_state.get_settings().await;
                            if !settings.idle_enabled {
                                continue;
                            }

                            if idle && idle_minutes >= settings.idle_threshold_minutes {
                                if !self.is_idle {
                                    info!(
                                        "Idle after {}m → \"{}\"",
                                        idle_minutes, settings.idle_message
                                    );
                                    if let Some(task) = settle_task.take() {
                                        task.abort();
                                    }
                                    self.pre_idle_data = Some(self.current_data.clone());
                                    self.is_idle = true;

                                    let idle_data = idle_presence(&settings.idle_message);
                                    self.schedule_or_apply_presence(
                                        DEFAULT_CLIENT_ID,
                                        idle_data,
                                        &mut rate_limit_task,
                                        &flush_tx,
                                    )
                                    .await;
                                }
                            } else if !idle && self.is_idle {
                                info!("Activity resumed → restoring presence");
                                self.is_idle = false;
                                if settings.claude_code_enabled {
                                    if let Some(session) = self.shown_claude_session(&settings).await {
                                        self.pre_idle_data = None;
                                        self.claude_override_active = true;
                                        let claude_data = build_claude_code_presence(
                                            &session,
                                            self.claude_tracker.active_count(),
                                            settings.claude_code_show_chat_title,
                                        );
                                        self.schedule_or_apply_presence(
                                            DEFAULT_CLIENT_ID,
                                            claude_data,
                                            &mut rate_limit_task,
                                            &flush_tx,
                                        )
                                        .await;
                                        continue;
                                    }
                                }

                                if self.claude_override_active {
                                    self.claude_override_active = false;
                                    self.pre_idle_data = None;
                                    self.restore_window_presence(
                                        &mut settle_task,
                                        &mut rate_limit_task,
                                        &flush_tx,
                                    )
                                    .await;
                                } else if let Some(pre_idle) = self.pre_idle_data.take() {
                                    // Update timestamp to now to reset timer
                                    let mut restored = pre_idle;
                                    restored.timestamp = chrono::Utc::now().timestamp();
                                    self.schedule_or_apply_presence(
                                        self.current_client_id,
                                        restored,
                                        &mut rate_limit_task,
                                        &flush_tx,
                                    )
                                    .await;
                                }
                            }
                        }
                        EngineEvent::Shutdown => {
                            info!("Engine received shutdown signal");
                            self.clear_pending(&mut rate_limit_task);
                            self.app_state.discord_handle.send(EngineCommand::Disconnect);
                            break;
                        }
                    }
                }

                Some((client_id, settled_data)) = settle_rx.recv() => {
                    // This data has survived the anti-flicker delay. Ignore if idle or
                    // if Claude Code took precedence while the settle timer ran.
                    if self.is_idle {
                        continue;
                    }
                    let settings = self.app_state.get_settings().await;
                    if self.shown_claude_session(&settings).await.is_some() {
                        continue;
                    }
                    self.schedule_or_apply_presence(
                        client_id,
                        settled_data,
                        &mut rate_limit_task,
                        &flush_tx,
                    )
                    .await;
                }

                Some(()) = flush_rx.recv() => {
                    rate_limit_task = None;
                    if let Some((client_id, pending_data)) = self.pending_presence.take() {
                        let settings = self.app_state.get_settings().await;
                        self.commit_presence(client_id, pending_data, settings.global_enabled).await;
                    }
                }
            }
        }
    }

    async fn resolve_window_presence(
        &mut self,
        snapshot: &WindowSnapshot,
        rules: &[AppRule],
    ) -> Option<(u64, PresenceData)> {
        // Disambiguate browser forks (e.g. Helium running as `chrome.exe`)
        // before matching rules or the catalog.
        let process_name = resolve_window_effective_name(snapshot, rules, &self.app_registry);

        let priority_info = PriorityInfo {
            active: snapshot.is_prioritized,
            prioritized_app: if snapshot.is_prioritized {
                Some(process_name.clone())
            } else {
                None
            },
            foreground_app: snapshot.foreground_app.clone(),
        };
        let _ = self
            .app_state
            .app_handle
            .emit("priority-info", &priority_info);

        if process_name.is_empty() {
            return None;
        }

        let matched_rule = rules.iter().find(|r| rule_matches(r, &process_name));

        let app_info = self.app_registry.find_app(&process_name).cloned();
        let client_id = app_info
            .as_ref()
            .map(|app| app.client_id.parse::<u64>().unwrap_or(DEFAULT_CLIENT_ID))
            .unwrap_or(DEFAULT_CLIENT_ID);
        let default_app_name = app_info.as_ref().map(|app| app.name.clone());

        let (discord_name, discord_icon) = self.resolve_discord_details(client_id).await;

        // Editor/IDE windows get rich presence (file, workspace, language icon)
        // unless their Apps rule carries custom text.
        let use_rich_ide = matched_rule.map_or(true, |rule| {
            rule_has_default_text(&rule.display_name, &rule.details, &rule.state)
        });
        let (ide_ctx, editor_ctx) =
            extract_editor_context(app_info.as_ref(), &snapshot.window_title);

        let new_data = if use_rich_ide {
            if let Some(ide_ctx) = ide_ctx {
                let workspace_key = format!(
                    "{:?}:{}",
                    ide_ctx.ide,
                    ide_ctx.workspace_name.as_deref().unwrap_or("")
                );
                let timestamp = self.track_ide_workspace_timestamp(workspace_key);
                crate::services::activity_builder::ActivityBuilder::build(&ide_ctx, timestamp)
            } else if let (Some(app), Some(editor_ctx)) = (app_info.as_ref(), editor_ctx) {
                let workspace_key = format!(
                    "{}:{}",
                    app.id,
                    editor_ctx.workspace_name.as_deref().unwrap_or("")
                );
                let timestamp = self.track_ide_workspace_timestamp(workspace_key);
                let custom_large_image = matched_rule.and_then(|rule| {
                    let image = rule.large_image.trim();
                    (!image.is_empty()
                        && !image.eq_ignore_ascii_case("auto")
                        && !image.eq_ignore_ascii_case("default"))
                    .then_some(image)
                });
                build_editor_presence(
                    app,
                    discord_name,
                    discord_icon,
                    custom_large_image,
                    &editor_ctx,
                    timestamp,
                )
            } else {
                self.current_ide_workspace = None;
                self.ide_workspace_start_time = None;
                match matched_rule {
                    Some(rule) => build_presence_from_rule(
                        &self.app_registry,
                        rule,
                        &process_name,
                        &snapshot.window_title,
                        discord_icon,
                        client_id == DEFAULT_CLIENT_ID,
                    ),
                    None => build_fallback_presence(
                        &self.app_registry,
                        &process_name,
                        &snapshot.window_title,
                        discord_name,
                        discord_icon,
                        default_app_name,
                    ),
                }
            }
        } else {
            self.current_ide_workspace = None;
            self.ide_workspace_start_time = None;
            match matched_rule {
                Some(rule) => build_presence_from_rule(
                    &self.app_registry,
                    rule,
                    &process_name,
                    &snapshot.window_title,
                    discord_icon,
                    client_id == DEFAULT_CLIENT_ID,
                ),
                None => build_fallback_presence(
                    &self.app_registry,
                    &process_name,
                    &snapshot.window_title,
                    discord_name,
                    discord_icon,
                    default_app_name,
                ),
            }
        };

        Some((client_id, new_data))
    }

    fn track_ide_workspace_timestamp(&mut self, workspace_key: String) -> i64 {
        if self.current_ide_workspace.as_ref() != Some(&workspace_key) {
            self.current_ide_workspace = Some(workspace_key);
            self.ide_workspace_start_time = Some(chrono::Utc::now().timestamp());
        }
        self.ide_workspace_start_time
            .unwrap_or_else(|| chrono::Utc::now().timestamp())
    }

    /// The Claude Code session to show instead of the window presence: the active
    /// session, when the integration is on and Claude Code's entry in the Apps
    /// list outranks the app window detection selected.
    async fn shown_claude_session(&self, settings: &Settings) -> Option<ClaudeSession> {
        if !settings.claude_code_enabled {
            return None;
        }
        let session = self.claude_tracker.active_session()?;
        let rules = self.app_state.get_app_rules().await;
        let window_app = self
            .last_window_snapshot
            .as_ref()
            .map(|snapshot| resolve_window_effective_name(snapshot, &rules, &self.app_registry));
        let ranked: Vec<(&str, bool)> = rules
            .iter()
            .map(|r| (r.process_name.as_str(), r.enabled))
            .collect();
        claude_code_takes_precedence(&ranked, window_app.as_deref()).then(|| session.clone())
    }

    async fn restore_window_presence(
        &mut self,
        settle_task: &mut Option<tauri::async_runtime::JoinHandle<()>>,
        rate_limit_task: &mut Option<tauri::async_runtime::JoinHandle<()>>,
        flush_tx: &mpsc::Sender<()>,
    ) {
        if let Some(task) = settle_task.take() {
            task.abort();
        }
        let Some(snapshot) = self.last_window_snapshot.clone() else {
            self.schedule_or_apply_presence(
                DEFAULT_CLIENT_ID,
                PresenceData::default(),
                rate_limit_task,
                flush_tx,
            )
            .await;
            return;
        };

        let rules = self.app_state.get_app_rules().await;
        match self.resolve_window_presence(&snapshot, &rules).await {
            Some((client_id, data)) => {
                self.schedule_or_apply_presence(client_id, data, rate_limit_task, flush_tx)
                    .await;
            }
            None => {
                self.clear_pending(rate_limit_task);
                self.app_state
                    .discord_handle
                    .send(EngineCommand::ClearActivity);
            }
        }
    }

    async fn sync_claude_status(&self) {
        let active_count = self.claude_tracker.active_count();
        let active_session = self.claude_tracker.active_session();
        let active_project = active_session.and_then(|s| s.project_name.clone());
        let active_activity = active_session.map(|s| s.state_text_rich(true));
        let selected_session_id = active_session.map(|s| s.session_id.clone());
        let pinned_session_id = self
            .claude_tracker
            .pinned_session_id()
            .map(ToString::to_string);
        let sessions: Vec<ClaudeSessionSummary> = self
            .claude_tracker
            .sessions_ordered()
            .into_iter()
            .map(|s| ClaudeSessionSummary {
                session_id: s.session_id.clone(),
                project_name: s.project_name.clone(),
                chat_title: s.chat_title.clone(),
                activity: s.action_text(),
                is_waiting: s.activity.is_waiting(),
                model: s.model.clone(),
                git_branch: s.git_branch.clone(),
                started_at: s.started_at,
                last_event_at: s.last_event_at,
            })
            .collect();

        if self
            .app_state
            .set_claude_session_snapshot(
                active_count,
                active_project,
                active_activity,
                sessions,
                selected_session_id,
                pinned_session_id,
            )
            .await
        {
            let status = self.app_state.get_claude_code_status().await;
            let _ = self
                .app_state
                .app_handle
                .emit("claude-code-status-changed", &status);
        }
    }

    fn clear_pending(
        &mut self,
        rate_limit_task: &mut Option<tauri::async_runtime::JoinHandle<()>>,
    ) {
        self.pending_presence = None;
        if let Some(task) = rate_limit_task.take() {
            task.abort();
        }
    }

    async fn sync_state(&mut self) {
        let is_connected = self.app_state.discord_handle.is_connected();

        // Use AppState connection status helper
        let (conn_changed, state_changed, new_state) =
            self.app_state.update_connection_status(is_connected).await;

        if conn_changed {
            if is_connected {
                let settings = self.app_state.get_settings().await;
                if settings.global_enabled {
                    info!(
                        "Discord reconnected → resending {}",
                        presence_app_label(&self.current_data)
                    );
                    self.app_state
                        .discord_handle
                        .send(EngineCommand::SetActivity {
                            client_id: self.current_client_id,
                            data: self.current_data.clone(),
                        });
                }
            }
            let conn_payload = self.app_state.get_connection_status().await;
            let _ = self
                .app_state
                .app_handle
                .emit("connection-changed", &conn_payload);
        }

        if state_changed {
            self.current_state = new_state;
            let _ = self.app_state.app_handle.emit("state-changed", &new_state);
        }

        // Sync presence data and source
        let presence_changed = self
            .app_state
            .update_presence_data(self.current_client_id, self.current_data.clone())
            .await;
        if presence_changed {
            let _ = self
                .app_state
                .app_handle
                .emit("presence-updated", &self.current_data);
        }

        crate::interface::tray::refresh_tray_from_state(&self.app_state).await;
    }

    async fn resolve_discord_details(
        &mut self,
        client_id: u64,
    ) -> (Option<String>, Option<String>) {
        if client_id == DEFAULT_CLIENT_ID {
            return (None, None);
        }

        if !self.client_cache.contains_key(&client_id) {
            if let Some((name, icon_hash)) = fetch_discord_app_details(client_id).await {
                self.client_cache.insert(client_id, (name, icon_hash));
            } else {
                self.client_cache
                    .insert(client_id, (String::new(), String::new()));
            }
        }

        let cached_details = self.client_cache.get(&client_id);
        let mut discord_icon = None;
        let mut discord_name = None;

        if let Some((name, icon_hash)) = cached_details {
            if !name.is_empty() {
                discord_name = Some(name.clone());
            }
            if !icon_hash.is_empty() {
                discord_icon = Some(format!(
                    "https://cdn.discordapp.com/app-icons/{}/{}.png?size=128",
                    client_id, icon_hash
                ));
            }
        }

        (discord_name, discord_icon)
    }

    /// Makes a presence match what Discord will show: the large image becomes a
    /// URL Discord can render and `activity_name` the title Discord prints.
    fn prepare_for_discord(&self, client_id: u64, data: &mut PresenceData) {
        let discord_name = self
            .client_cache
            .get(&client_id)
            .map(|(name, _)| name.as_str());
        prepare_for_discord(&self.app_registry, client_id, discord_name, data);
    }

    /// Applies `new_data` immediately when outside Discord's 15-second rate-limit
    /// window, or coalesces it into a single pending slot flushed once the window
    /// opens so rapid events never block the engine loop.
    async fn schedule_or_apply_presence(
        &mut self,
        client_id: u64,
        mut new_data: PresenceData,
        rate_limit_task: &mut Option<tauri::async_runtime::JoinHandle<()>>,
        flush_tx: &mpsc::Sender<()>,
    ) {
        self.prepare_for_discord(client_id, &mut new_data);

        if self.pending_presence.is_none()
            && client_id == self.current_client_id
            && same_visible_content(&new_data, &self.current_data)
        {
            return;
        }

        let settings = self.app_state.get_settings().await;
        // While sharing is off the presence is still tracked and shown in the
        // preview; `AppState::update_settings` sends it when sharing turns on.
        let sharing = settings.global_enabled;

        if let (true, Some(last_time)) = (sharing, self.last_update_time) {
            let elapsed = last_time.elapsed();
            let debounce = Duration::from_secs(settings.debounce_seconds.max(15)); // Discord requires ~15s
            if elapsed < debounce {
                let wait_time = debounce - elapsed;
                debug!(
                    "Rate-limited: queuing {} in {:.1}s ({})",
                    presence_app_label(&new_data),
                    wait_time.as_secs_f64(),
                    format_presence_summary(&new_data)
                );
                self.pending_presence = Some((client_id, new_data));
                if rate_limit_task.is_none() {
                    let tx = flush_tx.clone();
                    *rate_limit_task = Some(tauri::async_runtime::spawn(async move {
                        sleep(wait_time).await;
                        let _ = tx.send(()).await;
                    }));
                }
                return;
            }
        }

        self.clear_pending(rate_limit_task);
        self.commit_presence(client_id, new_data, sharing).await;
    }

    async fn commit_presence(&mut self, client_id: u64, mut new_data: PresenceData, sharing: bool) {
        if client_id == self.current_client_id
            && same_visible_content(&new_data, &self.current_data)
        {
            return;
        }

        new_data.timestamp = resolve_timestamp(
            self.current_client_id,
            self.current_data.timestamp,
            client_id,
            new_data.timestamp,
            chrono::Utc::now().timestamp(),
        );

        let prev_label = presence_app_label(&self.current_data).to_string();
        let next_label = presence_app_label(&new_data).to_string();
        let summary = format_presence_summary(&new_data);
        let suffix = if sharing { "" } else { " (sharing paused)" };

        if !self.has_committed_presence {
            info!("Active: {} | {}{}", next_label, summary, suffix);
        } else if prev_label != next_label {
            info!(
                "Switch: {} → {} | {}{}",
                prev_label, next_label, summary, suffix
            );
        } else {
            info!("Update: {} | {}{}", next_label, summary, suffix);
        }
        self.has_committed_presence = true;

        self.current_data = new_data;
        self.current_client_id = client_id;

        if sharing {
            self.last_update_time = Some(Instant::now());
            self.app_state
                .discord_handle
                .send(EngineCommand::SetActivity {
                    client_id: self.current_client_id,
                    data: self.current_data.clone(),
                });
        }

        // Sync to shared state and emit events
        self.sync_state().await;
    }
}

/// Chooses the timestamp for an outgoing presence: honors explicit non-zero
/// session/workspace timestamps (e.g. Claude Code sessions, IDEs, idle),
/// otherwise preserves the current timer on the same client ID.
fn resolve_timestamp(
    current_client_id: u64,
    current_timestamp: i64,
    next_client_id: u64,
    requested_timestamp: i64,
    now: i64,
) -> i64 {
    if requested_timestamp > 0 {
        requested_timestamp
    } else if next_client_id == current_client_id && current_timestamp > 0 {
        current_timestamp
    } else {
        now
    }
}

/// Builds the presence payload for an active Claude Code session.
pub fn build_claude_code_presence(
    session: &ClaudeSession,
    active_count: usize,
    show_chat_title: bool,
) -> PresenceData {
    PresenceData {
        activity_name: String::new(),
        details: session.details_text(),
        state: session.state_text_rich(show_chat_title),
        large_image: CLAUDE_CODE_CATALOG_ID.to_string(),
        large_text: session.large_text(active_count),
        small_image: String::new(),
        small_text: String::new(),
        timestamp: session.started_at,
    }
}

/// Normalizes a process or rule name for matching: trimmed, lowercase, without `.exe`.
pub fn normalize_process_name(name: &str) -> String {
    let lower = name.trim().to_lowercase();
    match lower.strip_suffix(".exe") {
        Some(stripped) => stripped.to_string(),
        None => lower,
    }
}

/// Whether an enabled rule targets the given process name.
pub fn rule_matches(rule: &AppRule, process_name: &str) -> bool {
    rule.enabled
        && normalize_process_name(&rule.process_name) == normalize_process_name(process_name)
}

/// Resolves a window snapshot to its effective app name, applying browser
/// fork disambiguation (e.g. Helium running as `chrome.exe`) before rule and
/// catalog alias lookup.
fn resolve_window_effective_name(
    snapshot: &WindowSnapshot,
    rules: &[AppRule],
    registry: &AppRegistry,
) -> String {
    if let Some(browser_id) = crate::domain::browser::disambiguate_browser(
        &snapshot.process_name,
        snapshot.exe_path.as_deref(),
        &snapshot.window_title,
    ) {
        return browser_id.to_string();
    }
    resolve_effective_name(&snapshot.process_name, &snapshot.aliases, rules, registry)
}

/// Picks the name the engine uses for a window: the first candidate
/// (`process_name`, then `aliases`) that matches an enabled rule, else the
/// first that matches the app catalog, else `process_name` unchanged.
pub fn resolve_effective_name(
    process_name: &str,
    aliases: &[String],
    rules: &[AppRule],
    registry: &AppRegistry,
) -> String {
    if aliases.is_empty() {
        return process_name.to_string();
    }

    let candidates: Vec<&str> = std::iter::once(process_name)
        .chain(aliases.iter().map(String::as_str))
        .filter(|c| !c.trim().is_empty())
        .collect();

    if let Some(c) = candidates
        .iter()
        .find(|c| rules.iter().any(|r| rule_matches(r, c)))
    {
        return c.to_string();
    }

    if let Some(c) = candidates.iter().find(|c| registry.find_app(c).is_some()) {
        return c.to_string();
    }

    process_name.to_string()
}

/// Extracts IDE/editor context from a window title, gated so non-editor
/// processes (such as browsers) are never misclassified as IDEs.
fn extract_editor_context(
    app: Option<&AppEntry>,
    window_title: &str,
) -> (Option<IdeContext>, Option<EditorTitleContext>) {
    if app.is_none_or(|a| a.category == "editor") {
        if let Some(ide_ctx) = crate::services::title_parser::TitleParser::parse(window_title) {
            return (Some(ide_ctx), None);
        }
    }
    if let Some(editor_app) = app.filter(|a| a.category == "editor") {
        return (
            None,
            Some(parse_editor_title(
                &editor_app.id,
                &editor_app.name,
                window_title,
            )),
        );
    }
    (None, None)
}

fn build_editor_presence(
    app: &AppEntry,
    discord_name: Option<String>,
    discord_icon: Option<String>,
    custom_large_image: Option<&str>,
    editor_ctx: &EditorTitleContext,
    timestamp: i64,
) -> PresenceData {
    let editor_name = discord_name.unwrap_or_else(|| app.name.clone());
    let large_image = custom_large_image
        .map(str::to_string)
        .or_else(|| app.icon_url.clone())
        .or(discord_icon)
        .unwrap_or_default();
    let (small_image, small_text) = editor_ctx.language_badge();

    PresenceData {
        activity_name: String::new(),
        details: ide_details(&editor_name, editor_ctx.workspace_name.as_deref()),
        state: ide_state(
            editor_ctx.file_name.as_deref(),
            editor_ctx.workspace_name.as_deref(),
            editor_ctx.language.map(|lang| lang.name),
        ),
        large_image,
        large_text: editor_name,
        small_image,
        small_text,
        timestamp,
    }
}

fn build_presence_from_rule(
    registry: &AppRegistry,
    rule: &crate::models::types::AppRule,
    process_name: &str,
    window_title: &str,
    discord_icon: Option<String>,
    shared_client: bool,
) -> PresenceData {
    let app = registry.find_app(process_name);
    let activity = match app {
        Some(app) => format_app_presence(&app.category, &app.name, window_title),
        None => format_app_presence("generic", &rule.display_name, window_title),
    };
    let (ide_ctx, editor_ctx) = extract_editor_context(app, window_title);
    let (editor_file, editor_lang) = if let Some(ctx) = ide_ctx {
        let lang = ctx.file_name.as_deref().and_then(detect_language);
        (ctx.file_name, lang)
    } else if let Some(ctx) = editor_ctx {
        (ctx.file_name, ctx.language)
    } else {
        (None, None)
    };
    let fallback_file = activity.strip_prefix("Editing ").unwrap_or(&activity);
    let file_name = editor_file.as_deref().unwrap_or(fallback_file);

    let rendered_details = rule
        .details
        .replace("{file}", file_name)
        .replace("{title}", window_title);
    let state = rule
        .state
        .replace("{file}", file_name)
        .replace("{title}", window_title);

    let image = rule.large_image.trim();
    let large_image = if image.is_empty() || image == "auto" || image == "default" {
        app.and_then(|app| app.icon_url.clone())
            .or(discord_icon)
            .unwrap_or_default()
    } else {
        // Resolved to a URL by `prepare_for_discord`.
        image.to_string()
    };

    let (small_image, small_text) = match editor_lang {
        Some(lang) => (language_icon_url(lang.icon), lang.name.to_string()),
        None => (String::new(), String::new()),
    };

    PresenceData {
        activity_name: String::new(),
        details: rule_details(&rule.display_name, &rendered_details, shared_client),
        state,
        large_image,
        large_text: rule.display_name.clone(),
        small_image,
        small_text,
        timestamp: 0,
    }
}

/// Presence for an app no rule matches: the catalog (or Discord) name as
/// `details`, the category's activity line as `state`.
fn build_fallback_presence(
    registry: &AppRegistry,
    process_name: &str,
    window_title: &str,
    discord_name: Option<String>,
    discord_icon: Option<String>,
    default_app_name: Option<String>,
) -> PresenceData {
    let app = registry.find_app(process_name);
    let app_name = discord_name
        .or(default_app_name)
        .unwrap_or_else(|| display_process_name(process_name));
    let category = app.map_or("generic", |app| app.category.as_str());

    PresenceData {
        activity_name: String::new(),
        details: app_name.clone(),
        state: format_app_presence(category, &app_name, window_title),
        large_image: app
            .and_then(|app| app.icon_url.clone())
            .or(discord_icon)
            .unwrap_or_default(),
        large_text: app_name,
        small_image: String::new(),
        small_text: String::new(),
        timestamp: 0,
    }
}

/// See [`PresenceEngine::prepare_for_discord`]. `discord_name` is the name Discord
/// reported for `client_id`'s application, if it was looked up.
fn prepare_for_discord(
    registry: &AppRegistry,
    client_id: u64,
    discord_name: Option<&str>,
    data: &mut PresenceData,
) {
    let shared_client = client_id == DEFAULT_CLIENT_ID;
    let lookup = |key: &str| {
        registry
            .find_by_id_or_executable(key)
            .and_then(|app| app.icon_url.clone())
    };
    data.large_image = resolve_large_image(&data.large_image, shared_client, &lookup);
    data.small_image = resolve_small_image(&data.small_image, shared_client, &lookup);
    data.activity_name = activity_name(shared_client, discord_name, &data.large_text);
}

/// Whether two presences show the same thing (everything but the timer start).
fn same_visible_content(a: &PresenceData, b: &PresenceData) -> bool {
    PresenceData {
        timestamp: 0,
        ..a.clone()
    } == PresenceData {
        timestamp: 0,
        ..b.clone()
    }
}

/// A process name as shown to people: trimmed, without a `.exe` suffix.
fn display_process_name(process_name: &str) -> String {
    let name = process_name.trim();
    let name = if name.len() > 4 && name[name.len() - 4..].eq_ignore_ascii_case(".exe") {
        &name[..name.len() - 4]
    } else {
        name
    };
    if name.is_empty() {
        "Unknown app".to_string()
    } else {
        name.to_string()
    }
}

/// Presence shown while the user is idle; `message` is the user's idle message.
fn idle_presence(message: &str) -> PresenceData {
    PresenceData {
        activity_name: String::new(),
        details: message.to_string(),
        state: "Idle".to_string(),
        large_image: String::new(),
        large_text: "Idle".to_string(),
        small_image: String::new(),
        small_text: String::new(),
        timestamp: chrono::Utc::now().timestamp(),
    }
}

fn presence_app_label(data: &PresenceData) -> &str {
    let details = data.details.trim();
    if details.starts_with("Claude Code") {
        return "Claude Code";
    }
    let large_text = data.large_text.trim();
    if !large_text.is_empty() {
        return large_text;
    }
    if !details.is_empty() {
        return details;
    }
    "Unknown"
}

fn format_presence_summary(data: &PresenceData) -> String {
    let details = data.details.trim();
    let state = data.state.trim();
    match (details.is_empty(), state.is_empty()) {
        (false, false) => format!("\"{details}\" · \"{state}\""),
        (false, true) => format!("\"{details}\""),
        (true, false) => format!("\"{state}\""),
        (true, true) => "No details".to_string(),
    }
}

fn resolve_window_label(
    snapshot: &WindowSnapshot,
    rules: &[AppRule],
    registry: &AppRegistry,
) -> String {
    let effective = resolve_window_effective_name(snapshot, rules, registry);
    if effective.is_empty() {
        return "Unknown".to_string();
    }
    if let Some(rule) = rules.iter().find(|r| rule_matches(r, &effective)) {
        if !rule.display_name.trim().is_empty() {
            return rule.display_name.trim().to_string();
        }
    }
    if let Some(app) = registry.find_app(&effective) {
        return app.name.clone();
    }
    display_process_name(&effective)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rule(process_name: &str, enabled: bool) -> AppRule {
        let mut r = AppRule::new(process_name, process_name, "", "", "auto");
        r.enabled = enabled;
        r
    }

    #[test]
    fn test_effective_name_without_aliases_is_unchanged() {
        let registry = AppRegistry::new();
        assert_eq!(
            resolve_effective_name("electron", &[], &[], &registry),
            "electron"
        );
    }

    #[test]
    fn test_effective_name_prefers_rule_match_on_alias() {
        let registry = AppRegistry::new();
        let rules = vec![rule("code.exe", true)];
        let aliases = vec!["code".to_string()];
        assert_eq!(
            resolve_effective_name("electron", &aliases, &rules, &registry),
            "code"
        );
    }

    #[test]
    fn test_effective_name_ignores_disabled_rules() {
        let registry = AppRegistry::new();
        let rules = vec![rule("my-tool", false)];
        let aliases = vec!["my-tool".to_string()];
        assert_eq!(
            resolve_effective_name("python3", &aliases, &rules, &registry),
            "python3"
        );
    }

    #[test]
    fn test_effective_name_falls_back_to_catalog_alias() {
        let registry = AppRegistry::new();
        let aliases = vec!["discord".to_string()];
        assert_eq!(
            resolve_effective_name("electron", &aliases, &[], &registry),
            "discord"
        );
    }

    #[test]
    fn test_effective_name_primary_wins_when_it_matches() {
        let registry = AppRegistry::new();
        let aliases = vec!["discord".to_string()];
        assert_eq!(
            resolve_effective_name("firefox", &aliases, &[], &registry),
            "firefox"
        );
    }

    #[test]
    fn test_effective_name_no_match_keeps_primary() {
        let registry = AppRegistry::new();
        let aliases = vec!["some-unknown-class".to_string()];
        assert_eq!(
            resolve_effective_name("unknown-bin", &aliases, &[], &registry),
            "unknown-bin"
        );
    }

    #[test]
    fn test_window_effective_name_disambiguates_helium_and_tor_and_operagx() {
        let registry = AppRegistry::new();
        let helium = WindowSnapshot {
            process_name: "chrome.exe".to_string(),
            exe_path: Some(
                r"C:\Users\alice\AppData\Local\imput\Helium\Application\chrome.exe".to_string(),
            ),
            aliases: Vec::new(),
            window_title: "GitHub - Helium".to_string(),
            is_prioritized: false,
            foreground_app: None,
        };
        let effective = resolve_window_effective_name(&helium, &[], &registry);
        assert_eq!(effective, "helium");
        let app = registry.find_app(&effective).unwrap();
        assert_eq!(app.name, "Helium Browser");
        let data = prepared(build_fallback_presence(
            &registry,
            &effective,
            &helium.window_title,
            None,
            None,
            Some(app.name.clone()),
        ));
        assert_eq!(data.details, "Helium Browser");
        assert_eq!(data.state, "Browsing GitHub");
        assert!(
            data.large_image.ends_with("/helium.png"),
            "{}",
            data.large_image
        );

        let tor = WindowSnapshot {
            process_name: "firefox.exe".to_string(),
            exe_path: Some(r"C:\Users\alice\Desktop\Tor Browser\Browser\firefox.exe".to_string()),
            aliases: Vec::new(),
            window_title: "DuckDuckGo — Tor Browser".to_string(),
            is_prioritized: false,
            foreground_app: None,
        };
        assert_eq!(
            resolve_window_effective_name(&tor, &[], &registry),
            "torbrowser"
        );

        let operagx = WindowSnapshot {
            process_name: "opera.exe".to_string(),
            exe_path: Some(r"C:\Users\alice\AppData\Local\Programs\Opera GX\opera.exe".to_string()),
            aliases: Vec::new(),
            window_title: "YouTube - Opera".to_string(),
            is_prioritized: false,
            foreground_app: None,
        };
        assert_eq!(
            resolve_window_effective_name(&operagx, &[], &registry),
            "operagx"
        );
    }

    #[test]
    fn test_rule_matches_is_exe_and_case_insensitive() {
        assert!(rule_matches(&rule("Code.EXE", true), "code"));
        assert!(rule_matches(&rule("code", true), "code.exe"));
        assert!(!rule_matches(&rule("code", false), "code"));
    }

    #[test]
    fn test_same_visible_content_ignores_only_the_timestamp() {
        let a = PresenceData {
            timestamp: 1,
            ..PresenceData::default()
        };
        let b = PresenceData {
            timestamp: 2,
            ..a.clone()
        };
        assert!(same_visible_content(&a, &b));

        let c = PresenceData {
            large_text: "Other".to_string(),
            ..a.clone()
        };
        assert!(!same_visible_content(&a, &c));
    }

    fn prepared(mut data: PresenceData) -> PresenceData {
        prepare_for_discord(&AppRegistry::new(), DEFAULT_CLIENT_ID, None, &mut data);
        data
    }

    #[test]
    fn test_catalog_app_layout() {
        let registry = AppRegistry::new();
        let data = prepared(build_fallback_presence(
            &registry,
            "firefox",
            "GitHub - Mozilla Firefox",
            None,
            None,
            Some("Firefox".to_string()),
        ));
        assert_eq!(data.activity_name, "Better Rich Presence");
        assert_eq!(data.details, "Firefox");
        assert_eq!(data.state, "Browsing GitHub");
        assert_eq!(data.large_text, "Firefox");
        assert!(
            data.large_image.starts_with("https://cdn.jsdelivr.net/")
                && data.large_image.ends_with("/firefox.png"),
            "{}",
            data.large_image
        );
    }

    #[test]
    fn test_unknown_app_layout() {
        let registry = AppRegistry::new();
        let data = prepared(build_fallback_presence(
            &registry,
            "my-tool.exe",
            "Report Q3",
            None,
            None,
            None,
        ));
        assert_eq!(data.details, "my-tool");
        assert_eq!(data.state, "Report Q3");
        assert_eq!(data.large_image, crate::domain::presence::LOGO_IMAGE_URL);

        let untitled = build_fallback_presence(&registry, "my-tool", "", None, None, None);
        assert_eq!(untitled.state, "Active");
    }

    #[test]
    fn test_ide_presence_resolves_catalog_icon() {
        let ctx = crate::services::title_parser::TitleParser::parse(
            "main.rs - my-project - Visual Studio Code",
        )
        .unwrap();
        let data = prepared(crate::services::activity_builder::ActivityBuilder::build(
            &ctx, 0,
        ));
        assert_eq!(data.details, "Visual Studio Code · my-project");
        assert_eq!(data.state, "Editing main.rs (Rust)");
        assert_eq!(data.large_text, "Visual Studio Code");
        assert!(
            data.large_image.starts_with("https://cdn.jsdelivr.net/")
                && data.large_image.ends_with("/visualstudiocode.png"),
            "{}",
            data.large_image
        );
        assert_eq!(
            data.small_image,
            "https://cdn.jsdelivr.net/gh/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord@main/public/assets/icons/rust.png"
        );
        assert_eq!(data.small_text, "Rust");

        let ctx =
            crate::services::title_parser::TitleParser::parse("app.ts - ui - Cursor").unwrap();
        let data = prepared(crate::services::activity_builder::ActivityBuilder::build(
            &ctx, 0,
        ));
        assert_eq!(data.details, "Cursor · ui");
        assert_eq!(data.state, "Editing app.ts (TypeScript)");
        assert!(
            data.large_image.ends_with("/cursor.png"),
            "{}",
            data.large_image
        );
        assert!(
            data.small_image.starts_with("https://cdn.jsdelivr.net/")
                && data.small_image.ends_with("/typescript.png"),
            "{}",
            data.small_image
        );
        assert_eq!(data.small_text, "TypeScript");

        let ctx = crate::services::title_parser::TitleParser::parse("a.rs - w - VSCodium").unwrap();
        let data = prepared(crate::services::activity_builder::ActivityBuilder::build(
            &ctx, 0,
        ));
        assert!(
            data.large_image.starts_with("https://cdn.jsdelivr.net/")
                && data.large_image.ends_with("/vscodium.png"),
            "{}",
            data.large_image
        );
        assert!(
            data.small_image.starts_with("https://cdn.jsdelivr.net/")
                && data.small_image.ends_with("/rust.png"),
            "{}",
            data.small_image
        );

        let ctx = crate::services::title_parser::TitleParser::parse("test.unknown - ui - Cursor")
            .unwrap();
        let data = prepared(crate::services::activity_builder::ActivityBuilder::build(
            &ctx, 0,
        ));
        assert_eq!(data.small_image, "");
        assert_eq!(data.small_text, "");
    }

    #[test]
    fn test_rule_presence_keeps_user_text() {
        let registry = AppRegistry::new();
        let mut notes = AppRule::new(
            "notes-app",
            "Notes",
            "Writing {title}",
            "Focus mode",
            "auto",
        );
        notes.enabled = true;
        let data = prepared(build_presence_from_rule(
            &registry,
            &notes,
            "notes-app",
            "Plan",
            None,
            true,
        ));
        assert_eq!(data.details, "Notes - Writing Plan");
        assert_eq!(data.state, "Focus mode");
        assert_eq!(data.large_text, "Notes");
        assert_eq!(data.large_image, crate::domain::presence::LOGO_IMAGE_URL);

        notes.details = String::new();
        let data = build_presence_from_rule(&registry, &notes, "notes-app", "Plan", None, true);
        assert_eq!(data.details, "Notes");
        assert_eq!(data.small_image, "");
        assert_eq!(data.small_text, "");
    }

    #[test]
    fn test_custom_vscode_rule_keeps_text_and_shows_language_badge() {
        let registry = AppRegistry::new();
        let mut vscode = AppRule::new("code.exe", "VScode", "Coding", "Working", "auto");
        vscode.enabled = true;
        let data = prepared(build_presence_from_rule(
            &registry,
            &vscode,
            "code.exe",
            "App.tsx - Better-Rich-Presence - Visual Studio Code",
            None,
            true,
        ));
        assert_eq!(data.details, "VScode - Coding");
        assert_eq!(data.state, "Working");
        assert_eq!(data.large_text, "VScode");
        assert!(
            data.large_image.ends_with("/visualstudiocode.png"),
            "{}",
            data.large_image
        );
        assert_eq!(
            data.small_image,
            "https://cdn.jsdelivr.net/gh/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord@main/public/assets/icons/react.png"
        );
        assert_eq!(data.small_text, "TypeScript");
    }

    #[test]
    fn test_custom_vscode_rule_substitutes_file_and_clears_badge_for_workspace_only() {
        let registry = AppRegistry::new();
        let mut vscode = AppRule::new("code.exe", "VScode", "Hacking {file}", "Focused", "auto");
        vscode.enabled = true;
        let with_file = prepared(build_presence_from_rule(
            &registry,
            &vscode,
            "code.exe",
            "engine.rs - Better-Rich-Presence - Visual Studio Code",
            None,
            true,
        ));
        assert_eq!(with_file.details, "VScode - Hacking engine.rs");
        assert_eq!(with_file.state, "Focused");
        assert!(
            with_file.small_image.ends_with("/rust.png"),
            "{}",
            with_file.small_image
        );
        assert_eq!(with_file.small_text, "Rust");

        let workspace_only = prepared(build_presence_from_rule(
            &registry,
            &vscode,
            "code.exe",
            "my-app - Visual Studio Code",
            None,
            true,
        ));
        assert_eq!(workspace_only.small_image, "");
        assert_eq!(workspace_only.small_text, "");
    }

    #[test]
    fn test_zed_and_jetbrains_rich_and_custom_presences() {
        let registry = AppRegistry::new();

        let zed_app = registry.find_app("zed.exe").unwrap();
        let (_, zed_ctx) =
            extract_editor_context(Some(zed_app), "Better-Rich-Presence — engine.rs");
        let zed_rich = prepared(build_editor_presence(
            zed_app,
            None,
            None,
            None,
            &zed_ctx.unwrap(),
            1234,
        ));
        assert_eq!(zed_rich.details, "Zed · Better-Rich-Presence");
        assert_eq!(zed_rich.state, "Editing engine.rs (Rust)");
        assert!(
            zed_rich.large_image.ends_with("/zedindustries.png"),
            "{}",
            zed_rich.large_image
        );
        assert!(
            zed_rich.small_image.ends_with("/rust.png"),
            "{}",
            zed_rich.small_image
        );
        assert_eq!(zed_rich.small_text, "Rust");

        let mut zed_rule = AppRule::new("zed.exe", "Zed", "Hacking {file}", "Focused", "auto");
        zed_rule.enabled = true;
        let zed_custom = prepared(build_presence_from_rule(
            &registry,
            &zed_rule,
            "zed.exe",
            "Better-Rich-Presence — engine.rs",
            None,
            true,
        ));
        assert_eq!(zed_custom.details, "Zed - Hacking engine.rs");
        assert_eq!(zed_custom.state, "Focused");
        assert!(
            zed_custom.small_image.ends_with("/rust.png"),
            "{}",
            zed_custom.small_image
        );
        assert_eq!(zed_custom.small_text, "Rust");

        let pycharm_app = registry.find_app("pycharm64.exe").unwrap();
        let (_, py_ctx) = extract_editor_context(
            Some(pycharm_app),
            "my-api – src/server.py [C:\\work\\my-api]",
        );
        let py_rich = prepared(build_editor_presence(
            pycharm_app,
            None,
            None,
            None,
            &py_ctx.unwrap(),
            0,
        ));
        assert_eq!(py_rich.details, "PyCharm · my-api");
        assert_eq!(py_rich.state, "Editing server.py (Python)");
        assert!(
            py_rich.large_image.ends_with("/pycharm.png"),
            "{}",
            py_rich.large_image
        );
        assert!(
            py_rich.small_image.ends_with("/python.png"),
            "{}",
            py_rich.small_image
        );
        assert_eq!(py_rich.small_text, "Python");

        let mut py_rule = AppRule::new("pycharm64.exe", "PyCharm", "Coding", "Working", "auto");
        py_rule.enabled = true;
        let py_custom = prepared(build_presence_from_rule(
            &registry,
            &py_rule,
            "pycharm64.exe",
            "my-api – src/server.py [C:\\work\\my-api]",
            None,
            true,
        ));
        assert_eq!(py_custom.details, "PyCharm - Coding");
        assert_eq!(py_custom.state, "Working");
        assert!(
            py_custom.small_image.ends_with("/python.png"),
            "{}",
            py_custom.small_image
        );
        assert_eq!(py_custom.small_text, "Python");
    }

    #[test]
    fn test_non_editor_browser_never_receives_language_badge() {
        let registry = AppRegistry::new();
        let firefox = registry.find_app("firefox").unwrap();
        let (ide_ctx, editor_ctx) = extract_editor_context(Some(firefox), "main.rs - Cursor");
        assert!(ide_ctx.is_none());
        assert!(editor_ctx.is_none());

        let mut rule = AppRule::new("firefox", "Firefox", "Browsing", "Docs", "auto");
        rule.enabled = true;
        let data = prepared(build_presence_from_rule(
            &registry,
            &rule,
            "firefox",
            "main.rs - Cursor",
            None,
            true,
        ));
        assert_eq!(data.small_image, "");
        assert_eq!(data.small_text, "");
    }

    #[test]
    fn test_rule_image_catalog_id_resolves_to_icon() {
        let registry = AppRegistry::new();
        let rule = AppRule::new("notes-app", "Notes", "x", "y", "firefox");
        let data = prepared(build_presence_from_rule(
            &registry,
            &rule,
            "notes-app",
            "",
            None,
            true,
        ));
        assert!(
            data.large_image.ends_with("/firefox.png"),
            "{}",
            data.large_image
        );
    }

    #[test]
    fn test_display_process_name() {
        assert_eq!(display_process_name("my-tool"), "my-tool");
        assert_eq!(display_process_name(" My-Tool.EXE "), "My-Tool");
        assert_eq!(display_process_name(""), "Unknown app");
    }

    #[test]
    fn test_idle_presence_is_english() {
        let idle = idle_presence("Away");
        assert_eq!(idle.details, "Away");
        assert_eq!(idle.state, "Idle");
        assert_eq!(idle.large_text, "Idle");
    }

    #[test]
    fn test_default_presence_is_english_with_logo() {
        let data = PresenceData::default();
        assert_eq!(data.details, "Browsing the PC");
        assert_eq!(data.state, "Just chilling...");
        assert_eq!(data.large_image, crate::domain::presence::LOGO_IMAGE_URL);
        assert_eq!(
            data.activity_name,
            crate::domain::presence::DEFAULT_ACTIVITY_NAME
        );
    }

    #[test]
    fn test_claude_code_presence_resolves_catalog_icon_and_session_timestamp() {
        let session = ClaudeSession {
            session_id: "sess-1".to_string(),
            project_name: Some("Better-RPC".to_string()),
            cwd: Some("/work/Better-RPC".to_string()),
            transcript_path: None,
            chat_title: Some("Fix rich preview".to_string()),
            has_explicit_title: true,
            model: Some("Claude Opus 4.6".to_string()),
            git_branch: Some("main".to_string()),
            permission_mode: None,
            activity: crate::domain::claude_code::ClaudeActivity::EditingFile(Some(
                "engine.rs".to_string(),
            )),
            started_at: 1_700_000_000,
            last_event_at: 1_700_000_050,
        };
        let data = prepared(build_claude_code_presence(&session, 2, true));
        assert_eq!(
            data.activity_name,
            crate::domain::presence::DEFAULT_ACTIVITY_NAME
        );
        assert_eq!(data.details, "Claude Code · Better-RPC");
        assert_eq!(data.state, "Editing engine.rs · Fix rich preview");
        assert_eq!(data.large_text, "Claude Opus 4.6 · main · 2 chats");
        assert_eq!(data.timestamp, 1_700_000_000);
        assert!(
            data.large_image.starts_with("https://cdn.jsdelivr.net/")
                && data.large_image.ends_with("/claude.png"),
            "{}",
            data.large_image
        );
    }

    #[test]
    fn test_resolve_timestamp_honors_explicit_session_timestamp() {
        // Explicit session start overrides prior shared-client timestamp.
        assert_eq!(resolve_timestamp(1, 100, 1, 250, 300), 250);
        // Zero requested timestamp preserves existing timer on the same client.
        assert_eq!(resolve_timestamp(1, 100, 1, 0, 300), 100);
        // Zero requested timestamp starts fresh when switching client IDs.
        assert_eq!(resolve_timestamp(1, 100, 2, 0, 300), 300);
    }

    #[test]
    fn test_presence_app_label_and_summary() {
        let ide = PresenceData {
            details: "Visual Studio Code · demo".to_string(),
            state: "Editing main.rs (Rust)".to_string(),
            large_text: "Visual Studio Code".to_string(),
            ..PresenceData::default()
        };
        assert_eq!(presence_app_label(&ide), "Visual Studio Code");
        assert_eq!(
            format_presence_summary(&ide),
            "\"Visual Studio Code · demo\" · \"Editing main.rs (Rust)\""
        );

        let claude = PresenceData {
            details: "Claude Code · Better-RPC".to_string(),
            state: "Thinking".to_string(),
            large_text: "Claude Opus 4.6".to_string(),
            ..PresenceData::default()
        };
        assert_eq!(presence_app_label(&claude), "Claude Code");

        let idle = idle_presence("BRB");
        assert_eq!(presence_app_label(&idle), "Idle");
        assert_eq!(format_presence_summary(&idle), "\"BRB\" · \"Idle\"");

        let details_only = PresenceData {
            details: "Notes".to_string(),
            state: String::new(),
            large_text: String::new(),
            ..PresenceData::default()
        };
        assert_eq!(presence_app_label(&details_only), "Notes");
        assert_eq!(format_presence_summary(&details_only), "\"Notes\"");

        let empty = PresenceData {
            details: String::new(),
            state: String::new(),
            large_text: String::new(),
            ..PresenceData::default()
        };
        assert_eq!(presence_app_label(&empty), "Unknown");
        assert_eq!(format_presence_summary(&empty), "No details");
    }
}

/// Helper function to spawn the engine task
pub fn start_engine(rx: mpsc::Receiver<EngineEvent>, app_state: AppState) {
    let engine = PresenceEngine::new(app_state);
    tauri::async_runtime::spawn(async move {
        engine.run(rx).await;
    });
}
