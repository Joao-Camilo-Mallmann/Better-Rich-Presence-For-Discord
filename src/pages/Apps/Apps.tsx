import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useAppRules } from "../../hooks/useAppRules";
import { AppRuleCard } from "../../components/AppRuleCard/AppRuleCard";
import { PresenceSource } from "../../types";

export function Apps() {
  const { rules, loading, updateRule, deleteRule, addRule } = useAppRules();
  const [filterSource, setFilterSource] = useState<"All" | PresenceSource>("All");
  const [showPicker, setShowPicker] = useState(false);
  const [pickerSearch, setPickerSearch] = useState("");
  const [systemProcesses, setSystemProcesses] = useState<{ process_name: string; display_name: string }[]>([]);
  const [loadingProcesses, setLoadingProcesses] = useState(false);

  const filteredRules = rules.filter(r => {
    return filterSource === "All" || r.source === filterSource;
  });

  const handleOpenPicker = async () => {
    try {
      setLoadingProcesses(true);
      const procs = await invoke<{ process_name: string; display_name: string }[]>("get_running_processes");
      setSystemProcesses(procs);
    } catch (err) {
      console.error("Failed to load running processes:", err);
      setSystemProcesses([]);
    } finally {
      setLoadingProcesses(false);
    }
    setShowPicker(true);
  };

  const selectProcess = async (proc: { process_name: string; display_name: string }) => {
    const process_lower = proc.process_name.toLowerCase();
    
    // Simple heuristic
    let source: PresenceSource = "Work";
    let details = `Trabalhando no ${proc.display_name}`;
    let state = `No ${proc.display_name}`;
    let priority = 2;

    const gameKeywords = ["game", "play", "cs2", "eldenring", "minecraft", "steam", "gta", "valorant", "counterstrike"];
    const browserKeywords = ["chrome", "firefox", "edge", "opera", "brave", "safari", "browser", "explorer", "spotify"];

    if (gameKeywords.some(kw => process_lower.includes(kw))) {
      source = "Game";
      details = `Jogando ${proc.display_name}`;
      state = "Em jogo";
      priority = 0;
    } else if (browserKeywords.some(kw => process_lower.includes(kw))) {
      source = "Browser";
      details = `Navegando no ${proc.display_name}`;
      state = "Navegando";
      priority = 3;
    }

    if (process_lower.includes("code") || process_lower.includes("cursor") || process_lower.includes("studio") || process_lower.includes("devenv") || process_lower.includes("figma")) {
      details = "Editando {file}";
      state = `No ${proc.display_name}`;
    }

    // Avoid duplicate rules
    if (rules.some(r => r.process_name === proc.process_name)) {
      setShowPicker(false);
      return;
    }

    await addRule({
      process_name: proc.process_name,
      display_name: proc.display_name,
      details,
      state,
      large_image: "auto",
      source,
      priority,
      enabled: true,
    });
    setShowPicker(false);
    setPickerSearch("");
  };

  if (loading) {
    return <div className="text-muted-ink p-4 text-center">Carregando regras...</div>;
  }

  return (
    <div className="flex flex-col gap-4">
      {/* Header Section */}
      <div className="flex justify-between items-center">
        <h3 className="text-[10px] text-muted-ink uppercase tracking-wider font-extrabold font-display">Aplicativos</h3>
        <button 
          onClick={handleOpenPicker}
          className="bg-green-accent text-black text-xs px-3 py-1.5 rounded-sm font-bold hover:brightness-95 transition"
        >
          + Buscar do Sistema
        </button>
      </div>

      {/* Categories chips filter */}
      <div className="flex flex-wrap gap-1.5">
        {["All", "Work", "Browser", "Game"].map(f => (
          <button 
            key={f}
            className={`px-2.5 py-1 rounded-sm text-xs transition ${
              filterSource === f 
                ? "bg-white/15 text-ink font-semibold" 
                : "text-muted-ink bg-transparent hover:bg-white/5 hover:text-ink"
            }`}
            onClick={() => setFilterSource(f as any)}
          >
            {f === "All" ? "Todos" : f}
          </button>
        ))}
      </div>

      {/* Rules list */}
      <div className="flex flex-col gap-2">
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
          <div className="text-center p-6 text-xs text-muted-ink bg-surface-indigo/30 rounded-md border border-dashed border-hairline/25">
            Nenhum aplicativo ativado.
          </div>
        )}
      </div>

      {/* System Process Picker Modal */}
      {showPicker && (
        <div className="fixed inset-0 bg-black/60 backdrop-blur-xs flex items-center justify-center p-4 z-50">
          <div className="bg-surface-indigo border border-hairline rounded-md w-full max-w-[340px] max-h-[460px] flex flex-col p-4 shadow-xl">
            <div className="flex justify-between items-center mb-3">
              <h4 className="text-sm font-bold text-ink font-display uppercase tracking-wider">Selecionar do Sistema</h4>
              <button 
                onClick={() => {
                  setShowPicker(false);
                  setPickerSearch("");
                }} 
                className="text-muted-ink hover:text-ink text-sm"
              >
                ✕
              </button>
            </div>
            <input 
              type="text"
              placeholder="Pesquisar processo..."
              value={pickerSearch}
              onChange={(e) => setPickerSearch(e.target.value)}
              className="bg-surface-onyx border border-hairline/50 text-ink text-sm px-3 py-2 rounded-sm mb-3 focus:border-primary focus:outline-none w-full"
            />
            {loadingProcesses ? (
              <div className="text-center py-8 text-xs text-muted-ink">Carregando processos...</div>
            ) : (
              <div className="flex-1 overflow-y-auto flex flex-col gap-1 pr-1">
                {systemProcesses
                  .filter(p => 
                    p.display_name.toLowerCase().includes(pickerSearch.toLowerCase()) || 
                    p.process_name.toLowerCase().includes(pickerSearch.toLowerCase())
                  ).length > 0 ? (
                    systemProcesses
                      .filter(p => 
                        p.display_name.toLowerCase().includes(pickerSearch.toLowerCase()) || 
                        p.process_name.toLowerCase().includes(pickerSearch.toLowerCase())
                      )
                      .map(p => {
                        const isAlreadyAdded = rules.some(r => r.process_name === p.process_name);
                        return (
                          <button
                            key={p.process_name}
                            onClick={() => !isAlreadyAdded && selectProcess(p)}
                            disabled={isAlreadyAdded}
                            className={`flex justify-between items-center p-2 rounded-xs hover:bg-white/5 text-left text-sm transition ${
                              isAlreadyAdded ? "opacity-40 cursor-not-allowed" : ""
                            }`}
                          >
                            <div>
                              <div className="font-semibold text-ink">{p.display_name}</div>
                              <div className="text-xs text-muted-ink font-mono">{p.process_name}</div>
                            </div>
                            <span className="text-[9px] font-bold text-muted-ink uppercase border border-hairline/25 px-1.5 py-0.5 rounded-xs bg-surface-onyx">
                              {isAlreadyAdded ? "Ativo" : "Adicionar"}
                            </span>
                          </button>
                        );
                      })
                  ) : (
                    <div className="text-center py-8 text-xs text-muted-ink">Nenhum processo em execução encontrado.</div>
                  )
                }
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
