import { ConnectionInfo, PresenceState } from "../types";
import { Toggle } from "./Toggle";

export type BandState = "off" | "paused" | "live" | "connecting" | "disconnected";

/** Reduces sharing, engine and connection state to the one state the band shows. */
export function getBandState(
  sharing: boolean,
  presenceState: PresenceState,
  connection: ConnectionInfo,
): BandState {
  if (!sharing) return "off";
  if (presenceState === "PausedByGame") return "paused";
  if (
    connection.connected ||
    presenceState === "Connected" ||
    presenceState === "Updating"
  ) {
    return "live";
  }
  if (presenceState === "WaitingDiscord") return "connecting";
  return "disconnected";
}

const BAND: Record<
  BandState,
  { label: string; band: string; dot: string; pulse: boolean }
> = {
  live: {
    label: "Live on Discord",
    band: "bg-primary text-white",
    dot: "bg-green-accent",
    pulse: true,
  },
  off: {
    label: "Sharing off",
    band: "bg-yellow-accent text-ink-dark",
    dot: "bg-transparent",
    pulse: false,
  },
  paused: {
    label: "Paused for a game",
    band: "bg-yellow-accent text-ink-dark",
    dot: "bg-ink-dark",
    pulse: false,
  },
  connecting: {
    label: "Connecting to Discord",
    band: "bg-surface-black text-ink",
    dot: "bg-yellow-accent",
    pulse: true,
  },
  disconnected: {
    label: "Not connected to Discord",
    band: "bg-surface-black text-ink",
    dot: "bg-danger",
    pulse: false,
  },
};

interface StatusBandProps {
  state: BandState;
  sharing: boolean;
  onSharingChange: (sharing: boolean) => void;
}

/** Full-width band under the title bar: what Discord shows right now, plus the global Share switch. */
export function StatusBand({ state, sharing, onSharingChange }: StatusBandProps) {
  const look = BAND[state];

  return (
    <div
      className={`shrink-0 flex items-center justify-between gap-3 px-4 py-2.5 ${look.band}`}
      style={{ borderBottom: "3px solid var(--neo-border-color)" }}
    >
      <div role="status" className="flex items-center gap-2.5 min-w-0">
        <span
          aria-hidden
          className={`w-3 h-3 shrink-0 ${look.dot} ${look.pulse ? "motion-safe:animate-[neo-pulse_2s_infinite]" : ""}`}
          style={{ border: "2px solid var(--neo-border-color)" }}
        />
        <span className="text-[13px] font-extrabold font-display uppercase tracking-wide truncate">
          {look.label}
        </span>
      </div>
      <div className="flex items-center gap-2 shrink-0">
        <span
          aria-hidden
          className="text-[12px] font-extrabold font-display uppercase tracking-wider"
        >
          Share
        </span>
        <Toggle
          checked={sharing}
          onChange={onSharingChange}
          label="Share activity on Discord"
        />
      </div>
    </div>
  );
}
