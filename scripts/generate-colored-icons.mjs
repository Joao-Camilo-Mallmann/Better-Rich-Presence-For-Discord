// Regenerates public/assets/icons/*.png from theSVG (https://thesvg.org).
//
// The icons to build come from scripts/icon-sources.json:
//   - "thesvg.commit" pins the glincker/thesvg commit, so output is reproducible.
//   - "icons" maps a PNG name to a theSVG slug, or to { "slug", "variant" }.
//   - "keep" lists PNGs theSVG has no logo for; they are left untouched.
//
// Usage:
//   npm run assets:generate                 # all icons
//   npm run assets:generate -- --only a,b   # only the named icons

import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import sharp from 'sharp';

const rootDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const SOURCES_PATH = path.join(rootDir, 'scripts', 'icon-sources.json');
const APPS_JSON_PATH = path.join(rootDir, 'apps.json');
const ICONS_DIR = path.join(rootDir, 'public', 'assets', 'icons');

const SIZE = 256;
const MARGIN = 8;
// SVGs are rasterized with their longest side at this size before trimming.
const RASTER_SIZE = 1024;
// A pixel is "dark neutral" (hard to see on a dark background) when its linear
// luminance is below DARK_LUMINANCE and its chroma is below DARK_CHROMA, so
// saturated brand colors (purple, red, navy) do not count. A logo is near-black
// when at least NEAR_BLACK_SHARE of its opaque area is dark neutral.
const DARK_LUMINANCE = 0.08;
const DARK_CHROMA = 0.25;
const NEAR_BLACK_SHARE = 0.6;
// Special variant: the `mono` SVG with every opaque pixel painted white.
const MONO_WHITE = 'mono-white';
const CONCURRENCY = 8;
const ALLOWED_COLLECTIONS = new Set(['brands', 'community']);

// ── Inputs ───────────────────────────────────────────────────────────

function parseOnlyFlag(argv) {
  const index = argv.indexOf('--only');
  if (index === -1) return null;
  return new Set((argv[index + 1] ?? '').split(',').map((s) => s.trim()).filter(Boolean));
}

/** Normalizes an `icons` entry to { slug, variant }. */
function toEntry(value) {
  return typeof value === 'string' ? { slug: value, variant: null } : { slug: value.slug, variant: value.variant ?? null };
}

/** PNG names referenced by `icon_url` in apps.json. */
function catalogIconNames(apps) {
  const names = new Set();
  for (const app of apps) {
    const match = app.icon_url?.match(/\/assets\/icons\/([^/]+)\.png$/);
    if (match) names.add(match[1]);
  }
  return names;
}

// ── theSVG access ────────────────────────────────────────────────────

async function fetchOk(url) {
  const response = await fetch(url);
  if (!response.ok) throw new Error(`HTTP ${response.status} for ${url}`);
  return response;
}

function makeTheSvgClient({ repo, commit }) {
  const base = `https://cdn.jsdelivr.net/gh/${repo}@${commit}`;
  return {
    async manifest() {
      const icons = await (await fetchOk(`${base}/src/data/icons.json`)).json();
      return new Map(icons.map((icon) => [icon.slug, icon]));
    },
    async svg(variantPath) {
      const text = await (await fetchOk(`${base}/public${variantPath}`)).text();
      if (!text.includes('<svg')) throw new Error(`not an SVG: ${variantPath}`);
      return Buffer.from(text);
    },
  };
}

// ── Rendering ────────────────────────────────────────────────────────

/** Rasterizes an SVG, trims transparent borders and centers it on a SIZE×SIZE canvas. */
async function renderPng(svg) {
  const inner = SIZE - MARGIN * 2;
  // Pick the density from the SVG's own canvas so tiny and huge viewBoxes both
  // rasterize at about RASTER_SIZE pixels (a fixed density overflows sharp's
  // pixel limit on large canvases).
  const { width = 24, height = 24 } = await sharp(svg).metadata();
  const density = Math.min(10000, Math.max(1, (72 * RASTER_SIZE) / Math.max(width, height)));
  const raster = await sharp(svg, { density }).png().toBuffer();
  let trimmed;
  try {
    trimmed = await sharp(raster).trim().toBuffer();
  } catch {
    trimmed = raster; // Fully transparent or uniform image: nothing to trim.
  }
  const logo = await sharp(trimmed)
    .resize(inner, inner, { fit: 'contain', background: { r: 0, g: 0, b: 0, alpha: 0 } })
    .toBuffer();
  return sharp({ create: { width: SIZE, height: SIZE, channels: 4, background: { r: 0, g: 0, b: 0, alpha: 0 } } })
    .composite([{ input: logo, left: MARGIN, top: MARGIN }])
    .png({ compressionLevel: 9, adaptiveFiltering: true })
    .toBuffer();
}

const toLinear = (channel) => {
  const c = channel / 255;
  return c <= 0.04045 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4;
};

/** Alpha-weighted share (0..1) of a PNG's opaque area that is dark neutral. */
async function darkNeutralShare(png) {
  const { data } = await sharp(png).ensureAlpha().raw().toBuffer({ resolveWithObject: true });
  let dark = 0;
  let alphaSum = 0;
  for (let i = 0; i < data.length; i += 4) {
    const alpha = data[i + 3] / 255;
    if (alpha === 0) continue;
    const [r, g, b] = [data[i], data[i + 1], data[i + 2]];
    const luminance = 0.2126 * toLinear(r) + 0.7152 * toLinear(g) + 0.0722 * toLinear(b);
    const chroma = (Math.max(r, g, b) - Math.min(r, g, b)) / 255;
    if (luminance < DARK_LUMINANCE && chroma < DARK_CHROMA) dark += alpha;
    alphaSum += alpha;
  }
  return alphaSum === 0 ? 0 : dark / alphaSum;
}

