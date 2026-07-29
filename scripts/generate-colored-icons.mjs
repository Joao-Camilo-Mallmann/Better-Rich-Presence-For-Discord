import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import sharp from 'sharp';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const rootDir = path.resolve(__dirname, '..');

const APPS_JSON_PATH = path.join(rootDir, 'apps.json');
const ICONS_DIR = path.join(rootDir, 'public', 'assets', 'icons');

const SIMPLE_ICONS_CDN = 'https://cdn.simpleicons.org';
const ICONIFY_API = 'https://api.iconify.design';

// ── Slug extraction ──────────────────────────────────────────────────
// Parses the `icon` field from apps.json entries.
// Supports "simple-icons:{slug}" and "{prefix}:{name}" patterns.

/**
 * Extracts the prefix and name from an icon identifier string.
 * @param {string} iconField - e.g. "simple-icons:discord" or "material-symbols:antigravity"
 * @returns {{ prefix: string, name: string } | null}
 */
function parseIconField(iconField) {
  if (!iconField || !iconField.includes(':')) return null;
  const colonIndex = iconField.indexOf(':');
  const prefix = iconField.substring(0, colonIndex);
  const name = iconField.substring(colonIndex + 1);
  if (!prefix || !name) return null;
  return { prefix, name };
}

// ── SVG fetchers ─────────────────────────────────────────────────────

/**
 * Fetches a colored SVG from the Simple Icons CDN.
 * URL format: https://cdn.simpleicons.org/{slug}
 * Returns the SVG with the official brand color applied as fill.
 */
async function fetchFromSimpleIconsCDN(slug) {
  const url = `${SIMPLE_ICONS_CDN}/${slug}`;
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`HTTP ${response.status} ${response.statusText}`);
  }
  const text = await response.text();
  if (!text.includes('<svg')) {
    throw new Error('Response does not appear to be SVG');
  }
  return text;
}

/**
 * Fetches a colored SVG from the Iconify API.
 * URL format: https://api.iconify.design/{prefix}/{name}.svg
 * Used as fallback for non-simple-icons entries (e.g. material-symbols:*).
 */
async function fetchFromIconifyAPI(prefix, name) {
  const url = `${ICONIFY_API}/${prefix}/${name}.svg`;
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`HTTP ${response.status} ${response.statusText}`);
  }
  const text = await response.text();
  if (!text.includes('<svg')) {
    throw new Error('Response does not appear to be SVG');
  }
  return text;
}

// ── SVG → PNG conversion ────────────────────────────────────────────

/**
 * Converts an SVG string to a 1024×1024 PNG buffer using sharp.
 * Uses `fit: contain` with a transparent background.
 */
async function svgToPng(svgString) {
  return sharp(Buffer.from(svgString))
    .resize(1024, 1024, {
      fit: 'contain',
      background: { r: 0, g: 0, b: 0, alpha: 0 },
    })
    .png()
    .toBuffer();
}

// ── Main pipeline ────────────────────────────────────────────────────

async function generateColoredIcons() {
  console.log('🎨 Starting colored icon generation...\n');

  await fs.mkdir(ICONS_DIR, { recursive: true });

  const content = await fs.readFile(APPS_JSON_PATH, 'utf-8');
  const apps = JSON.parse(content);

  // Deduplicate by icon field — multiple apps can share the same icon
  // (e.g. vscode and vscode_insiders both use simple-icons:visualstudiocode)
  const uniqueIcons = new Map();
  for (const app of apps) {
    if (!app.icon) continue;
    const parsed = parseIconField(app.icon);
    if (!parsed) continue;
    if (!uniqueIcons.has(app.icon)) {
      uniqueIcons.set(app.icon, parsed);
    }
  }

  let successCount = 0;
  let failedCount = 0;
  let skippedCount = 0;
  const total = uniqueIcons.size;
  let current = 0;

  for (const [iconField, { prefix, name }] of uniqueIcons) {
    current++;
    const slug = name;
    const outputPath = path.join(ICONS_DIR, `${slug}.png`);

    try {
      let svgContent;

      if (prefix === 'simple-icons') {
        // Primary source: Simple Icons CDN (returns SVG with brand color fill)
        console.log(
          `[${current}/${total}] Fetching: ${iconField} → cdn.simpleicons.org/${slug}`
        );
        try {
          svgContent = await fetchFromSimpleIconsCDN(slug);
        } catch {
          // Fallback: Iconify mirrors simple-icons with broader coverage
          console.log(
            `  ↳ CDN miss, trying Iconify fallback → api.iconify.design/simple-icons/${slug}.svg`
          );
          svgContent = await fetchFromIconifyAPI('simple-icons', name);
        }
      } else {
        // Non-simple-icons entries: Iconify API directly
        console.log(
          `[${current}/${total}] Fetching: ${iconField} → api.iconify.design/${prefix}/${name}.svg`
        );
        svgContent = await fetchFromIconifyAPI(prefix, name);
      }

      // Convert SVG → PNG (1024×1024, transparent background)
      const pngBuffer = await svgToPng(svgContent);
      await fs.writeFile(outputPath, pngBuffer);
      console.log(`  ✓ Saved: ${slug}.png`);
      successCount++;
    } catch (err) {
      // Log warning and skip — existing PNG is preserved (only overwritten on success)
      console.warn(`  ⚠ Failed: ${iconField} — ${err.message}`);
      failedCount++;
    }
  }

  // ── Summary report ───────────────────────────────────────────────
  console.log('\n' + '═'.repeat(50));
  console.log('📊 Icon Generation Summary');
  console.log('═'.repeat(50));
  console.log(`  Total unique icons:  ${total}`);
  console.log(`  ✓ Successful:        ${successCount}`);
  console.log(`  ✗ Failed:            ${failedCount}`);
  console.log(`  ⊘ Skipped:           ${skippedCount}`);
  console.log('═'.repeat(50));

  if (failedCount > 0) {
    console.log(
      `\n⚠ ${failedCount} icon(s) failed. Existing PNGs were preserved.`
    );
  }

  console.log('\nDone! 🎉');
}

generateColoredIcons().catch((err) => {
  console.error('Fatal error:', err);
  process.exit(1);
});
