import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { getIconUrl } from "../../utils/iconUrl";
import { AppRule } from "../../types";

interface ProcessPickerProps {
  onClose: () => void;
  onSelect: (proc: { process_name: string; display_name: string }) => void;
  existingRules: AppRule[];
}

export function ProcessPicker({ onClose, onSelect, existingRules }: ProcessPickerProps) {
  const [pickerSearch, setPickerSearch] = useState("");
  const [systemProcesses, setSystemProcesses] = useState<{ process_name: string; display_name: string }[]>([]);
  const [loadingProcesses, setLoadingProcesses] = useState(false);

  useEffect(() => {
    let mounted = true;
    
    const fetchProcesses = async () => {
      try {
        setLoadingProcesses(true);
        const procs = await invoke<{ process_name: string; display_name: string }[]>("get_running_processes");
        if (!mounted) return;
        
        // Deduplicate and sort
        const unique = Array.from(new Map(procs.map((p) => [p.process_name, p])).values());
        unique.sort((a, b) => a.display_name.localeCompare(b.display_name));
        setSystemProcesses(unique);
      } catch (err) {
        console.error("Failed to load running processes:", err);
        if (mounted) setSystemProcesses([]);
      } finally {
        if (mounted) setLoadingProcesses(false);
      }
    };

    fetchProcesses();

    return () => {
      mounted = false;
    };
  }, []);

  const filteredProcesses = systemProcesses.filter(
    (p) =>
      p.display_name.toLowerCase().includes(pickerSearch.toLowerCase()) ||
      p.process_name.toLowerCase().includes(pickerSearch.toLowerCase())
  );

  return (
    <div className="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-start justify-center pt-[10vh] px-4 z-50 animate-in fade-in duration-200">
      <div className="bg-surface-base border border-hairline/50 rounded-lg w-full max-w-[500px] flex flex-col overflow-hidden shadow-2xl animate-in zoom-in-95 duration-200">
        {/* Palette Header / Input */}
        <div className="relative border-b border-hairline/40 bg-surface-indigo">
          <span className="absolute left-4 top-1/2 -translate-y-1/2 text-xl opacity-60">🔎</span>
          <input
            type="text"
            placeholder="Search running processes..."
            autoFocus
            value={pickerSearch}
            onChange={(e) => setPickerSearch(e.target.value)}
            className="w-full bg-transparent text-ink text-base pl-12 pr-12 py-4 focus:outline-none placeholder:text-muted-ink/60"
          />
          <button
            onClick={onClose}
            className="absolute right-3 top-1/2 -translate-y-1/2 text-xs font-bold bg-surface-onyx text-muted-ink hover:text-ink px-2 py-1 rounded-sm border border-hairline transition-colors"
          >
            ESC
          </button>
        </div>

        {/* Palette List */}
        <div className="max-h-[350px] overflow-y-auto flex flex-col bg-surface-base/50">
          {loadingProcesses ? (
            <div className="flex flex-col items-center justify-center py-10 gap-3">
              <div className="w-6 h-6 rounded-full border-2 border-primary border-t-transparent animate-spin"></div>
              <span className="text-xs text-muted-ink">Scanning system...</span>
            </div>
          ) : (
            <>
              {filteredProcesses.length > 0 ? (
                <div className="p-2 flex flex-col gap-1">
                  {filteredProcesses.map((p) => {
                    const isAlreadyAdded = existingRules.some((r) => r.process_name === p.process_name);
                    const iconUrl = getIconUrl(p.process_name, p.display_name);
                    return (
                      <button
                        key={p.process_name}
                        onClick={() => !isAlreadyAdded && onSelect(p)}
                        disabled={isAlreadyAdded}
                        className={`flex items-center gap-3 px-3 py-2.5 rounded-md text-left transition-all group ${
                          isAlreadyAdded
                            ? "opacity-50 cursor-not-allowed bg-transparent"
                            : "hover:bg-primary/10 hover:shadow-inner cursor-pointer"
                        }`}
                      >
                        <img
                          src={iconUrl}
                          alt=""
                          className="w-8 h-8 object-contain rounded-sm"
                          onError={(e) => {
                            (e.target as HTMLImageElement).src = "https://www.google.com/s2/favicons?sz=128&domain=google.com";
                          }}
                        />
                        <div className="flex-1 min-w-0">
                          <div className="font-semibold text-ink text-sm truncate">{p.display_name}</div>
                          <div className="text-[10px] text-muted-ink font-mono truncate">{p.process_name}</div>
                        </div>
                        <div>
                          {isAlreadyAdded ? (
                            <span className="text-[10px] font-bold text-green-accent flex items-center gap-1">
                              <span>✓</span> Added
                            </span>
                          ) : (
                            <span className="text-[10px] font-bold text-primary opacity-0 group-hover:opacity-100 transition-opacity bg-primary/20 px-2 py-1 rounded-sm">
                              Enter ↵
                            </span>
                          )}
                        </div>
                      </button>
                    );
                  })}
                </div>
              ) : (
                <div className="text-center py-10 px-4">
                  <span className="text-2xl opacity-50 mb-2 block">🔍</span>
                  <p className="text-xs text-muted-ink">No running process matches your search.</p>
                </div>
              )}
            </>
          )}
        </div>
        
        {/* Palette Footer */}
        <div className="bg-surface-indigo border-t border-hairline/30 px-4 py-2 flex justify-between items-center text-[10px] text-muted-ink">
          <span>Use arrows to navigate</span>
          <span>{systemProcesses.length} active processes</span>
        </div>
      </div>
    </div>
  );
}
