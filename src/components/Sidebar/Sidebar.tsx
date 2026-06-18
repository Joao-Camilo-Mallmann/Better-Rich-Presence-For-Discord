import styles from "./Sidebar.module.css";
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
    <aside className={styles.sidebar}>
      <div className={styles.header}>
        <h1 className={styles.title}>Better RPC</h1>
      </div>

      <nav className={styles.nav}>
        {navItems.map((item) => (
          <button
            key={item.id}
            className={`${styles.navItem} ${currentPage === item.id ? styles.active : ""}`}
            onClick={() => onNavigate(item.id)}
          >
            <span className={styles.icon}>{item.icon}</span>
            {item.label}
          </button>
        ))}
      </nav>

      <div className={styles.footer}>
        <div className={styles.separator} />
        <StatusIndicator info={connectionInfo} />
      </div>
    </aside>
  );
}
