export type PresenceSource = "Game" | "Manual" | "Work" | "Browser" | "Idle";

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
  source: PresenceSource;
  priority: number;
  enabled: boolean;
}

export interface PresenceData {
  details: string;
  state: string;
  large_image: string;
  large_text: string;
  source: PresenceSource;
  timestamp: number | null;
}

export interface ConnectionInfo {
  connected: boolean;
  state: PresenceState;
  last_error: string | null;
  reconnect_attempts: number;
}

export interface Settings {
  idle_enabled: boolean;
  idle_threshold_minutes: number;
  idle_message: string;
  autostart_enabled: boolean;
  debounce_seconds: number;
  settle_delay_seconds: number;
}
