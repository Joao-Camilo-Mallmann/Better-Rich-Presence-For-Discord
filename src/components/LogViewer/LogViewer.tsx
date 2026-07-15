import { useState, useEffect, useRef } from "react";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

interface LogMessage {
  level: string;
  message: string;
  timestamp: string;
}

const levelStyles: Record<string, { badge: string; text: string }> = {
  INFO: { badge: "bg-green-accent text-ink-dark", text: "text-green-accent" },
  WARN: { badge: "bg-yellow-accent text-ink-dark", text: "text-yellow-accent" },
  ERROR: { badge: "bg-danger text-white", text: "text-danger" },
  DEBUG: { badge: "bg-primary text-white", text: "text-primary" },
  TRACE: { badge: "bg-surface-onyx text-muted-ink", text: "text-muted-ink" },
};

export function LogViewer() {
  const [logs, setLogs] = useState<LogMessage[]>([]);
  const [filter, setFilter] = useState<"ALL" | "INFO" | "WARN" | "ERROR" | "DEBUG" | "TRACE">("ALL");
  const containerRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    let unlistenLogs: UnlistenFn;
    async function setupListener() {
      try {
        unlistenLogs = await listen<LogMessage>("app-log", (event) => {
          setLogs((prev) => {
            const next = [...prev, event.payload];
            return next.length > 200 ? next.slice(next.length - 200) : next;
          });
        });
      } catch (err) {
        console.error("Failed to start log listener:", err);
      }
    }
    setupListener();
    return () => { if (unlistenLogs) unlistenLogs(); };
  }, []);

  // Auto-scroll to bottom of logs
  useEffect(() => {
    if (containerRef.current) containerRef.current.scrollTop = containerRef.current.scrollHeight;
  }, [logs]);

  const filteredLogs = filter === "ALL" ? logs : logs.filter((log) => log.level === filter);

  return (
    <div className="flex flex-col gap-2 w-full">
      {/* Filter Toolbar */}
      <div className="flex justify-between items-center px-2 py-2 flex-wrap gap-2 neo-border-2 bg-surface-onyx"
        style={{ borderRadius: '6px' }}>
        <div className="flex gap-1">
          {(["ALL", "INFO", "WARN", "ERROR", "DEBUG"] as const).map((lvl) => {
            const isActive = filter === lvl;
            const lvlStyle = lvl === "ALL" ? null : levelStyles[lvl];
            return (
              <button key={lvl} onClick={() => setFilter(lvl)}
                className={`px-2.5 py-1 text-[10px] uppercase font-extrabold font-display transition-all neo-press ${
                  isActive
                    ? (lvlStyle ? `${lvlStyle.badge} neo-border-2` : "bg-ink text-ink-dark neo-border-2")
                    : "text-muted-ink hover:text-ink"
                }`}
                style={{
                  borderRadius: '4px',
                  boxShadow: isActive ? '2px 2px 0 var(--neo-shadow-color)' : 'none',
                }}>
                {lvl === "ALL" ? "All" : lvl}
              </button>
            );
          })}
        </div>
        <button onClick={() => setLogs([])}
          className="neo-btn bg-danger text-white px-3 py-1 text-[10px]"
          style={{ borderRadius: '4px' }}>
          Clear
        </button>
      </div>
      {/* Log Container */}
      <div ref={containerRef}
        className="p-3 h-[160px] overflow-y-auto font-mono text-[11px] leading-relaxed select-text neo-border-2 bg-surface-black"
        style={{ borderRadius: '6px' }}>
        {filteredLogs.length > 0 ? filteredLogs.map((log, idx) => {
          const style = levelStyles[log.level];
          return (
            <div key={idx} className="flex gap-2 items-start py-[2px]">
              <span className="text-muted-ink shrink-0 opacity-60">[{log.timestamp}]</span>
              <span className={`shrink-0 uppercase tracking-wider text-[10px] font-extrabold px-1 neo-border-2 ${style?.badge || "bg-surface-onyx text-ink"}`}
                style={{ borderRadius: '2px', fontSize: '9px', lineHeight: '16px' }}>
                {log.level}
              </span>
              <span className="text-ink break-all flex-1">{log.message}</span>
            </div>
          );
        }) : (
          <div className="text-center py-12 text-muted-ink font-display uppercase font-bold text-xs">No logs recorded.</div>
        )}
      </div>
    </div>
  );
}
