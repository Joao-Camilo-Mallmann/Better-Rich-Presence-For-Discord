export interface ProcessDefaults {
  details: string;
  state: string;
}

export function getDefaultProcessRules(processName: string, displayName: string): ProcessDefaults {
  const process_lower = processName.toLowerCase();

  let details = displayName;
  let state = "Working";

  if (["game", "play", "cs2", "eldenring", "minecraft", "steam", "gta", "valorant", "counterstrike", "league"].some((kw) => process_lower.includes(kw))) {
    state = "Playing";
  } else if (["chrome", "firefox", "edge", "opera", "brave", "safari", "browser", "explorer", "spotify"].some((kw) => process_lower.includes(kw))) {
    state = "Browsing";
  }

  if (["code", "cursor", "studio", "devenv", "figma"].some((kw) => process_lower.includes(kw))) {
    details = `Editing in ${displayName}`;
    state = "Developing";
  }

  return { details, state };
}
