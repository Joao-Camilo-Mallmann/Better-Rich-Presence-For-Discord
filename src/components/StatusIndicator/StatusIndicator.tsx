import styles from "./StatusIndicator.module.css";
import { ConnectionInfo } from "../../types";

const stateMap: Record<string, { color: string; text: string }> = {
  Connected: { color: styles.connected, text: "Conectado" },
  WaitingDiscord: { color: styles.waiting, text: "Aguardando Discord" },
  PausedByGame: { color: styles.paused, text: "Pausado (Jogo)" },
  Updating: { color: styles.updating, text: "Atualizando..." },
};

export function StatusIndicator({ info }: { info: ConnectionInfo }) {
  const { color = styles.disconnected, text = "Desconectado" } = stateMap[info.state] || {};
  return (
    <div className={styles.container}>
      <div className={`${styles.dot} ${color}`}></div>
      <span className={styles.text}>{text}</span>
    </div>
  );
}
