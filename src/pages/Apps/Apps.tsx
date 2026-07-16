import { useState, useMemo } from "react";
import { useAppRules } from "../../hooks/useAppRules";
import { AppRule } from "../../types";
import { ProcessPicker } from "../../components/ProcessPicker/ProcessPicker";
import { AppList } from "../../components/AppList/AppList";
import { getDefaultProcessRules } from "../../utils/processDefaults";
import { Search, ArrowUpDown, AlertTriangle } from "lucide-react";

export function Apps() {
  const { rules, loading, updateRule, deleteRule, addRule, reorderRules } = useAppRules();
  const [ruleSearch, setRuleSearch] = useState("");
  const [showPicker, setShowPicker] = useState(false);

  // Only allow drag-and-drop when no search is active to maintain accurate positions
  const canReorder = ruleSearch.trim() === "";

  const filteredRules = useMemo(() => {
    const search = ruleSearch.toLowerCase();
    return rules.filter((r) =>
      r.display_name.toLowerCase().includes(search) || r.process_name.toLowerCase().includes(search)
    );
  }, [rules, ruleSearch]);

  const selectProcess = async (proc: { process_name: string; display_name: string }) => {
    if (rules.some((r) => r.process_name === proc.process_name)) return setShowPicker(false);
    const defaults = getDefaultProcessRules(proc.process_name, proc.display_name);
    await addRule({
      process_name: proc.process_name,
      display_name: proc.display_name,
      details: defaults.details,
      state: defaults.state,
      large_image: "auto",
      enabled: true,
    } as AppRule);
    setShowPicker(false);
  };

  if (loading) {
    return (
      <div className="flex h-full items-center justify-center p-8">
        <div className="flex flex-col items-center gap-3">
          {/* Chunky spinning square */}
          <div className="w-8 h-8 neo-border-2 bg-primary"
            style={{ animation: 'neo-spin 0.8s linear infinite' }} />
          <span className="text-muted-ink text-sm font-bold font-display uppercase" style={{ animation: 'neo-pulse 2s infinite' }}>
            Loading applications...
          </span>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full relative">
      <div className="pb-4 pt-1 flex flex-col gap-4 mb-4">
        {/* Header */}
        <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-3">
          <div className="flex flex-col">
            <h3 className="text-lg text-ink font-extrabold font-display uppercase">Configured Applications</h3>
            <span className="text-xs text-muted-ink mt-0.5 max-w-[280px]">
              Manage priority and appearance of apps on your Discord.
            </span>
          </div>
          <div className="flex gap-2">
            <button
              onClick={() => setShowPicker(true)}
              className="neo-btn bg-green-accent text-ink-dark text-xs px-3.5 py-2 flex items-center gap-1.5"
              style={{ borderRadius: '6px' }}
            >
              <span className="text-base font-black">+</span> Fetch from System
            </button>
          </div>
        </div>

        {/* Search */}
        <div className="flex flex-col sm:flex-row gap-3 sm:items-center justify-between p-2 neo-border-2 bg-surface-onyx"
          style={{ borderRadius: '8px' }}>
          <div className="relative flex-1 w-full">
            <span className="absolute left-3 top-1/2 -translate-y-1/2 text-muted-ink flex items-center">
              <Search size={14} />
            </span>
            <input
              type="text"
              placeholder="Search applications..."
              value={ruleSearch}
              onChange={(e) => setRuleSearch(e.target.value)}
              className="w-full neo-input text-xs pl-9 pr-3 py-2"
              style={{ borderRadius: '6px' }}
            />
          </div>
        </div>

        {/* Reorder hint */}
        <div className="h-4 flex items-center justify-end">
          {canReorder ? (
            <span className="text-[10px] text-muted-ink font-bold flex items-center gap-1.5 font-display uppercase">
              <ArrowUpDown size={12} /> Drag cards to reorder display priority
            </span>
          ) : (
            <span className="text-[10px] text-yellow-accent font-bold flex items-center gap-1.5 font-display uppercase">
              <AlertTriangle size={12} /> Ordering disabled due to active search
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
      />

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
