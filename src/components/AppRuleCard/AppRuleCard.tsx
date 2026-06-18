import { useState } from "react";
import styles from "./AppRuleCard.module.css";
import { AppRule, PresenceSource } from "../../types";

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

  const sourceColors: Record<string, string> = {
    Game: "var(--status-purple)",
    Work: "var(--accent-primary)",
    Browser: "var(--status-green)",
    Idle: "var(--status-yellow)",
    Manual: "var(--text-muted)",
  };

  return (
    <div className={`${styles.card} ${!rule.enabled ? styles.disabled : ""}`}>
      <div 
        className={styles.header} 
        onClick={() => !expanded && setExpanded(true)}
      >
        <div className={styles.info}>
          <div className={styles.titleRow}>
            <h3 className={styles.displayName}>{rule.display_name}</h3>
            <span 
              className={styles.badge}
              style={{ backgroundColor: `${sourceColors[rule.source]}33`, color: sourceColors[rule.source] }}
            >
              {rule.source}
            </span>
          </div>
          <div className={styles.processName}>{rule.process_name}</div>
          <div className={styles.preview}>
            "{rule.details}" • "{rule.state}"
          </div>
        </div>
        
        <div className={styles.actions}>
          <button 
            className={`${styles.toggle} ${rule.enabled ? styles.toggleOn : ""}`}
            onClick={handleToggle}
            title={rule.enabled ? "Desativar" : "Ativar"}
          >
            <div className={styles.toggleKnob}></div>
          </button>
        </div>
      </div>

      {expanded && (
        <div className={styles.editor}>
          <div className={styles.formGroup}>
            <label>Nome de Exibição</label>
            <input 
              type="text" 
              value={editRule.display_name}
              onChange={(e) => setEditRule({...editRule, display_name: e.target.value})}
            />
          </div>
          
          <div className={styles.formRow}>
            <div className={styles.formGroup}>
              <label>Detalhes (Linha 1)</label>
              <input 
                type="text" 
                value={editRule.details}
                onChange={(e) => setEditRule({...editRule, details: e.target.value})}
              />
            </div>
            
            <div className={styles.formGroup}>
              <label>Estado (Linha 2)</label>
              <input 
                type="text" 
                value={editRule.state}
                onChange={(e) => setEditRule({...editRule, state: e.target.value})}
              />
            </div>
          </div>
          
          <div className={styles.formRow}>
            <div className={styles.formGroup}>
              <label>Tipo (Fonte)</label>
              <select 
                value={editRule.source}
                onChange={(e) => setEditRule({...editRule, source: e.target.value as PresenceSource})}
              >
                <option value="Game">Jogo</option>
                <option value="Work">Trabalho</option>
                <option value="Browser">Navegador</option>
              </select>
            </div>
            
            <div className={styles.formGroup}>
              <label>Imagem (Asset Key)</label>
              <input 
                type="text" 
                value={editRule.large_image}
                onChange={(e) => setEditRule({...editRule, large_image: e.target.value})}
              />
            </div>
          </div>

          <div className={styles.editorActions}>
            <button className={styles.btnDelete} onClick={() => onDelete(rule.process_name)}>
              Excluir
            </button>
            <div className={styles.rightActions}>
              <button className={styles.btnCancel} onClick={handleCancel}>
                Cancelar
              </button>
              <button className={styles.btnSave} onClick={handleSave}>
                Salvar
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
