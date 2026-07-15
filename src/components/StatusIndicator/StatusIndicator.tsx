import styles from "./StatusIndicator.module.css";
import { ConnectionInfo } from "../../types";

const stateMap: Record<string, { color: string; text: string }> = {
  Connected: { color: styles.connected, text: "Connected" },
  WaitingDiscord: { color: styles.waiting, text: "Waiting for Discord" },
  PausedByGame: { color: styles.paused, text: "Paused (Game)" },
  Updating: { color: styles.updating, text: "Updating..." },
};

export function StatusIndicator({ info }: { info: ConnectionInfo }) {
  const { color = styles.disconnected, text = "Disconnected" } = stateMap[info.state] || {};
  return (
    <div className={styles.container}>
      <div className={`${styles.dot} ${color}`}></div>
      <span className={styles.text}>{text}</span>
    </div>
  );
}
