import { usePresence } from "../../hooks/usePresence";
import { PresenceCard } from "../../components/PresenceCard/PresenceCard";
import { useEffect, useState } from "react";

export function Dashboard() {
  const { presence, connectionInfo, source } = usePresence();
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
    <div className="flex flex-col gap-4">
      {/* Live Preview */}
      <div className="flex flex-col gap-2">
        <h3 className="text-[10px] text-muted-ink uppercase tracking-wider font-extrabold font-display">Pré-visualização</h3>
        <PresenceCard presence={presence} />
      </div>

      {/* Stats Summary Row */}
      <div className="flex flex-wrap gap-2 text-xs">
        <span className="bg-surface-indigo border border-hairline/30 px-3 py-1.5 rounded-sm font-semibold flex items-center gap-1.5 shadow-sm">
          <span className={`w-2 h-2 rounded-full ${connectionInfo.state === "Connected" ? "bg-green-accent animate-pulse" : "bg-danger"}`} />
          {connectionInfo.state === "Connected" ? "Conectado" : "Desconectado"}
        </span>
        <span className="bg-surface-indigo border border-hairline/30 px-3 py-1.5 rounded-sm font-semibold text-muted-ink shadow-sm">
          Fonte: <span className="text-ink font-bold">{source}</span>
        </span>
        <span className="bg-surface-indigo border border-hairline/30 px-3 py-1.5 rounded-sm font-semibold text-muted-ink shadow-sm">
          Uptime: <span className="text-ink font-bold">{elapsed}</span>
        </span>
      </div>
    </div>
  );
}
