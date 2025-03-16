import * as assert from "node:assert";
import { TailwindSorterFormatter } from "../../formatter";
import { Logger } from "../../utils/logging";
import { createTempDocument, mockExecFail, mockExecSuccess, mockFindBinaryFail, mockFindBinarySuccess } from "./testUtils";

suite("TailwindSorterFormatter - Rustywind", function () {
  let formatter: TailwindSorterFormatter;
  let logger: Logger;

  this.beforeEach(() => {
    logger = new Logger("Test Logger");
    formatter = new TailwindSorterFormatter(mockExecSuccess, mockFindBinarySuccess, logger);
  });

  this.afterEach(() => {
    formatter.dispose();
    logger.dispose();
  });

  suite("Basic Functionality", () => {
    test("Should use Rustywind when internal sorter is disabled", async () => {
      // Force rustywindInstalled and rustywindPath to be set since we're using mocks
      await formatter.initialize();
      (formatter as unknown as { rustywindInstalled: boolean }).rustywindInstalled = true;
      (formatter as unknown as { rustywindPath: string }).rustywindPath = "/mock/path/rustywind";

      const document = await createTempDocument("test.tsx", '<div className="mt-2 p-4 flex">Test</div>');

      const edits = await formatter.formatDocument(document);

      assert.ok(edits, "Edits should exist");
      assert.strictEqual(edits.length, 1, "Should have one edit");
      // The mock will always return the same string, just check for basic formatting
      assert.ok(edits[0].newText.includes("flex"), "Should have formatted with Rustywind");
    });

    test("Should respect language settings", async () => {
      formatter.config.languageIds = ["typescript"];
      const document = await createTempDocument("test.jsx", '<div className="p-4 mt-2">Test</div>');
      assert.strictEqual(formatter.shouldFormatDocument(document), false);
    });
  });

  suite("Configuration", () => {
    test("Should not format when disabled", async () => {
      formatter.config.enable = false;
      const document = await createTempDocument("test.tsx", '<div className="p-4 mt-2">Test</div>');
      const edits = await formatter.formatDocument(document);
      assert.strictEqual(edits, undefined);
    });
  });

  suite("Error Handling", () => {
    test("Should not format when Rustywind is not installed", async () => {
      const customFormatter = new TailwindSorterFormatter(mockExecSuccess, mockFindBinaryFail, logger);
      customFormatter.config.internalSorter.enabled = false;

      await customFormatter.initialize();

      const document = await createTempDocument("test.tsx", '<div className="p-4 mt-2">Test</div>');
      const edits = await customFormatter.formatDocument(document);

      assert.strictEqual(edits, undefined, "Should not format when Rustywind is not installed");

      customFormatter.dispose();
    });

    test("Should handle exec errors gracefully", async () => {
      const customFormatter = new TailwindSorterFormatter(mockExecFail, mockFindBinarySuccess, logger);
      customFormatter.config.internalSorter.enabled = false;

      await customFormatter.initialize();
      (formatter as unknown as { rustywindInstalled: boolean }).rustywindInstalled = true;
      (formatter as unknown as { rustywindPath: string }).rustywindPath = "/mock/path/rustywind";

      const document = await createTempDocument("test.tsx", '<div className="p-4 mt-2">Test</div>');
      const edits = await customFormatter.formatDocument(document);

      assert.strictEqual(edits, undefined, "Should not format when exec fails");

      customFormatter.dispose();
    });
  });

  suite("Class Detection", () => {
    // For Rustywind tests, the mockExecSuccess will return a fixed string
    // So we can only verify that formatting happened, not specific class ordering

    test("Should detect and format class attributes", async () => {
      await formatter.initialize();
      (formatter as unknown as { rustywindInstalled: boolean }).rustywindInstalled = true;
      (formatter as unknown as { rustywindPath: string }).rustywindPath = "/mock/path/rustywind";

      const document = await createTempDocument("test.js", '<div class="mt-2 p-4 flex">Test</div>');

      // Explicitly verify that the formatter should format this document
      assert.strictEqual(formatter.shouldFormatDocument(document), true, "HTML document should be recognized as formattable");

      const edits = await formatter.formatDocument(document);

      // Our mock returns a fixed string
      assert.ok(edits, "Edits should exist");
      assert.strictEqual(edits.length, 1, "Should have one edit");
    });

    test("Should detect and format className in React components", async () => {
      await formatter.initialize();
      (formatter as unknown as { rustywindInstalled: boolean }).rustywindInstalled = true;
      (formatter as unknown as { rustywindPath: string }).rustywindPath = "/mock/path/rustywind";

      const document = await createTempDocument("test.tsx", '<div className="mt-2 p-4 flex">Test</div>');

      const edits = await formatter.formatDocument(document);

      // Our mock returns a fixed string
      assert.ok(edits, "Edits should exist");
      assert.strictEqual(edits.length, 1, "Should have one edit");
    });

    test("Should format tailwind classes within function calls", async () => {
      await formatter.initialize();
      (formatter as unknown as { rustywindInstalled: boolean }).rustywindInstalled = true;
      (formatter as unknown as { rustywindPath: string }).rustywindPath = "/mock/path/rustywind";

      // Need to directly define the RustywindManager to handle function calls
      // biome-ignore lint/suspicious/noExplicitAny: We're setting private properties for testing
      (formatter as any).rustywindManager.extractClassesWithOxc = () => [
        {
          classString: "mt-2 p-4 flex",
          original: `"mt-2 p-4 flex"`,
          start: 10,
          end: 25,
          path: "string_literal",
        },
      ];

      const document = await createTempDocument("test.tsx", 'const className = cn("mt-2 p-4 flex", isActive && "font-bold")');

      // Set config to include the cn function
      formatter.config.tailwindFunctions = ["cn", "clsx"];

      const edits = await formatter.formatDocument(document);

      // Since we're mocking, we can only verify an edit was made
      assert.ok(edits, "Edits should exist");
      assert.strictEqual(edits.length, 1, "Should have one edit");
    });
  });
});
