import { IconifyClient } from "./iconify-client";
import { IconCache } from "./icon-cache";

export class IconService {
  /**
   * Generates the public Iconify design API URL for the SVG icon.
   */
  getIconUrl(icon: string): string {
    return IconifyClient.getIconUrl(icon);
  }

  /**
   * Downloads the raw SVG text content of the icon from Iconify.
   */
  async downloadIcon(icon: string): Promise<string> {
    return await IconifyClient.downloadIcon(icon);
  }

  /**
   * Caches the icon locally in the filesystem and returns the local cache path or fallback URL.
   */
  async cacheIcon(icon: string): Promise<string> {
    try {
      // Normalize icon string for filename (e.g. simple-icons:visualstudiocode -> simple-icons-visualstudiocode)
      const iconId = icon.replace(":", "-");
      
      const cached = await IconCache.isCached(iconId);
      if (cached) {
        const path = await IconCache.getCachePath(iconId);
        if (path) return path;
      }

      // Download raw SVG and cache it
      const svgContent = await this.downloadIcon(icon);
      const success = await IconCache.writeCache(iconId, svgContent);
      if (success) {
        const path = await IconCache.getCachePath(iconId);
        if (path) return path;
      }
    } catch (e) {
      console.warn(`[IconService] Failed to cache icon ${icon}, falling back to remote URL:`, e);
    }

    // Fallback directly to public SVG URL if caching fails (network down, directory issues, etc.)
    return this.getIconUrl(icon);
  }
}

export const iconService = new IconService();
