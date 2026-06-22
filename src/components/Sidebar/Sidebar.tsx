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
    { id: "apps", label: "Aplicativos", icon: "📱" },
    { id: "settings", label: "Configurações", icon: "⚙️" },
  ] as const;

  return (
    <aside className="w-[220px] h-screen flex flex-col bg-surface-indigo p-4 border-r border-hairline shrink-0">
      <div className="p-2 mb-4 flex items-center gap-2">
        <img src="/logo.png" alt="Better RPC Logo" width="32" height="24" />
        <h1 className="text-lg font-bold text-ink tracking-tight">Better RPC</h1>
      </div>

      <nav className="flex-1 flex flex-col gap-1">
        {navItems.map((item) => (
          <button
            key={item.id}
            className={`group flex items-center gap-3 px-3 py-2 rounded-md font-medium transition duration-100 relative text-left ${
              currentPage === item.id
                ? "bg-white/15 text-ink before:content-[''] before:absolute before:-left-4 before:top-1/2 before:-translate-y-1/2 before:w-1 before:h-6 before:bg-primary before:rounded-r-md"
                : "text-muted-ink hover:bg-white/5 hover:text-ink"
            }`}
            onClick={() => onNavigate(item.id)}
          >
            <span className={`text-lg transition duration-100 ${currentPage === item.id ? "opacity-100" : "opacity-80 group-hover:opacity-100"}`}>
              {item.icon}
            </span>
            {item.label}
          </button>
        ))}
      </nav>

      <div className="mt-auto flex flex-col gap-4">
        <div className="h-[1px] bg-hairline mx-2" />
        <StatusIndicator info={connectionInfo} />
      </div>
    </aside>
  );
}
