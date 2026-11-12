import { TriangleAlert } from "lucide-react";
import { PlatformStatus } from "../types";

const BACKEND_LABELS: Record<string, string> = {
  win32: "Windows",
  "gnome-window-calls": "GNOME · Window Calls",
  "gnome-focused-window": "GNOME · Focused Window D-Bus",
  "gnome-mutter": "GNOME · Mutter",
  hyprland: "Hyprland",
  sway: "Sway",
  x11: "X11",
  xprintidle: "xprintidle",
  "process-scan": "Rule apps only",
};

const label = (backend: string | null) =>
  backend ? (BACKEND_LABELS[backend] ?? backend) : "Unavailable";

/** Shows how activity is detected on this platform, plus a setup hint when degraded. */
export function DetectionStatus({ status }: { status: PlatformStatus | null }) {
  if (!status) return null;

  const session = [status.os, status.session_type, status.desktop]
    .filter(Boolean)
    .join(" · ");

  return (
    <div className="flex flex-col gap-2">
      {status.hint && (
        <div
          role="status"
          className="neo-card bg-yellow-accent text-black p-3 flex gap-2 items-start text-[11px] leading-snug"
          style={{ borderRadius: "6px", boxShadow: "4px 4px 0px var(--neo-shadow-color)" }}
        >
          <TriangleAlert size={14} className="shrink-0 mt-0.5" aria-hidden />
          <span className="select-text">{status.hint}</span>
        </div>
      )}

      <div
        className="flex flex-col gap-1 py-2 text-[10px] text-muted-ink"
        style={{ borderBottom: "2px solid var(--neo-border-color)" }}
      >
        <span className="text-xs font-bold font-display uppercase text-ink">
          Activity Detection
        </span>
        <span>Session: {session}</span>
        <span>Window: {label(status.window_backend)}</span>
        <span>Idle: {label(status.idle_backend)}</span>
      </div>
    </div>
  );
}
