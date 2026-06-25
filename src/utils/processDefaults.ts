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

  const gameKeywords = [
    "game", "play", "cs2", "eldenring", "minecraft", "steam", "gta", "valorant", "counterstrike", "league"
  ];
  const browserKeywords = [
    "chrome", "firefox", "edge", "opera", "brave", "safari", "browser", "explorer", "spotify"
  ];

  if (gameKeywords.some((kw) => process_lower.includes(kw))) {
    source = "Game";
    details = displayName;
    state = "In a match";
    priority = 0;
  } else if (browserKeywords.some((kw) => process_lower.includes(kw))) {
    source = "Browser";
    details = displayName;
    state = "Browsing";
    priority = 3;
  }

  if (
    process_lower.includes("code") ||
    process_lower.includes("cursor") ||
    process_lower.includes("studio") ||
    process_lower.includes("devenv") ||
    process_lower.includes("figma")
  ) {
    details = `Editing in ${displayName}`;
    state = "Developing";
  }

  return { source, details, state, priority };
}
