import { useEffect, useState } from "react";
import { PresenceCard } from "../../components/PresenceCard/PresenceCard";
import { usePresence } from "../../hooks/usePresence";

export function Dashboard() {
  const { presence, connectionInfo, source, priorityInfo } = usePresence();
  const [elapsed, setElapsed] = useState<string>("0m");

  useEffect(() => {
    if (!presence?.timestamp) { setElapsed("0m"); return; }
    const updateTimer = () => {
      const diff = Math.max(0, Math.floor(Date.now() / 1000) - presence.timestamp!);
      setElapsed(`${Math.floor(diff / 60)}m`);
    };
    updateTimer();
    const interval = setInterval(updateTimer, 60000);
    return () => clearInterval(interval);
  }, [presence?.timestamp]);

  return (
    <div className="flex flex-col gap-6">
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="flex flex-col gap-3 items-center justify-center w-full bg-surface-indigo/40 rounded-md border border-hairline/50 p-8 shadow-xl">
          <h3 className="text-[10px] text-muted-ink uppercase tracking-wider font-extrabold font-display">
            Status Preview
          </h3>
          <PresenceCard presence={presence} />
        </div>

        {/* 3D Floating Mockup */}
        <div className="group relative flex items-center justify-center bg-surface-indigo/40 rounded-md border border-hairline/50 p-8 perspective-[1000px] overflow-hidden">
          <h3 className="absolute top-8 text-[10px] text-muted-ink uppercase tracking-wider font-extrabold font-display z-10">
            App Mockup
          </h3>
          {/* Animated 3D Container */}
          <div className="relative w-full max-w-[280px] aspect-[4/3] mt-6 transition-all duration-500 ease-out transform group-hover:rotate-x-12 group-hover:-rotate-y-12 group-hover:scale-105 shadow-2xl rounded-lg">
            <div className="absolute inset-0 bg-gradient-to-tr from-primary/30 to-magenta-accent/30 rounded-lg blur-xl opacity-0 group-hover:opacity-100 transition-opacity duration-500"></div>
            <img 
              src="/image.png" 
              alt="App Mockup" 
              className="relative z-10 w-full h-full object-cover rounded-lg border border-white/10 shadow-[0_20px_50px_rgba(0,0,0,0.5)]"
            />
            {/* Glossy reflection overlay */}
            <div className="absolute inset-0 z-20 bg-gradient-to-br from-white/10 to-transparent rounded-lg pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity duration-500" />
          </div>
        </div>
      </div>

      {/* Performance Charts (Rust vs Electron) */}
      <div className="flex flex-col gap-3 w-full bg-surface-indigo/40 rounded-md border border-hairline/50 p-6 shadow-xl">
        <h3 className="text-[10px] text-muted-ink uppercase tracking-wider font-extrabold font-display mb-2">
          System Performance Showcase (Rust vs Electron)
        </h3>
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-6">
          {/* RAM Chart */}
          <div className="flex flex-col gap-2">
            <div className="flex justify-between text-xs text-muted-ink font-medium">
              <span>RAM Usage</span>
              <span className="text-primary font-bold">Better RPC (8 MB)</span>
            </div>
            <div className="h-4 w-full bg-surface-onyx rounded-full overflow-hidden flex shadow-inner">
              <div className="h-full bg-primary transition-all duration-1000 ease-out w-[5%]" title="Better RPC (Rust): ~8MB" />
              <div className="h-full bg-danger/80 transition-all duration-1000 ease-out w-[85%]" title="Typical Electron App: ~150MB" />
            </div>
            <div className="flex justify-end text-[10px] text-muted-ink/60">
              <span className="flex items-center gap-1"><span className="w-2 h-2 rounded-full bg-danger/80 inline-block"/> Typical Electron (~150 MB)</span>
            </div>
          </div>
          
          {/* CPU Chart */}
          <div className="flex flex-col gap-2">
            <div className="flex justify-between text-xs text-muted-ink font-medium">
              <span>CPU Usage (Idle)</span>
              <span className="text-green-accent font-bold">~0.1%</span>
            </div>
            <div className="h-4 w-full bg-surface-onyx rounded-full overflow-hidden flex shadow-inner">
              <div className="h-full bg-green-accent transition-all duration-1000 ease-out w-[1%]" title="Better RPC: ~0.1%" />
              <div className="h-full bg-danger/80 transition-all duration-1000 ease-out w-[15%]" title="Typical Electron: ~2-5%" />
            </div>
             <div className="flex justify-end text-[10px] text-muted-ink/60">
              <span className="flex items-center gap-1"><span className="w-2 h-2 rounded-full bg-danger/80 inline-block"/> Typical Electron (~2-5%)</span>
            </div>
          </div>
        </div>
      </div>

      <div className="flex flex-wrap gap-2 text-xs border-t border-hairline/25 pt-4">
        <span className="bg-surface-indigo border border-hairline/30 px-3 py-1.5 rounded-sm font-semibold flex items-center gap-1.5 shadow-sm">
          <span
            className={`w-2 h-2 rounded-full ${connectionInfo.state === "Connected" ? "bg-green-accent animate-pulse" : "bg-danger"}`}
          />
          Engine Status:{" "}
          {connectionInfo.state === "Connected" ? "Connected to Discord" : "Disconnected from Discord"}
        </span>
        <span className="bg-surface-indigo border border-hairline/30 px-3 py-1.5 rounded-sm font-semibold text-muted-ink shadow-sm">
          Source: <span className="text-ink font-bold">{source}</span>
        </span>
        <span className="bg-surface-indigo border border-hairline/30 px-3 py-1.5 rounded-sm font-semibold text-muted-ink shadow-sm">
          Session: <span className="text-ink font-bold">{elapsed}</span>
        </span>

        {/* Priority Mode Indicator — shown when a higher-priority app overrides foreground */}
        {priorityInfo.active && (
          <span
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-sm font-semibold text-ink shadow-sm border animate-pulse"
            style={{ background: "rgba(236,72,189,0.12)", borderColor: "rgba(236,72,189,0.35)" }}
            title={`Foreground: ${priorityInfo.foreground_app || "unknown"}`}
          >
            <span className="text-[13px]">⬆️</span>
            <span>
              Prioritizing:{" "}
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
