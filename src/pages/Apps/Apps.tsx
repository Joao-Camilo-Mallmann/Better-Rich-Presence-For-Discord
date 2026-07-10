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
  const [showJsonEditor, setShowJsonEditor] = useState(false);
  const [jsonInput, setJsonInput] = useState("");

  // Only allow drag-and-drop when no filters are active to maintain accurate positions
  const canReorder = filterSource === "All" && ruleSearch.trim() === "";

  const filteredRules = useMemo(() => {
    const search = ruleSearch.toLowerCase();
    return rules.filter((r) =>
      (filterSource === "All" || r.source === filterSource) &&
      (r.display_name.toLowerCase().includes(search) || r.process_name.toLowerCase().includes(search))
    );
  }, [rules, filterSource, ruleSearch]);

  const selectProcess = async (proc: { process_name: string; display_name: string }) => {
    if (rules.some((r) => r.process_name === proc.process_name)) return setShowPicker(false);
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
      <div className="pb-4 pt-1 flex flex-col gap-4 mb-4">
        <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-3">
          <div className="flex flex-col">
            <h3 className="text-lg text-ink font-bold font-display">Configured Applications</h3>
            <span className="text-xs text-muted-ink/80 mt-0.5 max-w-[280px]">
              Manage priority and appearance of apps on your Discord.
            </span>
          </div>
          <div className="flex gap-2">
            <button
              onClick={() => {
                setJsonInput(JSON.stringify(rules, null, 2));
                setShowJsonEditor(true);
              }}
              className="w-fit sm:w-auto justify-center bg-surface-onyx hover:bg-white/10 text-ink text-xs px-3.5 py-1.5 rounded-sm font-semibold transition-all shadow-sm border border-hairline/30 active:scale-95"
            >
              JSON Editor
            </button>
            <button
              onClick={() => setShowPicker(true)}
              className="w-fit sm:w-auto justify-center bg-primary hover:bg-primary-hover text-white text-xs px-3.5 py-1.5 rounded-sm font-semibold transition-all shadow-sm flex items-center gap-1.5 active:scale-95"
            >
              <span>+</span> Fetch from System
            </button>
          </div>
        </div>

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
            {["All", "Game", "Work", "Browser"].map((filter) => (
              <button
                key={filter}
                onClick={() => setFilterSource(filter as any)}
                className={`px-3 py-1.5 rounded-full text-xs font-medium transition-all whitespace-nowrap ${
                  filterSource === filter
                    ? "bg-primary text-white shadow-sm"
                    : "bg-surface-indigo border border-hairline/30 text-muted-ink hover:text-ink hover:bg-surface-indigo/80 hover:border-hairline/60"
                }`}
              >
                {filter === "All" ? "All Apps" : filter}
              </button>
            ))}
          </div>
        </div>

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

      {showPicker && (
        <ProcessPicker
          onClose={() => setShowPicker(false)}
          onSelect={selectProcess}
          existingRules={rules}
        />
      )}

      {showJsonEditor && (
        <div className="fixed inset-0 bg-black/60 backdrop-blur-sm flex items-center justify-center p-4 z-50">
          <div className="bg-surface-base border border-hairline/50 rounded-lg w-full max-w-2xl flex flex-col overflow-hidden shadow-2xl">
            <div className="p-4 border-b border-hairline/40 bg-surface-indigo flex justify-between items-center">
              <h3 className="font-display font-bold text-ink">JSON Rule Editor</h3>
              <button onClick={() => setShowJsonEditor(false)} className="text-muted-ink hover:text-ink text-xs font-bold bg-surface-onyx px-2 py-1 rounded-sm border border-hairline">ESC</button>
            </div>
            <textarea
              value={jsonInput}
              onChange={(e) => setJsonInput(e.target.value)}
              className="w-full h-[50vh] bg-[#1e1e1e] text-[#d4d4d4] p-4 font-mono text-xs focus:outline-none resize-none"
              spellCheck="false"
            />
            <div className="p-4 bg-surface-indigo border-t border-hairline/30 flex justify-end gap-2">
              <button onClick={() => setShowJsonEditor(false)} className="text-xs px-4 py-2 font-medium text-muted-ink hover:text-ink">Cancel</button>
              <button 
                onClick={async () => {
                  try {
                    const parsed = JSON.parse(jsonInput);
                    if (Array.isArray(parsed)) {
                      await reorderRules(parsed.map(r => r.process_name));
                      for (const r of parsed) {
                        await updateRule(r);
                      }
                      setShowJsonEditor(false);
                    }
                  } catch (e) {
                    alert("Invalid JSON format");
                  }
                }}
                className="text-xs px-4 py-2 bg-primary hover:bg-primary-hover text-white rounded-sm font-bold shadow-sm"
              >
                Save Changes
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
