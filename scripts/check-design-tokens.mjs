#!/usr/bin/env node
// Design-token lint (UX-13): fail the build if any `var(--token)` used in the
// frontend references a custom property that App.css never declares. Undeclared
// tokens fail silently in CSS (they fall through to inherited/initial), so a
// typo like `var(--color-danger)` renders no color and ships unnoticed. This
// gate makes that a hard error.
import { readFileSync, readdirSync, statSync } from "node:fs";
import { join, extname, relative } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = join(fileURLToPath(new URL(".", import.meta.url)), "..");
const srcDir = join(repoRoot, "src");
const tokenSourceFile = join(srcDir, "App.css");

// 1. Collect every declared custom property from App.css (both light + dark
//    :root blocks). A declaration looks like `--name:`; a usage looks like
//    `var(--name)` — the trailing `:` vs `)` disambiguates them.
const cssText = readFileSync(tokenSourceFile, "utf8");
const declared = new Set();
for (const m of cssText.matchAll(/(--[a-zA-Z0-9-]+)\s*:/g)) {
  declared.add(m[1]);
}

if (declared.size === 0) {
  console.error(`FAIL: no custom properties found in ${relative(repoRoot, tokenSourceFile)}`);
  process.exit(1);
}

// 2. Walk src/ for files that can reference tokens.
const exts = new Set([".ts", ".tsx", ".css"]);
const files = [];
(function walk(dir) {
  for (const entry of readdirSync(dir)) {
    const full = join(dir, entry);
    if (statSync(full).isDirectory()) {
      walk(full);
    } else if (exts.has(extname(entry))) {
      files.push(full);
    }
  }
})(srcDir);

// 3. Find every var(--token) usage and check it against the declared set.
const violations = [];
for (const file of files) {
  const text = readFileSync(file, "utf8");
  const lines = text.split(/\r?\n/);
  lines.forEach((line, i) => {
    for (const m of line.matchAll(/var\(\s*(--[a-zA-Z0-9-]+)/g)) {
      const token = m[1];
      if (!declared.has(token)) {
        violations.push({ file: relative(repoRoot, file), line: i + 1, token });
      }
    }
  });
}

if (violations.length > 0) {
  console.error("FAIL: undeclared design tokens referenced via var():\n");
  for (const v of violations) {
    console.error(`  ${v.file}:${v.line}  ${v.token}`);
  }
  console.error(
    `\n${violations.length} undeclared token reference(s). ` +
      `Declare them in src/App.css (both light and dark :root) or fix the typo.`
  );
  process.exit(1);
}

console.log(`OK: all var(--token) references resolve to one of ${declared.size} tokens declared in src/App.css.`);
