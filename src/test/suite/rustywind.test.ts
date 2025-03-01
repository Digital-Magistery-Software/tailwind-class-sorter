import * as assert from "node:assert";
import * as os from "node:os";
import * as path from "node:path";
import * as vscode from "vscode";
import { RustywindManager } from "../../rustywind";
import { Logger } from "../../utils/logging";
import { createTempDocument, mockExecFail, mockExecSuccess, mockFindBinarySuccess } from "./testUtils";

suite("RustywindManager", function () {
  let manager: RustywindManager;
  let logger: Logger;

  this.beforeEach(() => {
    logger = new Logger("Test Logger");
    manager = new RustywindManager(logger, mockExecSuccess, mockFindBinarySuccess);
  });

  this.afterEach(() => {
    logger.dispose();
  });

  test("findRustywindPath finds binary with custom path", async () => {
    const config = {
      enable: true,
      includeFiles: [],
      languageIds: [],
      customBinaryPath: "/custom/path/to/rustywind",
      debug: false,
      tailwindFunctions: [],
    };

    const result = await manager.findRustywindPath(config);
    assert.strictEqual(result, "/custom/path/to/rustywind");
  });

  test("findRustywindPath falls back to global binary", async () => {
    // Create a manager with workspace root that doesn't exist
    const manager = new RustywindManager(logger, mockExecSuccess, mockFindBinarySuccess);

    const config = {
      enable: true,
      includeFiles: [],
      languageIds: [],
      debug: false,
      tailwindFunctions: [],
    };

    // Create a document in a directory that doesn't exist
    const nonExistentPath = path.join(os.tmpdir(), "nonexistent", "test.tsx");
    const document = {
      uri: vscode.Uri.file(nonExistentPath),
      fileName: nonExistentPath,
    } as vscode.TextDocument;

    const result = await manager.findRustywindPath(config, document);
    assert.strictEqual(result, "/mock/path/rustywind", "Should fall back to global binary");
  });

  test("sortClasses successfully sorts classes", async () => {
    const result = await manager.sortClasses('<div className="mt-2 p-4 flex">Test</div>', "test.tsx", "/mock/path/rustywind", []);
    assert.ok(result.includes("flex") && result.includes("p-4") && result.includes("mt-2"));
  });

  test("wouldFormatChange detects needed changes", async () => {
    const document = await createTempDocument("test.tsx", '<div className="mt-2 p-4 flex">Test</div>');
    const result = await manager.wouldFormatChange(document, "/mock/path/rustywind", []);
    assert.strictEqual(result, true);
  });

  test("handles rustywind errors gracefully", async () => {
    const errorManager = new RustywindManager(logger, mockExecFail, mockFindBinarySuccess);
    await assert.rejects(
      () => errorManager.sortClasses('<div className="mt-2 p-4 flex">Test</div>', "test.tsx", "/mock/path/rustywind", []),
      /Error: Something went wrong/
    );
  });
});
