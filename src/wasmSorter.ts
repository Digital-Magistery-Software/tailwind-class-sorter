import type { Logger } from "./utils/logging";
import type { TailwindSorterConfig } from "./utils/types";

let wasmSorter: {
  sort_tailwind_classes: (document: string, fileExtension: string) => string;
  configure_tailwind_sorter: (removeDuplicates: boolean, debugMode: boolean) => void;
} | null = null;

export async function initWasmSorter(logger: Logger, config: TailwindSorterConfig): Promise<void> {
  try {
    const wasm = await import("../out/wasm/digital_magistery_tailwind_sorter.js");
    wasmSorter = wasm;

    // Initialize configuration
    configureWasmSorter(config, logger);

    logger.debugLog("WASM sorter initialized successfully");
  } catch (error) {
    logger.debugLog(`Failed to initialize WASM sorter: ${error}`);
    wasmSorter = null;
  }
}

export function configureWasmSorter(config: TailwindSorterConfig, logger: Logger): void {
  if (!wasmSorter) {
    logger.debugLog("Cannot configure WASM sorter: not initialized");
    return;
  }

  const removeDuplicates = config.internalSorter.removeDuplicateClasses;
  const debugMode = config.internalSorter.debug;

  logger.debugLog(`Configuring WASM sorter with removeDuplicateClasses=${removeDuplicates}, debug=${debugMode}`);

  wasmSorter.configure_tailwind_sorter(removeDuplicates, debugMode);
}

export async function sortClassesWithWasm(document: string, fileName: string, logger: Logger, config: TailwindSorterConfig): Promise<string> {
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
