import * as fs from "node:fs";
import * as os from "node:os";
import * as path from "node:path";
import type { Logger } from "./logging";
import type { TempFileOptions } from "./types";

export async function withTempFile<T>(
  content: string,
  callback: (filePath: string) => Promise<T>,
  options: TempFileOptions = {},
  logger?: Logger
): Promise<T> {
  const { prefix = "tailwind-sorter-", extension = ".txt", deleteAfter = true } = options;

  const tmpFileName = `${prefix}${Date.now()}${extension}`;
  const tmpFilePath = path.join(os.tmpdir(), tmpFileName);

  try {
    await fs.promises.writeFile(tmpFilePath, content, "utf8");
    return await callback(tmpFilePath);
  } finally {
    if (deleteAfter) {
      try {
        await fs.promises.unlink(tmpFilePath);
      } catch (error) {
        logger?.debugLog(`Failed to delete temp file ${tmpFilePath}:`, error);
      }
    }
  }
}
