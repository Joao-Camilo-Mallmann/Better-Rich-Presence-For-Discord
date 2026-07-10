import { useState, useEffect, useRef } from "react";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

interface LogMessage {
  level: string;
  message: string;
  timestamp: string;
}

const levelColors: Record<string, string> = {
  INFO: "text-green-accent font-semibold",
  WARN: "text-yellow-500 font-semibold",
  ERROR: "text-danger font-bold",
  DEBUG: "text-primary/70",
  TRACE: "text-muted-ink/60",
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
      <div className="flex justify-between items-center bg-surface-onyx px-2 py-1.5 rounded-sm border border-hairline/30 flex-wrap gap-2">
        <div className="flex gap-1">
          {(["ALL", "INFO", "WARN", "ERROR", "DEBUG"] as const).map((lvl) => (
            <button key={lvl} onClick={() => setFilter(lvl)}
              className={`px-2 py-0.5 rounded-xs text-[10px] uppercase font-bold transition ${
                filter === lvl ? "bg-white/10 text-ink font-extrabold" : "text-muted-ink hover:text-ink"
              }`}>
              {lvl === "ALL" ? "All" : lvl}
            </button>
          ))}
        </div>
        <button onClick={() => setLogs([])}
          className="text-[10px] text-muted-ink hover:text-danger hover:underline font-bold uppercase transition">
          Clear
        </button>
      </div>
      <div ref={containerRef}
        className="bg-surface-onyx rounded-sm border border-hairline/30 p-2.5 h-[160px] overflow-y-auto font-mono text-[11px] leading-relaxed select-text">
        {filteredLogs.length > 0 ? filteredLogs.map((log, idx) => (
          <div key={idx} className="flex gap-2 items-start py-[1px]">
            <span className="text-muted-ink/50 shrink-0">[{log.timestamp}]</span>
            <span className={`shrink-0 uppercase tracking-wider text-[10px] ${levelColors[log.level] || "text-ink"}`}>
              [{log.level}]
            </span>
            <span className="text-ink break-all flex-1">{log.message}</span>
          </div>
        )) : (
          <div className="text-center py-12 text-muted-ink">No logs recorded.</div>
        )}
      </div>
    </div>
  );
}
