import { useState, useEffect } from "react";
import { Dashboard } from "./pages/Dashboard/Dashboard";
import { Apps } from "./pages/Apps/Apps";
import { Settings } from "./pages/Settings/Settings";
import { LogViewer } from "./components/LogViewer/LogViewer";
import { LandingPage } from "./pages/LandingPage/LandingPage";

const isTauri = typeof window !== "undefined" && (window as any).__TAURI_INTERNALS__ !== undefined;

function App() {
  const [rpcActive, setRpcActive] = useState(true);
  const [showSettings, setShowSettings] = useState(false);
  const [showLogs, setShowLogs] = useState(false);
  const [viewMode, setViewMode] = useState<"landing" | "app">(() => {
    return isTauri ? "app" : "landing";
  });

  const [systemTheme, setSystemTheme] = useState<"indigo" | "dark" | "amoled" | "light">(() => {
    return (localStorage.getItem("system_theme") as any) || "indigo";
  });

  useEffect(() => {
    localStorage.setItem("system_theme", systemTheme);
    document.documentElement.setAttribute("data-theme", systemTheme);
  }, [systemTheme]);

  if (viewMode === "landing") {
    return <LandingPage onEnterApp={() => setViewMode("app")} />;
  }

  return (
    <div className="w-full h-screen flex flex-col bg-canvas text-ink overflow-hidden">
      {/* Header */}
      <header className="flex justify-between items-center px-4 py-3 border-b border-hairline bg-surface-indigo shrink-0">
        <div className="flex items-center gap-3">
          {!isTauri && (
            <button 
              onClick={() => setViewMode("landing")}
              className="text-xs font-bold text-muted-ink hover:text-ink transition-colors px-2.5 py-1.5 border border-hairline/60 bg-surface-onyx/40 rounded-xs cursor-pointer hover:scale-[1.02] active:scale-95"
            >
              ⬅ Voltar ao Site
            </button>
          )}
          <div className="flex items-center gap-2">
            <img src="/logo.png" alt="Better RPC Logo" width="28" height="21" className="object-contain" />
            <h1 className="text-md font-bold tracking-tight text-ink font-display">Better RPC</h1>
          </div>
        </div>
        
        {/* Header Controls */}
        <div className="flex items-center gap-4">
          {/* Theme Selector */}
          <div className="flex items-center gap-1.5 bg-surface-onyx/40 px-2.5 py-1 rounded-sm border border-hairline/45">
            <span className="text-[10px] text-muted-ink font-bold uppercase select-none">Tema:</span>
            <select
              value={systemTheme}
              onChange={(e) => setSystemTheme(e.target.value as any)}
              className="bg-transparent text-ink text-xs font-semibold focus:outline-none cursor-pointer border-0 p-0"
            >
              <option value="indigo" className="bg-surface-indigo">🌌 Índigo</option>
              <option value="dark" className="bg-surface-indigo">🌑 Escuro</option>
              <option value="amoled" className="bg-surface-indigo">🖤 AMOLED</option>
              <option value="light" className="bg-surface-indigo">☀️ Claro</option>
            </select>
          </div>

          <div className="w-[1px] h-4 bg-hairline/60" />

          {/* Engine Switch */}
          <div className="flex items-center gap-2">
            <span className={`text-xs font-bold tracking-wider ${rpcActive ? "text-green-accent" : "text-muted-ink"}`}>
              {rpcActive ? "ATIVO" : "INATIVO"}
            </span>
            <label className="relative inline-block w-9 h-5 shrink-0">
              <input 
                type="checkbox" 
                className="peer sr-only"
                checked={rpcActive}
                onChange={(e) => setRpcActive(e.target.checked)}
              />
              <span className="absolute cursor-pointer inset-0 bg-white/20 transition-colors duration-200 rounded-full before:absolute before:content-[''] before:h-3.5 before:w-3.5 before:left-[3px] before:bottom-[3px] before:bg-white before:transition-transform before:duration-200 before:rounded-full before:shadow-sm peer-checked:bg-green-accent peer-checked:before:translate-x-4"></span>
            </label>
          </div>
        </div>
      </header>


      {/* Main Viewport */}
      <main className="flex-1 overflow-y-auto flex flex-col gap-6 p-4">
        {/* Dashboard Preview Section (PresenceCard inside Dashboard) */}
        <section className={`transition-opacity duration-150 ${!rpcActive ? "opacity-50 pointer-events-none" : ""}`}>
          <Dashboard />
        </section>

        {/* Library Section (Apps manager) */}
        <section className={`transition-opacity duration-150 ${!rpcActive ? "opacity-50 pointer-events-none" : ""}`}>
          <Apps />
        </section>

        {/* Collapsible Logs History */}
        <section className="border-t border-hairline/20 pt-4">
          <button 
            onClick={() => setShowLogs(!showLogs)}
            className="w-full flex justify-between items-center text-xs font-bold tracking-wider text-muted-ink hover:text-ink transition-colors py-2 px-1 uppercase"
          >
            <span>📜 Histórico de Logs</span>
            <span>{showLogs ? "▲" : "▼"}</span>
          </button>
          
          {showLogs && (
            <div className="mt-2 p-3 bg-surface-indigo rounded-md border border-hairline">
              <LogViewer />
            </div>
          )}
        </section>

        {/* Collapsible Advanced Settings */}
        <section className="mt-auto border-t border-hairline/20 pt-4 pb-2">
          <button 
            onClick={() => setShowSettings(!showSettings)}
            className="w-full flex justify-between items-center text-xs font-bold tracking-wider text-muted-ink hover:text-ink transition-colors py-2 px-1 uppercase"
          >
            <span>⚙️ Configurações Avançadas</span>
            <span>{showSettings ? "▲" : "▼"}</span>
          </button>
          
          {showSettings && (
            <div className="mt-2 p-3 bg-surface-indigo rounded-md border border-hairline">
              <Settings />
            </div>
          )}
        </section>
      </main>
    </div>
  );
}

export default App;
