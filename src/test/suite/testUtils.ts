import * as fs from "node:fs";
import * as os from "node:os";
import * as path from "node:path";
import * as vscode from "vscode";
import type { ExecFunction } from "../../utils/types";

export async function createTempDocument(fileName: string, content: string): Promise<vscode.TextDocument> {
  const tmpFile = path.join(os.tmpdir(), `test-${Date.now()}-${fileName}`);
  await fs.promises.writeFile(tmpFile, content, "utf8");
  const uri = vscode.Uri.file(tmpFile);
  const document = await vscode.workspace.openTextDocument(uri);

  setTimeout(async () => {
    try {
      await fs.promises.unlink(tmpFile);
    } catch (error) {
      console.error("Failed to clean up temp file:", error);
    }
  }, 100);

  return document;
}

// This mock simulates rustywind output by always returning a specific output format
export const mockExecSuccess: ExecFunction = (command: string) => {
  if (command.includes("--stdin")) {
    // Extract the actual class string from the command
    const match = command.match(/echo ".*className=["'`]([^"'`]+)["'`]/);
    const classString = match?.[1] || "";

    // Simplified sorting algorithm to simulate rustywind for test purposes
    // This isn't the actual sorting algorithm, just a simplified version for testing
    const sorted = mockSortForTesting(classString);

    return Promise.resolve({
      stdout: `<div className="${sorted}"></div>`,
      stderr: "",
    });
  }

  if (command.includes("--dry-run")) {
    return Promise.resolve({
      stdout: "Would sort classes",
      stderr: "",
    });
  }

  return Promise.resolve({
    stdout: "flex items-center justify-between bg-white p-4 mt-2",
    stderr: "",
  });
};

// A simple sorting function to simulate rustywind behavior for tests
function mockSortForTesting(classString: string): string {
  // Split classes and sort them according to a simplified priority
  const classes = classString.split(/\s+/).filter(Boolean);

  // This is a very simplified mock sorting that moves flex-related classes first,
  // followed by position, spacing, and appearance
  const flexClasses = classes.filter((c) => c.startsWith("flex") || c === "flex" || c.includes("items-") || c.includes("justify-"));
  const bgClasses = classes.filter((c) => c.startsWith("bg-"));
  const marginClasses = classes.filter(
    (c) =>
      c.startsWith("m-") ||
      c.startsWith("mx-") ||
      c.startsWith("my-") ||
      c.startsWith("mt-") ||
      c.startsWith("mb-") ||
      c.startsWith("ml-") ||
      c.startsWith("mr-")
  );
  const paddingClasses = classes.filter(
    (c) =>
      c.startsWith("p-") ||
      c.startsWith("px-") ||
      c.startsWith("py-") ||
      c.startsWith("pt-") ||
      c.startsWith("pb-") ||
      c.startsWith("pl-") ||
      c.startsWith("pr-")
  );
  const textClasses = classes.filter((c) => c.startsWith("text-") || c.startsWith("font-"));
  const otherClasses = classes.filter(
    (c) => !(flexClasses.includes(c) || bgClasses.includes(c) || marginClasses.includes(c) || paddingClasses.includes(c) || textClasses.includes(c))
  );

  // Combine the sorted categories
  return [...flexClasses, ...bgClasses, ...paddingClasses, ...marginClasses, ...textClasses, ...otherClasses].join(" ");
}

export const mockExecFail: ExecFunction = async () => ({
  stdout: "",
  stderr: "Error: Something went wrong",
});

export const mockFindBinarySuccess = async () => "/mock/path/rustywind";
export const mockFindBinaryFail = async () => null;
