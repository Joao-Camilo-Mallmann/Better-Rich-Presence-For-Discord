import { useState, useRef, useCallback, useEffect } from "react";
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

/** Minimum pixels the pointer must move before a drag gesture is recognized. */
const DRAG_THRESHOLD = 5;

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
  const [draggingIndex, setDraggingIndex] = useState<number | null>(null);
  const [dragOverIndex, setDragOverIndex] = useState<number | null>(null);
  const [optimisticRules, setOptimisticRules] = useState<AppRule[] | null>(null);

  const pointerStartY = useRef(0);
  const pointerStartIdx = useRef<number | null>(null);
  const isDragging = useRef(false);
  const didDrag = useRef(false);
  const itemRefs = useRef<Map<number, HTMLDivElement>>(new Map());
  const containerRef = useRef<HTMLDivElement>(null);

  const displayRules = optimisticRules ?? filteredRules;

  const commitReorder = useCallback(async () => {
    const finalOrder = optimisticRules;
    if (!finalOrder || !didDrag.current) {
      setDraggingIndex(null);
      setDragOverIndex(null);
      setOptimisticRules(null);
      return;
    }

    // Map the optimistic filtered order back into the full rules list
    const filteredProcessNames = new Set(finalOrder.map((r) => r.process_name));
    const newFullRules: AppRule[] = [];
    let filteredIdx = 0;

    for (const r of rules) {
      if (filteredProcessNames.has(r.process_name)) {
        newFullRules.push(finalOrder[filteredIdx++]);
      } else {
        newFullRules.push(r);
      }
    }

    setDraggingIndex(null);
    setDragOverIndex(null);

    try {
      await onReorderRules(newFullRules.map((r) => r.process_name));
    } catch (err) {
      console.error("Failed to reorder rules:", err);
    } finally {
      setOptimisticRules(null);
    }
  }, [optimisticRules, rules, onReorderRules]);

  const handlePointerDown = useCallback(
    (e: React.PointerEvent, index: number) => {
      if (e.button !== 0 || !canReorder) return;

      // Don't initiate drag on interactive elements
      const target = e.target as HTMLElement;
      if (target.closest("button, input, select, textarea, a, label")) return;

      pointerStartY.current = e.clientY;
      pointerStartIdx.current = index;
      isDragging.current = false;
      didDrag.current = false;
    },
    [canReorder]
  );

  useEffect(() => {
    const handlePointerMove = (e: PointerEvent) => {
      if (pointerStartIdx.current === null) return;

      const dy = Math.abs(e.clientY - pointerStartY.current);

      // Start the drag after passing the threshold
      if (!isDragging.current && dy > DRAG_THRESHOLD) {
        isDragging.current = true;
        setDraggingIndex(pointerStartIdx.current);
        setOptimisticRules(null);
      }

      if (!isDragging.current) return;

      // Determine which item the pointer is currently over
      for (const [idx, el] of itemRefs.current.entries()) {
        const rect = el.getBoundingClientRect();
        if (e.clientY >= rect.top && e.clientY <= rect.bottom) {
          if (idx !== dragOverIndex) setDragOverIndex(idx);

          // Optimistically swap if the pointer crossed into a different item
          setOptimisticRules((prev) => {
            const current = prev ?? [...filteredRules];
            const fromIdx = pointerStartIdx.current!;
            if (fromIdx === idx) return current;

            didDrag.current = true;
            const reordered = [...current];
            const [moved] = reordered.splice(fromIdx, 1);
            reordered.splice(idx, 0, moved);
            pointerStartIdx.current = idx;
            return reordered;
          });
          break;
        }
      }
    };

    const handlePointerUp = () => {
      if (pointerStartIdx.current === null) return;
      const wasDragging = isDragging.current;
      isDragging.current = false;
      pointerStartIdx.current = null;

      if (wasDragging && didDrag.current) {
        commitReorder();
      } else {
        setDraggingIndex(null);
        setDragOverIndex(null);
        setOptimisticRules(null);
      }
    };

    window.addEventListener("pointermove", handlePointerMove);
    window.addEventListener("pointerup", handlePointerUp);
    return () => {
      window.removeEventListener("pointermove", handlePointerMove);
      window.removeEventListener("pointerup", handlePointerUp);
    };
  }, [filteredRules, dragOverIndex, commitReorder]);

  const setItemRef = useCallback((index: number, el: HTMLDivElement | null) => {
    if (el) itemRefs.current.set(index, el);
    else itemRefs.current.delete(index);
  }, []);

  const hasFilters = ruleSearch || filterSource !== "All";

  return (
    <div className="flex flex-col gap-2.5 pb-8" ref={containerRef}>
      {displayRules.length > 0 ? (
        displayRules.map((rule, idx) => {
          const globalIdx = rules.findIndex((r) => r.process_name === rule.process_name);
          const isItemDragging = draggingIndex === idx;
          const isOver = dragOverIndex === idx && draggingIndex !== idx;

          return (
            <div
              key={rule.process_name}
              ref={(el) => setItemRef(idx, el)}
              onPointerDown={(e) => handlePointerDown(e, idx)}
              onClickCapture={(e) => {
                // Suppress click events fired after a drag gesture
                // so the AppRuleCard does not accidentally expand.
                if (didDrag.current) {
                  e.stopPropagation();
                  e.preventDefault();
                  didDrag.current = false;
                }
              }}
              className={`group relative transition-all duration-200 ${
                isItemDragging ? "opacity-30 scale-[0.98] z-30" : "z-10"
              } ${isOver ? "translate-y-[-4px] shadow-lg shadow-primary/20" : ""}`}
              style={{
                outline: isOver ? "1px solid rgba(88,101,242,0.8)" : "1px solid transparent",
                borderRadius: 6,
                touchAction: canReorder ? "none" : "auto",
              }}
            >
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
                    {Array(6).fill(0).map((_, i) => (
                      <div key={i} className="w-1 h-1 bg-white rounded-full" />
                    ))}
                  </div>
                </div>
              )}
              <div className={canReorder ? "pl-8" : ""}>
                <AppRuleCard rule={rule} onUpdate={onUpdateRule} onDelete={onDeleteRule} />
              </div>
            </div>
          );
        })
      ) : (
        <div className="flex flex-col items-center justify-center py-16 px-4 bg-surface-indigo/20 rounded-md border border-dashed border-hairline/30 text-center">
          <span className="text-4xl mb-3 opacity-80">👻</span>
          <h4 className="text-ink font-semibold mb-1">No applications found</h4>
          <p className="text-xs text-muted-ink max-w-[250px]">
            {hasFilters
              ? "Try clearing your filters or search term."
              : "You haven't added any applications to track yet."}
          </p>
          {!hasFilters && onEmptyActionClick && (
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
