import { exec } from "node:child_process";
import { promisify } from "node:util";
import * as vscode from "vscode";
import { TailwindSorterFormatter } from "./formatter";
import { findGlobalBinary } from "./rustywind";
import { Logger } from "./utils/logging";

export async function activate(context: vscode.ExtensionContext) {
  const logger = new Logger("Digital Magistery Tailwind Sorter");
  const formatter = new TailwindSorterFormatter(promisify(exec), findGlobalBinary, logger);

  logger.debugLog("Starting extension activation");
  await formatter.initialize();

  logger.debugLog("Extension activated");
  logger.debugLog(`Workspace folders: ${vscode.workspace.workspaceFolders?.map((f) => f.uri.fsPath).join(", ") || "none"}`);

  const configurationChangeDisposable = vscode.workspace.onDidChangeConfiguration((event) => {
    if (event.affectsConfiguration("tailwindSorter")) {
      formatter.updateConfig();
      logger.debugLog("Configuration updated");
    }
  });

  // Register the formatting provider
  const formattingProvider = vscode.languages.registerDocumentFormattingEditProvider(
    { scheme: "file" },
    {
      provideDocumentFormattingEdits: async (document: vscode.TextDocument) => {
        logger.debugLog(`Formatting requested for: ${document.fileName}`);
        if (!formatter.shouldFormatDocument(document)) {
          logger.debugLog(`Skipping format for unsupported file: ${document.fileName}`);
          return undefined;
        }
        return await formatter.formatDocument(document);
      },
    }
  );

  // Handle format on save
  const saveDisposable = vscode.workspace.onWillSaveTextDocument((e) => {
    if (!formatter.shouldFormatDocument(e.document)) {
      return;
    }

    logger.debugLog("Format on save triggered for:", e.document.fileName);

    e.waitUntil(
      (async () => {
        const edits = await formatter.formatDocument(e.document);
        if (edits) {
          const workspaceEdit = new vscode.WorkspaceEdit();
          workspaceEdit.set(e.document.uri, edits);
          await vscode.workspace.applyEdit(workspaceEdit);
        }
      })()
    );
  });

  // Register commands
  const showOutputDisposable = vscode.commands.registerCommand("tailwindSorter.showOutput", () => formatter.showOutput());

  const testFormatterDisposable = vscode.commands.registerCommand("tailwindSorter.testFormatter", async () => {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
      vscode.window.showInformationMessage("No active editor");
      return;
    }

    logger.debugLog("Running formatter diagnostics...");

    const diagnostics = await formatter.diagnose(editor.document);

    if (!formatter.config.languageIds.includes(editor.document.languageId)) {
      logger.debugLog("❌ Language not configured for formatting");
      vscode.window.showInformationMessage(`Language '${editor.document.languageId}' is not configured for Tailwind sorting`);
      return;
    }

    if (!diagnostics.fileSupported) {
      logger.debugLog("❌ File type not supported");
      vscode.window.showInformationMessage("File type not supported for Tailwind sorting");
      return;
    }

    if (!diagnostics.hasTailwindClasses) {
      logger.debugLog("❌ No Tailwind classes found");
      vscode.window.showInformationMessage("No Tailwind classes found in file");
      return;
    }

    if (!diagnostics.rustywindPath) {
      logger.debugLog("❌ Rustywind binary not found");
      vscode.window.showErrorMessage("Rustywind binary not found. Please install rustywind.");
      return;
    }

    logger.debugLog(`✓ Found rustywind at: ${diagnostics.rustywindPath}`);

    if (diagnostics.wouldFormat) {
      logger.debugLog("✓ Classes would be sorted differently");
      vscode.window.showInformationMessage("Formatter would sort Tailwind classes in this file");
    } else {
      logger.debugLog("✓ Classes already in correct order");
      vscode.window.showInformationMessage("Classes are already in correct order");
    }
  });

  // Add all disposables to the extension's subscriptions
  context.subscriptions.push(
    logger,
    formatter,
    configurationChangeDisposable,
    formattingProvider,
    saveDisposable,
    showOutputDisposable,
    testFormatterDisposable
  );
}
