import type { ExecOptions } from "node:child_process";

export interface InternalSorterConfig {
  enabled: boolean;
  debug: boolean;
  removeDuplicateClasses: boolean;
  normalizeWhitespace: boolean;
}

export interface TailwindSorterConfig {
  enable: boolean;
  includeFiles: string[];
  languageIds: string[];
  customBinaryPath?: string;
  debug: boolean;
  tailwindFunctions: string[];
  internalSorter: InternalSorterConfig;
}

export interface TempFileOptions {
  prefix?: string;
  extension?: string;
  deleteAfter?: boolean;
}

export interface DiagnosticResult {
  fileSupported: boolean;
  rustywindPath: string | null;
  wouldFormat: boolean;
}

export interface ExecResult {
  stdout: string;
  stderr: string;
}

export type ExecFunction = (command: string, options?: ExecOptions) => Promise<ExecResult>;
