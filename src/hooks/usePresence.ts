import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { PresenceData, PresenceState, ConnectionInfo, PriorityInfo, DiscordUser } from "../types";

export function usePresence() {
  const [presence, setPresence] = useState<PresenceData | null>(null);
  const [presenceState, setPresenceState] = useState<PresenceState>("Disconnected");
  const [discordUser, setDiscordUser] = useState<DiscordUser | null>(null);
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
        const [initPresence, initState, initConnection, initUser] = await Promise.all([
          invoke<PresenceData | null>("get_current_presence"),
          invoke<PresenceState>("get_presence_state"),
          invoke<ConnectionInfo>("get_connection_status"),
          invoke<DiscordUser | null>("get_discord_user"),
        ]);
        setPresence(initPresence);
        setPresenceState(initState);
        setConnectionInfo(initConnection);
        setDiscordUser(initUser);

        unlistens.push(
          await listen<PresenceData>("presence-updated", (e) => {
            setPresence(e.payload);
          }),
          await listen<PresenceState>("state-changed", (e) => setPresenceState(e.payload)),
          await listen<ConnectionInfo>("connection-changed", (e) => setConnectionInfo(e.payload)),
          // Listen for priority mode info from the engine
          await listen<PriorityInfo>("priority-info", (e) => setPriorityInfo(e.payload)),
          // Listen for Discord user profile updates from the backend connection
          await listen<DiscordUser | null>("discord-user-updated", (e) => setDiscordUser(e.payload)),
        );
      } catch (error) {
        console.error("Failed to initialize presence hooks:", error);
      }
    }

    init();
    return () => unlistens.forEach((fn) => fn());
  }, []);

  return { presence, presenceState, connectionInfo, priorityInfo, discordUser };
}
