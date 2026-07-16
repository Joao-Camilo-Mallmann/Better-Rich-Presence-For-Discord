import { useCallback } from "react";
import { AppRuleCard } from "../AppRuleCard/AppRuleCard";
import { AppRule } from "../../types";
import { Gamepad2 } from "lucide-react";

interface AppListProps {
  rules: AppRule[];
  filteredRules: AppRule[];
  canReorder: boolean;
  onUpdateRule: (rule: AppRule) => Promise<void>;
  onDeleteRule: (processName: string) => Promise<void>;
  onReorderRules: (newOrder: string[]) => Promise<void>;
  onEmptyActionClick?: () => void;
  ruleSearch: string;
}

export function AppList({
  rules,
  filteredRules,
  onUpdateRule,
  onDeleteRule,
  onReorderRules,
  onEmptyActionClick,
  ruleSearch,
}: AppListProps) {
  const moveRule = useCallback(async (currentIndex: number, direction: "up" | "down") => {
    const targetIndex = direction === "up" ? currentIndex - 1 : currentIndex + 1;
    if (targetIndex < 0 || targetIndex >= filteredRules.length) return;

    // Swap the elements in the filtered rules
    const newFiltered = [...filteredRules];
    const [moved] = newFiltered.splice(currentIndex, 1);
    newFiltered.splice(targetIndex, 0, moved);

    // Map back to the full list
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
  }, [filteredRules, rules, onReorderRules]);

  const hasFilters = !!ruleSearch.trim();

  return (
    <div className="flex flex-col gap-3 pb-8">
      {filteredRules.length > 0 ? (
        filteredRules.map((rule, idx) => {
          return (
            <div key={rule.process_name}>
              <AppRuleCard
                rule={rule}
                index={idx}
                totalRules={filteredRules.length}
                onUpdate={onUpdateRule}
                onDelete={onDeleteRule}
                onMoveUp={() => moveRule(idx, "up")}
                onMoveDown={() => moveRule(idx, "down")}
              />
            </div>
          );
        })
      ) : (
        <div className="flex flex-col items-center justify-center py-16 px-4 bg-surface-indigo text-center neo-border"
          style={{ borderStyle: 'dashed', borderRadius: '12px' }}>
          <Gamepad2 size={40} className="mb-3 text-muted-ink" />
          <h4 className="text-ink font-extrabold mb-1 font-display uppercase">No applications found</h4>
          <p className="text-xs text-muted-ink max-w-[250px]">
            {hasFilters
              ? "Try clearing your filters or search term."
              : "You haven't added any applications to track yet."}
          </p>
          {!hasFilters && onEmptyActionClick && (
            <button
              onClick={onEmptyActionClick}
              className="mt-4 neo-btn bg-primary text-white px-4 py-2 text-xs"
            >
              Add first application
            </button>
          )}
        </div>
      )}
    </div>
  );
}
