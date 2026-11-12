import { getCurrentWindow } from "@tauri-apps/api/window";
import { Copy, Minus, Square, X } from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { getBandState, StatusBand } from "./components/StatusBand";
import { usePresence } from "./hooks/usePresence";
import { useSettings } from "./hooks/useSettings";
import { useWindowMaximized } from "./hooks/useWindowMaximized";
import { Apps } from "./pages/Apps";
import { Dashboard } from "./pages/Dashboard";
import { Settings } from "./pages/Settings";

const appWindow = getCurrentWindow();

type View = "now" | "apps" | "settings";

const VIEWS: { id: View; label: string }[] = [
  { id: "now", label: "Now" },
  { id: "apps", label: "Apps" },
  { id: "settings", label: "Settings" },
];

/** Content column shared by the tab bar and the views, so both line up when the window is wide. */
const COLUMN = "w-full max-w-[560px] mx-auto px-4";

const windowButton =
  "w-6 h-6 flex items-center justify-center neo-border-2 neo-press";

function App() {
  const { settings, updateSettings } = useSettings();
  const { presence, presenceState, connectionInfo } = usePresence();
  const isMaximized = useWindowMaximized();
  const [view, setViewState] = useState<View>("now");
  const tabRefs = useRef<(HTMLButtonElement | null)[]>([]);
  const mainRef = useRef<HTMLElement>(null);

  // The views share one scroll area, so each view opens at its top.
  const setView = (next: View) => {
    if (next === view) return;
    setViewState(next);
    mainRef.current?.scrollTo({ top: 0 });
  };

  const sharing = settings?.global_enabled ?? true;
  const bandState = getBandState(sharing, presenceState, connectionInfo);

  const handleSharingChange = async (checked: boolean) => {
    if (settings) {
      await updateSettings({ ...settings, global_enabled: checked });
    }
  };

  const selectViewAt = (index: number) => {
    const next = (index + VIEWS.length) % VIEWS.length;
    setView(VIEWS[next].id);
    tabRefs.current[next]?.focus();
  };

  const handleTabKeyDown = (e: React.KeyboardEvent, index: number) => {
    if (e.key === "ArrowRight") selectViewAt(index + 1);
    else if (e.key === "ArrowLeft") selectViewAt(index - 1);
    else if (e.key === "Home") selectViewAt(0);
    else if (e.key === "End") selectViewAt(VIEWS.length - 1);
    else return;
    e.preventDefault();
  };

  // The taskbar entry follows the app being shown on Discord.
  const appName = presence?.large_text || "Better RPC";
  let iconUrl = "/logo.png";
  if (presence?.large_image && presence.large_image !== "default") {
    iconUrl = presence.large_image;
  }

  useEffect(() => {
    document.title = appName;
    let link = document.querySelector("link[rel~='icon']") as HTMLLinkElement;
    if (!link) {
      link = document.createElement("link");
      link.rel = "icon";
      document.head.appendChild(link);
    }
    link.href = iconUrl;
  }, [appName, iconUrl]);

  useEffect(() => {
    document.documentElement.setAttribute("data-theme", "dark");
  }, []);

  return (
    <div className="w-full h-screen flex flex-col bg-canvas text-ink overflow-hidden">
      {/* ── Title bar: one row at every width ── */}
      <header
        data-tauri-drag-region
        className="shrink-0 flex items-center justify-between gap-3 px-4 py-2.5 bg-surface-indigo select-none cursor-default"
        style={{ borderBottom: "3px solid var(--neo-border-color)" }}
      >
        <div data-tauri-drag-region className="flex items-center gap-2 min-w-0">
          <img
            data-tauri-drag-region
            src="/logo.png"
            alt=""
            className="w-6 h-6 object-contain shrink-0"
          />
          <h1
            data-tauri-drag-region
            className="text-[15px] font-extrabold tracking-tight text-ink font-display uppercase truncate"
          >
            Better RPC
          </h1>
        </div>

        <div className="flex gap-2 shrink-0">
          <button
            onClick={() => appWindow.minimize()}
            className={`${windowButton} bg-yellow-accent text-ink-dark`}
            style={{ borderRadius: "4px" }}
            title="Minimize"
            aria-label="Minimize"
          >
            <Minus size={12} strokeWidth={3.5} />
          </button>
          <button
            onClick={() => appWindow.toggleMaximize()}
            className={`${windowButton} bg-green-accent text-ink-dark`}
            style={{ borderRadius: "4px" }}
            title={isMaximized ? "Restore" : "Maximize"}
            aria-label={isMaximized ? "Restore" : "Maximize"}
          >
            {isMaximized ? (
              <Copy size={11} strokeWidth={3} className="-scale-x-100" />
            ) : (
              <Square size={11} strokeWidth={3} />
            )}
          </button>
          <button
            onClick={() => appWindow.close()}
            className={`${windowButton} bg-danger text-white`}
            style={{ borderRadius: "4px" }}
            title="Close"
            aria-label="Close"
          >
            <X size={13} strokeWidth={3.5} />
          </button>
        </div>
      </header>

      {/* ── Status band: what Discord shows right now ── */}
      <StatusBand
        state={bandState}
        sharing={sharing}
        onSharingChange={handleSharingChange}
      />

      <div className="flex-1 min-h-0 flex flex-col dot-grid">
        {/* ── View tabs ── */}
        <div className={`${COLUMN} shrink-0 pt-4 pb-3`}>
          <div
            role="tablist"
            aria-label="Views"
            className="grid grid-cols-3 gap-3 pr-1"
          >
            {VIEWS.map((v, i) => (
              <button
                key={v.id}
                ref={(el) => {
                  tabRefs.current[i] = el;
                }}
                id={`tab-${v.id}`}
                role="tab"
                aria-selected={view === v.id}
                aria-controls={`panel-${v.id}`}
                tabIndex={view === v.id ? 0 : -1}
                onClick={() => setView(v.id)}
                onKeyDown={(e) => handleTabKeyDown(e, i)}
                className="neo-tab"
              >
                {v.label}
              </button>
            ))}
          </div>
        </div>

        {/* ── Views: all mounted, only the selected one visible ── */}
        <main ref={mainRef} className="flex-1 overflow-y-auto">
          <div className={`${COLUMN} pt-2 pb-6`}>
            <section
              id="panel-now"
              role="tabpanel"
              aria-labelledby="tab-now"
              hidden={view !== "now"}
            >
              <Dashboard bandState={bandState} />
            </section>
            <section
              id="panel-apps"
              role="tabpanel"
              aria-labelledby="tab-apps"
              hidden={view !== "apps"}
            >
              <Apps priorityMode={settings?.priority_mode_enabled ?? false} />
            </section>
            <section
              id="panel-settings"
              role="tabpanel"
              aria-labelledby="tab-settings"
              hidden={view !== "settings"}
            >
              <Settings settings={settings} onSave={updateSettings} />
            </section>
          </div>
        </main>
      </div>
    </div>
  );
}

export default App;
