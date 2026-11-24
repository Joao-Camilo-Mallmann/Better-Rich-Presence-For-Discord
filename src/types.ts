export type PresenceState =
  | "Disconnected"
  | "WaitingDiscord"
  | "Connected"
  | "PausedByGame"
  | "Updating";

export interface AppRule {
  process_name: string;
  display_name: string;
  details: string;
  state: string;
  large_image: string;
  enabled: boolean;
  client_id?: string;
}

export interface PresenceData {
  /** Bold title Discord shows: the Discord application's name (e.g. "Better Rich Presence"). */
  activity_name?: string;
  /** First line; the app name for detected apps (e.g. "Firefox"). Member lists show "Playing <details>". */
  details: string;
  state: string;
  large_image: string;
  large_text: string;
  small_image?: string;
  small_text?: string;
  timestamp: number | null;
}

export interface ConnectionInfo {
  connected: boolean;
  state: PresenceState;
  last_error: string | null;
  reconnect_attempts: number;
}

export interface Settings {
  global_enabled: boolean;
  idle_enabled: boolean;
  idle_threshold_minutes: number;
  idle_message: string;
  autostart_enabled: boolean;
  debounce_seconds: number;
  settle_delay_seconds: number;
  /** When true, the backend scans all running processes and shows the highest-priority one */
  priority_mode_enabled: boolean;
  /** When true, an active Claude Code session takes priority over window detection */
  claude_code_enabled: boolean;
  /** Localhost port for the Claude Code HTTP hook receiver */
  claude_code_port: number;
  /** Minutes of inactivity before a waiting Claude Code session auto-expires */
  claude_code_timeout_minutes: number;
  /** Whether to include the active chat title in Discord's state line */
  claude_code_show_chat_title: boolean;
}

export type ClaudeHookInstallState = "not_installed" | "installed" | "outdated";

export interface ClaudeSessionSummary {
  session_id: string;
  project_name: string | null;
  chat_title: string | null;
  activity: string;
  is_waiting: boolean;
  model: string | null;
  git_branch: string | null;
  started_at: number;
  last_event_at: number;
}

export interface ClaudeCodeStatus {
  install_state: ClaudeHookInstallState;
  settings_path: string;
  endpoint_url: string;
  server_listening: boolean;
  server_error: string | null;
  active_sessions: number;
  active_project: string | null;
  active_activity: string | null;
  sessions: ClaudeSessionSummary[];
  selected_session_id: string | null;
  pinned_session_id: string | null;
}

export interface DiscordProfile {
  displayName: string;
  username: string;
  avatarUrl: string;
  customStatus: string;
  status: "online" | "idle" | "dnd" | "offline";
  themePrimary: string;
  themeSecondary: string;
  isGradient: boolean;
}

/** Emitted by the backend when priority mode overrides the foreground window */
export interface PriorityInfo {
  active: boolean;
  prioritized_app: string;
  foreground_app: string;
}

/** Returned by the `get_platform_status` command */
export interface PlatformStatus {
  /** "windows", "linux", … */
  os: string;
  /** Linux only: "wayland" or "x11" */
  session_type: string | null;
  /** Linux only: XDG_CURRENT_DESKTOP, e.g. "ubuntu:GNOME" */
  desktop: string | null;
  /** e.g. "win32", "gnome-window-calls", "x11", or "process-scan" */
  window_backend: string | null;
  /** e.g. "win32", "gnome-mutter", "xprintidle"; null when idle detection is unavailable */
  idle_backend: string | null;
  /** Setup hint when detection is degraded */
  hint: string | null;
}

export interface DiscordUser {
  id: string;
  username: string;
  avatar_url: string;
}
