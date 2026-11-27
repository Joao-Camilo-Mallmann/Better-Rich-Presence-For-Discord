import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Settings } from "../types";
import { enable, disable, isEnabled } from "@tauri-apps/plugin-autostart";

export function useSettings() {
  const [settings, setSettings] = useState<Settings | null>(null);
  const [loading, setLoading] = useState(true);

  const fetchSettings = async () => {
    try {
      setLoading(true);
      const data = await invoke<Settings>("get_settings");
      // Sync autostart plugin state with settings
      try {
        const autostartActive = await isEnabled();
        // Keep settings object in sync with actual system state
        if (autostartActive !== data.autostart_enabled) data.autostart_enabled = autostartActive;
      } catch (e) {
        console.warn("Could not check autostart status", e);
      }
      setSettings(data);
    } catch (error) {
      console.error("Failed to fetch settings:", error);
    }
    // No `finally`: the React Compiler cannot compile try/finally yet.
    setLoading(false);
  };

  useEffect(() => {
    fetchSettings();
    const unlistenPromise = listen<Settings>("settings-updated", (event) => {
      setSettings(event.payload);
    });
    return () => {
      unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

  const updateSettings = async (newSettings: Settings) => {
    // Computed outside `try`: the React Compiler cannot compile logical or
    // conditional expressions inside try/catch yet.
    const autostartChanged =
      settings !== null && settings.autostart_enabled !== newSettings.autostart_enabled;
    const previous = settings;
    // Shown at once, so a second change made while this one is still saving builds on it.
    setSettings(newSettings);
    try {
      // Handle autostart plugin
      if (autostartChanged) {
        if (newSettings.autostart_enabled) await enable();
        else await disable();
      }
      await invoke("update_settings", { settings: newSettings });
    } catch (error) {
      console.error("Failed to update settings:", error);
      setSettings(previous);
      throw error;
    }
  };

  return { settings, loading, updateSettings, refresh: fetchSettings };
}
