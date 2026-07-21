import { ArrowUp } from "lucide-react";
import { useEffect, useState } from "react";
import { PresenceCard } from "../components/PresenceCard";
import { usePresence } from "../hooks/usePresence";

export function Dashboard() {
  const { presence, connectionInfo, priorityInfo, discordUser } = usePresence();
  const [elapsed, setElapsed] = useState<string>("0m");

  useEffect(() => {
    if (!presence?.timestamp) {
      setElapsed("0m");
      return;
    }
    const updateTimer = () => {
      const diff = Math.max(
        0,
        Math.floor(Date.now() / 1000) - presence.timestamp!,
      );
      setElapsed(`${Math.floor(diff / 60)}m`);
    };
    updateTimer();
    const interval = setInterval(updateTimer, 60000);
    return () => clearInterval(interval);
  }, [presence?.timestamp]);

  return (
    <div className="flex flex-col gap-6">
      <div className="flex justify-center w-full">
        <div
          className="flex flex-col gap-3 items-center justify-center w-full max-w-2xl bg-surface-indigo p-8 neo-card dot-grid"
          style={{ borderRadius: "12px" }}
        >
          {/* Tilted eyebrow label */}
          <h3
            className="text-[24px] text-ink uppercase tracking-wider font-extrabold font-display neo-tilt px-3 py-1 bg-magenta-accent neo-border-2"
            style={{
              boxShadow: "2px 2px 0 var(--neo-shadow-color)",
              borderRadius: "4px",
              color: "var(--ink-dark)",
            }}
          >
            Status Preview
          </h3>
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
      </div>

      {/* Status Chips — Hard bordered */}
      <div
        className="flex flex-wrap gap-3 text-xs pt-4"
        style={{ borderTop: "3px solid var(--neo-border-color)" }}
      >
        <span
          className="bg-surface-indigo px-3 py-2 font-bold flex items-center gap-2 neo-border-2 neo-shadow-sm neo-press font-display uppercase"
          style={{ borderRadius: "6px" }}
        >
          <span
            className={`w-3 h-3 ${connectionInfo.state === "Connected" ? "bg-green-accent" : "bg-danger"}`}
            style={{ border: "2px solid var(--neo-border-color)" }}
          />
          Engine:{" "}
          {connectionInfo.state === "Connected" ? "Connected" : "Disconnected"}
        </span>

        <span
          className="bg-surface-indigo px-3 py-2 font-bold text-muted-ink neo-border-2 neo-shadow-sm neo-press font-display uppercase"
          style={{ borderRadius: "6px" }}
        >
          Session: <span className="text-ink">{elapsed}</span>
        </span>

        {/* Priority Mode Indicator */}
        {priorityInfo.active && (
          <span
            className="flex items-center gap-2 px-3 py-2 font-bold text-ink neo-border-2 neo-shadow-sm font-display uppercase"
            style={{
              backgroundColor: "var(--magenta-accent)",
              color: "var(--ink-dark)",
              borderRadius: "6px",
              animation: "neo-pulse 2s infinite",
            }}
            title={`Foreground: ${priorityInfo.foreground_app || "unknown"}`}
          >
            <ArrowUp size={14} />
            <span>
              Prioritizing:{" "}
              <span className="font-extrabold">
                {priorityInfo.prioritized_app || "app"}
              </span>
            </span>
          </span>
        )}
      </div>
    </div>
  );
}
