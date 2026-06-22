import { useState } from "react";
import { Dashboard } from "./pages/Dashboard/Dashboard";
import { Apps } from "./pages/Apps/Apps";
import { Settings } from "./pages/Settings/Settings";

function App() {
  const [rpcActive, setRpcActive] = useState(true);
  const [showSettings, setShowSettings] = useState(false);

  return (
    <div className="w-full h-screen flex flex-col bg-canvas text-ink overflow-hidden">
      {/* Header */}
      <header className="flex justify-between items-center px-4 py-3 border-b border-hairline bg-surface-indigo shrink-0">
        <div className="flex items-center gap-2">
          <img src="/logo.png" alt="Better RPC Logo" width="28" height="21" className="object-contain" />
          <h1 className="text-md font-bold tracking-tight text-ink font-display">Better RPC</h1>
        </div>
        
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
