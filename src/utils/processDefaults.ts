import { PresenceSource } from "../types";

export interface ProcessDefaults {
  source: PresenceSource;
  details: string;
  state: string;
  priority: number;
}

export function getDefaultProcessRules(processName: string, displayName: string): ProcessDefaults {
  const process_lower = processName.toLowerCase();

  let source: PresenceSource = "Work";
  let details = displayName;
  let state = "Working";
  let priority = 2;

  if (["game", "play", "cs2", "eldenring", "minecraft", "steam", "gta", "valorant", "counterstrike", "league"].some((kw) => process_lower.includes(kw))) {
    source = "Game";
    state = "In a match";
    priority = 0;
  } else if (["chrome", "firefox", "edge", "opera", "brave", "safari", "browser", "explorer", "spotify"].some((kw) => process_lower.includes(kw))) {
    source = "Browser";
    state = "Browsing";
    priority = 3;
  }

  if (["code", "cursor", "studio", "devenv", "figma"].some((kw) => process_lower.includes(kw))) {
    details = `Editing in ${displayName}`;
    state = "Developing";
  }

  return { source, details, state, priority };
}
