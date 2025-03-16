import * as assert from "node:assert";
import * as vscode from "vscode";
import { TailwindSorterFormatter } from "../../formatter";
import { Logger } from "../../utils/logging";
import { createTempDocument, mockExecSuccess, mockFindBinarySuccess } from "./testUtils";

suite("TailwindSorterFormatter - Core Functionality", function () {
  let formatter: TailwindSorterFormatter;

  this.beforeEach(() => {
    formatter = new TailwindSorterFormatter(mockExecSuccess, mockFindBinarySuccess, new Logger("Test Logger"));
  });

  this.afterEach(() => {
    formatter.dispose();
  });

  suite("Extension Setup", () => {
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

  suite("Configuration", () => {
    test("Should update configuration", () => {
      const originalConfig = { ...formatter.config };
      formatter.config.enable = !originalConfig.enable;
      // Test that updateConfig reverts to default configuration
      formatter.updateConfig();
      assert.notStrictEqual(formatter.config.enable, !originalConfig.enable);
    });

    test("Should respect file include patterns", async () => {
      formatter.config.includeFiles = ["**/*.jsx"];

      const tsxDocument = await createTempDocument("test.tsx", '<div className="p-4">Test</div>');
      const jsxDocument = await createTempDocument("test.jsx", '<div className="p-4">Test</div>');

      assert.strictEqual(formatter.shouldFormatDocument(tsxDocument), false);
      assert.strictEqual(formatter.shouldFormatDocument(jsxDocument), true);
    });
  });
});
