import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
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
        if (autostartActive !== data.autostart_enabled) {
          // Keep settings object in sync with actual system state
          data.autostart_enabled = autostartActive;
          // Don't save it back immediately to avoid loops, just update local state
        }
      } catch (e) {
        console.warn("Could not check autostart status", e);
      }
      
      setSettings(data);
    } catch (error) {
      console.error("Failed to fetch settings:", error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchSettings();
  }, []);

  const updateSettings = async (newSettings: Settings) => {
    try {
      // Handle autostart plugin
      if (settings && settings.autostart_enabled !== newSettings.autostart_enabled) {
        if (newSettings.autostart_enabled) {
          await enable();
        } else {
          await disable();
        }
      }

      await invoke("update_settings", { settings: newSettings });
      setSettings(newSettings);
    } catch (error) {
      console.error("Failed to update settings:", error);
      throw error;
    }
  };

  return { settings, loading, updateSettings, refresh: fetchSettings };
}
