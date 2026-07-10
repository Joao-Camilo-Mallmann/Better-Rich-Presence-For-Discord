import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useSettings } from "../../hooks/useSettings";
import { Settings as SettingsType } from "../../types";

const TOGGLE_STYLE = "absolute cursor-pointer inset-0 bg-muted-ink/30 transition-colors duration-200 rounded-full before:absolute before:content-[''] before:h-3.5 before:w-3.5 before:left-[2px] before:bottom-[2.5px] before:bg-white before:transition-transform before:duration-200 before:rounded-full peer-checked:bg-green-accent peer-checked:before:translate-x-3.5";

const Toggle = ({ checked, onChange }: { checked: boolean; onChange: (v: boolean) => void }) => (
  <label className="relative inline-block w-8 h-4.5 shrink-0">
    <input type="checkbox" className="peer sr-only" checked={checked} onChange={(e) => onChange(e.target.checked)} />
    <span className={TOGGLE_STYLE}></span>
  </label>
);

const NumberInput = ({ value, onChange, min, max }: { value: number; onChange: (v: number) => void; min: number; max: number }) => (
  <input
    className="w-16 text-right bg-surface-onyx border border-hairline/30 text-ink text-xs px-2 py-1 rounded-xs focus:border-primary focus:outline-none"
    type="number"
    min={min}
    max={max}
    value={value}
    onChange={(e) => onChange(parseInt(e.target.value) || min)}
  />
);

const SettingRow = ({ label, description, icon, children, borderB = true }: {
  label: string; description?: string; icon?: string; children: React.ReactNode; borderB?: boolean;
}) => (
  <div className={`flex justify-between items-center py-2 gap-4${borderB ? " border-b border-hairline/10" : ""}`}>
    <div className="flex flex-col">
      <span className="font-semibold text-ink text-xs flex items-center gap-1.5">
        {icon && <span className="text-magenta-accent">{icon}</span>}
        {label}
      </span>
      {description && <span className="text-[10px] text-muted-ink">{description}</span>}
    </div>
    {children}
  </div>
);

export function Settings() {
  const { settings, loading, updateSettings } = useSettings();
  const [localSettings, setLocalSettings] = useState(settings);
  const [isSaving, setIsSaving] = useState(false);

  useEffect(() => {
    if (settings && !localSettings) setLocalSettings(settings);
  }, [settings]);

  if (loading || !localSettings) {
    return <div className="text-muted-ink p-4 text-center text-xs">Loading settings...</div>;
  }

  const set = <K extends keyof SettingsType>(key: K, value: SettingsType[K]) =>
    setLocalSettings({ ...localSettings, [key]: value });

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

  return (
    <div className="flex flex-col gap-4 text-sm">
      <div className="flex flex-col gap-3">
        {/* ── Modo Prioridade Máxima ── */}
        <SettingRow
          label="Max Priority Mode"
          icon="⬆️"
          description="Always displays the highest priority app running on the PC, even if not in focus. The rule list order defines the priority."
        >
          <Toggle checked={localSettings.priority_mode_enabled} onChange={(v) => set("priority_mode_enabled", v)} />
        </SettingRow>

        <SettingRow label="Start with Windows" description="Open minimized on boot">
          <Toggle checked={localSettings.autostart_enabled} onChange={(v) => set("autostart_enabled", v)} />
        </SettingRow>

        <SettingRow label="Enable Idle Detection" description="Change status when inactive">
          <Toggle checked={localSettings.idle_enabled} onChange={(v) => set("idle_enabled", v)} />
        </SettingRow>

        {localSettings.idle_enabled && (
          <SettingRow label="Idle Time (minutes)" description="">
            <NumberInput value={localSettings.idle_threshold_minutes} onChange={(v) => set("idle_threshold_minutes", v)} min={1} max={60} />
          </SettingRow>
        )}

        {localSettings.idle_enabled && (
          <div className="flex flex-col gap-1 py-2 border-b border-hairline/10">
            <span className="text-xs text-muted-ink font-semibold">Idle Message</span>
            <input
              className="bg-surface-onyx border border-hairline/30 text-ink text-xs px-2 py-1 rounded-xs focus:border-primary focus:outline-none"
              type="text"
              value={localSettings.idle_message}
              onChange={(e) => set("idle_message", e.target.value)}
            />
          </div>
        )}

        <SettingRow label="Discord Debounce (seconds)">
          <NumberInput value={localSettings.debounce_seconds} onChange={(v) => set("debounce_seconds", v)} min={15} max={60} />
        </SettingRow>

        <SettingRow label="Anti-flicker Delay (seconds)" borderB={false}>
          <NumberInput value={localSettings.settle_delay_seconds} onChange={(v) => set("settle_delay_seconds", v)} min={1} max={10} />
        </SettingRow>
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
