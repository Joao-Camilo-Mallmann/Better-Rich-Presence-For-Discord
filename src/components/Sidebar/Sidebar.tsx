import { usePresence } from "../../hooks/usePresence";
import { StatusIndicator } from "../StatusIndicator/StatusIndicator";

interface SidebarProps {
  currentPage: "dashboard" | "apps" | "settings";
  onNavigate: (page: "dashboard" | "apps" | "settings") => void;
}

export function Sidebar({ currentPage, onNavigate }: SidebarProps) {
  const { connectionInfo } = usePresence();

  const navItems = [
    { id: "dashboard", label: "Dashboard", icon: "🏠" },
    { id: "apps", label: "Apps", icon: "📱" },
    { id: "settings", label: "Settings", icon: "⚙️" },
  ] as const;

  return (
    <aside className="w-[220px] h-screen flex flex-col bg-surface-indigo p-4 shrink-0"
      style={{ borderRight: '3px solid var(--neo-border-color)' }}>
      <div className="p-2 mb-4 flex items-center gap-2">
        <img src="/logo.png" alt="Better RPC Logo" width="32" height="24" />
        <h1 className="text-lg font-extrabold text-ink tracking-tight font-display uppercase neo-stroke"
          style={{ WebkitTextStroke: '1px var(--neo-border-color)' }}>
          Better RPC
        </h1>
      </div>
      <nav className="flex-1 flex flex-col gap-2">
        {navItems.map((item) => {
          const isActive = currentPage === item.id;
          return (
            <button
              key={item.id}
              className={`group flex items-center gap-3 px-3 py-2 font-extrabold transition-all text-left font-display uppercase text-sm neo-border-2 ${
                isActive
                  ? "bg-primary text-white"
                  : "bg-transparent text-muted-ink hover:bg-white/5 hover:text-ink neo-press"
              }`}
              style={{
                borderRadius: '6px',
                boxShadow: isActive ? '4px 4px 0 var(--neo-shadow-color)' : 'none',
                borderColor: isActive ? 'var(--neo-border-color)' : 'transparent',
              }}
              onClick={() => onNavigate(item.id)}
            >
              {isActive && (
                <div className="w-1 h-5 bg-white mr-1" style={{ borderRadius: '0' }} />
              )}
              <span className={`text-lg transition-all duration-100 ${isActive ? "opacity-100" : "opacity-60 group-hover:opacity-100"}`}>
                {item.icon}
              </span>
              {item.label}
            </button>
          );
        })}
      </nav>
      <div className="mt-auto flex flex-col gap-4">
        <div className="h-[3px] bg-[var(--neo-border-color)] mx-2" />
        <StatusIndicator info={connectionInfo} />
      </div>
    </aside>
  );
}
