import type { Logger } from "./utils/logging";

let wasmSorter: {
  sort_tailwind_classes: (document: string, fileExtension: string) => string;
} | null = null;

export async function initWasmSorter(logger: Logger): Promise<void> {
  try {
    const wasm = await import("../out/wasm/wasm.js");
    wasmSorter = wasm;
    logger.debugLog("WASM sorter initialized successfully");
  } catch (error) {
    logger.debugLog(`Failed to initialize WASM sorter: ${error}`);
    wasmSorter = null;
  }
}

export async function sortClassesWithWasm(document: string, fileName: string, logger: Logger): Promise<string> {
  if (!wasmSorter) {
    logger.debugLog("WASM sorter not initialized");
    throw new Error("Internal sorter not initialized");
  }

  const fileExtension = fileName.split(".").pop() || "";
  return wasmSorter.sort_tailwind_classes(document, fileExtension);
}
