import fs from 'node:fs';

const filePath = 'apps.json';
const content = fs.readFileSync(filePath, 'utf8');
const updated = content.replaceAll('.svg"', '.png"');
fs.writeFileSync(filePath, updated, 'utf8');
console.log('Successfully updated all icon_url entries in apps.json from .svg to .png');
