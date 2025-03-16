import * as vscode from "vscode";
import type { TailwindSorterConfig } from "./utils/types";

export function getConfig(): TailwindSorterConfig {
  const config = vscode.workspace.getConfiguration("tailwindSorter");
  return {
    enable: config.get("enable", true),
    includeFiles: config.get("includeFiles", ["**/*.{js,jsx,ts,tsx,html}"]),
    languageIds: config.get("languageIds", ["typescript", "typescriptreact", "javascript", "javascriptreact", "html"]),
    customBinaryPath: config.get("customBinaryPath", ""),
    debug: config.get("debug", false),
    tailwindFunctions: config.get("tailwindFunctions", ["cn", "cva", "clsx"]),
    internalSorter: {
      enabled: config.get("internalSorter.enabled", false),
      debug: config.get("internalSorter.debug", false),
      removeDuplicateClasses: config.get("internalSorter.removeDuplicateClasses", true),
      normalizeWhitespace: config.get("internalSorter.normalizeWhitespace", true),
    },
  };
}
