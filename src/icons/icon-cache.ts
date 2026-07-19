import { invoke } from "@tauri-apps/api/core";

export class IconCache {
  static async isCached(iconId: string): Promise<boolean> {
    try {
      return await invoke<boolean>("is_icon_cached", { iconId });
    } catch (e) {
      console.error("[IconCache] Error checking cache status:", e);
      return false;
    }
  }

  static async writeCache(iconId: string, svgContent: string): Promise<boolean> {
    try {
      await invoke("write_cached_icon", { iconId, svgContent });
      return true;
    } catch (e) {
      console.error("[IconCache] Error writing cache:", e);
      return false;
    }
  }

  static async getCachePath(iconId: string): Promise<string | null> {
    try {
      return await invoke<string | null>("get_cached_icon_path", { iconId });
    } catch (e) {
      console.error("[IconCache] Error getting cache path:", e);
      return null;
    }
  }
}
