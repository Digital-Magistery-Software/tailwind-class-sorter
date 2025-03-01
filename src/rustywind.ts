import * as fs from "node:fs";
import { existsSync } from "node:fs";
import * as path from "node:path";
import * as vscode from "vscode";
import { extractClassesWithOxc } from "./utils/classExtractorOxc";
import type { Logger } from "./utils/logging";
import type { ExecFunction, TailwindSorterConfig } from "./utils/types";

export async function findGlobalBinary(binaryName: string): Promise<string | null> {
  const paths = process.env.PATH?.split(path.delimiter) || [];
  const exts = process.platform === "win32" ? process.env.PATHEXT?.split(path.delimiter) || [".EXE", ".CMD", ".BAT"] : [""];

  for (const dir of paths) {
    for (const ext of exts) {
      const fullPath = path.join(dir, binaryName + ext);
      try {
        await fs.promises.access(fullPath, fs.constants.X_OK);
        return fullPath;
      } catch {
        // Didn't find the binary in this path
      }
    }
  }
  return null;
}

export class RustywindManager {
  constructor(
    private readonly logger: Logger,
    private readonly execCommand: ExecFunction,
    private readonly findBinary: (binaryName: string) => Promise<string | null>
  ) {}

  async findRustywindPath(config: TailwindSorterConfig, document?: vscode.TextDocument): Promise<string | null> {
    try {
      this.logger.debugLog(`Starting rustywind search${document ? ` for file: ${document.uri.fsPath}` : ""}`);

      if (config.customBinaryPath) {
        this.logger.debugLog(`Custom path configured: ${config.customBinaryPath}`);
        return config.customBinaryPath;
      }

      let currentDir = document ? path.dirname(document.uri.fsPath) : vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;

      if (!currentDir) {
        return null;
      }

      while (currentDir !== path.dirname(currentDir)) {
        // Check for Yarn PnP
        if (existsSync(path.join(currentDir, ".pnp.cjs"))) {
          this.logger.debugLog(`Found Yarn PnP at: ${path.join(currentDir, ".pnp.cjs")}`);

          // Look in .yarn/unplugged for rustywind
          const unpluggedPath = path.join(currentDir, ".yarn/unplugged");
          if (existsSync(unpluggedPath)) {
            try {
              const dirs = await fs.promises.readdir(unpluggedPath);
              const rustywindDir = dirs.find((dir) => dir.startsWith("rustywind-npm-"));

              if (rustywindDir) {
                const binaryPath = path.join(unpluggedPath, rustywindDir, "node_modules/rustywind/bin/rustywind");
                if (existsSync(binaryPath)) {
                  this.logger.debugLog(`Found PnP rustywind at: ${binaryPath}`);
                  return binaryPath;
                }
              }
            } catch (error) {
              this.logger.debugLog("Error reading unplugged directory:", error);
            }
          }
        }

        // Traditional node_modules check
        const localPath = path.join(currentDir, "node_modules", ".bin", "rustywind");
        if (existsSync(localPath)) {
          this.logger.debugLog(`Found app-local rustywind at: ${localPath}`);
          return localPath;
        }

        currentDir = path.dirname(currentDir);
      }

      // Try global as last resort
      const globalPath = await this.findBinary("rustywind");
      if (globalPath) {
        this.logger.debugLog(`Found global rustywind at: ${globalPath}`);
        return globalPath;
      }

      this.logger.debugLog("No rustywind installation found");
      return null;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      this.logger.debugLog(`Error in findRustywindPath: ${errorMessage}`);
      return null;
    }
  }

