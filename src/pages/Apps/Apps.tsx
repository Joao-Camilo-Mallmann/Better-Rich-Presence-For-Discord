import { useState, useMemo } from "react";
import { useAppRules } from "../../hooks/useAppRules";
import { AppRule, PresenceSource } from "../../types";
import { ProcessPicker } from "../../components/ProcessPicker/ProcessPicker";
import { AppList } from "../../components/AppList/AppList";
import { getDefaultProcessRules } from "../../utils/processDefaults";

export function Apps() {
  const { rules, loading, updateRule, deleteRule, addRule, reorderRules } = useAppRules();
  const [filterSource, setFilterSource] = useState<"All" | PresenceSource>("All");
  const [ruleSearch, setRuleSearch] = useState("");
  const [showPicker, setShowPicker] = useState(false);

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

  const selectProcess = async (proc: { process_name: string; display_name: string }) => {
    if (rules.some((r) => r.process_name === proc.process_name)) {
      setShowPicker(false);
      return;
    }

    const defaults = getDefaultProcessRules(proc.process_name, proc.display_name);

    await addRule({
      process_name: proc.process_name,
      display_name: proc.display_name,
      details: defaults.details,
      state: defaults.state,
      large_image: "auto",
      source: defaults.source,
      priority: defaults.priority,
      enabled: true,
    } as AppRule);
    
    setShowPicker(false);
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
            onClick={() => setShowPicker(true)}
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

      {/* ── Rules List Component ── */}
      <AppList
        rules={rules}
        filteredRules={filteredRules}
        canReorder={canReorder}
        onUpdateRule={updateRule}
        onDeleteRule={deleteRule}
        onReorderRules={reorderRules}
        onEmptyActionClick={() => setShowPicker(true)}
        ruleSearch={ruleSearch}
        filterSource={filterSource}
      />

      {/* ── System Process Command Palette (Modal) ── */}
      {showPicker && (
        <ProcessPicker
          onClose={() => setShowPicker(false)}
          onSelect={selectProcess}
          existingRules={rules}
        />
      )}
    </div>
  );
}
