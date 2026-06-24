import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { PresenceData, PresenceState, PresenceSource, ConnectionInfo, PriorityInfo } from "../types";

export function usePresence() {
  const [presence, setPresence] = useState<PresenceData | null>(null);
  const [presenceState, setPresenceState] = useState<PresenceState>("Disconnected");
  const [source, setSource] = useState<PresenceSource>("Idle");
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
    let unlistenPresence: UnlistenFn;
    let unlistenState: UnlistenFn;
    let unlistenConnection: UnlistenFn;
    let unlistenPriority: UnlistenFn;

    async function init() {
      try {
        // Load initial state
        const initialPresence = await invoke<PresenceData | null>("get_current_presence");
        const initialState = await invoke<PresenceState>("get_presence_state");
        const initialSource = await invoke<PresenceSource>("get_current_source");
        const initialConnection = await invoke<ConnectionInfo>("get_connection_status");

        setPresence(initialPresence);
        setPresenceState(initialState);
        setSource(initialSource);
        setConnectionInfo(initialConnection);

        // Setup event listeners
        unlistenPresence = await listen<PresenceData>("presence-updated", (event) => {
          setPresence(event.payload);
          setSource(event.payload.source);
        });

        unlistenState = await listen<PresenceState>("state-changed", (event) => {
          setPresenceState(event.payload);
        });

        unlistenConnection = await listen<ConnectionInfo>("connection-changed", (event) => {
          setConnectionInfo(event.payload);
        });

        // Listen for priority mode info from the engine
        unlistenPriority = await listen<PriorityInfo>("priority-info", (event) => {
          setPriorityInfo(event.payload);
        });
      } catch (error) {
        console.error("Failed to initialize presence hooks:", error);
      }
    }

    init();

    return () => {
      if (unlistenPresence) unlistenPresence();
      if (unlistenState) unlistenState();
      if (unlistenConnection) unlistenConnection();
      if (unlistenPriority) unlistenPriority();
    };
  }, []);

  return { presence, presenceState, connectionInfo, source, priorityInfo };
}
