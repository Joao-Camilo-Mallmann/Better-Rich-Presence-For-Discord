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
}

export interface PresenceData {
  details: string;
  state: string;
  large_image: string;
  large_text: string;
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

export interface DiscordUser {
  id: string;
  username: string;
  avatar_url: string;
}
