import { useState, useMemo } from "react";
import { useAppRules } from "../../hooks/useAppRules";
import { AppRule } from "../../types";
import { ProcessPicker } from "../../components/ProcessPicker/ProcessPicker";
import { AppList } from "../../components/AppList/AppList";
import { getDefaultProcessRules } from "../../utils/processDefaults";

export function Apps() {
  const { rules, loading, updateRule, deleteRule, addRule, reorderRules } = useAppRules();
  const [ruleSearch, setRuleSearch] = useState("");
  const [showPicker, setShowPicker] = useState(false);
  const [showJsonEditor, setShowJsonEditor] = useState(false);
  const [jsonInput, setJsonInput] = useState("");

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
              onClick={() => {
                setJsonInput(JSON.stringify(rules, null, 2));
                setShowJsonEditor(true);
              }}
              className="neo-btn bg-surface-onyx text-ink text-xs px-3.5 py-2"
              style={{ borderRadius: '6px' }}
            >
              JSON Editor
            </button>
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
            <span className="absolute left-3 top-1/2 -translate-y-1/2 text-muted-ink text-xs">
              🔍
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
              <span>↕️</span> Drag cards to reorder display priority
            </span>
          ) : (
            <span className="text-[10px] text-yellow-accent font-bold flex items-center gap-1.5 font-display uppercase">
              <span>⚠️</span> Ordering disabled due to active search
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

      {/* JSON Editor Modal */}
      {showJsonEditor && (
        <div className="fixed inset-0 bg-black/70 flex items-center justify-center p-4 z-50">
          <div className="bg-surface-indigo w-full max-w-2xl flex flex-col overflow-hidden neo-card neo-shadow-heavy"
            style={{ borderRadius: '12px' }}>
            <div className="p-4 flex justify-between items-center bg-primary"
              style={{ borderBottom: '3px solid var(--neo-border-color)' }}>
              <h3 className="font-display font-extrabold text-white uppercase">JSON Rule Editor</h3>
              <button onClick={() => setShowJsonEditor(false)}
                className="neo-btn bg-surface-black text-white px-2 py-1 text-[10px]"
                style={{ borderRadius: '4px' }}>
                ESC
              </button>
            </div>
            <textarea
              value={jsonInput}
              onChange={(e) => setJsonInput(e.target.value)}
              className="w-full h-[50vh] bg-surface-black text-ink p-4 font-mono text-xs focus:outline-none resize-none"
              spellCheck="false"
              style={{ borderBottom: '3px solid var(--neo-border-color)' }}
            />
            <div className="p-4 bg-surface-indigo flex justify-end gap-2">
              <button onClick={() => setShowJsonEditor(false)}
                className="neo-btn bg-surface-onyx text-ink px-4 py-2 text-xs"
                style={{ borderRadius: '6px' }}>
                Cancel
              </button>
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
                className="neo-btn bg-green-accent text-ink-dark px-4 py-2 text-xs font-extrabold"
                style={{ borderRadius: '6px' }}
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
