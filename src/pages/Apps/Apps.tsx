import { useState, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useAppRules } from "../../hooks/useAppRules";
import { AppRuleCard } from "../../components/AppRuleCard/AppRuleCard";
import { AppRule, PresenceSource } from "../../types";

export function Apps() {
  const { rules, loading, updateRule, deleteRule, addRule, reorderRules } = useAppRules();
  const [filterSource, setFilterSource] = useState<"All" | PresenceSource>("All");
  const [showPicker, setShowPicker] = useState(false);
  const [pickerSearch, setPickerSearch] = useState("");
  const [systemProcesses, setSystemProcesses] = useState<{ process_name: string; display_name: string }[]>([]);
  const [loadingProcesses, setLoadingProcesses] = useState(false);

  // Drag-and-drop state
  const [draggingIndex, setDraggingIndex] = useState<number | null>(null);
  const [dragOverIndex, setDragOverIndex] = useState<number | null>(null);
  const dragItem = useRef<number | null>(null);

  // Only show all rules for drag-and-drop when no filter is active
  const showDragHint = filterSource === "All";
  const filteredRules = rules.filter((r) => filterSource === "All" || r.source === filterSource);

  // ── Drag-and-drop handlers ──────────────────────────────────────────────
  const handleDragStart = (index: number) => {
    dragItem.current = index;
    setDraggingIndex(index);
  };

  const handleDragOver = (e: React.DragEvent, index: number) => {
    e.preventDefault();
    setDragOverIndex(index);
  };

  const handleDrop = async (dropIndex: number) => {
    if (dragItem.current === null || dragItem.current === dropIndex) {
      setDraggingIndex(null);
      setDragOverIndex(null);
      dragItem.current = null;
      return;
    }

    // Reorder the FULL rules list (not filtered) based on drag from/to positions in filteredRules
    const sourceRule = filteredRules[dragItem.current];
    const targetRule = filteredRules[dropIndex];

    if (!sourceRule || !targetRule) return;

    // Build new order for the full list
    const sourceIdx = rules.findIndex((r) => r.process_name === sourceRule.process_name);
    const targetIdx = rules.findIndex((r) => r.process_name === targetRule.process_name);

    const newRules = [...rules];
    const [moved] = newRules.splice(sourceIdx, 1);
    newRules.splice(targetIdx, 0, moved);

    const newOrder = newRules.map((r) => r.process_name);

    try {
      await reorderRules(newOrder);
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

  // ── Process picker ──────────────────────────────────────────────────────
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

    let source: PresenceSource = "Work";
    let details = `Trabalhando no ${proc.display_name}`;
    let state = `No ${proc.display_name}`;
    let priority = 2;

    const gameKeywords = ["game", "play", "cs2", "eldenring", "minecraft", "steam", "gta", "valorant", "counterstrike"];
    const browserKeywords = ["chrome", "firefox", "edge", "opera", "brave", "safari", "browser", "explorer", "spotify"];

    if (gameKeywords.some((kw) => process_lower.includes(kw))) {
      source = "Game";
      details = `Jogando ${proc.display_name}`;
      state = "Em jogo";
      priority = 0;
    } else if (browserKeywords.some((kw) => process_lower.includes(kw))) {
      source = "Browser";
      details = `Navegando no ${proc.display_name}`;
      state = "Navegando";
      priority = 3;
    }

    if (
      process_lower.includes("code") ||
      process_lower.includes("cursor") ||
      process_lower.includes("studio") ||
      process_lower.includes("devenv") ||
      process_lower.includes("figma")
    ) {
      details = "Editando {file}";
      state = `No ${proc.display_name}`;
    }

    if (rules.some((r) => r.process_name === proc.process_name)) {
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
    } as AppRule);
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
        <div className="flex flex-col">
          <h3 className="text-[10px] text-muted-ink uppercase tracking-wider font-extrabold font-display">
            Aplicativos
          </h3>
          {showDragHint && (
            <span className="text-[9px] text-muted-ink/60 mt-0.5">
              Arraste para reordenar · posição = prioridade
            </span>
          )}
        </div>
        <button
          onClick={handleOpenPicker}
          className="bg-green-accent text-black text-xs px-3 py-1.5 rounded-sm font-bold hover:brightness-95 transition"
        >
          + Buscar do Sistema
        </button>
      </div>

      {/* Categories chips filter */}
      <div className="flex flex-wrap gap-1.5">
        {["All", "Work", "Browser", "Game"].map((f) => (
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

      {/* Rules list (drag-and-drop enabled) */}
      <div className="flex flex-col gap-2">
        {filteredRules.length > 0 ? (
          filteredRules.map((rule, idx) => {
            // Global position in the full rules list = visual priority number
            const globalIdx = rules.findIndex((r) => r.process_name === rule.process_name);
            const isDragging = draggingIndex === idx;
            const isOver = dragOverIndex === idx && draggingIndex !== idx;

            return (
              <div
                key={rule.process_name}
                draggable={showDragHint}
                onDragStart={() => handleDragStart(idx)}
                onDragOver={(e) => handleDragOver(e, idx)}
                onDrop={() => handleDrop(idx)}
                onDragEnd={handleDragEnd}
                className={`group relative transition-all duration-150 ${
                  isDragging ? "opacity-40 scale-[0.98]" : ""
                } ${isOver ? "translate-y-[-2px]" : ""}`}
                style={{
                  outline: isOver ? "1px solid rgba(88,101,242,0.5)" : undefined,
                  borderRadius: 6,
                }}
              >
                {/* Drag handle + priority badge */}
                {showDragHint && (
                  <div className="absolute left-0 top-0 bottom-0 flex flex-col items-center justify-center w-6 z-10 cursor-grab active:cursor-grabbing select-none">
                    <span className="text-[8px] font-bold text-primary/70 leading-none">
                      #{globalIdx + 1}
                    </span>
                    <span className="text-muted-ink/40 text-[10px] mt-0.5">⠿</span>
                  </div>
                )}

                {/* Card with left padding when drag is enabled */}
                <div className={showDragHint ? "pl-5" : ""}>
                  <AppRuleCard
                    rule={rule}
                    onUpdate={updateRule}
                    onDelete={deleteRule}
                  />
                </div>
              </div>
            );
          })
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
              <h4 className="text-sm font-bold text-ink font-display uppercase tracking-wider">
                Selecionar do Sistema
              </h4>
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
                {systemProcesses.filter(
                  (p) =>
                    p.display_name.toLowerCase().includes(pickerSearch.toLowerCase()) ||
                    p.process_name.toLowerCase().includes(pickerSearch.toLowerCase())
                ).length > 0 ? (
                  systemProcesses
                    .filter(
                      (p) =>
                        p.display_name.toLowerCase().includes(pickerSearch.toLowerCase()) ||
                        p.process_name.toLowerCase().includes(pickerSearch.toLowerCase())
                    )
                    .map((p) => {
                      const isAlreadyAdded = rules.some((r) => r.process_name === p.process_name);
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
                  <div className="text-center py-8 text-xs text-muted-ink">
                    Nenhum processo em execução encontrado.
                  </div>
                )}
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
