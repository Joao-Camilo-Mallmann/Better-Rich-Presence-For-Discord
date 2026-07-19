import { detectApplication } from "../apps/app-detector";

/**
 * Shared utility to resolve a process/display name to a favicon URL.
 * Single source of truth — used by AppRuleCard, PresenceCard, and anywhere else needed.
 */
export function getIconUrl(processName: string, displayName?: string): string {
  if (
    displayName &&
    (displayName.startsWith("http://") || displayName.startsWith("https://"))
  ) {
    return displayName;
  }
  if (processName.startsWith("http://") || processName.startsWith("https://")) {
    return processName;
  }

  // Check our App Catalog first to use high-quality Iconify SVGs in the UI
  const app =
    detectApplication(processName) ||
    (displayName ? detectApplication(displayName) : null);
  if (app && app.icon) {
    const parts = app.icon.split(":");
    if (parts.length === 2) {
      return `https://api.iconify.design/${parts[0]}/${parts[1]}.svg`;
    }
  }

  const name = (displayName || processName).toLowerCase();

  if (/idle|afk|using the computer|usando o computador/.test(name)) {
    return "https://api.iconify.design/lucide/monitor.svg";
  }

  const domainMap: Array<[RegExp, string]> = [
    // Dev / Editors
    [/vscode|visual studio code/, "visualstudio.com"],
    [/cursor/, "cursor.com"],
    [/antigravity/, "deepmind.com"],
    [/intellij/, "jetbrains.com"],
    [/android studio/, "developer.android.com"],
    [/visual studio/, "visualstudio.com"],
    [/sublime/, "sublimetext.com"],
    [/pycharm|webstorm|rider|clion|datagrip/, "jetbrains.com"],
    [/postman/, "postman.com"],
    [/insomnia/, "insomnia.rest"],
    [/pgadmin/, "pgadmin.org"],
    [/dbeaver/, "dbeaver.io"],
    // Browsers
    [/chrome/, "google.com"],
    [/firefox/, "mozilla.org"],
    [/edge/, "microsoft.com"],
    [/opera/, "opera.com"],
    [/brave/, "brave.com"],
    [/safari/, "apple.com"],
    // Office / Productivity
    [/notion/, "notion.so"],
    [/obsidian/, "obsidian.md"],
    [/excel|powerpoint|teams|\bword\b/, "microsoft.com"],
    [/trello/, "trello.com"],
    [/asana/, "asana.com"],
    [/jira|confluence/, "atlassian.com"],
    [/slack/, "slack.com"],
    [/discord/, "discord.com"],
    [/telegram/, "telegram.org"],
    [/whatsapp/, "whatsapp.com"],
    [/zoom/, "zoom.us"],
    [/skype/, "skype.com"],
    // Design / Media
    [/figma/, "figma.com"],
    [/photoshop|illustrator|premiere|after effects/, "adobe.com"],
    [/canva/, "canva.com"],
    [/blender/, "blender.org"],
    [/unity/, "unity.com"],
    [/unreal/, "unrealengine.com"],
    [/vlc/, "videolan.org"],
    [/obs/, "obsproject.com"],
  ];

  for (const [pattern, domain] of domainMap) {
    if (pattern.test(name)) {
      return `https://icons.duckduckgo.com/ip3/${domain}.ico`;
    }
  }

  // Fallback: guess domain from first word of process name
  const cleanName = processName
    .replace(/\.exe$/i, "")
    .replace(/[^a-z0-9\s-]/gi, "");
  const firstWord = cleanName.split(/[\s_-]/)[0] || cleanName;
  return `https://icons.duckduckgo.com/ip3/${firstWord}.com.ico`;
}
