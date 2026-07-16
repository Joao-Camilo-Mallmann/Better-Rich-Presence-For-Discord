import { useEffect, useState } from "react";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useSettings } from "./hooks/useSettings";
import { Apps } from "./pages/Apps/Apps";
import { Dashboard } from "./pages/Dashboard/Dashboard";
import { Settings } from "./pages/Settings/Settings";
import { Settings as SettingsIcon } from "lucide-react";

const appWindow = getCurrentWindow();

function App() {
  const { settings, updateSettings } = useSettings();
  const [showSettings, setShowSettings] = useState(false);

  const rpcActive = settings?.global_enabled ?? true;

  const handleToggleRpc = async (checked: boolean) => {
    if (settings) {
      await updateSettings({ ...settings, global_enabled: checked });
    }
  };

  useEffect(() => {
    document.documentElement.setAttribute("data-theme", "dark");
  }, []);

  return (
    <div className="w-full h-screen flex flex-col bg-canvas text-ink overflow-hidden">
      {/* ── Header — Neo-Brutalist Top Bar with Drag Region ── */}
      <header
        data-tauri-drag-region
        className="flex flex-col sm:flex-row justify-between items-center gap-3 px-4 py-3 bg-surface-indigo neo-border shrink-0 select-none cursor-default"
        style={{ borderTop: 'none', borderLeft: 'none', borderRight: 'none' }}
      >
        <div data-tauri-drag-region className="flex items-center gap-3 w-full sm:w-auto justify-center sm:justify-start">
          <div data-tauri-drag-region className="flex items-center gap-2">
            <img
              src="/logo.png"
              alt="Better RPC Logo"
              width="28"
              height="21"
              className="object-contain"
            />
            <h1 data-tauri-drag-region className="text-lg font-extrabold tracking-tight text-ink font-display uppercase">
              Better RPC
            </h1>
          </div>
        </div>

        {/* Header Controls */}
        <div className="flex flex-wrap items-center justify-center sm:justify-end gap-3 w-full sm:w-auto">
          {/* Engine Switch — Arcade toggle */}
          <div className="flex items-center gap-2">
            <span
              className={`text-xs font-extrabold tracking-wider font-display uppercase ${rpcActive ? "text-green-accent" : "text-muted-ink"}`}
            >
              {rpcActive ? "ACTIVE" : "OFF"}
            </span>
            <div
              className={`neo-toggle ${rpcActive ? "active" : ""}`}
              onClick={() => handleToggleRpc(!rpcActive)}
            >
              <div className="neo-toggle-knob" />
            </div>
          </div>

          {/* Separator */}
          <div className="w-[3px] h-5 bg-[var(--neo-border-color)]" />

          {/* Custom Window Buttons */}
          <div className="flex gap-2">
            <button
              onClick={() => appWindow.minimize()}
              className="w-6 h-6 flex items-center justify-center text-xs font-black bg-yellow-accent text-ink-dark neo-border-2 neo-press"
              style={{ borderRadius: '4px' }}
              title="Minimize"
            >
              —
            </button>
            <button
              onClick={() => appWindow.toggleMaximize()}
              className="w-6 h-6 flex items-center justify-center text-[10px] font-black bg-green-accent text-ink-dark neo-border-2 neo-press animate-none"
              style={{ borderRadius: '4px' }}
              title="Maximize"
            >
              ▢
            </button>
            <button
              onClick={() => appWindow.close()}
              className="w-6 h-6 flex items-center justify-center text-xs font-black bg-danger text-white neo-border-2 neo-press"
              style={{ borderRadius: '4px' }}
              title="Close"
            >
              ✕
            </button>
          </div>
        </div>
      </header>

      {/* ── Main Viewport — Dot Grid Background ── */}
      <main className="flex-1 overflow-y-auto flex flex-col gap-6 p-4 dot-grid">
        {/* Dashboard Preview Section */}
        <section
          className={`transition-opacity duration-150 ${!rpcActive ? "opacity-40 pointer-events-none" : ""}`}
        >
          <Dashboard />
        </section>

        {/* Library Section (Apps manager) */}
        <section
          className={`transition-opacity duration-150 ${!rpcActive ? "opacity-40 pointer-events-none" : ""}`}
        >
          <Apps />
        </section>



        {/* ── Collapsible Settings — Arcade Accordion ── */}
        <section className="mt-auto pb-2">
          <div
            className="transition-all duration-100"
            style={{
              transform: showSettings ? 'translate(4px, 4px)' : 'none',
              width: showSettings ? 'calc(100% - 4px)' : '100%',
            }}
          >
            <button
              onClick={() => setShowSettings(!showSettings)}
              className="w-full flex justify-between items-center neo-border-2 bg-surface-indigo px-4 py-2.5 text-xs font-extrabold tracking-wider text-ink hover:bg-primary/20 transition-colors uppercase font-display neo-press"
              style={{
                boxShadow: showSettings ? '0px 0px 0px var(--neo-shadow-color)' : '4px 4px 0px var(--neo-shadow-color)',
              }}
            >
              <span className="neo-tilt flex items-center gap-2">
                <SettingsIcon size={14} /> Advanced Settings
              </span>
              <span className="text-sm">{showSettings ? "▲" : "▼"}</span>
            </button>

            {showSettings && (
              <div className="mt-0 p-3 bg-surface-onyx neo-border" style={{ borderTop: 'none' }}>
                <Settings />
              </div>
            )}
          </div>
        </section>
      </main>
    </div>
  );
}

export default App;
