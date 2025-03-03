import type { Logger } from "./utils/logging";

let wasmSorter: {
  sort_tailwind_classes: (document: string, fileExtension: string) => string;
} | null = null;

export async function initWasmSorter(logger: Logger): Promise<void> {
  try {
    const wasm = await import("../out/wasm/digital_magistery_tailwind_sorter.js");
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

  try {
    const fileExtension = fileName.split(".").pop() || "";

    logger.debugLog(`Sorting Tailwind classes in ${fileName} with internal sorter`);

    const result = wasmSorter.sort_tailwind_classes(document, fileExtension);

    logger.debugLog("Tailwind classes sorted successfully with internal sorter");
    return result;
  } catch (error) {
    logger.debugLog(`Error during internal sorting: ${error}`);
    throw new Error(`Internal Tailwind class sorter error: ${error}`);
  }
}
