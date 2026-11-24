import { Plus, Search } from "lucide-react";
import { useState } from "react";
import { AppList } from "../components/AppList";
import { useClaudeCode } from "../hooks/useClaudeCode";
import { usePresence } from "../hooks/usePresence";
import { ProcessPicker } from "../components/ProcessPicker";
import { SegmentedControl } from "../components/SegmentedControl";
import { useAppRules } from "../hooks/useAppRules";
import { AppRule } from "../types";
import { getDefaultProcessRules } from "../utils/processDefaults";

interface AppsProps {
  /** Whether Priority list mode is on, which is when list order matters. */
  priorityMode: boolean;
  /** Saves the presence mode right away; there is no separate save step. */
  onPriorityModeChange: (priorityMode: boolean) => void;
}

const PRESENCE_MODES: {
  value: "dynamic" | "priority";
  label: string;
  hint: string;
}[] = [
  {
    value: "dynamic",
    label: "Dynamic",
    hint: "Discord shows whatever app you're using and switches along with you.",
  },
  {
    value: "priority",
    label: "Priority list",
    hint: "Discord sticks to the highest app in this list that's running, even while you use something else.",
  },
];

/** The Claude Code entry's preview line: the live session, or that none is active. */
function claudeCodeLineFor(status: ReturnType<typeof useClaudeCode>["status"]) {
  if (!status || status.active_sessions === 0) return "Waiting for a session";
  const parts = [status.active_activity, status.active_project].filter(Boolean);
  return parts.length > 0 ? parts.join(" · ") : "Active";
}

/** The "Apps" view: the rules that decide what Discord shows for each app. */
export function Apps({ priorityMode, onPriorityModeChange }: AppsProps) {
  const { status: claudeStatus } = useClaudeCode();
  const claudeCodeLine = claudeCodeLineFor(claudeStatus);
  // Only IDE presences carry a language badge; the matching row shows it live.
  const { presence } = usePresence();
  const livePresence = presence?.small_image ? presence : null;
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
      <div className="flex flex-col gap-2">
        <div className="flex items-center justify-between gap-3">
          <h2 className="text-[15px] text-ink font-extrabold font-display uppercase min-w-0">
            Your apps
          </h2>
          <button
            onClick={() => setShowPicker(true)}
            className="neo-btn shrink-0 bg-green-accent text-ink-dark text-[12px] px-3 py-2 flex items-center gap-1.5"
            style={{ borderRadius: "6px" }}
          >
            <Plus size={14} strokeWidth={3.5} /> Add app
          </button>
        </div>
        <div className="flex">
          <SegmentedControl
            label="Presence mode"
            options={PRESENCE_MODES}
            value={priorityMode ? "priority" : "dynamic"}
            onChange={(mode) => onPriorityModeChange(mode === "priority")}
          />
        </div>
        <p className="text-[12px] leading-snug text-muted-ink">
          {priorityMode
            ? "The highest running app in this list is shown, and Claude Code shows only over apps below it."
            : "Discord shows the app in focus. Claude Code shows over apps below it in this list."}
        </p>
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
        claudeCodeLine={claudeCodeLine}
        livePresence={livePresence}
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
