import fs from 'node:fs';
import path from 'node:path';
import sharp from 'sharp';

const iconsDir = path.resolve('public/assets/icons');

async function convertSvgs() {
  if (!fs.existsSync(iconsDir)) {
    console.error('Directory does not exist:', iconsDir);
    process.exit(1);
  }

  const files = fs.readdirSync(iconsDir).filter((file) => file.endsWith('.svg'));
  console.log(`Found ${files.length} SVG files to convert...`);

  let count = 0;
  for (const file of files) {
    const svgPath = path.join(iconsDir, file);
    const baseName = path.basename(file, '.svg');
    const pngPath = path.join(iconsDir, `${baseName}.png`);

    try {
      await sharp(svgPath)
        .resize(1024, 1024, { fit: 'contain', background: { r: 0, g: 0, b: 0, alpha: 0 } })
        .png()
        .toFile(pngPath);

      // Also create short asset-key named PNG if simple-icons-* or material-symbols-*
      let assetKey = baseName;
      if (baseName.startsWith('simple-icons-')) {
        assetKey = baseName.replace('simple-icons-', '');
      } else if (baseName.startsWith('material-symbols-')) {
        assetKey = baseName.replace('material-symbols-', '');
      }

      if (assetKey !== baseName) {
        const assetKeyPngPath = path.join(iconsDir, `${assetKey}.png`);
        await sharp(svgPath)
          .resize(1024, 1024, { fit: 'contain', background: { r: 0, g: 0, b: 0, alpha: 0 } })
          .png()
          .toFile(assetKeyPngPath);
      }

      count++;
    } catch (err) {
      console.error(`Error converting ${file}:`, err.message);
    }
  }

  console.log(`Successfully converted ${count} SVG files to 1024x1024 PNG files!`);
}

convertSvgs();
