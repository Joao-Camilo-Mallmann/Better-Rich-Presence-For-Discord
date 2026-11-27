// Generates crisp application icons, a dedicated high-contrast system tray
// icon, and a multi-frame Windows icon.ico from public/logo.png.
//
// Usage:
//   bun run icons:app

import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import sharp from 'sharp';

const rootDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const SOURCE_LOGO = path.join(rootDir, 'public', 'logo.png');
const PUBLIC_LOGO_256 = path.join(rootDir, 'public', 'logo-256.png');
const TAURI_ICONS_DIR = path.join(rootDir, 'src-tauri', 'icons');

// Dedicated high-contrast 64x64 vector mark designed for 16-32 px tray & small ICO frames.
const SMALL_TRAY_SVG = Buffer.from(`
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" width="64" height="64">
  <!-- Subtle light halo keeps the dark outline distinct on dark Windows 11 taskbars -->
  <rect x="2.5" y="2.5" width="59" height="59" rx="16.5" fill="none" stroke="#9AA4FF" stroke-opacity="0.45" stroke-width="2" />
  <!-- Neo-Brutalist Blurple badge -->
  <rect x="4" y="4" width="56" height="56" rx="15" fill="#5865F2" stroke="#0E1017" stroke-width="4" />
  <!-- Top inner sheen -->
  <path d="M16 9.5 H48" stroke="#8E98FF" stroke-width="2.5" stroke-linecap="round" opacity="0.7" />
  <!-- Broadcast signal arcs + beacon dot -->
  <path d="M20 23.5 A16 16 0 0 1 44 23.5" fill="none" stroke="#FFFFFF" stroke-width="4.5" stroke-linecap="round" />
  <circle cx="32" cy="27" r="3.2" fill="#FFFFFF" />
  <!-- Discord-inspired visor eyes with generous spacing so they stay distinct at 16px -->
  <rect x="14" y="34" width="36" height="18" rx="9" fill="#0E1017" />
  <rect x="19" y="38.5" width="9" height="9" rx="4" fill="#FFFFFF" />
  <rect x="36" y="38.5" width="9" height="9" rx="4" fill="#FFFFFF" />
</svg>
`);

/** Trims transparent padding from `public/logo.png` once for crisp resizing. */
async function loadTrimmedMaster() {
  return sharp(SOURCE_LOGO).ensureAlpha().trim().toBuffer();
}

/** Renders a balanced transparent square PNG from the trimmed master logo. */
async function renderMasterPng(trimmedBuf, size) {
  const padding = Math.max(1, Math.round(size * 0.05));
  const inner = Math.max(1, size - padding * 2);

  let pipeline = sharp(trimmedBuf).resize(inner, inner, {
    fit: 'contain',
    kernel: sharp.kernel.lanczos3,
    background: { r: 0, g: 0, b: 0, alpha: 0 },
  });

  if (size <= 64) {
    pipeline = pipeline.sharpen({ sigma: 0.7 });
  }

  const resized = await pipeline.png().toBuffer();

  return sharp({
    create: {
      width: size,
      height: size,
      channels: 4,
      background: { r: 0, g: 0, b: 0, alpha: 0 },
    },
  })
    .composite([{ input: resized, gravity: 'center' }])
    .png({ compressionLevel: 9, adaptiveFiltering: true })
    .toBuffer();
}

/** Rasterizes the dedicated high-contrast SVG at the requested square size. */
async function renderSmallTrayPng(size) {
  const density = Math.max(72, Math.round((72 * size * 4) / 64));
  let pipeline = sharp(SMALL_TRAY_SVG, { density }).resize(size, size, {
    fit: 'contain',
    kernel: sharp.kernel.lanczos3,
    background: { r: 0, g: 0, b: 0, alpha: 0 },
  });

  if (size <= 32) {
    pipeline = pipeline.sharpen({ sigma: 0.5 });
  }

  return pipeline.png({ compressionLevel: 9, adaptiveFiltering: true }).toBuffer();
}

/**
 * Encodes an uncompressed 32-bpp BGRA DIB (BITMAPINFOHEADER + bottom-up XOR + AND mask)
 * for ICO frames under 256px so Windows `rc.exe` and Explorer accept every frame.
 */
