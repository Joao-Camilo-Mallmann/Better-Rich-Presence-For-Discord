import { useState } from "react";
import { AppRule, PresenceSource } from "../../types";
import { getIconUrl } from "../../utils/iconUrl";

interface AppRuleCardProps {
  rule: AppRule;
  onUpdate: (rule: AppRule) => void;
  onDelete: (processName: string) => void;
}

export function AppRuleCard({ rule, onUpdate, onDelete }: AppRuleCardProps) {
  const [expanded, setExpanded] = useState(false);
  
  // Local state for editing
  const [editRule, setEditRule] = useState<AppRule>({ ...rule });

  const handleSave = () => {
    onUpdate(editRule);
    setExpanded(false);
  };

  const handleCancel = () => {
    setEditRule({ ...rule });
    setExpanded(false);
  };

  const handleToggle = (e: React.MouseEvent) => {
    e.stopPropagation();
    onUpdate({ ...rule, enabled: !rule.enabled });
  };

  const sourceBadgeStyles: Record<string, { bg: string; text: string }> = {
    Game: { bg: "bg-magenta-accent/20", text: "text-magenta-accent" },
    Work: { bg: "bg-primary/20", text: "text-primary" },
    Browser: { bg: "bg-green-accent/20", text: "text-green-accent" },
    Idle: { bg: "bg-white/10", text: "text-muted-ink" },
    Manual: { bg: "bg-white/10", text: "text-muted-ink" },
  };

  const currentStyles = sourceBadgeStyles[rule.source] || { bg: "bg-white/10", text: "text-muted-ink" };
  const iconUrl = rule.large_image && rule.large_image !== "auto" ? rule.large_image : getIconUrl(rule.process_name, rule.display_name);

  return (
    <div className={`bg-surface-indigo rounded-md border border-hairline overflow-hidden hover:border-white/20 transition duration-100 ${!rule.enabled ? "opacity-60 hover:opacity-80" : ""}`}>
      <div 
        className="p-2.5 flex justify-between items-center gap-3 cursor-pointer" 
        onClick={() => !expanded && setExpanded(true)}
      >
        <div className="flex items-center gap-3 flex-1 min-w-0">
          {/* Icon */}
          <div className="w-10 h-10 rounded-sm bg-surface-onyx border border-hairline flex items-center justify-center overflow-hidden flex-shrink-0">
            <img src={iconUrl} alt={rule.display_name} className="w-7 h-7 object-contain" onError={(e) => {
              (e.target as HTMLImageElement).src = "https://www.google.com/s2/favicons?sz=128&domain=google.com";
            }} />
          </div>

          <div className="flex flex-col gap-0.5 min-w-0 flex-1">
            <div className="flex items-center gap-2 flex-wrap">
              <h3 className="text-sm text-ink font-bold m-0 truncate">{rule.display_name}</h3>
              <span className={`text-[9px] px-1.5 py-[1px] rounded-xs font-bold uppercase ${currentStyles.bg} ${currentStyles.text}`}>
                {rule.source}
              </span>
            </div>
            <div className="text-[10px] text-muted-ink font-mono truncate">{rule.process_name}</div>
            <div className="text-xs text-muted-ink truncate">
              "{rule.details}" • "{rule.state}"
            </div>
          </div>
        </div>
        
        <div className="flex items-center gap-2 flex-shrink-0">
          <button 
            className={`w-9 h-5 rounded-full relative transition duration-150 ${rule.enabled ? "bg-green-accent" : "bg-surface-onyx"}`}
            onClick={handleToggle}
            title={rule.enabled ? "Desativar" : "Ativar"}
          >
            <div className={`w-[14px] h-[14px] bg-white rounded-full absolute top-[3px] left-[3px] transition-transform duration-150 shadow-sm ${rule.enabled ? "translate-x-4" : "translate-x-0"}`}></div>
          </button>
        </div>
      </div>

      {expanded && (
        <div className="p-4 bg-surface-onyx border-t border-hairline flex flex-col gap-3 transition duration-150">
          <div className="flex flex-col gap-1 flex-1">
            <label className="text-[10px] text-muted-ink font-bold uppercase tracking-wider">Nome de Exibição</label>
            <input 
              className="bg-surface-indigo border border-hairline text-ink px-3 py-2 rounded-xs focus:border-primary focus:outline-none transition duration-100"
              type="text" 
              value={editRule.display_name}
              onChange={(e) => setEditRule({...editRule, display_name: e.target.value})}
            />
          </div>
          
          <div className="flex flex-col sm:flex-row gap-4">
            <div className="flex flex-col gap-1 flex-1">
              <label className="text-[10px] text-muted-ink font-bold uppercase tracking-wider">Details (Line 1)</label>
              <input 
                className="bg-surface-indigo border border-hairline text-ink px-3 py-2 rounded-xs focus:border-primary focus:outline-none transition duration-100"
                type="text" 
                value={editRule.details}
                onChange={(e) => setEditRule({...editRule, details: e.target.value})}
              />
            </div>
            
            <div className="flex flex-col gap-1 flex-1">
              <label className="text-[10px] text-muted-ink font-bold uppercase tracking-wider">State (Line 2)</label>
              <input 
                className="bg-surface-indigo border border-hairline text-ink px-3 py-2 rounded-xs focus:border-primary focus:outline-none transition duration-100"
                type="text" 
                value={editRule.state}
                onChange={(e) => setEditRule({...editRule, state: e.target.value})}
              />
            </div>
          </div>
          
          <div className="flex flex-col sm:flex-row gap-4">
            <div className="flex flex-col gap-1 flex-1">
              <label className="text-[10px] text-muted-ink font-bold uppercase tracking-wider">Type (Source)</label>
              <select 
                className="bg-surface-indigo border border-hairline text-ink px-3 py-2 rounded-xs focus:border-primary focus:outline-none transition duration-100"
                value={editRule.source}
                onChange={(e) => setEditRule({...editRule, source: e.target.value as PresenceSource})}
              >
                <option value="Game">Jogo</option>
                <option value="Work">Trabalho</option>
                <option value="Browser">Navegador</option>
              </select>
            </div>
            
            <div className="flex flex-col gap-1 flex-1">
              <label className="text-[10px] text-muted-ink font-bold uppercase tracking-wider">Imagem (Asset Key)</label>
              <input 
                className="bg-surface-indigo border border-hairline text-ink px-3 py-2 rounded-xs focus:border-primary focus:outline-none transition duration-100"
                type="text" 
                value={editRule.large_image}
                onChange={(e) => setEditRule({...editRule, large_image: e.target.value})}
              />
            </div>
          </div>

          <div className="flex flex-col-reverse sm:flex-row justify-between mt-4 gap-3">
            <button className="bg-transparent text-danger px-4 py-2 rounded-sm text-sm font-medium hover:underline transition duration-100 w-full sm:w-auto" onClick={() => onDelete(rule.process_name)}>
              Delete
            </button>
            <div className="flex gap-2 w-full sm:w-auto">
              <button className="bg-transparent text-ink px-4 py-2 rounded-sm text-sm font-medium hover:underline transition duration-100 flex-1 sm:flex-none" onClick={handleCancel}>
                Cancel
              </button>
              <button className="bg-primary text-white px-4 py-2 rounded-sm text-sm font-semibold hover:opacity-90 transition duration-100 flex-1 sm:flex-none" onClick={handleSave}>
                Save
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
