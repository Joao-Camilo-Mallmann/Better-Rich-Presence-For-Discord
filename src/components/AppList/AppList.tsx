import { useState, useRef } from "react";
import { AppRuleCard } from "../AppRuleCard/AppRuleCard";
import { AppRule } from "../../types";

interface AppListProps {
  rules: AppRule[];
  filteredRules: AppRule[];
  canReorder: boolean;
  onUpdateRule: (rule: AppRule) => Promise<void>;
  onDeleteRule: (processName: string) => Promise<void>;
  onReorderRules: (newOrder: string[]) => Promise<void>;
  onEmptyActionClick?: () => void;
  ruleSearch: string;
  filterSource: string;
}

export function AppList({
  rules,
  filteredRules,
  canReorder,
  onUpdateRule,
  onDeleteRule,
  onReorderRules,
  onEmptyActionClick,
  ruleSearch,
  filterSource,
}: AppListProps) {
  // Drag-and-drop state
  const [draggingIndex, setDraggingIndex] = useState<number | null>(null);
  const [dragOverIndex, setDragOverIndex] = useState<number | null>(null);
  const dragItem = useRef<number | null>(null);

  const handleDragStart = (e: React.DragEvent, index: number) => {
    e.dataTransfer.effectAllowed = "move";
    e.dataTransfer.setData("text/plain", index.toString());
    dragItem.current = index;
    setDraggingIndex(index);
  };

  const handleDragOver = (e: React.DragEvent, index: number) => {
    e.preventDefault();
    setDragOverIndex(index);
  };

  const handleDrop = async (e: React.DragEvent, dropIndex: number) => {
    e.preventDefault();
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
      await onReorderRules(newOrder);
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

  return (
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
              onDragStart={(e) => handleDragStart(e, idx)}
              onDragOver={(e) => handleDragOver(e, idx)}
              onDragEnter={(e) => e.preventDefault()}
              onDrop={(e) => handleDrop(e, idx)}
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
                  onUpdate={onUpdateRule}
                  onDelete={onDeleteRule}
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
          {!ruleSearch && filterSource === "All" && onEmptyActionClick && (
            <button
              onClick={onEmptyActionClick}
              className="mt-4 text-xs font-bold text-primary hover:text-primary-hover underline underline-offset-4"
            >
              Add first application
            </button>
          )}
        </div>
      )}
    </div>
  );
}
