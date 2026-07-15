import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useSettings } from "../../hooks/useSettings";
import { Settings as SettingsType } from "../../types";

const Toggle = ({ checked, onChange }: { checked: boolean; onChange: (v: boolean) => void }) => (
  <div
    className={`neo-toggle ${checked ? "active" : ""}`}
    onClick={() => onChange(!checked)}
  >
    <div className="neo-toggle-knob" />
  </div>
);

const NumberInput = ({ value, onChange, min, max }: { value: number; onChange: (v: number) => void; min: number; max: number }) => (
  <input
    className="w-16 text-right neo-input text-xs py-1"
    style={{ borderRadius: '4px' }}
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
  <div className={`flex justify-between items-center py-3 gap-4${borderB ? "" : ""}`}
    style={{ borderBottom: borderB ? '2px solid var(--neo-border-color)' : 'none' }}>
    <div className="flex flex-col">
      <span className="font-bold text-ink text-xs flex items-center gap-2 font-display uppercase">
        {icon && <span className="text-magenta-accent">{icon}</span>}
        {label}
      </span>
      {description && <span className="text-[10px] text-muted-ink mt-0.5">{description}</span>}
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
    return <div className="text-muted-ink p-4 text-center text-xs font-display font-bold uppercase">Loading settings...</div>;
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
      <div className="flex flex-col gap-1">
        {/* Max Priority Mode */}
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
          <div className="flex flex-col gap-1 py-3" style={{ borderBottom: '2px solid var(--neo-border-color)' }}>
            <span className="text-xs text-muted-ink font-bold font-display uppercase">Idle Message</span>
            <input
              className="neo-input text-xs py-1"
              style={{ borderRadius: '4px' }}
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

      {/* Save Button — Green CTA with hard shadow */}
      <button
        className="w-full neo-btn bg-green-accent text-ink-dark text-sm py-2.5 font-extrabold disabled:opacity-60 disabled:cursor-not-allowed mt-2"
        style={{ borderRadius: '6px' }}
        onClick={handleSave}
        disabled={isSaving}
      >
        {isSaving ? "Saving..." : "Save Settings"}
      </button>
    </div>
  );
}