/** Paints every pixel white, keeping the alpha channel. */
async function paintWhite(png) {
  const { data, info } = await sharp(png).ensureAlpha().raw().toBuffer({ resolveWithObject: true });
  for (let i = 0; i < data.length; i += 4) {
    data[i] = 255;
    data[i + 1] = 255;
    data[i + 2] = 255;
  }
  return sharp(data, { raw: info }).png({ compressionLevel: 9, adaptiveFiltering: true }).toBuffer();
}

/**
 * Builds the PNG for one icon. Without a pinned variant, a near-black default
 * falls back to `dark`, then to white `mono`, so it reads on dark backgrounds.
 * Returns { png, variant }.
 */
async function buildIcon(client, icon, pinnedVariant) {
  const variants = icon.variants ?? {};
  const render = async (variant) => {
    if (variant === MONO_WHITE) {
      if (!variants.mono) throw new Error(`no mono variant for ${icon.slug}`);
      return paintWhite(await renderPng(await client.svg(variants.mono)));
    }
    if (!variants[variant]) throw new Error(`no "${variant}" variant for ${icon.slug}`);
    return renderPng(await client.svg(variants[variant]));
  };

  if (pinnedVariant) return { png: await render(pinnedVariant), variant: pinnedVariant };

  const png = await render('default');
  if ((await darkNeutralShare(png)) < NEAR_BLACK_SHARE) return { png, variant: 'default' };
  if (variants.dark) return { png: await render('dark'), variant: 'dark' };
  if (variants.mono) return { png: await render(MONO_WHITE), variant: MONO_WHITE };
  return { png, variant: 'default (near-black, no light alternative)' };
}

// ── Main ─────────────────────────────────────────────────────────────

async function runPool(items, worker) {
  let next = 0;
  const runners = Array.from({ length: Math.min(CONCURRENCY, items.length) }, async () => {
    while (next < items.length) {
      const item = items[next++];
      await worker(item);
    }
  });
  await Promise.all(runners);
}

async function main() {
  const sources = JSON.parse(await fs.readFile(SOURCES_PATH, 'utf-8'));
  const apps = JSON.parse(await fs.readFile(APPS_JSON_PATH, 'utf-8'));
  const only = parseOnlyFlag(process.argv.slice(2));
  const keep = new Set(sources.keep ?? []);
  const entries = Object.entries(sources.icons).map(([name, value]) => ({ name, ...toEntry(value) }));

  const mapped = new Set(entries.map((entry) => entry.name));
  const unmapped = [...catalogIconNames(apps)].filter((name) => !mapped.has(name) && !keep.has(name)).sort();

  const client = makeTheSvgClient(sources.thesvg);
  console.log(`Loading theSVG manifest (${sources.thesvg.repo}@${sources.thesvg.commit.slice(0, 7)})...`);
  const manifest = await client.manifest();
  await fs.mkdir(ICONS_DIR, { recursive: true });

  const todo = only ? entries.filter((entry) => only.has(entry.name)) : entries;
  const failed = [];
  const licenses = new Map();
  let generated = 0;

  await runPool(todo, async ({ name, slug, variant }) => {
    const icon = manifest.get(slug);
    if (!icon || !ALLOWED_COLLECTIONS.has(icon.collection)) {
      console.warn(`  ⚠ ${name}: theSVG slug "${slug}" not found in brands/community`);
      failed.push(name);
      return;
    }
    try {
      const result = await buildIcon(client, icon, variant);
      // Written only on success, so a failed icon keeps its previous PNG.
      await fs.writeFile(path.join(ICONS_DIR, `${name}.png`), result.png);
      licenses.set(icon.license ?? 'unknown', (licenses.get(icon.license ?? 'unknown') ?? 0) + 1);
      generated++;
      console.log(`  ✓ ${name}.png ← ${slug}/${result.variant}`);
    } catch (err) {
      console.warn(`  ⚠ ${name}: ${err.message}`);
      failed.push(name);
    }
  });

  console.log('\n' + '═'.repeat(50));
  console.log('Icon generation summary');
  console.log('═'.repeat(50));
  console.log(`  Generated:  ${generated}`);
  console.log(`  Kept as is: ${only ? 0 : keep.size}${keep.size ? ` (${[...keep].sort().join(', ')})` : ''}`);
  console.log(`  Failed:     ${failed.length}${failed.length ? ` (${failed.sort().join(', ')})` : ''}`);
  console.log(`  Unmapped:   ${unmapped.length}${unmapped.length ? ` (${unmapped.join(', ')})` : ''}`);
  console.log(`  Licenses:   ${[...licenses].map(([license, count]) => `${license} ×${count}`).join(', ')}`);
  console.log('═'.repeat(50));

  if (unmapped.length) {
    console.error('\napps.json references icons missing from scripts/icon-sources.json (add them to "icons" or "keep").');
  }
  if (failed.length || unmapped.length) process.exit(1);
}

main().catch((err) => {
  console.error('Fatal error:', err);
  process.exit(1);
});