  private transformCustomPropertiesToTailwind3(classes: string): string {
    return classes.replace(/([a-z-]+)-\(--([^)]+)\)/g, "$1-[--$2]");
  }

  private transformCustomPropertiesBackToTailwind4(classes: string): string {
    return classes.replace(/([a-z-]+)-\[--([^\]]+)\]/g, "$1-(--$2)");
  }

  public async sortClasses(text: string, filename: string, rustywindPath: string, tailwindFunctions: string[]): Promise<string> {
    // Extract class strings using Oxc
    const classMatches = extractClassesWithOxc(text, filename, tailwindFunctions);
    this.logger.debugLog(`Found ${classMatches.length} class strings`);

    // Keep track of edited positions to avoid double-editing
    const editedPositions = new Set<string>();

    // Sort matches from end to start to avoid offset issues
    const sortedMatches = [...classMatches].sort((a, b) => b.start - a.start);

    // Process each match
    let formattedText = text;
    for (const match of sortedMatches) {
      try {
        // Skip if we've already edited this position
        const posKey = `${match.start}:${match.end}`;
        if (editedPositions.has(posKey)) {
          this.logger.debugLog(`Skipping already processed position: ${posKey}`);
          continue;
        }

        this.logger.debugLog(`Processing class string: ${match.classString}`);

        // Skip empty strings
        if (!match.classString.trim()) {
          this.logger.debugLog("Skipping empty class string");
          continue;
        }

        // Transform Tailwind 4 custom properties to Tailwind 3 format (Rustywind doesn't seem to support Tailwind 4 custom properties)
        const transformedClasses = this.transformCustomPropertiesToTailwind3(match.classString);

        // Prepare the class string for Rustywind
        const wrappedClasses = `<div className="${transformedClasses}"></div>`;

        // Sort the classes using Rustywind
        const { stdout, stderr } = await this.execCommand(`echo "${wrappedClasses}" | "${rustywindPath}" --stdin`);

        if (stderr && !stderr.includes("No classes were found")) {
          this.logger.debugLog(`Warning while sorting classes: ${stderr}`);
          continue;
        }

        // Extract sorted classes from Rustywind output
        const sortedMatch = stdout.match(/className=["'`]([^"'`]+)["'`]/);
        if (!sortedMatch) {
          this.logger.debugLog("Could not extract sorted classes from Rustywind output");
          continue;
        }

        const sortedClasses = sortedMatch[1].trim();
        if (sortedClasses && sortedClasses !== transformedClasses) {
          // Transform back to Tailwind 4 format
          const finalClasses = this.transformCustomPropertiesBackToTailwind4(sortedClasses);

          // Check if this is a template literal quasi
          const isTemplateLiteralQuasi = match.path?.includes(".quasis[");

          let replacement: string;
          if (isTemplateLiteralQuasi) {
            // For template literals, preserve the exact format from the original
            // Extract any leading/trailing whitespace from the original
            const original = match.original;
            const leadingWhitespace = original?.match(/^(\s*)/)?.[0] ?? "";
            const trailingWhitespace = original?.match(/(\s*)$/)?.[0] ?? "";

            // Apply the whitespace to the sorted classes
            replacement = leadingWhitespace + finalClasses + trailingWhitespace;
          } else {
            // For regular literals, preserve quotes
            const original = match.original;
            let openingQuote = "";
            let closingQuote = "";

            if (original.startsWith('"') || original.startsWith("'") || original.startsWith("`")) {
              openingQuote = original.charAt(0);
              closingQuote = original.charAt(original.length - 1);
            }

            replacement = openingQuote + finalClasses + closingQuote;
          }

          const before = formattedText.slice(0, match.start);
          const after = formattedText.slice(match.end);
          formattedText = before + replacement + after;

          // Mark this position as edited
          editedPositions.add(posKey);
        }
      } catch (error) {
        this.logger.debugLog(`Error sorting classes: ${error}`);
      }
    }

    return formattedText;
  }

  async wouldFormatChange(document: vscode.TextDocument, rustywindPath: string, tailwindFunctions: string[]): Promise<boolean> {
    const text = document.getText();
    const classMatches = extractClassesWithOxc(text, document.fileName, tailwindFunctions);

    for (const match of classMatches) {
      try {
        this.logger.debugLog(`Processing class string: ${match.classString}`);
        // Skip empty strings
        if (!match.classString.trim()) {
          this.logger.debugLog("Skipping empty class string");
          continue;
        }

        const transformedClasses = this.transformCustomPropertiesToTailwind3(match.classString);
        const wrappedClasses = `<div className="${transformedClasses}"></div>`;
        const { stdout, stderr } = await this.execCommand(`echo "${wrappedClasses}" | "${rustywindPath}" --stdin`);

        if (stderr && !stderr.includes("No classes were found")) {
          this.logger.debugLog(`Warning while sorting classes: ${stderr}`);
          continue;
        }

        const sortedMatch = stdout.match(/className=["'`]([^"'`]+)["'`]/);
        const sortedClasses = sortedMatch?.[1].trim() ?? "";

        this.logger.debugLog(`Sorted classes: ${sortedClasses}`);
        this.logger.debugLog(`Transformed classes: ${transformedClasses}`);

        if (sortedClasses && sortedClasses !== transformedClasses) {
          return true;
        }

        this.logger.debugLog("Classes are the same; no formatting needed.");
      } catch (error) {
        this.logger.debugLog(`Error checking classes: ${error}`);
      }
    }

    return false;
  }
}
