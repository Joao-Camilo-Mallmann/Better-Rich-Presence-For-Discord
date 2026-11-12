import { Plus, Search } from "lucide-react";
import { useState } from "react";
import { AppList } from "../components/AppList";
import { ProcessPicker } from "../components/ProcessPicker";
import { useAppRules } from "../hooks/useAppRules";
import { AppRule } from "../types";
import { getDefaultProcessRules } from "../utils/processDefaults";

interface AppsProps {
  /** Whether Max priority mode is on, which is when list order matters. */
  priorityMode: boolean;
}

/** The "Apps" view: the rules that decide what Discord shows for each app. */
export function Apps({ priorityMode }: AppsProps) {
  const { rules, loading, updateRule, deleteRule, addRule, reorderRules } =
    useAppRules();
  const [ruleSearch, setRuleSearch] = useState("");
  const [showPicker, setShowPicker] = useState(false);

  // Reordering a filtered list would hide where an app lands, so it needs an empty search.
  const canReorder = ruleSearch.trim() === "";

  const search = ruleSearch.toLowerCase();
  const filteredRules = rules.filter(
    (r) =>
      r.display_name.toLowerCase().includes(search) ||
      r.process_name.toLowerCase().includes(search),
  );

  const selectProcess = async (proc: {
    process_name: string;
    display_name: string;
  }) => {
    if (rules.some((r) => r.process_name === proc.process_name))
      return setShowPicker(false);
    const defaults = getDefaultProcessRules(
      proc.process_name,
      proc.display_name,
    );
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
      <div className="flex items-center justify-center py-16">
        <div className="flex flex-col items-center gap-3">
          <div
            className="w-8 h-8 neo-border-2 bg-primary"
            style={{ animation: "neo-spin 0.8s linear infinite" }}
          />
          <span className="text-muted-ink text-[13px] font-bold font-display uppercase">
            Loading apps…
          </span>
        </div>
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-start justify-between gap-3">
        <div className="flex flex-col gap-1 min-w-0">
          <h2 className="text-[15px] text-ink font-extrabold font-display uppercase">
            Your apps
          </h2>
          <p className="text-[12px] leading-snug text-muted-ink">
            {priorityMode
              ? "Max priority mode is on: the highest running app in this list is shown."
              : "Discord shows the app in focus. List order sets priority in Max priority mode."}
          </p>
        </div>
        <button
          onClick={() => setShowPicker(true)}
          className="neo-btn shrink-0 bg-green-accent text-ink-dark text-[12px] px-3 py-2 flex items-center gap-1.5"
          style={{ borderRadius: "6px" }}
        >
          <Plus size={14} strokeWidth={3.5} /> Add app
        </button>
      </div>

      {rules.length > 0 && (
        <label
          className="relative w-full neo-border-2 bg-surface-onyx overflow-hidden"
          style={{ borderRadius: "8px" }}
        >
          <span className="sr-only">Search apps</span>
          <span className="absolute left-3 top-1/2 -translate-y-1/2 text-muted-ink flex items-center pointer-events-none">
            <Search size={16} />
          </span>
          <input
            type="search"
            placeholder="Search your apps"
            value={ruleSearch}
            onChange={(e) => setRuleSearch(e.target.value)}
            className="w-full bg-transparent text-ink text-[13px] pl-10 pr-4 py-2.5 focus:outline-none placeholder:text-muted-ink"
          />
        </label>
      )}

      <AppList
        rules={rules}
        filteredRules={filteredRules}
        canReorder={canReorder}
        onUpdateRule={updateRule}
        onDeleteRule={deleteRule}
        onReorderRules={reorderRules}
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
