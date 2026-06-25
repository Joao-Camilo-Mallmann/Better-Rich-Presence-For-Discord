import { useState, useRef, useMemo } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useAppRules } from "../../hooks/useAppRules";
import { AppRuleCard } from "../../components/AppRuleCard/AppRuleCard";
import { AppRule, PresenceSource } from "../../types";
import { getIconUrl } from "../../utils/iconUrl";

export function Apps() {
  const { rules, loading, updateRule, deleteRule, addRule, reorderRules } = useAppRules();
  const [filterSource, setFilterSource] = useState<"All" | PresenceSource>("All");
  const [ruleSearch, setRuleSearch] = useState("");
  const [showPicker, setShowPicker] = useState(false);
  const [pickerSearch, setPickerSearch] = useState("");
  const [systemProcesses, setSystemProcesses] = useState<{ process_name: string; display_name: string }[]>([]);
  const [loadingProcesses, setLoadingProcesses] = useState(false);

  // Drag-and-drop state
  const [draggingIndex, setDraggingIndex] = useState<number | null>(null);
  const [dragOverIndex, setDragOverIndex] = useState<number | null>(null);
  const dragItem = useRef<number | null>(null);

  // Only allow drag-and-drop when no filters are active to maintain accurate positions
  const canReorder = filterSource === "All" && ruleSearch.trim() === "";

  // ── Filter Rules ────────────────────────────────────────────────────────
  const filteredRules = useMemo(() => {
    return rules.filter((r) => {
      const matchSource = filterSource === "All" || r.source === filterSource;
      const searchLower = ruleSearch.toLowerCase();
      const matchSearch =
        r.display_name.toLowerCase().includes(searchLower) ||
        r.process_name.toLowerCase().includes(searchLower);
      return matchSource && matchSearch;
    });
  }, [rules, filterSource, ruleSearch]);

  // ── Drag-and-drop handlers ──────────────────────────────────────────────
  const handleDragStart = (index: number) => {
    dragItem.current = index;
    setDraggingIndex(index);
  };

  const handleDragOver = (e: React.DragEvent, index: number) => {
    e.preventDefault();
    setDragOverIndex(index);
  };

  const handleDrop = async (dropIndex: number) => {
    if (dragItem.current === null || dragItem.current === dropIndex) {
      setDraggingIndex(null);
      setDragOverIndex(null);
      dragItem.current = null;
      return;
    }

    const sourceRule = filteredRules[dragItem.current];
    const targetRule = filteredRules[dropIndex];

    if (!sourceRule || !targetRule) return;

    const sourceIdx = rules.findIndex((r) => r.process_name === sourceRule.process_name);
    const targetIdx = rules.findIndex((r) => r.process_name === targetRule.process_name);

    const newRules = [...rules];
    const [moved] = newRules.splice(sourceIdx, 1);
    newRules.splice(targetIdx, 0, moved);

    const newOrder = newRules.map((r) => r.process_name);

    try {
      await reorderRules(newOrder);
    } catch (err) {
      console.error("Failed to reorder rules:", err);
    }

    setDraggingIndex(null);
    setDragOverIndex(null);
    dragItem.current = null;
  };

  const handleDragEnd = () => {
    setDraggingIndex(null);
    setDragOverIndex(null);
    dragItem.current = null;
  };

  // ── Process picker ──────────────────────────────────────────────────────
  const handleOpenPicker = async () => {
    try {
      setLoadingProcesses(true);
      setShowPicker(true);
      const procs = await invoke<{ process_name: string; display_name: string }[]>("get_running_processes");
      // Deduplicate and sort
      const unique = Array.from(new Map(procs.map((p) => [p.process_name, p])).values());
      unique.sort((a, b) => a.display_name.localeCompare(b.display_name));
      setSystemProcesses(unique);
    } catch (err) {
      console.error("Failed to load running processes:", err);
      setSystemProcesses([]);
    } finally {
      setLoadingProcesses(false);
    }
  };

  const selectProcess = async (proc: { process_name: string; display_name: string }) => {
    const process_lower = proc.process_name.toLowerCase();

    let source: PresenceSource = "Work";
    let details = proc.display_name;
    let state = "Working";
    let priority = 2;

    const gameKeywords = ["game", "play", "cs2", "eldenring", "minecraft", "steam", "gta", "valorant", "counterstrike", "league"];
    const browserKeywords = ["chrome", "firefox", "edge", "opera", "brave", "safari", "browser", "explorer", "spotify"];

    if (gameKeywords.some((kw) => process_lower.includes(kw))) {
      source = "Game";
      details = proc.display_name;
      state = "In a match";
      priority = 0;
    } else if (browserKeywords.some((kw) => process_lower.includes(kw))) {
      source = "Browser";
      details = proc.display_name;
      state = "Browsing";
      priority = 3;
    }

    if (
      process_lower.includes("code") ||
      process_lower.includes("cursor") ||
      process_lower.includes("studio") ||
      process_lower.includes("devenv") ||
      process_lower.includes("figma")
    ) {
      details = `Editing in ${proc.display_name}`;
      state = "Developing";
    }

    if (rules.some((r) => r.process_name === proc.process_name)) {
      setShowPicker(false);
      return;
    }

    await addRule({
      process_name: proc.process_name,
      display_name: proc.display_name,
      details,
      state,
      large_image: "auto",
      source,
      priority,
      enabled: true,
    } as AppRule);
    
    setShowPicker(false);
    setPickerSearch("");
  };

  if (loading) {
    return (
      <div className="flex h-full items-center justify-center p-8">
        <div className="flex flex-col items-center gap-3">
          <div className="w-8 h-8 rounded-full border-2 border-primary border-t-transparent animate-spin"></div>
          <span className="text-muted-ink text-sm font-medium animate-pulse">Loading applications...</span>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full relative">
      {/* ── Sticky Header Section ── */}
      <div className="sticky top-0 z-20 bg-surface-base/95 backdrop-blur-md pb-4 pt-1 flex flex-col gap-4 border-b border-hairline/30 mb-4">
        <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-3">
          <div className="flex flex-col">
            <h3 className="text-lg text-ink font-bold font-display">Configured Applications</h3>
            <span className="text-xs text-muted-ink/80 mt-0.5 max-w-[280px]">
              Manage priority and appearance of apps on your Discord.
            </span>
          </div>
          <button
            onClick={handleOpenPicker}
            className="w-full sm:w-auto justify-center bg-primary hover:bg-primary-hover text-white text-sm px-4 py-2 rounded-sm font-semibold transition-all shadow-sm flex items-center gap-2 active:scale-95"
          >
            <span>+</span> Fetch from System
          </button>
        </div>

        {/* Toolbar: Search and Filters */}
        <div className="flex flex-col sm:flex-row gap-3 sm:items-center justify-between bg-surface-onyx/50 p-2 rounded-md border border-hairline/20">
          <div className="relative flex-1 w-full sm:max-w-[240px]">
            <span className="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-ink text-xs opacity-70">
              🔍
            </span>
            <input
              type="text"
              placeholder="Search applications..."
              value={ruleSearch}
              onChange={(e) => setRuleSearch(e.target.value)}
              className="w-full bg-surface-onyx border border-hairline/40 text-ink text-xs pl-8 pr-3 py-2 rounded-sm focus:border-primary focus:outline-none transition-colors"
            />
          </div>

          <div className="flex items-center gap-1.5 overflow-x-auto pb-1 sm:pb-0 hide-scrollbar">
            {["All", "Game", "Work", "Browser"].map((filter) => {
              const isActive = filterSource === filter;
              return (
                <button
                  key={filter}
                  onClick={() => setFilterSource(filter as any)}
                  className={`px-3 py-1.5 rounded-full text-xs font-medium transition-all whitespace-nowrap ${
                    isActive
                      ? "bg-primary text-white shadow-sm"
                      : "bg-surface-indigo border border-hairline/30 text-muted-ink hover:text-ink hover:bg-surface-indigo/80 hover:border-hairline/60"
                  }`}
                >
                  {filter === "All" ? "All Apps" : filter}
                </button>
              );
            })}
          </div>
        </div>

        {/* Drag Hint (only shows when reorder is allowed) */}
        <div className="h-4 flex items-center justify-end">
          {canReorder ? (
            <span className="text-[10px] text-muted-ink/70 font-medium flex items-center gap-1.5 animate-in fade-in slide-in-from-top-1">
              <span>↕️</span> Drag cards to reorder display priority
            </span>
          ) : (
            <span className="text-[10px] text-yellow-500/70 font-medium flex items-center gap-1.5">
              <span>⚠️</span> Ordering disabled due to active filters
            </span>
          )}
        </div>
      </div>

      {/* ── Rules List ── */}
      <div className="flex flex-col gap-2.5 pb-8">
        {filteredRules.length > 0 ? (
          filteredRules.map((rule, idx) => {
            const globalIdx = rules.findIndex((r) => r.process_name === rule.process_name);
            const isDragging = draggingIndex === idx;
            const isOver = dragOverIndex === idx && draggingIndex !== idx;

            return (
              <div
                key={rule.process_name}
                draggable={canReorder}
                onDragStart={() => handleDragStart(idx)}
                onDragOver={(e) => handleDragOver(e, idx)}
                onDrop={() => handleDrop(idx)}
                onDragEnd={handleDragEnd}
                className={`group relative transition-all duration-200 ${
                  isDragging ? "opacity-30 scale-[0.98] z-30" : "z-10"
                } ${isOver ? "translate-y-[-4px] shadow-lg shadow-primary/20" : ""}`}
                style={{
                  outline: isOver ? "1px solid rgba(88,101,242,0.8)" : "1px solid transparent",
                  borderRadius: 6,
                }}
              >
                {/* Visual Priority Badge and Drag Handle */}
                {canReorder && (
                  <div className="absolute left-0 top-0 bottom-0 flex flex-col items-center justify-center w-8 z-10 cursor-grab active:cursor-grabbing select-none group-hover:bg-white/5 rounded-l-md transition-colors border-r border-transparent group-hover:border-hairline/20">
                    <span
                      className={`text-[9px] font-black leading-none mb-1 ${
                        globalIdx === 0 ? "text-magenta-accent" : "text-primary/80"
                      }`}
                    >
                      #{globalIdx + 1}
                    </span>
                    <div className="grid grid-cols-2 gap-[2px] opacity-20 group-hover:opacity-60 transition-opacity">
                      <div className="w-1 h-1 bg-white rounded-full" />
                      <div className="w-1 h-1 bg-white rounded-full" />
                      <div className="w-1 h-1 bg-white rounded-full" />
                      <div className="w-1 h-1 bg-white rounded-full" />
                      <div className="w-1 h-1 bg-white rounded-full" />
                      <div className="w-1 h-1 bg-white rounded-full" />
                    </div>
                  </div>
                )}

                <div className={canReorder ? "pl-8" : ""}>
                  <AppRuleCard
                    rule={rule}
                    onUpdate={updateRule}
                    onDelete={deleteRule}
                  />
                </div>
              </div>
            );
          })
        ) : (
          <div className="flex flex-col items-center justify-center py-16 px-4 bg-surface-indigo/20 rounded-md border border-dashed border-hairline/30 text-center">
            <span className="text-4xl mb-3 opacity-80">👻</span>
            <h4 className="text-ink font-semibold mb-1">No applications found</h4>
            <p className="text-xs text-muted-ink max-w-[250px]">
              {ruleSearch || filterSource !== "All"
                ? "Try clearing your filters or search term."
                : "You haven't added any applications to track yet."}
            </p>
            {(!ruleSearch && filterSource === "All") && (
              <button
                onClick={handleOpenPicker}
                className="mt-4 text-xs font-bold text-primary hover:text-primary-hover underline underline-offset-4"
              >
                Add first application
              </button>
            )}
          </div>
        )}
      </div>

      {/* ── System Process Command Palette (Modal) ── */}
      {showPicker && (
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
                onClick={() => {
                  setShowPicker(false);
                  setPickerSearch("");
                }}
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
                  {systemProcesses.filter(
                    (p) =>
                      p.display_name.toLowerCase().includes(pickerSearch.toLowerCase()) ||
                      p.process_name.toLowerCase().includes(pickerSearch.toLowerCase())
                  ).length > 0 ? (
                    <div className="p-2 flex flex-col gap-1">
                      {systemProcesses
                        .filter(
                          (p) =>
                            p.display_name.toLowerCase().includes(pickerSearch.toLowerCase()) ||
                            p.process_name.toLowerCase().includes(pickerSearch.toLowerCase())
                        )
                        .map((p) => {
                          const isAlreadyAdded = rules.some((r) => r.process_name === p.process_name);
                          const iconUrl = getIconUrl(p.process_name, p.display_name);
                          return (
                            <button
                              key={p.process_name}
                              onClick={() => !isAlreadyAdded && selectProcess(p)}
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
      )}
    </div>
  );
}
