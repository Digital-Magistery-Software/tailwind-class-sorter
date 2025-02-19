import type { ExecOptions } from "node:child_process";

export interface TailwindSorterConfig {
  enable: boolean;
  includeFiles: string[];
  languageIds: string[];
  customBinaryPath?: string;
  debug: boolean;
}

export interface TempFileOptions {
  prefix?: string;
  extension?: string;
  deleteAfter?: boolean;
}

export interface DiagnosticResult {
  fileSupported: boolean;
  hasTailwindClasses: boolean;
  rustywindPath: string | null;
  wouldFormat: boolean;
}

export interface ExecResult {
  stdout: string;
  stderr: string;
}

export type ExecFunction = (command: string, options?: ExecOptions) => Promise<ExecResult>;
