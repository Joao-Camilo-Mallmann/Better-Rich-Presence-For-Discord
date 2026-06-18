import { useState } from "react";
import styles from "./Apps.module.css";
import { useAppRules } from "../../hooks/useAppRules";
import { AppRuleCard } from "../../components/AppRuleCard/AppRuleCard";
import { PresenceSource, AppRule } from "../../types";

export function Apps() {
  const { rules, loading, updateRule, deleteRule, addRule, resetToDefaults } = useAppRules();
  const [searchTerm, setSearchTerm] = useState("");
  const [filterSource, setFilterSource] = useState<"All" | PresenceSource>("All");
  const [showAddForm, setShowAddForm] = useState(false);

  // New Rule Form State
  const [newRule, setNewRule] = useState<AppRule>({
    process_name: "",
    display_name: "",
    details: "",
    state: "",
    large_image: "default",
    source: "Work",
    priority: 2,
    enabled: true,
  });

  const filteredRules = rules.filter(r => {
    const matchesSearch = r.process_name.toLowerCase().includes(searchTerm.toLowerCase()) || 
                          r.display_name.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesFilter = filterSource === "All" || r.source === filterSource;
    return matchesSearch && matchesFilter;
  });

  const handleAddRule = async () => {
    if (!newRule.process_name || !newRule.display_name) return;
    
    // Assign correct priority based on source
    let priority = 2; // Work
    if (newRule.source === "Game") priority = 0;
    if (newRule.source === "Manual") priority = 1;
    if (newRule.source === "Browser") priority = 3;
    if (newRule.source === "Idle") priority = 4;

    await addRule({ ...newRule, priority });
    setShowAddForm(false);
    setNewRule({
      process_name: "",
      display_name: "",
      details: "",
      state: "",
      large_image: "default",
      source: "Work",
      priority: 2,
      enabled: true,
    });
  };

  if (loading) {
    return <div className={styles.loading}>Carregando regras...</div>;
  }

  return (
    <div className={styles.page}>
      <header className={styles.header}>
        <div>
          <h2 className={styles.title}>Biblioteca de Aplicativos</h2>
          <p className={styles.subtitle}>Gerencie quais programas acionam o Rich Presence</p>
        </div>
        <div className={styles.headerActions}>
          <button className={styles.btnReset} onClick={resetToDefaults}>
            Resetar Padrões
          </button>
          <button className={styles.btnAdd} onClick={() => setShowAddForm(!showAddForm)}>
            {showAddForm ? "Cancelar" : "+ Adicionar App"}
          </button>
        </div>
      </header>

      {showAddForm && (
        <div className={styles.addForm}>
          <h3>Novo Aplicativo</h3>
          <div className={styles.formGrid}>
            <div className={styles.formGroup}>
              <label>Processo (ex: code.exe)</label>
              <input 
                type="text" 
                value={newRule.process_name}
                onChange={e => setNewRule({...newRule, process_name: e.target.value})}
                placeholder="nome_do_processo.exe"
              />
            </div>
            <div className={styles.formGroup}>
              <label>Nome de Exibição</label>
              <input 
                type="text" 
                value={newRule.display_name}
                onChange={e => setNewRule({...newRule, display_name: e.target.value})}
                placeholder="Visual Studio Code"
              />
            </div>
            <div className={styles.formGroup}>
              <label>Detalhes</label>
              <input 
                type="text" 
                value={newRule.details}
                onChange={e => setNewRule({...newRule, details: e.target.value})}
                placeholder="Programando"
              />
            </div>
            <div className={styles.formGroup}>
              <label>Estado</label>
              <input 
                type="text" 
                value={newRule.state}
                onChange={e => setNewRule({...newRule, state: e.target.value})}
                placeholder="Desenvolvendo"
              />
            </div>
            <div className={styles.formGroup}>
              <label>Categoria</label>
              <select 
                value={newRule.source}
                onChange={e => setNewRule({...newRule, source: e.target.value as PresenceSource})}
              >
                <option value="Game">Jogo</option>
                <option value="Work">Trabalho</option>
                <option value="Browser">Navegador</option>
              </select>
            </div>
            <div className={styles.formGroup}>
              <label>Imagem (Asset)</label>
              <input 
                type="text" 
                value={newRule.large_image}
                onChange={e => setNewRule({...newRule, large_image: e.target.value})}
                placeholder="default"
              />
            </div>
          </div>
          <button 
            className={styles.btnSubmit} 
            onClick={handleAddRule}
            disabled={!newRule.process_name || !newRule.display_name}
          >
            Salvar Aplicativo
          </button>
        </div>
      )}

      <div className={styles.controls}>
        <input 
          type="text" 
          className={styles.search} 
          placeholder="Buscar aplicativos..." 
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
        />
        <div className={styles.filters}>
          {["All", "Work", "Browser", "Game"].map(f => (
            <button 
              key={f}
              className={`${styles.filterBtn} ${filterSource === f ? styles.filterActive : ""}`}
              onClick={() => setFilterSource(f as any)}
            >
              {f === "All" ? "Todos" : f}
            </button>
          ))}
        </div>
      </div>

      <div className={styles.rulesList}>
        {filteredRules.length > 0 ? (
          filteredRules.map(rule => (
            <AppRuleCard 
              key={rule.process_name} 
              rule={rule} 
              onUpdate={updateRule}
              onDelete={deleteRule}
            />
          ))
        ) : (
          <div className={styles.emptyState}>
            Nenhum aplicativo encontrado.
          </div>
        )}
      </div>
    </div>
  );
}
