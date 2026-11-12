import { useEffect, useState } from "react";
import { DetectionDetails, DetectionHint } from "../components/DetectionStatus";
import { Toggle } from "../components/Toggle";
import { usePlatformStatus } from "../hooks/usePlatformStatus";
import { Settings as SettingsType } from "../types";

/** Settings edited on this view. `global_enabled` is owned by the Share switch and never saved from here. */
const EDITABLE_KEYS: (keyof SettingsType)[] = [
  "autostart_enabled",
  "priority_mode_enabled",
  "idle_enabled",
  "idle_threshold_minutes",
  "idle_message",
  "debounce_seconds",
  "settle_delay_seconds",
];

const NumberInput = ({
  label,
  value,
  onChange,
  min,
  max,
}: {
  label: string;
  value: number;
  onChange: (v: number) => void;
  min: number;
  max: number;
}) => (
  <input
    aria-label={label}
    className="w-16 shrink-0 text-right neo-input text-[13px] py-1"
    style={{ borderRadius: "6px" }}
    type="number"
    min={min}
    max={max}
    value={value}
    onChange={(e) => onChange(parseInt(e.target.value) || min)}
  />
);

const Group = ({
  title,
  children,
}: {
  title: string;
  children: React.ReactNode;
}) => (
  <section className="flex flex-col gap-2">
    <h3 className="text-[12px] font-extrabold font-display uppercase tracking-wider text-muted-ink">
      {title}
    </h3>
    <div
      className="flex flex-col px-4 bg-surface-indigo neo-border neo-shadow-sm divide-y-2 divide-[var(--neo-border-color)]"
      style={{ borderRadius: "8px" }}
    >
      {children}
    </div>
  </section>
);

const SettingRow = ({
  label,
  description,
  children,
}: {
  label: string;
  description?: string;
  children: React.ReactNode;
}) => (
  <div className="flex justify-between items-center gap-4 py-3">
    <div className="flex flex-col gap-0.5 min-w-0">
      <span className="text-[13px] font-semibold text-ink">{label}</span>
      {description && (
        <span className="text-[12px] leading-snug text-muted-ink">
          {description}
        </span>
      )}
    </div>
    {children}
  </div>
);

interface SettingsProps {
  settings: SettingsType | null;
  onSave: (settings: SettingsType) => Promise<void>;
}

/** The "Settings" view: grouped behavior settings with one explicit save. */
export function Settings({ settings, onSave }: SettingsProps) {
  const [draft, setDraft] = useState<SettingsType | null>(settings);
  const [isSaving, setIsSaving] = useState(false);
  const [justSaved, setJustSaved] = useState(false);
  const platformStatus = usePlatformStatus();

  useEffect(() => {
    if (settings && !draft) setDraft(settings);
  }, [settings, draft]);

  if (!settings || !draft) {
    return (
      <div className="text-muted-ink py-16 text-center text-[13px] font-display font-bold uppercase">
        Loading settings…
      </div>
    );
  }

  const set = <K extends keyof SettingsType>(key: K, value: SettingsType[K]) =>
    setDraft({ ...draft, [key]: value });

  const isDirty = EDITABLE_KEYS.some((key) => draft[key] !== settings[key]);

  const handleSave = async () => {
    // Keep whatever the Share switch says right now.
    const next = { ...draft, global_enabled: settings.global_enabled };
    setIsSaving(true);
    try {
      await onSave(next);
      setJustSaved(true);
      setTimeout(() => setJustSaved(false), 1500);
    } catch (e) {
      console.error("Failed to save settings:", e);
    }
    setIsSaving(false);
  };

  let saveLabel = "Save changes";
  if (isSaving) saveLabel = "Saving…";
  else if (justSaved && !isDirty) saveLabel = "Saved";

  return (
    <div className="flex flex-col gap-5">
      {platformStatus?.hint && <DetectionHint hint={platformStatus.hint} />}

      <Group title="Startup">
        <SettingRow
          label="Start on login"
          description="Open Better RPC in the background when you sign in."
        >
          <Toggle
            label="Start on login"
            checked={draft.autostart_enabled}
            onChange={(v) => set("autostart_enabled", v)}
          />
        </SettingRow>
      </Group>

      <Group title="Priority">
        <SettingRow
          label="Max priority mode"
          description="Show the highest running app in your Apps list, even when another app is in focus."
        >
          <Toggle
            label="Max priority mode"
            checked={draft.priority_mode_enabled}
            onChange={(v) => set("priority_mode_enabled", v)}
          />
        </SettingRow>
      </Group>

      <Group title="When you're away">
        <SettingRow
          label="Away status"
          description="Switch to an away message when you stop using the computer."
        >
          <Toggle
            label="Away status"
            checked={draft.idle_enabled}
            onChange={(v) => set("idle_enabled", v)}
          />
        </SettingRow>

        {draft.idle_enabled && (
          <SettingRow label="Minutes before away">
            <NumberInput
              label="Minutes before away"
              value={draft.idle_threshold_minutes}
              onChange={(v) => set("idle_threshold_minutes", v)}
              min={1}
              max={60}
            />
          </SettingRow>
        )}

        {draft.idle_enabled && (
          <label className="flex flex-col gap-1.5 py-3">
            <span className="text-[13px] font-semibold text-ink">
              Away message
            </span>
            <input
              className="neo-input text-[13px] py-1.5"
              style={{ borderRadius: "6px" }}
              type="text"
              value={draft.idle_message}
              onChange={(e) => set("idle_message", e.target.value)}
            />
          </label>
        )}
      </Group>

      <Group title="Timing">
        <SettingRow
          label="Time between updates"
          description="Seconds between Discord updates. Discord accepts one every 15 seconds at most."
        >
          <NumberInput
            label="Time between updates in seconds"
            value={draft.debounce_seconds}
            onChange={(v) => set("debounce_seconds", v)}
            min={15}
            max={60}
          />
        </SettingRow>
        <SettingRow
          label="Delay before switching"
          description="Seconds an app must stay in focus before Discord shows it, so quick switches don't flicker."
        >
          <NumberInput
            label="Delay before switching in seconds"
            value={draft.settle_delay_seconds}
            onChange={(v) => set("settle_delay_seconds", v)}
            min={1}
            max={10}
          />
        </SettingRow>
      </Group>

      <Group title="Activity detection">
        <DetectionDetails status={platformStatus} />
      </Group>

      {/* Save bar stays at the bottom of the view while scrolling */}
      <div
        className="sticky bottom-0 -mx-4 px-4 pt-3 pb-4 bg-canvas"
        style={{ borderTop: "3px solid var(--neo-border-color)" }}
      >
        <button
          className="w-full neo-btn bg-primary text-white text-[13px] py-2.5 disabled:opacity-50 disabled:cursor-not-allowed"
          style={{ borderRadius: "6px" }}
          onClick={handleSave}
          disabled={!isDirty || isSaving}
        >
          {saveLabel}
        </button>
      </div>
    </div>
  );
}
