import { Gamepad2 } from "lucide-react";
import { AppRule } from "../types";
import { AppRuleCard } from "./AppRuleCard";

interface AppListProps {
  rules: AppRule[];
  filteredRules: AppRule[];
  canReorder: boolean;
  onUpdateRule: (rule: AppRule) => Promise<void>;
  onDeleteRule: (processName: string) => Promise<void>;
  onReorderRules: (newOrder: string[]) => Promise<void>;
  ruleSearch: string;
}

export function AppList({
  rules,
  filteredRules,
  canReorder,
  onUpdateRule,
  onDeleteRule,
  onReorderRules,
  ruleSearch,
}: AppListProps) {
  const moveRuleToIndex = async (currentIndex: number, targetIndex: number) => {
    if (
      targetIndex < 0 ||
      targetIndex >= filteredRules.length ||
      currentIndex === targetIndex
    )
      return;

    const newFiltered = [...filteredRules];
    const [moved] = newFiltered.splice(currentIndex, 1);
    newFiltered.splice(targetIndex, 0, moved);

    // Map the filtered order back onto the full list.
    const filteredProcessNames = new Set(newFiltered.map((r) => r.process_name));
    const newFullRules: AppRule[] = [];
    let filteredIdx = 0;
    for (const r of rules) {
      if (filteredProcessNames.has(r.process_name)) {
        newFullRules.push(newFiltered[filteredIdx++]);
      } else {
        newFullRules.push(r);
      }
    }

    try {
      await onReorderRules(newFullRules.map((r) => r.process_name));
    } catch (err) {
      console.error("Failed to reorder rules:", err);
    }
  };

  if (filteredRules.length > 0) {
    return (
      <ol className="flex flex-col gap-3 pr-1">
        {filteredRules.map((rule, idx) => (
          <li key={rule.process_name}>
            <AppRuleCard
              rule={rule}
              index={idx}
              totalRules={filteredRules.length}
              canReorder={canReorder}
              onUpdate={onUpdateRule}
              onDelete={onDeleteRule}
              onMoveUp={() => moveRuleToIndex(idx, idx - 1)}
              onMoveDown={() => moveRuleToIndex(idx, idx + 1)}
              onMoveTo={(newIndex) => moveRuleToIndex(idx, newIndex)}
            />
          </li>
        ))}
      </ol>
    );
  }

  const searching = ruleSearch.trim() !== "";

  return (
    <div
      className="flex flex-col items-center justify-center py-12 px-6 bg-surface-indigo text-center neo-border"
      style={{ borderStyle: "dashed", borderRadius: "12px" }}
    >
      <Gamepad2 size={36} className="mb-3 text-muted-ink" />
      <h3 className="text-[14px] text-ink font-extrabold mb-1 font-display uppercase">
        {searching ? "No matching apps" : "No apps yet"}
      </h3>
      <p className="text-[13px] leading-snug text-muted-ink max-w-[260px]">
        {searching
          ? "No app name or process name contains your search."
          : "Select Add app to pick an app that's running now. Discord shows it while it's open."}
      </p>
    </div>
  );
}
