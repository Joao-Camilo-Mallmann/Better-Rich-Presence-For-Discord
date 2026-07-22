import { useState } from "react";
import { AppRule } from "../types";
import { AppIcon } from "./AppIcon";

interface AppRuleCardProps {
  rule: AppRule;
  index: number;
  totalRules: number;
  onUpdate: (rule: AppRule) => void;
  onDelete: (processName: string) => void;
  onMoveUp: () => void;
  onMoveDown: () => void;
  onMoveTo: (newIndex: number) => void;
}

const labelCls =
  "text-[10px] text-muted-ink font-extrabold uppercase tracking-wider font-display";

export function AppRuleCard({
  rule,
  index,
  totalRules,
  onUpdate,
  onDelete,
  onMoveUp,
  onMoveDown,
  onMoveTo,
}: AppRuleCardProps) {
  const [expanded, setExpanded] = useState(false);
  const [editRule, setEditRule] = useState<AppRule>({ ...rule });
  const [isEditingPriority, setIsEditingPriority] = useState(false);
  const [priorityInput, setPriorityInput] = useState("");

  const handlePrioritySubmit = () => {
    setIsEditingPriority(false);
    const parsed = parseInt(priorityInput, 10);
    if (!isNaN(parsed) && parsed > 0 && parsed <= totalRules) {
      onMoveTo(parsed - 1);
    }
  };

  const handleToggle = (e: React.MouseEvent) => {
    e.stopPropagation();
    onUpdate({ ...rule, enabled: !rule.enabled });
  };

  const patch = (partial: Partial<AppRule>) =>
    setEditRule({ ...editRule, ...partial });

  const fields: { label: string; key: keyof AppRule; row?: number }[] = [
    { label: "Display Name", key: "display_name" },
    { label: "Details (Line 1)", key: "details", row: 1 },
    { label: "State (Line 2)", key: "state", row: 1 },
    { label: "Image (Asset Key)", key: "large_image", row: 2 },
    { label: "Discord Client ID (Opcional)", key: "client_id", row: 2 },
  ];

  const renderField = (label: string, field: keyof AppRule) => (
    <div className="flex flex-col gap-1 flex-1" key={field}>
      <label className={labelCls}>{label}</label>
      <input
        className="neo-input text-sm"
        type="text"
        value={(editRule[field] as string) || ""}
        onChange={(e) => patch({ [field]: e.target.value })}
      />
    </div>
  );

  return (
    <div
      className={`neo-border bg-surface-indigo overflow-hidden transition-all duration-100 ${!rule.enabled ? "opacity-50 hover:opacity-70" : ""}`}
      style={{
        borderRadius: "8px",
        boxShadow: expanded
          ? "0 0 0 var(--neo-shadow-color)"
          : "4px 4px 0 var(--neo-shadow-color)",
      }}
    >
      <div
        className="p-3 flex justify-between items-center gap-3 cursor-pointer hover:bg-white/5 transition-colors"
        onClick={() => !expanded && setExpanded(true)}
      >
        <div className="flex items-center gap-3 flex-1 min-w-0">
          {/* Priority slot badge inside the card */}
          {isEditingPriority ? (
            <input
              type="number"
              className="text-[10px] font-black text-ink bg-surface-onyx px-1 py-1 neo-border-2 font-display shrink-0 w-10 text-center focus:outline-none"
              style={{ borderRadius: "4px", borderColor: "var(--neo-border-color)", MozAppearance: "textfield" }}
              autoFocus
              value={priorityInput}
              onChange={(e) => setPriorityInput(e.target.value)}
              onBlur={handlePrioritySubmit}
              onKeyDown={(e) => {
                if (e.key === "Enter") handlePrioritySubmit();
                if (e.key === "Escape") setIsEditingPriority(false);
              }}
              onClick={(e) => e.stopPropagation()}
            />
          ) : (
            <div
              className="text-[10px] font-black text-ink bg-surface-onyx px-2 py-1 neo-border-2 select-none font-display shrink-0 cursor-text hover:bg-white/10 transition-colors"
              style={{
                borderRadius: "4px",
                borderColor: "var(--neo-border-color)",
              }}
              onClick={(e) => {
                e.stopPropagation();
                setPriorityInput((index + 1).toString());
                setIsEditingPriority(true);
              }}
              title="Click to edit priority"
            >
              #{index + 1}
            </div>
          )}
          <div
            className="w-10 h-10 bg-surface-onyx flex items-center justify-center overflow-hidden flex-shrink-0 neo-border-2"
            style={{ borderRadius: "6px" }}
          >
            <AppIcon
              name={
                rule.large_image && rule.large_image !== "auto"
                  ? rule.large_image
                  : rule.process_name
              }
              className="w-7 h-7 object-contain"
              size={28}
            />
          </div>
          <div className="flex flex-col gap-0.5 min-w-0 flex-1">
            <div className="flex items-center gap-2 flex-wrap">
              <h3 className="text-sm text-ink font-extrabold m-0 truncate font-display">
                {rule.display_name}
              </h3>
            </div>
            <div className="text-[10px] text-muted-ink font-mono truncate">
              {rule.process_name}
            </div>
            <div className="text-xs text-muted-ink truncate">
              "{rule.details}" • "{rule.state}"
            </div>
          </div>
        </div>
        <div
          className="flex items-center gap-3 flex-shrink-0"
          onClick={(e) => e.stopPropagation()}
        >
          {/* Priority Up/Down arrow buttons */}
          <div className="flex flex-col gap-[3px]">
            <button
              onClick={onMoveUp}
              disabled={index === 0}
              className="w-5 h-5 flex items-center justify-center text-[10px] font-black bg-surface-onyx text-ink neo-border-2 neo-press disabled:opacity-30 disabled:pointer-events-none"
              style={{ borderRadius: "3px" }}
              title="Move Up (Increase Priority)"
            >
              ▲
            </button>
            <button
              onClick={onMoveDown}
              disabled={index === totalRules - 1}
              className="w-5 h-5 flex items-center justify-center text-[10px] font-black bg-surface-onyx text-ink neo-border-2 neo-press disabled:opacity-30 disabled:pointer-events-none"
              style={{ borderRadius: "3px" }}
              title="Move Down (Decrease Priority)"
            >
              ▼
            </button>
          </div>

          {/* Arcade Toggle */}
          <div
            className={`neo-toggle ${rule.enabled ? "active" : ""}`}
            onClick={handleToggle}
            title={rule.enabled ? "Disable" : "Enable"}
          >
            <div className="neo-toggle-knob" />
          </div>
        </div>
      </div>

      {expanded && (
        <div
          className="p-4 bg-surface-onyx flex flex-col gap-3"
          style={{ borderTop: "3px solid var(--neo-border-color)" }}
        >
          {renderField(fields[0].label, fields[0].key)}
          <div className="flex flex-col sm:flex-row gap-4">
            {fields
              .filter((f) => f.row === 1)
              .map((f) => renderField(f.label, f.key))}
          </div>
          <div className="flex flex-col sm:flex-row gap-4">
            {fields
              .filter((f) => f.row === 2)
              .map((f) => renderField(f.label, f.key))}
          </div>
          <div className="flex flex-col-reverse sm:flex-row justify-between mt-4 gap-3">
            <button
              className="neo-btn bg-danger text-white px-4 py-2 text-sm w-full sm:w-auto"
              onClick={() => onDelete(rule.process_name)}
            >
              Delete
            </button>
            <div className="flex gap-2 w-full sm:w-auto">
              <button
                className="neo-btn bg-surface-indigo text-ink px-4 py-2 text-sm flex-1 sm:flex-none"
                onClick={() => {
                  setEditRule({ ...rule });
                  setExpanded(false);
                }}
              >
                Cancel
              </button>
              <button
                className="neo-btn bg-green-accent text-ink-dark px-4 py-2 text-sm font-extrabold flex-1 sm:flex-none"
                onClick={() => {
                  onUpdate(editRule);
                  setExpanded(false);
                }}
              >
                Save
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
