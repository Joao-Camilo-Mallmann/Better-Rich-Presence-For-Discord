import styles from "./StatusIndicator.module.css";
import { ConnectionInfo } from "../../types";

export function StatusIndicator({ info }: { info: ConnectionInfo }) {
  let colorClass = styles.disconnected;
  let text = "Desconectado";

  if (info.state === "Connected") {
    colorClass = styles.connected;
    text = "Conectado";
  } else if (info.state === "WaitingDiscord") {
    colorClass = styles.waiting;
    text = "Aguardando Discord";
  } else if (info.state === "PausedByGame") {
    colorClass = styles.paused;
    text = "Pausado (Jogo)";
  } else if (info.state === "Updating") {
    colorClass = styles.updating;
    text = "Atualizando...";
  }

  return (
    <div className={styles.container}>
      <div className={`${styles.dot} ${colorClass}`}></div>
      <span className={styles.text}>{text}</span>
    </div>
  );
}
