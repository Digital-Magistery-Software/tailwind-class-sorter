import * as assert from "node:assert";
import * as vscode from "vscode";
import { createTempDocument } from "./testUtils";

suite("Extension Activation", () => {
  test("Extension should activate", async () => {
    const ext = vscode.extensions.getExtension("digital-magistery-software.digital-magistery-tailwind-class-sorter");
    assert.ok(ext);
    await ext.activate();
    assert.strictEqual(ext.isActive, true);
  });

  test("Should register all commands", async () => {
    const commands = await vscode.commands.getCommands(true);
    assert.ok(commands.includes("tailwindSorter.showOutput"));
    assert.ok(commands.includes("tailwindSorter.testFormatter"));
  });

  test("Commands should execute without error", async () => {
    await vscode.commands.executeCommand("tailwindSorter.showOutput");

    const document = await createTempDocument("test.tsx", '<div className="p-4 mt-2">Test</div>');

    await vscode.window.showTextDocument(document);

    await vscode.commands.executeCommand("tailwindSorter.testFormatter");
  });
});

suite("Extension Settings", () => {
  test("Should load default settings", () => {
    const config = vscode.workspace.getConfiguration("tailwindSorter");
    assert.strictEqual(config.get("enable"), true);
    assert.deepStrictEqual(config.get("includeFiles"), ["**/*.{js,jsx,ts,tsx,html}"]);
    assert.deepStrictEqual(config.get("languageIds"), ["typescript", "typescriptreact", "javascript", "javascriptreact", "html"]);
  });
});
