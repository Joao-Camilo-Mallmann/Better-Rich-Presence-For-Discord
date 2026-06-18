import { useState } from "react";
import styles from "./App.module.css";
import { Sidebar } from "./components/Sidebar/Sidebar";
import { Dashboard } from "./pages/Dashboard/Dashboard";
import { Apps } from "./pages/Apps/Apps";
import { Settings } from "./pages/Settings/Settings";

function App() {
  const [currentPage, setCurrentPage] = useState<"dashboard" | "apps" | "settings">("dashboard");

  return (
    <div className={styles.appContainer}>
      <Sidebar currentPage={currentPage} onNavigate={setCurrentPage} />
      
      <div className={styles.contentArea}>
        <div className={styles.scrollArea}>
          {currentPage === "dashboard" && <Dashboard />}
          {currentPage === "apps" && <Apps />}
          {currentPage === "settings" && <Settings />}
        </div>
      </div>
    </div>
  );
}

export default App;
