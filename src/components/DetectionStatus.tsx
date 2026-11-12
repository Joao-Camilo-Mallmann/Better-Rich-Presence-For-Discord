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
  "process-scan": "Apps in your list only",
};

const label = (backend: string | null) =>
  backend ? (BACKEND_LABELS[backend] ?? backend) : "Unavailable";

/** Setup notice shown when activity detection is degraded. */
export function DetectionHint({ hint }: { hint: string }) {
  return (
    <div
      role="status"
      className="neo-card bg-yellow-accent text-black p-3 flex gap-2.5 items-start text-[13px] leading-snug"
      style={{ borderRadius: "8px", boxShadow: "4px 4px 0px var(--neo-shadow-color)" }}
    >
      <TriangleAlert size={16} className="shrink-0 mt-0.5" aria-hidden />
      <span className="select-text">{hint}</span>
    </div>
  );
}

/** How activity is detected on this platform: session, window backend and idle backend. */
export function DetectionDetails({ status }: { status: PlatformStatus | null }) {
  if (!status) {
    return <p className="py-3 text-[12px] text-muted-ink">Checking…</p>;
  }

  const session = [status.os, status.session_type, status.desktop]
    .filter(Boolean)
    .join(" · ");

  const rows: [string, string][] = [
    ["Session", session],
    ["Focused app", label(status.window_backend)],
    ["Away time", label(status.idle_backend)],
  ];

  return (
    <dl className="grid grid-cols-[auto_1fr] gap-x-4 gap-y-1.5 py-3 text-[12px]">
      {rows.map(([name, value]) => (
        <div key={name} className="contents">
          <dt className="text-muted-ink">{name}</dt>
          <dd className="text-ink select-text">{value}</dd>
        </div>
      ))}
    </dl>
  );
}
