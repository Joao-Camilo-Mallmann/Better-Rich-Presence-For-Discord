export interface ProcessDefaults {
  details: string;
  state: string;
}

export function getDefaultProcessRules(_processName: string, displayName: string): ProcessDefaults {
  return {
    details: displayName,
    state: "Working"
  };
}
