const { execSync } = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");

const rootDir = path.join(__dirname, "..");
const wasmDir = path.join(rootDir, "wasm");
const outDir = path.join(rootDir, "out", "wasm");
const pkgDir = path.join(wasmDir, "pkg");

// Run wasm-pack build
execSync("wasm-pack build --target nodejs", { cwd: wasmDir, stdio: "inherit" });

// Clear out/wasm directory if it exists
if (fs.existsSync(outDir)) {
  fs.rmSync(outDir, { recursive: true });
}

// Ensure out/wasm directory exists
fs.mkdirSync(outDir, { recursive: true });

// Move necessary files
const filesToMove = ["wasm_bg.wasm", "wasm.js", "wasm_bg.wasm.d.ts", "wasm.d.ts"];
for (const file of filesToMove) {
  const src = path.join(pkgDir, file);
  const dest = path.join(outDir, file);
  if (fs.existsSync(src)) {
    fs.renameSync(src, dest);
  }
}

console.log("Build and file move completed successfully.");
