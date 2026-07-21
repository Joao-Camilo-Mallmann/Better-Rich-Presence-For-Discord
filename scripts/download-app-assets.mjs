import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, '..');

const APPS_JSON_PATH = path.join(rootDir, 'apps.json');
const ICONS_DIR = path.join(rootDir, 'public', 'assets', 'icons');
const RAW_BASE_URL = 'https://raw.githubusercontent.com/Joao-Camilo-Mallmann/Better-Rich-Presence-For-Discord/main/public/assets/icons/';

async function downloadAppAssets() {
  console.log('Starting app icon localization...');

  try {
    await fs.mkdir(ICONS_DIR, { recursive: true });
    const content = await fs.readFile(APPS_JSON_PATH, 'utf-8');
    const apps = JSON.parse(content);

    let downloadedCount = 0;
    let skippedCount = 0;
    let failedCount = 0;
    let updatedCount = 0;

    const urlToFileMap = new Map();

    for (let i = 0; i < apps.length; i++) {
      const app = apps[i];
      if (!app.icon_url) continue;

      if (app.icon_url.startsWith(RAW_BASE_URL) || app.icon_url.startsWith('/assets/icons/')) {
        skippedCount++;
        continue;
      }

      if (app.icon_url.startsWith('http://') || app.icon_url.startsWith('https://')) {
        const remoteUrl = app.icon_url;

        let ext = '.svg';
        try {
          const parsedUrl = new URL(remoteUrl);
          const pathname = parsedUrl.pathname;
          const extractedExt = path.extname(pathname);
          if (extractedExt && extractedExt.length <= 5) {
            ext = extractedExt;
          }
        } catch {
          // fallback to .svg
        }

        let fileName;
        if (app.icon) {
          fileName = `${app.icon.replace(/[^a-zA-Z0-9_-]/g, '-')}${ext}`;
        } else if (app.id) {
          fileName = `${app.id}${ext}`;
        } else {
          fileName = `icon-${i}${ext}`;
        }

        const localPath = path.join(ICONS_DIR, fileName);
        const resolvedRawUrl = `${RAW_BASE_URL}${fileName}`;

        if (!urlToFileMap.has(remoteUrl)) {
          try {
            console.log(`Downloading (${i + 1}/${apps.length}): ${remoteUrl} -> ${fileName}`);
            const response = await fetch(remoteUrl);
            if (!response.ok) {
              throw new Error(`HTTP ${response.status} ${response.statusText}`);
            }
            const buffer = Buffer.from(await response.arrayBuffer());
            await fs.writeFile(localPath, buffer);
            urlToFileMap.set(remoteUrl, resolvedRawUrl);
            downloadedCount++;
          } catch (err) {
            console.warn(`[WARN] Failed to download icon for app "${app.id || app.name}" (${remoteUrl}): ${err.message}`);
            failedCount++;
            continue;
          }
        }

        const finalUrl = urlToFileMap.get(remoteUrl);
        if (finalUrl) {
          app.icon_url = finalUrl;
          updatedCount++;
        }
      }
    }

    await fs.writeFile(APPS_JSON_PATH, JSON.stringify(apps, null, 2) + '\n', 'utf-8');

    console.log('\n--- Asset Download Summary ---');
    console.log(`Total apps processed: ${apps.length}`);
    console.log(`Icons downloaded: ${downloadedCount}`);
    console.log(`Icons skipped (already pointing to repository raw base): ${skippedCount}`);
    console.log(`Failed downloads: ${failedCount}`);
    console.log(`apps.json entries updated: ${updatedCount}`);
    console.log('Done!');
  } catch (err) {
    console.error('Fatal error in downloadAppAssets:', err);
    process.exit(1);
  }
}

downloadAppAssets();
