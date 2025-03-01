import { minimatch } from "minimatch";
import * as vscode from "vscode";
import { getConfig } from "./config";
import { RustywindManager } from "./rustywind";
import type { Logger } from "./utils/logging";
import type { DiagnosticResult, ExecFunction, TailwindSorterConfig } from "./utils/types";

export class TailwindSorterFormatter implements vscode.Disposable {
  private statusBarItem: vscode.StatusBarItem;
  private rustywindInstalled = false;
  private rustywindPath: string | null = null;
  public config: TailwindSorterConfig;
  private rustywindManager: RustywindManager;

  constructor(execCommand: ExecFunction, findBinary: (binaryName: string) => Promise<string | null>, private readonly logger: Logger) {
    this.statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    this.config = getConfig();
    this.logger.updateConfig(this.config);
    this.rustywindManager = new RustywindManager(this.logger, execCommand, findBinary);
    this.setupStatusBar();
  }

  private setupStatusBar(): void {
    this.statusBarItem.text = "Tailwind Sorter";
    this.statusBarItem.tooltip = "Sort Tailwind Classes";
    this.statusBarItem.show();
  }

  public showOutput(): void {
    this.logger.showOutput();
  }

  private isFileIncluded(fileName: string): boolean {
    return this.config.includeFiles.some((pattern) => minimatch(fileName, pattern));
  }

  private handleFormatError(fileName: string, error: unknown): void {
    const errorMessage = error instanceof Error ? error.message : String(error);
    this.logger.debugLog(`Error formatting ${fileName}: ${errorMessage}`);
    this.logger.log(`Error formatting ${fileName}: ${errorMessage}`);
    this.statusBarItem.text = "$(alert) Tailwind Sorter";
    this.statusBarItem.tooltip = "Error occurred. Click to show details.";
    vscode.window.showErrorMessage(`Tailwind Sorter: ${errorMessage}`, "Show Details").then((selection) => {
      if (selection === "Show Details") {
        this.showOutput();
      }
    });
  }

  public async initialize(): Promise<void> {
    this.logger.debugLog("Starting initialization");
    this.rustywindPath = await this.rustywindManager.findRustywindPath(this.config);
    this.rustywindInstalled = !!this.rustywindPath;

    if (this.rustywindInstalled) {
      this.statusBarItem.text = "Tailwind Sorter";
      this.statusBarItem.tooltip = "Sort Tailwind Classes";
    } else {
      vscode.window.showErrorMessage(
        "Rustywind is not installed. The Digital Magistery Tailwind Class Sorter extension requires Rustywind. Please install it in your project."
      );
      this.statusBarItem.text = "$(alert) Tailwind Sorter (Rustywind not found)";
      this.statusBarItem.tooltip = "Rustywind not found. Click to show details.";
    }
  }

  public getDocumentSelectors(): vscode.DocumentSelector[] {
    return this.config.languageIds.map((language) => ({
      scheme: "file",
      language,
    }));
  }

  public shouldFormatDocument(document: vscode.TextDocument): boolean {
    return this.config.languageIds.includes(document.languageId) && this.isFileIncluded(document.fileName);
  }

  public updateConfig(): void {
    this.config = getConfig();
    this.logger.updateConfig(this.config);
  }

  public async diagnose(document: vscode.TextDocument): Promise<DiagnosticResult> {
    const fileSupported = this.shouldFormatDocument(document);

    let wouldFormat = false;
    if (fileSupported && this.rustywindPath) {
      wouldFormat = await this.rustywindManager.wouldFormatChange(document, this.rustywindPath, this.config.tailwindFunctions);
    }

    return {
      fileSupported,
      rustywindPath: this.rustywindPath,
      wouldFormat,
    };
  }

  public async formatDocument(document: vscode.TextDocument): Promise<vscode.TextEdit[] | undefined> {
    if (!this.config.enable) {
      this.logger.debugLog("Tailwind Sorter is disabled in settings");
      return;
    }

    if (!(this.rustywindInstalled && this.rustywindPath)) {
      return;
    }

    const fileName = document.fileName;
    if (!this.isFileIncluded(fileName)) {
      this.logger.debugLog(`File ${fileName} is not a supported file type`);
      return;
    }

    const text = document.getText();

    try {
      const formatted = await this.rustywindManager.sortClasses(text, fileName, this.rustywindPath, this.config.tailwindFunctions);
      if (formatted.trim() === text.trim()) {
        this.logger.debugLog("No changes needed - classes already sorted");
        return;
      }

      const fullRange = new vscode.Range(document.positionAt(0), document.positionAt(text.length));
      this.logger.debugLog("Classes sorted successfully");
      return [vscode.TextEdit.replace(fullRange, formatted)];
    } catch (error) {
      this.handleFormatError(fileName, error);
    }
  }

  dispose(): void {
    this.statusBarItem.dispose();
  }
}
