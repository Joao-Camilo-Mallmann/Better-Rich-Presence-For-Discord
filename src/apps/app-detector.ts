import { AppDefinition } from "./app-types";
import { APP_CATALOG } from "./app-catalog";

export function detectApplication(processName: string): AppDefinition | null {
  if (!processName) return null;
  
  // Extract file name if processName contains path separators
  const fileName = processName.split(/[\\/]/).pop() || processName;
  const normalized = fileName.toLowerCase().trim();

  // 1. Try matching by executables exactly
  for (const app of APP_CATALOG) {
    if (app.executables.some((exe) => exe.toLowerCase().trim() === normalized)) {
      return app;
    }
  }

  // 2. Try matching by ID or discordAsset exactly
  for (const app of APP_CATALOG) {
    if (app.id.toLowerCase() === normalized || app.discordAsset.toLowerCase() === normalized) {
      return app;
    }
  }

  // 3. Try fuzzy matching by checking if the app name matches or contains the name
  for (const app of APP_CATALOG) {
    const cleanAppName = app.name.toLowerCase().trim();
    if (normalized.includes(cleanAppName) || cleanAppName.includes(normalized)) {
      return app;
    }
  }

  // 4. Special manual aliases for editors
  if (normalized.includes("vscode") || normalized.includes("visual studio code") || normalized === "code") {
    return APP_CATALOG.find(app => app.id === "visual-studio-code") || null;
  }
  if (normalized.includes("cursor")) {
    return APP_CATALOG.find(app => app.id === "cursor") || null;
  }
  if (normalized.includes("antigravity")) {
    return APP_CATALOG.find(app => app.id === "antigravity-ide") || null;
  }
  if (normalized.includes("intellij") || normalized.includes("idea") || normalized === "idea64") {
    return APP_CATALOG.find(app => app.id === "intellij-idea") || null;
  }

  return null;
}