async function encodeBmpIcoFrame(pngBuf, size) {
  const { data, info } = await sharp(pngBuf)
    .ensureAlpha()
    .resize(size, size)
    .raw()
    .toBuffer({ resolveWithObject: true });

  const width = info.width;
  const height = info.height;
  const xorSize = width * height * 4;
  const andRowBytes = Math.ceil(width / 32) * 4;
  const andSize = andRowBytes * height;

  const header = Buffer.alloc(40);
  header.writeUInt32LE(40, 0); // biSize
  header.writeInt32LE(width, 4); // biWidth
  header.writeInt32LE(height * 2, 8); // biHeight (XOR + AND)
  header.writeUInt16LE(1, 12); // biPlanes
  header.writeUInt16LE(32, 14); // biBitCount
  header.writeUInt32LE(0, 16); // biCompression (BI_RGB)
  header.writeUInt32LE(xorSize + andSize, 20); // biSizeImage

  const xorMask = Buffer.alloc(xorSize);
  const andMask = Buffer.alloc(andSize);

  for (let y = 0; y < height; y++) {
    // DIB pixel rows are stored bottom-up.
    const srcY = height - 1 - y;
    for (let x = 0; x < width; x++) {
      const srcIdx = (srcY * width + x) * 4;
      const dstIdx = (y * width + x) * 4;
      const r = data[srcIdx];
      const g = data[srcIdx + 1];
      const b = data[srcIdx + 2];
      const a = data[srcIdx + 3];

      xorMask[dstIdx] = b;
      xorMask[dstIdx + 1] = g;
      xorMask[dstIdx + 2] = r;
      xorMask[dstIdx + 3] = a;

      if (a === 0) {
        const byteIdx = y * andRowBytes + (x >> 3);
        const bitMask = 0x80 >> (x & 7);
        andMask[byteIdx] |= bitMask;
      }
    }
  }

  return Buffer.concat([header, xorMask, andMask]);
}

/** Builds a multi-frame Windows .ico file from `{ size, data }` entries. */
function buildIcoBuffer(frames) {
  const header = Buffer.alloc(6);
  header.writeUInt16LE(0, 0); // Reserved
  header.writeUInt16LE(1, 2); // Icon type
  header.writeUInt16LE(frames.length, 4);

  const dirEntries = [];
  let offset = 6 + frames.length * 16;

  for (const frame of frames) {
    const entry = Buffer.alloc(16);
    entry.writeUInt8(frame.size >= 256 ? 0 : frame.size, 0);
    entry.writeUInt8(frame.size >= 256 ? 0 : frame.size, 1);
    entry.writeUInt8(0, 2); // Color palette count
    entry.writeUInt8(0, 3); // Reserved
    entry.writeUInt16LE(1, 4); // Planes
    entry.writeUInt16LE(32, 6); // Bit count
    entry.writeUInt32LE(frame.data.length, 8);
    entry.writeUInt32LE(offset, 12);
    dirEntries.push(entry);
    offset += frame.data.length;
  }

  return Buffer.concat([header, ...dirEntries, ...frames.map((f) => f.data)]);
}

async function main() {
  const trimmedMaster = await loadTrimmedMaster();

  const masterOutputs = [
    [path.join(PUBLIC_LOGO_256), 256],
    [path.join(TAURI_ICONS_DIR, '64x64.png'), 64],
    [path.join(TAURI_ICONS_DIR, '128x128.png'), 128],
    [path.join(TAURI_ICONS_DIR, '128x128@2x.png'), 256],
    [path.join(TAURI_ICONS_DIR, 'icon.png'), 512],
    [path.join(TAURI_ICONS_DIR, 'StoreLogo.png'), 50],
    [path.join(TAURI_ICONS_DIR, 'Square30x30Logo.png'), 30],
    [path.join(TAURI_ICONS_DIR, 'Square44x44Logo.png'), 44],
    [path.join(TAURI_ICONS_DIR, 'Square71x71Logo.png'), 71],
    [path.join(TAURI_ICONS_DIR, 'Square89x89Logo.png'), 89],
    [path.join(TAURI_ICONS_DIR, 'Square107x107Logo.png'), 107],
    [path.join(TAURI_ICONS_DIR, 'Square142x142Logo.png'), 142],
    [path.join(TAURI_ICONS_DIR, 'Square150x150Logo.png'), 150],
    [path.join(TAURI_ICONS_DIR, 'Square284x284Logo.png'), 284],
    [path.join(TAURI_ICONS_DIR, 'Square310x310Logo.png'), 310],
  ];

  for (const [outPath, size] of masterOutputs) {
    const png = await renderMasterPng(trimmedMaster, size);
    await fs.writeFile(outPath, png);
  }

  // Dedicated high-contrast system tray PNGs.
  await fs.writeFile(path.join(TAURI_ICONS_DIR, 'tray.png'), await renderSmallTrayPng(64));
  await fs.writeFile(path.join(TAURI_ICONS_DIR, '32x32.png'), await renderSmallTrayPng(32));

  // Multi-frame Windows .ico: crisp SVG mark for 16-32 px, trimmed master for 40-256 px.
  const icoSizes = [16, 20, 24, 32, 40, 48, 64, 256];
  const icoFrames = [];
  for (const size of icoSizes) {
    const png =
      size <= 32
        ? await renderSmallTrayPng(size)
        : await renderMasterPng(trimmedMaster, size);
    const data = size === 256 ? png : await encodeBmpIcoFrame(png, size);
    icoFrames.push({ size, data });
  }

  await fs.writeFile(path.join(TAURI_ICONS_DIR, 'icon.ico'), buildIcoBuffer(icoFrames));
  console.log('Generated app icons, tray.png, 32x32.png, and multi-size icon.ico.');
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
