import styles from "./Dashboard.module.css";
import { usePresence } from "../../hooks/usePresence";
import { PresenceCard } from "../../components/PresenceCard/PresenceCard";
import { useEffect, useState } from "react";

export function Dashboard() {
  const { presence, connectionInfo, source } = usePresence();
  const [elapsed, setElapsed] = useState<string>("0m");

  useEffect(() => {
    if (!presence?.timestamp) {
      setElapsed("0m");
      return;
    }

    const updateTimer = () => {
      const now = Math.floor(Date.now() / 1000);
      const diff = Math.max(0, now - presence.timestamp!);
      const mins = Math.floor(diff / 60);
      setElapsed(`${mins}m`);
    };

    updateTimer();
    const interval = setInterval(updateTimer, 60000);
    return () => clearInterval(interval);
  }, [presence?.timestamp]);

  return (
    <div className={styles.page}>
      <header className={styles.header}>
        <h2 className={styles.title}>Dashboard</h2>
        <p className={styles.subtitle}>Visão geral da sua presença no Discord</p>
      </header>

      <section className={styles.previewSection}>
        <h3 className={styles.sectionTitle}>Pré-visualização</h3>
        <PresenceCard presence={presence} />
      </section>

      <section className={styles.statsGrid}>
        <div className={styles.statCard}>
          <div className={styles.statLabel}>Status da Conexão</div>
          <div className={styles.statValue}>
            {connectionInfo.state === "Connected" ? "Online" : "Offline"}
          </div>
        </div>
        
        <div className={styles.statCard}>
          <div className={styles.statLabel}>Fonte Atual</div>
          <div className={styles.statValue}>{source}</div>
        </div>

        <div className={styles.statCard}>
          <div className={styles.statLabel}>Tempo de Atividade</div>
          <div className={styles.statValue}>{elapsed}</div>
        </div>
      </section>
    </div>
  );
}
