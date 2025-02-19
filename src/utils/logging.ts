import * as vscode from "vscode";
import { getConfig } from "../config";
import type { TailwindSorterConfig } from "./types";

export class Logger {
  private outputChannel: vscode.OutputChannel;
  private config: TailwindSorterConfig;

  constructor(channelName: string) {
    this.outputChannel = vscode.window.createOutputChannel(channelName);
    this.config = getConfig();
  }

  public updateConfig(config: TailwindSorterConfig): void {
    this.config = config;
  }

  public showOutput(): void {
    this.outputChannel.show();
  }

  public log(message: string): void {
    this.outputChannel.appendLine(message);
  }

  public debugLog(message: string | object, ...args: unknown[]): void {
    if (!this.config.debug) {
      return;
    }

    const prefix = `[DEBUG - Digital Magistery Tailwind Sorter] ${new Date().toISOString()} -`;

    if (typeof message === "string") {
      if (args.length > 0) {
        const formattedArgs = args.map((arg) => (typeof arg === "object" ? JSON.stringify(arg, null, 2) : arg));
        this.outputChannel.appendLine(`${prefix} ${message} ${formattedArgs.join(" ")}`);
      } else {
        this.outputChannel.appendLine(`${prefix} ${message}`);
      }
    } else {
      this.outputChannel.appendLine(`${prefix} ${JSON.stringify(message, null, 2)}`);
    }
  }

  public dispose(): void {
    this.outputChannel.dispose();
  }
}
