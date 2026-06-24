/**
 * Shared utility to resolve a process/display name to a favicon URL.
 * Single source of truth — used by AppRuleCard, PresenceCard, and anywhere else needed.
 */
export function getIconUrl(processName: string, displayName?: string): string {
  const name = (displayName || processName).toLowerCase();

  const domainMap: Array<[RegExp | string, string]> = [
    // Dev / Editors
    [/vscode|visual studio code/, "visualstudio.com"],
    [/cursor/, "cursor.com"],
    [/antigravity/, "deepmind.com"],
    [/intellij/, "jetbrains.com"],
    [/android studio/, "developer.android.com"],
    [/visual studio/, "visualstudio.com"],
    [/sublime/, "sublimetext.com"],
    [/webstorm|rider|clion|datagrip/, "jetbrains.com"],
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
    [/excel/, "microsoft.com"],
    [/\bword\b/, "microsoft.com"],
    [/powerpoint/, "microsoft.com"],
    [/teams/, "microsoft.com"],
    [/trello/, "trello.com"],
    [/asana/, "asana.com"],
    [/jira/, "atlassian.com"],
    [/confluence/, "atlassian.com"],
    [/slack/, "slack.com"],
    [/discord/, "discord.com"],
    [/telegram/, "telegram.org"],
    [/whatsapp/, "whatsapp.com"],
    [/zoom/, "zoom.us"],
    [/skype/, "skype.com"],
    // Design / Media
    [/figma/, "figma.com"],
    [/photoshop/, "adobe.com"],
    [/illustrator/, "adobe.com"],
    [/premiere/, "adobe.com"],
    [/after effects/, "adobe.com"],
    [/canva/, "canva.com"],
    [/blender/, "blender.org"],
    [/unity/, "unity.com"],
    [/unreal/, "unrealengine.com"],
    [/vlc/, "videolan.org"],
    [/obs/, "obsproject.com"],
    // Entertainment / Gaming
    [/spotify/, "spotify.com"],
    [/steam/, "steampowered.com"],
    [/github/, "github.com"],
    [/docker/, "docker.com"],
    [/netflix/, "netflix.com"],
    [/youtube/, "youtube.com"],
    [/twitch/, "twitch.tv"],
    [/minecraft/, "minecraft.net"],
    [/roblox/, "roblox.com"],
    [/league of legends/, "leagueoflegends.com"],
    [/valorant/, "playvalorant.com"],
    [/counter-strike|cs2|csgo/, "counter-strike.net"],
    [/terminal|powershell|cmd/, "microsoft.com"],
  ];

  for (const [pattern, domain] of domainMap) {
    if (typeof pattern === "string" ? name.includes(pattern) : pattern.test(name)) {
      return `https://www.google.com/s2/favicons?sz=128&domain=${domain}`;
    }
  }

  // Fallback: guess domain from first word of process name
  const cleanName = processName.replace(/\.exe$/i, "").replace(/[^a-z0-9\s-]/gi, "");
  const firstWord = cleanName.split(/[\s_-]/)[0] || cleanName;
  return `https://www.google.com/s2/favicons?sz=128&domain=${firstWord}.com`;
}
