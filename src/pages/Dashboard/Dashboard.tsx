import { useEffect, useState } from "react";
import { PresenceCard } from "../../components/PresenceCard/PresenceCard";
import { usePresence } from "../../hooks/usePresence";

export function Dashboard() {
  const { presence, connectionInfo, source, priorityInfo } = usePresence();
  const [elapsed, setElapsed] = useState<string>("0m");

  useEffect(() => {
    if (!presence?.timestamp) {
      setElapsed("0m");
      return;
    }

    const updateTimer = () => {
      const now = Math.floor(Date.now() / 1000);
      const diff = Math.max(0, now - presence.timestamp!);
      const mins = Math.floor(diff / 60);
      setElapsed(`${mins}m`);
    };

    updateTimer();
    const interval = setInterval(updateTimer, 60000);
    return () => clearInterval(interval);
  }, [presence?.timestamp]);

  return (
    <div className="flex flex-col gap-6">
      {/* Presence Preview Centered */}
      <div className="flex flex-col gap-3 items-center justify-center w-full bg-surface-indigo/40 rounded-md border border-hairline/50 p-8">
        <h3 className="text-[10px] text-muted-ink uppercase tracking-wider font-extrabold font-display">
          Pré-visualização do Status
        </h3>
        <PresenceCard presence={presence} />
      </div>

      {/* Info Status Row */}
      <div className="flex flex-wrap gap-2 text-xs border-t border-hairline/25 pt-4">
        <span className="bg-surface-indigo border border-hairline/30 px-3 py-1.5 rounded-sm font-semibold flex items-center gap-1.5 shadow-sm">
          <span
            className={`w-2 h-2 rounded-full ${connectionInfo.state === "Connected" ? "bg-green-accent animate-pulse" : "bg-danger"}`}
          />
          Engine Status:{" "}
          {connectionInfo.state === "Connected"
            ? "Conectado ao Discord"
            : "Desconectado do Discord"}
        </span>
        <span className="bg-surface-indigo border border-hairline/30 px-3 py-1.5 rounded-sm font-semibold text-muted-ink shadow-sm">
          Fonte:{" "}
          <span className="text-ink font-bold">{source}</span>
        </span>
        <span className="bg-surface-indigo border border-hairline/30 px-3 py-1.5 rounded-sm font-semibold text-muted-ink shadow-sm">
          Sessão:{" "}
          <span className="text-ink font-bold">{elapsed}</span>
        </span>

        {/* Priority Mode Indicator — shown when a higher-priority app overrides foreground */}
        {priorityInfo.active && (
          <span
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-sm font-semibold text-ink shadow-sm border animate-pulse"
            style={{
              background: "rgba(236,72,189,0.12)",
              borderColor: "rgba(236,72,189,0.35)",
            }}
            title={`Em foco: ${priorityInfo.foreground_app || "desconhecido"}`}
          >
            <span className="text-[13px]">⬆️</span>
            <span>
              Priorizando:{" "}
              <span className="font-bold" style={{ color: "#ec48bd" }}>
                {priorityInfo.prioritized_app || "app"}
              </span>
            </span>
          </span>
        )}
      </div>
    </div>
  );
}
