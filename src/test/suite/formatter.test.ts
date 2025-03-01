import * as assert from "node:assert";
import * as vscode from "vscode";
import { TailwindSorterFormatter } from "../../formatter";
import { Logger } from "../../utils/logging";
import { createTempDocument, mockExecSuccess, mockFindBinarySuccess } from "./testUtils";

suite("TailwindSorterFormatter", function () {
  let formatter: TailwindSorterFormatter;

  this.beforeEach(() => {
    formatter = new TailwindSorterFormatter(mockExecSuccess, mockFindBinarySuccess, new Logger("Test Logger"));
  });

  this.afterEach(() => {
    formatter.dispose();
  });

  suite("Basic Functionality", () => {
    test("Extension should be present", () => {
      assert.ok(vscode.extensions.getExtension("digital-magistery-software.digital-magistery-tailwind-class-sorter"));
    });

    test("Should have required commands registered", async () => {
      const commands = await vscode.commands.getCommands(true);
      assert.ok(commands.includes("tailwindSorter.showOutput"));
      assert.ok(commands.includes("tailwindSorter.testFormatter"));
    });
  });

  suite("File Detection", () => {
    test("Should detect supported file types", async () => {
      const diagnostics = await formatter.diagnose(await createTempDocument("test.tsx", '<div className="p-4">Test</div>'));
      assert.strictEqual(diagnostics.fileSupported, true);
    });

    test("Should reject unsupported file types", async () => {
      const diagnostics = await formatter.diagnose(await createTempDocument("test.css", ".test { color: red; }"));
      assert.strictEqual(diagnostics.fileSupported, false);
    });
  });

  suite("Formatting", () => {
    test("Should format document when conditions are met", async () => {
      await formatter.initialize();

      // Force rustywindInstalled and rustywindPath to be set since we're using mocks
      (formatter as unknown as { rustywindInstalled: boolean }).rustywindInstalled = true;
      (formatter as unknown as { rustywindPath: string }).rustywindPath = "/mock/path/rustywind";

      const document = await createTempDocument("test.tsx", '<div className="mt-2 p-4 flex items-center bg-white justify-between">Test</div>');
      const edits = await formatter.formatDocument(document);

      assert.ok(edits, "Edits should exist");
      assert.strictEqual(edits.length, 1, "Should have one edit");
      assert.ok(edits[0].newText.includes("flex") && edits[0].newText.includes("items-center") && edits[0].newText.includes("justify-between"));
    });

    test("Should not format when disabled", async () => {
      formatter.config.enable = false;
      const document = await createTempDocument("test.tsx", '<div className="p-4 mt-2">Test</div>');
      const edits = await formatter.formatDocument(document);
      assert.strictEqual(edits, undefined);
    });
  });

  suite("Configuration", () => {
    test("Should update configuration", () => {
      const originalConfig = { ...formatter.config };
      formatter.config.enable = !originalConfig.enable;
      // Testing that updateConfig changes the config back to default values
      formatter.updateConfig();
      assert.notStrictEqual(formatter.config.enable, !originalConfig.enable);
    });

    test("Should respect language settings", async () => {
      formatter.config.languageIds = ["typescript"];
      const document = await createTempDocument("test.jsx", '<div className="p-4 mt-2">Test</div>');
      assert.strictEqual(formatter.shouldFormatDocument(document), false);
    });
  });
});
