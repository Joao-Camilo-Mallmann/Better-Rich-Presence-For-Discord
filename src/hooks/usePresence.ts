import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { PresenceData, PresenceState, ConnectionInfo, PriorityInfo } from "../types";

export function usePresence() {
  const [presence, setPresence] = useState<PresenceData | null>(null);
  const [presenceState, setPresenceState] = useState<PresenceState>("Disconnected");
  const [connectionInfo, setConnectionInfo] = useState<ConnectionInfo>({
    connected: false,
    state: "Disconnected",
    last_error: null,
    reconnect_attempts: 0,
  });
  const [priorityInfo, setPriorityInfo] = useState<PriorityInfo>({
    active: false,
    prioritized_app: "",
    foreground_app: "",
  });

  useEffect(() => {
    const unlistens: UnlistenFn[] = [];

    async function init() {
      try {
        const [initPresence, initState, initConnection] = await Promise.all([
          invoke<PresenceData | null>("get_current_presence"),
          invoke<PresenceState>("get_presence_state"),
          invoke<ConnectionInfo>("get_connection_status"),
        ]);
        setPresence(initPresence);
        setPresenceState(initState);
        setConnectionInfo(initConnection);

        unlistens.push(
          await listen<PresenceData>("presence-updated", (e) => {
            setPresence(e.payload);
          }),
          await listen<PresenceState>("state-changed", (e) => setPresenceState(e.payload)),
          await listen<ConnectionInfo>("connection-changed", (e) => setConnectionInfo(e.payload)),
          // Listen for priority mode info from the engine
          await listen<PriorityInfo>("priority-info", (e) => setPriorityInfo(e.payload)),
        );
      } catch (error) {
        console.error("Failed to initialize presence hooks:", error);
      }
    }

    init();
    return () => unlistens.forEach((fn) => fn());
  }, []);

  return { presence, presenceState, connectionInfo, priorityInfo };
}
