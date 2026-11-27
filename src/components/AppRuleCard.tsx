import { ChevronDown, ChevronRight, ChevronUp } from "lucide-react";
import { useState } from "react";
import APP_CATALOG_JSON from "../../apps.json";
import { AppDefinition } from "../apps/app-types";
import { AppRule, PresenceData } from "../types";
import { AppIcon } from "./AppIcon";
import { Toggle } from "./Toggle";

interface AppRuleCardProps {
  rule: AppRule;
  index: number;
  totalRules: number;
  /** False while the list is filtered: positions are hidden and moving is disabled. */
  canReorder: boolean;
  onUpdate: (rule: AppRule) => void;
  onDelete: (processName: string) => void;
  onMoveUp: () => void;
  onMoveDown: () => void;
  onMoveTo: (newIndex: number) => void;
  /** Live preview line, used only by the Claude Code entry. */
  claudeCodeLine?: string;
  /** The presence shown right now, when it carries a language badge. */
  livePresence?: PresenceData | null;
}

/** `process_name` of the reserved entry that ranks Claude Code sessions. */
export const CLAUDE_CODE_RULE_ID = "claude_code";

const appCatalog = APP_CATALOG_JSON as AppDefinition[];

function normalizeExe(name: string): string {
  const lower = name.trim().toLowerCase();
  return lower.endsWith(".exe") ? lower.slice(0, -4) : lower;
}

/** Whether the live presence (named by its `large_text`, e.g. "Visual Studio
 *  Code" or a custom rule's display name) belongs to the app this rule matches. */
function isLiveFor(
  processName: string,
  displayName: string,
  presenceName: string,
): boolean {
  const target = presenceName.trim().toLowerCase();
  if (!target) return false;
  if (displayName.trim().toLowerCase() === target) {
    return true;
  }
  const exe = normalizeExe(processName);
  const app = appCatalog.find((entry) =>
    entry.executables.some((candidate) => normalizeExe(candidate) === exe),
  );
  return app !== undefined && app.name.toLowerCase() === target;
}

function hasDefaultRuleText(rule: AppRule): boolean {
  const details = rule.details.trim().toLowerCase();
  const state = rule.state.trim().toLowerCase();
  const displayName = rule.display_name.trim().toLowerCase();
  return (
    (details === "" || details === displayName) &&
    (state === "" || state === "working")
  );
}

const labelCls = "text-[12px] font-semibold text-ink";
const hintCls = "text-[11px] leading-snug text-muted-ink";
const moveBtnCls =
  "w-[22px] h-[22px] flex items-center justify-center bg-surface-onyx text-ink neo-border-2 hover:bg-primary hover:text-white transition-colors disabled:opacity-30 disabled:pointer-events-none";

