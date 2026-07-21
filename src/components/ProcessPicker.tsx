import { invoke } from "@tauri-apps/api/core";
import { Search } from "lucide-react";
import { useEffect, useState } from "react";
import { AppRule } from "../types";
import { AppIcon } from "./AppIcon";

interface ProcessPickerProps {
  onClose: () => void;
  onSelect: (proc: { process_name: string; display_name: string }) => void;
  existingRules: AppRule[];
}

export function ProcessPicker({
  onClose,
  onSelect,
  existingRules,
}: ProcessPickerProps) {
  const [pickerSearch, setPickerSearch] = useState("");
  const [systemProcesses, setSystemProcesses] = useState<
    { process_name: string; display_name: string }[]
  >([]);
  const [loadingProcesses, setLoadingProcesses] = useState(false);

  useEffect(() => {
    let mounted = true;
    (async () => {
      try {
        setLoadingProcesses(true);
        const procs = await invoke<
          { process_name: string; display_name: string }[]
        >("get_running_processes");
        if (!mounted) return;
        const unique = Array.from(
          new Map(procs.map((p) => [p.process_name, p])).values(),
        );
        unique.sort((a, b) => a.display_name.localeCompare(b.display_name));
        setSystemProcesses(unique);
      } catch (err) {
        console.error("Failed to load running processes:", err);
        if (mounted) setSystemProcesses([]);
      } finally {
        if (mounted) setLoadingProcesses(false);
      }
    })();
    return () => {
      mounted = false;
    };
  }, []);

  const filteredProcesses = systemProcesses.filter(
    (p) =>
      p.display_name.toLowerCase().includes(pickerSearch.toLowerCase()) ||
      p.process_name.toLowerCase().includes(pickerSearch.toLowerCase()),
  );

  return (
    <div
      className="fixed inset-0 bg-black/70 flex items-start justify-center pt-[10vh] px-4 z-50"
      style={{ animation: "neo-bounce-in 0.15s ease-out" }}
    >
      <div
        className="bg-surface-indigo w-full max-w-[500px] flex flex-col overflow-hidden neo-card neo-shadow-heavy"
        style={{ borderRadius: "12px" }}
      >
        {/* Search Header */}
        <div
          className="relative bg-primary"
          style={{ borderBottom: "3px solid var(--neo-border-color)" }}
        >
          <span className="absolute left-4 top-1/2 -translate-y-1/2 text-white/70 flex items-center">
            <Search size={20} />
          </span>
          <input
            type="text"
            placeholder="Search running processes..."
            autoFocus
            value={pickerSearch}
            onChange={(e) => setPickerSearch(e.target.value)}
            className="w-full bg-transparent text-white text-base pl-12 pr-14 py-4 focus:outline-none placeholder:text-white/50 font-display font-bold"
          />
          <button
            onClick={onClose}
            className="absolute right-3 top-1/2 -translate-y-1/2 neo-btn bg-surface-black text-white px-2 py-1 text-[10px]"
            style={{ borderRadius: "4px" }}
          >
            ESC
          </button>
        </div>
        {/* Process List */}
        <div className="max-h-[350px] overflow-y-auto flex flex-col">
          {loadingProcesses ? (
            <div className="flex flex-col items-center justify-center py-10 gap-3">
              {/* Chunky spinning square */}
              <div
                className="w-6 h-6 neo-border-2 bg-primary"
                style={{ animation: "neo-spin 0.8s linear infinite" }}
              />
              <span className="text-xs text-muted-ink font-display font-bold uppercase">
                Scanning system...
              </span>
            </div>
          ) : filteredProcesses.length > 0 ? (
            <div className="p-2 flex flex-col gap-1">
              {filteredProcesses.map((p) => {
                const isAlreadyAdded = existingRules.some(
                  (r) => r.process_name === p.process_name,
                );
                return (
                  <button
                    key={p.process_name}
                    onClick={() => !isAlreadyAdded && onSelect(p)}
                    disabled={isAlreadyAdded}
                    className={`flex items-center gap-3 px-3 py-2.5 text-left transition-all group neo-border-2 ${
                      isAlreadyAdded
                        ? "opacity-40 cursor-not-allowed bg-transparent"
                        : "hover:bg-primary/15 cursor-pointer neo-press bg-surface-onyx/30"
                    }`}
                    style={{
                      borderRadius: "6px",
                      boxShadow: isAlreadyAdded
                        ? "none"
                        : "3px 3px 0 var(--neo-shadow-color)",
                    }}
                  >
                    <AppIcon
                      name={p.process_name}
                      className="w-8 h-8 object-contain neo-border-2"
                      size={32}
                    />
                    <div className="flex-1 min-w-0">
                      <div className="font-bold text-ink text-sm truncate font-display">
                        {p.display_name}
                      </div>
                      <div className="text-[10px] text-muted-ink font-mono truncate">
                        {p.process_name}
                      </div>
                    </div>
                    <div>
                      {isAlreadyAdded ? (
                        <span className="text-[10px] font-extrabold text-green-accent flex items-center gap-1 font-display uppercase">
                          <span>✓</span> Added
                        </span>
                      ) : (
                        <span
                          className="text-[10px] font-extrabold text-primary opacity-0 group-hover:opacity-100 transition-opacity px-2 py-1 neo-border-2 bg-primary/20 font-display uppercase"
                          style={{ borderRadius: "4px" }}
                        >
                          Enter ↵
                        </span>
                      )}
                    </div>
                  </button>
                );
              })}
            </div>
          ) : (
            <div className="text-center py-10 px-4 flex flex-col items-center">
              <Search size={32} className="mb-2 text-muted-ink" />
              <p className="text-xs text-muted-ink font-display uppercase font-bold">
                No running process matches your search.
              </p>
            </div>
          )}
        </div>
        {/* Footer */}
        <div
          className="px-4 py-2 flex justify-between items-center text-[10px] text-muted-ink font-display font-bold uppercase bg-surface-black"
          style={{ borderTop: "3px solid var(--neo-border-color)" }}
        >
          <span>Use arrows to navigate</span>
          <span>{systemProcesses.length} active processes</span>
        </div>
      </div>
    </div>
  );
}
