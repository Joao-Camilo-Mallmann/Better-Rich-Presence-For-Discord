// Checks that the React Compiler can compile every component and hook in src/.
//
// With default options the compiler silently skips ("bails out" of) any
// function it cannot compile, so that component loses auto-memoization
// without any build error. This script runs the same Babel plugin with a
// logger and fails when any function is skipped.
//
// Usage: bun run compiler:check
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { transformSync } from '@babel/core';

const rootDir = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const srcDir = path.join(rootDir, 'src');

const sourceFiles = (dir) =>
  fs.readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) return sourceFiles(full);
    return /\.(tsx?|jsx?)$/.test(entry.name) && !entry.name.endsWith('.d.ts') ? [full] : [];
  });

// Pulls a readable reason and line number out of a CompileError event.
const describe = (detail = {}) => {
  const reason = detail.reason ?? detail.options?.reason ?? detail.message ?? 'unknown reason';
  const loc = detail.loc ?? detail.options?.details?.[0]?.loc ?? detail.primaryLocation?.();
  return { reason, line: loc?.start?.line ?? '?' };
};

let compiled = 0;
const failures = [];

for (const file of sourceFiles(srcDir)) {
  const rel = path.relative(rootDir, file);
  const logger = {
    logEvent(_filename, event) {
      if (event.kind === 'CompileSuccess') {
        compiled++;
      } else if (event.kind === 'CompileError' || event.kind === 'PipelineError') {
        const { reason, line } = describe(event.detail);
        failures.push(`${rel}:${line} ${event.fnName ?? ''} — ${reason}`);
      }
    },
  };

  transformSync(fs.readFileSync(file, 'utf8'), {
    filename: file,
    babelrc: false,
    configFile: false,
    parserOpts: { plugins: ['typescript', 'jsx'] },
    plugins: [['babel-plugin-react-compiler', { logger, panicThreshold: 'none' }]],
  });
}

if (failures.length > 0) {
  console.error(`React Compiler skipped ${failures.length} function(s):`);
  for (const failure of failures) console.error(`  ${failure}`);
  process.exit(1);
}
console.log(`React Compiler: ${compiled} components/hooks compiled, 0 skipped.`);
