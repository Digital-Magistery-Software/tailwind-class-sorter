import * as fs from "node:fs";
import { existsSync } from "node:fs";
import * as path from "node:path";
import * as vscode from "vscode";
import { extractClasses } from "./utils/classExtractor";
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

  async sortClasses(text: string, rustywindPath: string, tailwindFunctions: string[]): Promise<string> {
    const classMatches = extractClasses(text, tailwindFunctions);
    let formattedText = text;

    for (const { original, classString } of classMatches) {
      try {
        // Wrap the class string in a className attribute
        const wrappedClasses = `<div className="${classString}"></div>`;
        const { stdout, stderr } = await this.execCommand(`echo "${wrappedClasses}" | "${rustywindPath}" --stdin`);

        if (stderr) {
          this.logger.debugLog(`Warning while sorting classes: ${stderr}`);
          continue;
        }

        // Extract the sorted classes from the response
        const match = stdout.match(/className=["'`]([^"'`]+)["'`]/);
        if (!match) {
          this.logger.debugLog("Could not extract sorted classes from RustyWind output");
          continue;
        }

        const sortedClasses = match[1].trim();
        if (sortedClasses && sortedClasses !== classString) {
          formattedText = formattedText.replace(original, original.replace(classString, sortedClasses));
        }
      } catch (error) {
        this.logger.debugLog(`Error sorting classes: ${error}`);
      }
    }

    return formattedText;
  }

  async wouldFormatChange(document: vscode.TextDocument, rustywindPath: string, tailwindFunctions: string[]): Promise<boolean> {
    const text = document.getText();
    const classMatches = extractClasses(text, tailwindFunctions);

    for (const { classString } of classMatches) {
      try {
        const wrappedClasses = `<div className="${classString}"></div>`;
        const { stdout } = await this.execCommand(`echo "${wrappedClasses}" | "${rustywindPath}" --stdin --dry-run`);
        if (stdout.trim()) {
          return true;
        }
      } catch (error) {
        this.logger.debugLog(`Error checking classes: ${error}`);
      }
    }

    return false;
  }
}
