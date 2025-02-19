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

export const mockExecSuccess: ExecFunction = (command: string) => {
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

export const mockExecFail: ExecFunction = async () => ({
  stdout: "",
  stderr: "Error: Something went wrong",
});

export const mockFindBinarySuccess = async () => "/mock/path/rustywind";
export const mockFindBinaryFail = async () => null;
