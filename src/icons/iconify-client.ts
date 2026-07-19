export class IconifyClient {
  static getIconUrl(icon: string): string {
    const parts = icon.split(":");
    if (parts.length !== 2) {
      throw new Error(`Invalid icon format: ${icon}. Expected 'collection:name'.`);
    }
    const [collection, name] = parts;
    return `https://api.iconify.design/${collection}/${name}.svg`;
  }

  static async downloadIcon(icon: string): Promise<string> {
    const url = this.getIconUrl(icon);
    
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), 8000); // 8 seconds timeout

    try {
      const response = await fetch(url, { signal: controller.signal });
      clearTimeout(timeoutId);

      if (!response.ok) {
        throw new Error(`Iconify HTTP Error: ${response.status} ${response.statusText}`);
      }

      return await response.text();
    } catch (e) {
      clearTimeout(timeoutId);
      throw e;
    }
  }
}
