import { useState } from "react";
import { AppRule } from "../../types";
import { getIconUrl } from "../../utils/iconUrl";

interface AppRuleCardProps {
  rule: AppRule;
  onUpdate: (rule: AppRule) => void;
  onDelete: (processName: string) => void;
}

const labelCls = "text-[10px] text-muted-ink font-extrabold uppercase tracking-wider font-display";

export function AppRuleCard({ rule, onUpdate, onDelete }: AppRuleCardProps) {
  const [expanded, setExpanded] = useState(false);
  const [editRule, setEditRule] = useState<AppRule>({ ...rule });

  const handleToggle = (e: React.MouseEvent) => {
    e.stopPropagation();
    onUpdate({ ...rule, enabled: !rule.enabled });
  };

  const patch = (partial: Partial<AppRule>) => setEditRule({ ...editRule, ...partial });
  const iconUrl = rule.large_image && rule.large_image !== "auto" ? rule.large_image : getIconUrl(rule.process_name, rule.display_name);

  const fields: { label: string; key: keyof AppRule; row?: number }[] = [
    { label: "Display Name", key: "display_name" },
    { label: "Details (Line 1)", key: "details", row: 1 },
    { label: "State (Line 2)", key: "state", row: 1 },
    { label: "Image (Asset Key)", key: "large_image", row: 2 },
  ];

  const Field = ({ label, field }: { label: string; field: keyof AppRule }) => (
    <div className="flex flex-col gap-1 flex-1">
      <label className={labelCls}>{label}</label>
      <input className="neo-input text-sm" type="text" value={editRule[field] as string}
        onChange={(e) => patch({ [field]: e.target.value })} />
    </div>
  );

  return (
    <div className={`neo-border bg-surface-indigo overflow-hidden transition-all duration-100 ${!rule.enabled ? "opacity-50 hover:opacity-70" : ""}`}
      style={{
        borderRadius: '8px',
        boxShadow: expanded ? '0 0 0 var(--neo-shadow-color)' : '4px 4px 0 var(--neo-shadow-color)',
      }}>
      <div className="p-3 flex justify-between items-center gap-3 cursor-pointer hover:bg-white/5 transition-colors"
        onClick={() => !expanded && setExpanded(true)}>
        <div className="flex items-center gap-3 flex-1 min-w-0">
          <div className="w-10 h-10 bg-surface-onyx flex items-center justify-center overflow-hidden flex-shrink-0 neo-border-2"
            style={{ borderRadius: '6px' }}>
            <img src={iconUrl} alt={rule.display_name} className="w-7 h-7 object-contain" onError={(e) => {
              (e.target as HTMLImageElement).src = "https://www.google.com/s2/favicons?sz=128&domain=google.com";
            }} />
          </div>
          <div className="flex flex-col gap-0.5 min-w-0 flex-1">
            <div className="flex items-center gap-2 flex-wrap">
              <h3 className="text-sm text-ink font-extrabold m-0 truncate font-display">{rule.display_name}</h3>
            </div>
            <div className="text-[10px] text-muted-ink font-mono truncate">{rule.process_name}</div>
            <div className="text-xs text-muted-ink truncate">
              "{rule.details}" • "{rule.state}"
            </div>
          </div>
        </div>
        <div className="flex items-center gap-2 flex-shrink-0">
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
        <div className="p-4 bg-surface-onyx flex flex-col gap-3" style={{ borderTop: '3px solid var(--neo-border-color)' }}>
          <Field label={fields[0].label} field={fields[0].key} />
          <div className="flex flex-col sm:flex-row gap-4">
            {fields.filter(f => f.row === 1).map(f => <Field key={f.key} label={f.label} field={f.key} />)}
          </div>
          <div className="flex flex-col sm:flex-row gap-4">
            {fields.filter(f => f.row === 2).map(f => <Field key={f.key} label={f.label} field={f.key} />)}
          </div>
          <div className="flex flex-col-reverse sm:flex-row justify-between mt-4 gap-3">
            <button
              className="neo-btn bg-danger text-white px-4 py-2 text-sm w-full sm:w-auto"
              onClick={() => onDelete(rule.process_name)}>
              Delete
            </button>
            <div className="flex gap-2 w-full sm:w-auto">
              <button
                className="neo-btn bg-surface-indigo text-ink px-4 py-2 text-sm flex-1 sm:flex-none"
                onClick={() => { setEditRule({ ...rule }); setExpanded(false); }}>
                Cancel
              </button>
              <button
                className="neo-btn bg-green-accent text-ink-dark px-4 py-2 text-sm font-extrabold flex-1 sm:flex-none"
                onClick={() => { onUpdate(editRule); setExpanded(false); }}>
                Save
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
