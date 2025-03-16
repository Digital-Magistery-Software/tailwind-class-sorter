import * as assert from "node:assert";
import { TailwindSorterFormatter } from "../../formatter";
import { Logger } from "../../utils/logging";
import { createTempDocument } from "./testUtils";

// Use real implementations - no mocks for the internal sorter tests
// biome-ignore lint/suspicious/useAwait: We don't need to await these functions in tests
const execAsync = async (_command: string) => {
  throw new Error("Should not call exec in internal sorter tests");
};

const findBinary = async () => null; // We don't need Rustywind for these tests

suite("TailwindSorterFormatter - Internal Sorter", function () {
  let formatter: TailwindSorterFormatter;
  let logger: Logger;

  this.beforeEach(() => {
    logger = new Logger("Test Logger");
    formatter = new TailwindSorterFormatter(execAsync, findBinary, logger);

    formatter.config.internalSorter = {
      enabled: true,
      debug: false,
      removeDuplicateClasses: true,
      normalizeWhitespace: true,
    };
  });

  this.afterEach(() => {
    formatter.dispose();
    logger.dispose();
  });

  suite("Duplicate Class Handling", () => {
    test("Should remove exact duplicates when configured", async () => {
      formatter.config.internalSorter.removeDuplicateClasses = true;

      await formatter.initialize();

      const document = await createTempDocument("test.tsx", '<div className="p-4 flex p-4 items-center bg-white p-4 justify-between">Test</div>');

      const edits = await formatter.formatDocument(document);

      assert.ok(edits, "Edits should exist");
      assert.strictEqual(edits.length, 1, "Should have one edit");

      // Check that each class only appears once in the output
      const matches = edits[0].newText.match(/p-4/g);
      assert.strictEqual(matches?.length, 1, "Should only have one p-4 occurrence");

      // Check that the other classes are preserved
      assert.ok(edits[0].newText.includes("flex"), "Should contain flex");
      assert.ok(edits[0].newText.includes("items-center"), "Should contain items-center");
      assert.ok(edits[0].newText.includes("bg-white"), "Should contain bg-white");
      assert.ok(edits[0].newText.includes("justify-between"), "Should contain justify-between");
    });

    test("Should preserve duplicates when configured", async () => {
      formatter.config.internalSorter.removeDuplicateClasses = false;

      await formatter.initialize();

      const document = await createTempDocument("test.tsx", '<div className="p-4 flex p-4 items-center bg-white p-4 justify-between">Test</div>');

      const edits = await formatter.formatDocument(document);

      assert.ok(edits, "Edits should exist");
      assert.strictEqual(edits.length, 1, "Should have one edit");

      // Check that p-4 appears multiple times in the output
      const matches = edits[0].newText.match(/p-4/g);
      assert.strictEqual(matches?.length, 3, "Should have three p-4 occurrences");
    });

    test("Should handle variant-specific duplicates correctly", async () => {
      formatter.config.internalSorter.removeDuplicateClasses = true;

      await formatter.initialize();

      const document = await createTempDocument("test.tsx", '<div className="p-4 p-4 sm:p-4 sm:p-4 md:p-6 md:p-6">Test</div>');

      const edits = await formatter.formatDocument(document);

      assert.ok(edits, "Edits should exist");
      assert.strictEqual(edits.length, 1, "Should have one edit");

      // Each variant should appear only once
      const p4Matches = edits[0].newText.match(/(?<![sm]:)p-4/g); // p-4 not preceded by sm:
      const smP4Matches = edits[0].newText.match(/sm:p-4/g);
      const mdP6Matches = edits[0].newText.match(/md:p-6/g);

      assert.strictEqual(p4Matches?.length, 1, "Should only have one p-4 occurrence");
      assert.strictEqual(smP4Matches?.length, 1, "Should only have one sm:p-4 occurrence");
      assert.strictEqual(mdP6Matches?.length, 1, "Should only have one md:p-6 occurrence");
    });
  });

  suite("Whitespace Handling", () => {
    test("Should normalize whitespace when configured", async () => {
      formatter.config.internalSorter.normalizeWhitespace = true;

      await formatter.initialize();

      const document = await createTempDocument("test.tsx", '<div className="  p-4    flex   items-center  ">Test</div>');

      const edits = await formatter.formatDocument(document);

      assert.ok(edits, "Edits should exist");
      assert.strictEqual(edits.length, 1, "Should have one edit");

      // Normalized whitespace = single spaces between classes, no leading/trailing spaces
      const expected = 'className="flex items-center p-4"';
      assert.ok(edits[0].newText.includes(expected), `Whitespace should be normalized to: ${expected}\nBut got: ${edits[0].newText}`);
    });

    test("Should preserve whitespace pattern when configured", async () => {
      formatter.config.internalSorter.normalizeWhitespace = false;

      await formatter.initialize();

      const document = await createTempDocument("test.tsx", '<div className="  p-4    flex   items-center  ">Test</div>');

      const edits = await formatter.formatDocument(document);

      assert.ok(edits, "Edits should exist");
      assert.strictEqual(edits.length, 1, "Should have one edit");

      const expected = 'className="  flex    items-center   p-4  "';

      assert.ok(edits[0].newText.includes(expected), "Whitespace and class structure should match the expected output");
    });
  });

  suite("Sorting Behavior", () => {
    test("Should sort classes in the correct order", async () => {
      await formatter.initialize();

      const document = await createTempDocument(
        "test.tsx",
        '<div className="text-white px-4 sm:px-8 py-2 sm:py-3 bg-sky-700 hover:bg-sky-800">Test</div>'
      );

      const edits = await formatter.formatDocument(document);

      assert.ok(edits, "Edits should exist");
      assert.strictEqual(edits.length, 1, "Should have one edit");

      // Check for correctly sorted classes
      const expected = 'className="bg-sky-700 px-4 py-2 text-white hover:bg-sky-800 sm:px-8 sm:py-3"';
      assert.ok(edits[0].newText.includes(expected), `Expected formatted output to contain: ${expected}\nBut got: ${edits[0].newText}`);
    });

    test("Should handle arbitrary values correctly", async () => {
      await formatter.initialize();

      const document = await createTempDocument("test.tsx", '<div className="bg-[#123456] text-[16px] p-[10px] m-[5px]">Test</div>');

      const edits = await formatter.formatDocument(document);

      assert.ok(edits, "Edits should exist");
      assert.strictEqual(edits.length, 1, "Should have one edit");

      // Check for correctly sorted arbitrary values
      const expected = 'className="m-[5px] bg-[#123456] p-[10px] text-[16px]"';
      assert.ok(edits[0].newText.includes(expected), `Expected formatted output to contain: ${expected}\nBut got: ${edits[0].newText}`);
    });

    test("Should handle Tailwind v4 custom property syntax", async () => {
      await formatter.initialize();

      const document = await createTempDocument("test.tsx", '<div className="bg-(--color) text-(--text) p-4">Test</div>');

      const edits = await formatter.formatDocument(document);

      assert.ok(edits, "Edits should exist");
      assert.strictEqual(edits.length, 1, "Should have one edit");

      // Check for correctly sorted custom properties
      const expected = 'className="bg-(--color) p-4 text-(--text)"';
      assert.ok(edits[0].newText.includes(expected), `Expected formatted output to contain: ${expected}\nBut got: ${edits[0].newText}`);
    });
  });

  suite("Edge Cases", () => {
    test("Should handle empty class strings", async () => {
      await formatter.initialize();

      const document = await createTempDocument("test.tsx", '<div className="">Test</div>');

      const edits = await formatter.formatDocument(document);

      // Should not make changes to empty class strings
      assert.strictEqual(edits, undefined, "Should not format empty class strings");
    });

    test("Should handle classes with ellipsis", async () => {
      await formatter.initialize();

      const document = await createTempDocument("test.tsx", '<div className="p-4 ... bg-white">Test</div>');

      const edits = await formatter.formatDocument(document);

      assert.ok(edits, "Edits should exist");
      assert.strictEqual(edits.length, 1, "Should have one edit");

      // Check for correctly sorted classes with ellipsis at the end
      const expected = 'className="bg-white p-4 ..."';
      assert.ok(edits[0].newText.includes(expected), `Expected formatted output to contain: ${expected}\nBut got: ${edits[0].newText}`);
    });
  });
});
