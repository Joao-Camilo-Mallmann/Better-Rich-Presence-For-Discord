import { useEffect, useId, useRef, useState } from "react";

interface SegmentedOption<T extends string> {
  value: T;
  label: string;
  /** One friendly sentence shown in a tooltip while the option is hovered or focused. */
  hint?: string;
}

interface SegmentedControlProps<T extends string> {
  /** Accessible name of the group, e.g. "Presence mode". */
  label: string;
  options: SegmentedOption<T>[];
  value: T;
  onChange: (value: T) => void;
}

/** One-of-many choice shown as joined buttons. A radio group: arrow keys move the selection. */
export function SegmentedControl<T extends string>({
  label,
  options,
  value,
  onChange,
}: SegmentedControlProps<T>) {
  const buttons = useRef<(HTMLButtonElement | null)[]>([]);
  const hintId = useId();
  // Index of the option whose hint is showing: the hovered one, else the focused one.
  const [hovered, setHovered] = useState<number | null>(null);
  const [focused, setFocused] = useState<number | null>(null);
  const [dismissed, setDismissed] = useState(false);

  const shownIndex = hovered ?? focused;
  let shownHint: string | undefined;
  if (!dismissed && shownIndex !== null) shownHint = options[shownIndex]?.hint;

  // Escape hides the tooltip even when it only shows because of the pointer.
  const hintShown = shownHint !== undefined;
  useEffect(() => {
    if (!hintShown) return;
    const hideOnEscape = (event: KeyboardEvent) => {
      if (event.key === "Escape") setDismissed(true);
    };
    window.addEventListener("keydown", hideOnEscape);
    return () => window.removeEventListener("keydown", hideOnEscape);
  }, [hintShown]);

  const selectAt = (index: number) => {
    const next = (index + options.length) % options.length;
    onChange(options[next].value);
    buttons.current[next]?.focus();
  };

  const handleKeyDown = (event: React.KeyboardEvent, index: number) => {
    if (event.key === "ArrowRight" || event.key === "ArrowDown") {
      event.preventDefault();
      selectAt(index + 1);
    } else if (event.key === "ArrowLeft" || event.key === "ArrowUp") {
      event.preventDefault();
      selectAt(index - 1);
    }
  };

  return (
    <div className="relative flex shrink-0">
      <div
        role="radiogroup"
        aria-label={label}
        className="flex neo-border overflow-hidden"
        style={{ borderRadius: "6px" }}
      >
        {options.map((option, index) => {
          const selected = option.value === value;
          let className =
            "px-3 py-1.5 text-[12px] font-display font-bold uppercase whitespace-nowrap cursor-pointer transition-colors focus-visible:outline-offset-[-3px] focus-visible:outline-white";
          if (index > 0) className += " border-l-[3px] border-[var(--neo-border-color)]";
          className += selected
            ? " bg-primary text-white"
            : " bg-surface-onyx text-muted-ink hover:text-ink";
          return (
            <button
              key={option.value}
              ref={(element) => {
                buttons.current[index] = element;
              }}
              type="button"
              role="radio"
              aria-checked={selected}
              aria-describedby={option.hint ? `${hintId}-${index}` : undefined}
              tabIndex={selected ? 0 : -1}
              onClick={() => onChange(option.value)}
              onKeyDown={(event) => handleKeyDown(event, index)}
              onMouseEnter={() => {
                setHovered(index);
                setDismissed(false);
              }}
              onMouseLeave={() => setHovered(null)}
              onFocus={(event) => {
                // Keyboard focus only: a click also focuses, and would leave the hint stuck open.
                if (!event.currentTarget.matches(":focus-visible")) return;
                setFocused(index);
                setDismissed(false);
              }}
              onBlur={() => setFocused(null)}
              className={className}
            >
              {option.label}
            </button>
          );
        })}
      </div>

      {/* Hint text for assistive technology, always present so aria-describedby resolves. */}
      {options.map(
        (option, index) =>
          option.hint && (
            <span key={option.value} id={`${hintId}-${index}`} className="sr-only">
              {option.hint}
            </span>
          ),
      )}

      {shownHint && (
        <div
          aria-hidden="true"
          className="absolute left-0 top-full z-20 mt-2 w-max max-w-[260px] px-3 py-2 bg-surface-indigo text-ink text-[12px] leading-snug neo-border-2 neo-shadow-sm pointer-events-none"
          style={{ borderRadius: "6px" }}
        >
          {shownHint}
        </div>
      )}
    </div>
  );
}
