interface ToggleProps {
  checked: boolean;
  onChange: (checked: boolean) => void;
  /** Accessible name, e.g. "Start on login". */
  label: string;
  disabled?: boolean;
}

/** Arcade-style on/off switch. A real button, so it takes focus and toggles with Space or Enter. */
export function Toggle({ checked, onChange, label, disabled }: ToggleProps) {
  return (
    <button
      type="button"
      role="switch"
      aria-checked={checked}
      aria-label={label}
      title={label}
      disabled={disabled}
      onClick={() => onChange(!checked)}
      className={`neo-toggle disabled:opacity-40 disabled:cursor-not-allowed ${checked ? "active" : ""}`}
    >
      <span className="neo-toggle-knob" />
    </button>
  );
}
