import { ArrowUp } from "lucide-react";
import { PresenceCard } from "../components/PresenceCard";
import { BandState } from "../components/StatusBand";
import { usePresence } from "../hooks/usePresence";

/** One line of guidance for states where the preview is not what Discord shows. */
const GUIDANCE: Partial<Record<BandState, string>> = {
  off: "Sharing is off, so Discord shows nothing. Turn on Share above to show this card.",
  disconnected:
    "Open the Discord desktop app. Better RPC connects to it on its own.",
  paused:
    "A game is showing its own status on Discord. Better RPC steps aside until it closes.",
};

interface DashboardProps {
  bandState: BandState;
}

/** The "Now" view: a preview of the Discord profile card with the current activity. */
export function Dashboard({ bandState }: DashboardProps) {
  const { presence, priorityInfo, discordUser } = usePresence();
  const guidance = GUIDANCE[bandState];
  const dimmed = bandState === "off";

  return (
    <div className="flex flex-col items-center gap-4">
      <span
        className="neo-tilt self-center text-[12px] font-extrabold font-display uppercase tracking-wider px-2.5 py-0.5 bg-magenta-accent text-ink-dark neo-border-2"
        style={{
          boxShadow: "3px 3px 0 var(--neo-shadow-color)",
          borderRadius: "4px",
        }}
      >
        What friends see
      </span>

      <div
        className={`w-full flex justify-center transition-opacity duration-150 ${dimmed ? "opacity-40 grayscale" : ""}`}
      >
        <PresenceCard
          presence={presence}
          profile={
            discordUser
              ? {
                  displayName: discordUser.username,
                  username: discordUser.username,
                  avatarUrl: discordUser.avatar_url,
                  customStatus: "",
                  status: "online",
                  themePrimary: "#5865f2",
                  themeSecondary: "#2c327d",
                  isGradient: true,
                }
              : undefined
          }
        />
      </div>

      {guidance && (
        <p className="w-full max-w-[340px] text-center text-[13px] leading-snug text-muted-ink">
          {guidance}
        </p>
      )}

      {priorityInfo.active && bandState === "live" && (
        <div
          className="w-full max-w-[340px] flex items-start gap-3 p-3 bg-surface-indigo neo-border-2 neo-shadow-sm"
          style={{ borderRadius: "8px" }}
        >
          <span
            aria-hidden
            className="shrink-0 w-6 h-6 flex items-center justify-center bg-magenta-accent text-ink-dark neo-border-2"
            style={{ borderRadius: "4px" }}
          >
            <ArrowUp size={14} strokeWidth={3} />
          </span>
          <p className="text-[13px] leading-snug text-ink">
            Showing{" "}
            <strong className="font-bold">
              {priorityInfo.prioritized_app || "an app"}
            </strong>{" "}
            instead of{" "}
            <strong className="font-bold">
              {priorityInfo.foreground_app || "the app in focus"}
            </strong>
            , the app in focus.{" "}
            <span className="text-muted-ink">
              Max priority mode shows the highest app in your list.
            </span>
          </p>
        </div>
      )}
    </div>
  );
}
