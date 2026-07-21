import { APP_CATALOG } from "./app-catalog";
import { AppDefinition } from "./app-types";

export function detectApplication(processName: string): AppDefinition | null {
  if (!processName) return null;

  // Extract file name if processName contains path separators
  const fileName = processName.split(/[\\/]/).pop() || processName;
  const normalized = fileName.toLowerCase().trim();

  // 1. Try matching by executables exactly
  for (const app of APP_CATALOG) {
    if (
      app.executables.some((exe) => exe.toLowerCase().trim() === normalized)
    ) {
      return app;
    }
  }

  // 2. Try matching by ID exactly
  for (const app of APP_CATALOG) {
    if (app.id.toLowerCase() === normalized) {
      return app;
    }
  }

  // 3. Try fuzzy matching by checking if the app name matches or contains the name
  for (const app of APP_CATALOG) {
    const cleanAppName = app.name.toLowerCase().trim();
    if (
      normalized.includes(cleanAppName) ||
      cleanAppName.includes(normalized)
    ) {
      return app;
    }
  }



  return null;
}