export function AppRuleCard({
  rule,
  index,
  totalRules,
  canReorder,
  onUpdate,
  onDelete,
  onMoveUp,
  onMoveDown,
  onMoveTo,
  claudeCodeLine = "",
  livePresence = null,
}: AppRuleCardProps) {
  const isClaudeCode = rule.process_name === CLAUDE_CODE_RULE_ID;
  const [expanded, setExpanded] = useState(false);
  const [showMore, setShowMore] = useState(false);
  const [editRule, setEditRule] = useState<AppRule>({ ...rule });
  const [isEditingPriority, setIsEditingPriority] = useState(false);
  const [priorityInput, setPriorityInput] = useState("");

  const formId = `rule-form-${rule.process_name}`;
  const rulePreview =
    [rule.details, rule.state].filter(Boolean).join(" · ") || "No presence text";
  const live =
    livePresence &&
    isLiveFor(rule.process_name, rule.display_name, livePresence.large_text)
      ? livePresence
      : null;
  const preview = isClaudeCode
    ? `Claude Code · ${claudeCodeLine}`
    : live && hasDefaultRuleText(rule)
      ? live.state
      : rulePreview;
  const iconName =
    rule.large_image && rule.large_image !== "auto"
      ? rule.large_image
      : rule.process_name;

  const openEditor = () => {
    setEditRule({ ...rule });
    // Show the advanced fields right away when one of them is already in use.
    setShowMore(
      (rule.large_image !== "" && rule.large_image !== "auto") ||
        Boolean(rule.client_id),
    );
    setExpanded(true);
  };

  const closeEditor = () => setExpanded(false);

  const handlePrioritySubmit = () => {
    setIsEditingPriority(false);
    const parsed = parseInt(priorityInput, 10);
    if (!isNaN(parsed) && parsed > 0 && parsed <= totalRules) {
      onMoveTo(parsed - 1);
    }
  };

  const patch = (partial: Partial<AppRule>) =>
    setEditRule({ ...editRule, ...partial });

  const renderField = (
    label: string,
    field: keyof AppRule,
    placeholder?: string,
  ) => (
    <label className="flex flex-col gap-1 flex-1 min-w-0" key={field}>
      <span className={labelCls}>{label}</span>
      <input
        className="neo-input text-[13px] py-1.5"
        style={{ borderRadius: "6px" }}
        type="text"
        placeholder={placeholder}
        value={(editRule[field] as string) || ""}
        onChange={(e) => patch({ [field]: e.target.value })}
      />
    </label>
  );

  return (
    <div
      className="neo-border bg-surface-indigo overflow-hidden transition-shadow duration-100"
      style={{
        borderRadius: "8px",
        boxShadow: expanded
          ? "0 0 0 var(--neo-shadow-color)"
          : "4px 4px 0 var(--neo-shadow-color)",
      }}
    >
      <div className="flex items-center gap-2 py-2.5 pl-2.5 pr-3">
        {/* Position in the priority order; click to type a new one */}
        {canReorder &&
          (isEditingPriority ? (
            <input
              type="number"
              aria-label={`New position for ${rule.display_name}`}
              className="w-9 shrink-0 text-[12px] font-black text-ink bg-surface-onyx py-1 neo-border-2 font-display text-center focus:outline-none"
              style={{ borderRadius: "4px", MozAppearance: "textfield" }}
              autoFocus
              min={1}
              max={totalRules}
              value={priorityInput}
              onChange={(e) => setPriorityInput(e.target.value)}
              onBlur={handlePrioritySubmit}
              onKeyDown={(e) => {
                if (e.key === "Enter") handlePrioritySubmit();
                if (e.key === "Escape") setIsEditingPriority(false);
              }}
            />
          ) : (
            <button
              type="button"
              className="w-9 shrink-0 text-[12px] font-black text-ink bg-surface-onyx py-1 neo-border-2 font-display hover:bg-white/10 transition-colors"
              style={{ borderRadius: "4px" }}
              onClick={() => {
                setPriorityInput((index + 1).toString());
                setIsEditingPriority(true);
              }}
              title="Position in priority order. Click to change it."
              aria-label={`Position ${index + 1}. Change position of ${rule.display_name}`}
            >
              {index + 1}
            </button>
          ))}

        {/* Row body opens the editor; the Claude Code entry has none. */}
        <button
          type="button"
          onClick={expanded ? closeEditor : openEditor}
          disabled={isClaudeCode}
          aria-expanded={expanded}
          aria-controls={formId}
          aria-label={isClaudeCode ? rule.display_name : `Edit ${rule.display_name}`}
          title={isClaudeCode ? "Claude Code text comes from your live sessions." : undefined}
          className="flex items-center gap-3 flex-1 min-w-0 text-left rounded-md px-1 py-0.5 enabled:hover:bg-white/5 transition-colors"
        >
          <span className="relative w-9 h-9 shrink-0">
            <span
              className={`w-9 h-9 flex items-center justify-center overflow-hidden bg-surface-onyx neo-border-2 ${rule.enabled ? "" : "opacity-40"}`}
              style={{ borderRadius: "6px" }}
            >
              <AppIcon name={iconName} className="w-6 h-6 object-contain" size={24} />
            </span>
            {live?.small_image && (
              <span
                className="absolute -bottom-1.5 -right-1.5 w-5 h-5 rounded-full flex items-center justify-center overflow-hidden bg-surface-black neo-border-2"
                title={live.small_text || ""}
              >
                <AppIcon name={live.small_image} className="w-3 h-3 object-contain" size={12} />
              </span>
            )}
          </span>
          <span
            className={`flex flex-col min-w-0 flex-1 ${rule.enabled ? "" : "opacity-50"}`}
          >
            <span className="text-[14px] text-ink font-bold font-display truncate">
              {rule.display_name}
            </span>
            <span className="text-[12px] text-muted-ink truncate">{preview}</span>
          </span>
          {!isClaudeCode && (
            <ChevronDown
              size={16}
              aria-hidden
              className={`shrink-0 text-muted-ink transition-transform ${expanded ? "rotate-180" : ""}`}
            />
          )}
        </button>

        {canReorder && (
          <div className="flex flex-col gap-1 shrink-0">
            <button
              type="button"
              onClick={onMoveUp}
              disabled={index === 0}
              className={moveBtnCls}
              style={{ borderRadius: "3px" }}
              title="Move up (higher priority)"
              aria-label={`Move ${rule.display_name} up`}
            >
              <ChevronUp size={13} strokeWidth={3} />
            </button>
            <button
              type="button"
              onClick={onMoveDown}
              disabled={index === totalRules - 1}
              className={moveBtnCls}
              style={{ borderRadius: "3px" }}
              title="Move down (lower priority)"
              aria-label={`Move ${rule.display_name} down`}
            >
              <ChevronDown size={13} strokeWidth={3} />
            </button>
          </div>
        )}

        <Toggle
          checked={rule.enabled}
          onChange={(enabled) => onUpdate({ ...rule, enabled })}
          label={`Show ${rule.display_name} on Discord`}
        />
      </div>

      {expanded && !isClaudeCode && (
        <div
          id={formId}
          className="p-4 bg-surface-onyx flex flex-col gap-3"
          style={{ borderTop: "3px solid var(--neo-border-color)" }}
        >
          <p className={hintCls}>
            Matches the process{" "}
            <code className="font-mono text-ink">{rule.process_name}</code>
          </p>

          {renderField("Name", "display_name")}

          <div className="flex flex-col gap-1">
            <div className="flex flex-col min-[480px]:flex-row gap-3">
              {renderField("First line", "details")}
              {renderField("Second line", "state")}
            </div>
            <p className={hintCls}>
              Use <code className="font-mono text-ink">{"{title}"}</code> for the
              window title and{" "}
              <code className="font-mono text-ink">{"{file}"}</code> for the open
              file.
            </p>
          </div>

          <button
            type="button"
            onClick={() => setShowMore(!showMore)}
            aria-expanded={showMore}
            className="self-start flex items-center gap-1 text-[12px] font-bold font-display uppercase tracking-wide text-muted-ink hover:text-ink transition-colors"
          >
            <ChevronRight
              size={14}
              strokeWidth={3}
              aria-hidden
              className={`transition-transform ${showMore ? "rotate-90" : ""}`}
            />
            More options
          </button>

          {showMore && (
            <div className="flex flex-col gap-3">
              <div className="flex flex-col gap-1">
                {renderField("Image", "large_image", "auto")}
                <p className={hintCls}>
                  "auto" uses the app's own icon. You can also enter an image URL.
                </p>
              </div>
              {renderField("Discord application ID (optional)", "client_id")}
            </div>
          )}

          <div className="flex flex-wrap items-center justify-between gap-3 mt-2">
            <button
              type="button"
              className="neo-btn bg-danger text-white px-3.5 py-2 text-[12px]"
              style={{ borderRadius: "6px" }}
              onClick={() => onDelete(rule.process_name)}
            >
              Delete
            </button>
            <div className="flex gap-3">
              <button
                type="button"
                className="neo-btn bg-surface-indigo text-ink px-3.5 py-2 text-[12px]"
                style={{ borderRadius: "6px" }}
                onClick={closeEditor}
              >
                Cancel
              </button>
              <button
                type="button"
                className="neo-btn bg-primary text-white px-4 py-2 text-[12px]"
                style={{ borderRadius: "6px" }}
                onClick={() => {
                  onUpdate(editRule);
                  closeEditor();
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
