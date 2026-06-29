import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useSettings } from "../../hooks/useSettings";
import { Settings as SettingsType } from "../../types";

export function Settings() {
  const { settings, loading, updateSettings } = useSettings();
  const [localSettings, setLocalSettings] = useState(settings);
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    if (settings && !localSettings) {
      setLocalSettings(settings);
    }
  }, [settings]);

  if (loading || !localSettings) {
    return <div className="text-muted-ink p-4 text-center text-xs">Loading settings...</div>;
  }

  const handleSave = async () => {
    try {
      setIsSaving(true);
      const freshSettings = await invoke<SettingsType>("get_settings");
      await updateSettings({ ...localSettings, global_enabled: freshSettings.global_enabled });
      setTimeout(() => setIsSaving(false), 1000);
    } catch (e) {
      console.error(e);
      setIsSaving(false);
    }
  };

  const toggleStyle = "absolute cursor-pointer inset-0 bg-muted-ink/30 transition-colors duration-200 rounded-full before:absolute before:content-[''] before:h-3.5 before:w-3.5 before:left-[2px] before:bottom-[2.5px] before:bg-white before:transition-transform before:duration-200 before:rounded-full peer-checked:bg-green-accent peer-checked:before:translate-x-3.5";

  return (
    <div className="flex flex-col gap-4 text-sm">
      <div className="flex flex-col gap-3">

        {/* ── Modo Prioridade Máxima ── */}
        <div className="flex justify-between items-center py-2 border-b border-hairline/10 gap-4">
          <div className="flex flex-col">
            <span className="font-semibold text-ink text-xs flex items-center gap-1.5">
              <span className="text-magenta-accent">⬆️</span> Max Priority Mode
            </span>
            <span className="text-[10px] text-muted-ink">
              Always displays the highest priority app running on the PC, even if not in focus.
              The rule list order defines the priority.
            </span>
          </div>
          <label className="relative inline-block w-8 h-4.5 shrink-0">
            <input
              type="checkbox"
              className="peer sr-only"
              checked={localSettings.priority_mode_enabled}
              onChange={(e) => setLocalSettings({ ...localSettings, priority_mode_enabled: e.target.checked })}
            />
            <span className={toggleStyle}></span>
          </label>
        </div>

        {/* ── Windows Autostart ── */}
        <div className="flex justify-between items-center py-2 border-b border-hairline/10 gap-4">
          <div className="flex flex-col">
            <span className="font-semibold text-ink text-xs">Start with Windows</span>
            <span className="text-[10px] text-muted-ink">Open minimized on boot</span>
          </div>
          <label className="relative inline-block w-8 h-4.5 shrink-0">
            <input
              type="checkbox"
              className="peer sr-only"
              checked={localSettings.autostart_enabled}
              onChange={(e) => setLocalSettings({ ...localSettings, autostart_enabled: e.target.checked })}
            />
            <span className={toggleStyle}></span>
          </label>
        </div>

        {/* ── Idle Detection ── */}
        <div className="flex justify-between items-center py-2 border-b border-hairline/10 gap-4">
          <div className="flex flex-col">
            <span className="font-semibold text-ink text-xs">Enable Idle Detection</span>
            <span className="text-[10px] text-muted-ink">Change status when inactive</span>
          </div>
          <label className="relative inline-block w-8 h-4.5 shrink-0">
            <input
              type="checkbox"
              className="peer sr-only"
              checked={localSettings.idle_enabled}
              onChange={(e) => setLocalSettings({ ...localSettings, idle_enabled: e.target.checked })}
            />
            <span className={toggleStyle}></span>
          </label>
        </div>

        {/* Idle Threshold */}
        {localSettings.idle_enabled && (
          <div className="flex justify-between items-center py-2 border-b border-hairline/10 gap-4">
            <span className="text-xs text-muted-ink font-semibold">Idle Time (minutes)</span>
            <input
              className="w-16 text-right bg-surface-onyx border border-hairline/30 text-ink text-xs px-2 py-1 rounded-xs focus:border-primary focus:outline-none"
              type="number"
              min="1"
              max="60"
              value={localSettings.idle_threshold_minutes}
              onChange={(e) => setLocalSettings({ ...localSettings, idle_threshold_minutes: parseInt(e.target.value) || 5 })}
            />
          </div>
        )}

        {/* Idle Message */}
        {localSettings.idle_enabled && (
          <div className="flex flex-col gap-1 py-2 border-b border-hairline/10">
            <span className="text-xs text-muted-ink font-semibold">Idle Message</span>
            <input
              className="bg-surface-onyx border border-hairline/30 text-ink text-xs px-2 py-1 rounded-xs focus:border-primary focus:outline-none"
              type="text"
              value={localSettings.idle_message}
              onChange={(e) => setLocalSettings({ ...localSettings, idle_message: e.target.value })}
            />
          </div>
        )}

        {/* Debounce */}
        <div className="flex justify-between items-center py-2 border-b border-hairline/10 gap-4">
          <span className="text-xs text-muted-ink font-semibold">Discord Debounce (seconds)</span>
          <input
            className="w-16 text-right bg-surface-onyx border border-hairline/30 text-ink text-xs px-2 py-1 rounded-xs focus:border-primary focus:outline-none"
            type="number"
            min="15"
            max="60"
            value={localSettings.debounce_seconds}
            onChange={(e) => setLocalSettings({ ...localSettings, debounce_seconds: parseInt(e.target.value) || 15 })}
          />
        </div>

        {/* Anti-flicker delay */}
        <div className="flex justify-between items-center py-2 gap-4">
          <span className="text-xs text-muted-ink font-semibold">Anti-flicker Delay (seconds)</span>
          <input
            className="w-16 text-right bg-surface-onyx border border-hairline/30 text-ink text-xs px-2 py-1 rounded-xs focus:border-primary focus:outline-none"
            type="number"
            min="1"
            max="10"
            value={localSettings.settle_delay_seconds}
            onChange={(e) => setLocalSettings({ ...localSettings, settle_delay_seconds: parseInt(e.target.value) || 3 })}
          />
        </div>
      </div>

      <button
        className="w-full bg-primary text-white text-xs py-2 rounded-xs font-bold hover:opacity-90 disabled:opacity-75 disabled:cursor-not-allowed transition duration-100 mt-2"
        onClick={handleSave}
        disabled={isSaving}
      >
        {isSaving ? "Saving..." : "Save Settings"}
      </button>
    </div>
  );
}
