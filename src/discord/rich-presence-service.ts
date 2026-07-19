import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import { detectApplication } from "../apps/app-detector";
import { iconService } from "../icons/icon-service";
import { PresenceData } from "../types";

export interface ActiveProcessEvent {
  process_name: string;
  window_title: string;
}

class RichPresenceService {
  private unlisten: UnlistenFn | null = null;

  async startListening() {
    if (this.unlisten) return;

    this.unlisten = await listen<ActiveProcessEvent>("active-process-changed", async (event) => {
      try {
        await this.handleProcessChange(event.payload);
      } catch (e) {
        console.error("[RichPresenceService] Error handling process change:", e);
      }
    });
    console.log("[RichPresenceService] Started listening for process changes.");
  }

  stopListening() {
    if (this.unlisten) {
      this.unlisten();
      this.unlisten = null;
      console.log("[RichPresenceService] Stopped listening for process changes.");
    }
  }

  private async handleProcessChange(payload: ActiveProcessEvent) {
    const { process_name, window_title } = payload;
    if (!process_name) {
      await invoke("clear_rich_presence");
      return;
    }

    // 1. Detect application using app-detector.ts
    const app = detectApplication(process_name);

    // 2. Resolve client ID using Rust process_db
    const clientId = await invoke<number>("get_client_id_for_process", { processName: process_name });

    // 3. Resolve Presence Data
    let details = "";
    let state = "";
    let largeImage = "";
    let largeText = "";

    // Load active rules from the store to see if the user customized it
    const rules = await invoke<any[]>("get_app_rules");
    const matchedRule = rules.find((r) => {
      if (!r.enabled) return false;
      const ruleProc = r.process_name.replace(/\.exe$/i, "").toLowerCase();
      const proc = process_name.replace(/\.exe$/i, "").toLowerCase();
      return ruleProc === proc;
    });

    if (matchedRule) {
      const cleanFileName = parseFileName(window_title, process_name);
      details = matchedRule.details
        .replace(/{file}/g, cleanFileName)
        .replace(/{title}/g, window_title);
      state = matchedRule.state
        .replace(/{file}/g, cleanFileName)
        .replace(/{title}/g, window_title);

      largeImage = matchedRule.large_image;
      largeText = matchedRule.display_name;
    } else if (app) {
      details = `Using ${app.name}`;
      state = window_title || "Active";
      largeImage = "auto";
      largeText = app.name;
    } else {
      const prettyName = process_name.replace(/\.exe$/i, "");
      details = `Using ${prettyName}`;
      state = window_title || "Active";
      largeImage = "default";
      largeText = prettyName;
    }

    // 4. Handle Iconify caching/resolution and PNG proxying for Discord Rich Presence
    const DEFAULT_CLIENT_ID = 1517170930764480552;
    
    if (largeImage === "auto" || !largeImage) {
      if (clientId === DEFAULT_CLIENT_ID && app && app.icon) {
        // Caches locally in background
        await iconService.cacheIcon(app.icon);
        // Use images.weserv.nl PNG proxy for Discord since it doesn't support SVGs
        const svgUrl = iconService.getIconUrl(app.icon);
        largeImage = `https://images.weserv.nl/?url=${encodeURIComponent(svgUrl)}&output=png&w=512&h=512`;
      } else if (app) {
        largeImage = app.discordAsset;
      } else {
        largeImage = "default";
      }
    } else if (largeImage.includes(":")) {
      // Custom Iconify icon from rule (e.g. "simple-icons:docker")
      await iconService.cacheIcon(largeImage);
      const svgUrl = iconService.getIconUrl(largeImage);
      largeImage = `https://images.weserv.nl/?url=${encodeURIComponent(svgUrl)}&output=png&w=512&h=512`;
    } else if (clientId === DEFAULT_CLIENT_ID && largeImage !== "default" && largeImage !== "idle" && !largeImage.startsWith("http")) {
      // Custom asset key (like "vscode") but on fallback Client ID -> translate to Iconify PNG proxy if catalog matches
      if (app && app.icon) {
        await iconService.cacheIcon(app.icon);
        const svgUrl = iconService.getIconUrl(app.icon);
        largeImage = `https://images.weserv.nl/?url=${encodeURIComponent(svgUrl)}&output=png&w=512&h=512`;
      } else {
        const domain = getFallbackDomain(process_name, window_title);
        largeImage = `https://icons.duckduckgo.com/ip3/${domain}.ico`;
      }
    }

    const presenceData: PresenceData = {
      details,
      state,
      large_image: largeImage,
      large_text: largeText,
      timestamp: null,
    };

    // 5. Submit resolved presence to Rust presence engine
    await invoke("submit_resolved_presence", { clientId, data: presenceData });
  }
}

function parseFileName(windowTitle: string, processName: string): string {
  const parts = windowTitle.split(" - ");
  if (parts.length >= 2) {
    return parts[0].trim().replace(/^[●*]\s*/, "");
  }
  return windowTitle || "No file open";
}

function getFallbackDomain(processName: string, windowTitle: string): string {
  const nameToSearch = (windowTitle || processName).toLowerCase();
  if (nameToSearch.includes("vscode") || nameToSearch.includes("visual studio code")) return "visualstudio.com";
  if (nameToSearch.includes("chrome")) return "google.com";
  if (nameToSearch.includes("spotify")) return "spotify.com";
  
  const clean = processName.replace(/\.exe$/i, "").replace(/[^a-z0-9\s-]/gi, "");
  const firstWord = clean.split(/[\s_-]/)[0] || clean;
  return `${firstWord}.com`;
}

export const richPresenceService = new RichPresenceService();
