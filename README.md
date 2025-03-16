# Tailwind Class Sorter (VS Code Extension)

A VS Code extension that automatically sorts Tailwind CSS classes using [RustyWind](https://github.com/avencera/rustywind) or the built-in experimental internal sorter.

[View Changelog](CHANGELOG.md)

## Features

- Automatically sorts Tailwind CSS classes on save
- Works with JavaScript, TypeScript, JSX, and TSX files (HTML support coming soon)
- Choice between RustyWind or built-in WebAssembly sorter
- Debug output for troubleshooting
- Test command to check if sorting would occur
- Configurable file and language support

## Tailwind Function Support (RustyWind Mode)

When using the default RustyWind sorter, the extension can detect and sort Tailwind classes within specified function calls. By default, it supports:

- `cn` (for class-variance-authority + tailwind-merge pattern)
- `cva` (class-variance-authority)
- `clsx`

You can add your own function names through the `tailwindSorter.tailwindFunctions` setting.

Example usage with the `cn` function:

```tsx
className={cn(
  "flex items-center text-nowrap",
  isActive && "font-bold",
  isPending && "pointer-events-none"
)}
```

## Enhanced Class Detection

Version 1.2.0 introduces AST-based class detection for more reliable and comprehensive Tailwind class sorting.
The extension now properly handles:

- Template literals: `` className={`px-4 ${condition ? "pt-2" : "pb-2"}`} ``
- Function calls with multiple arguments: `className={cn("px-4", condition && "pt-2")}`
- Conditional expressions: `className={condition ? "px-4" : "py-2"}`
- Object expressions: `className={cn({ "px-4": isActive })}`
- Tailwind v4 custom property syntax: `border-(--custom-color)`

This provides much more reliable sorting across complex component structures.

## Experimental Internal Sorter (New in 1.3.0)

Version 1.3.0 introduces an experimental built-in Rust-based WebAssembly sorter as an alternative to using RustyWind. The internal sorter provides class sorting functionality without requiring any external dependencies.

### Benefits of the Internal Sorter:

- **No external dependencies**: No need to install RustyWind separately
- **Built-in sorting**: Works right out of the box
- **Configurable duplicate handling**: Option to remove or preserve duplicate classes
- **Whitespace control**: Option to normalize or preserve whitespace patterns
- **WebAssembly performance**: Fast, reliable sorting through compiled Rust
- **Full support** for Tailwind v4 custom property syntax

### Limitations:

- **Experimental**: This feature is new and may have issues not present in RustyWind
- **Different detection mechanism**: Doesn't use the `tailwindFunctions` setting; instead identifies potential Tailwind classes based on patterns

To enable the internal sorter:

1. Open VS Code settings
2. Search for "tailwindSorter.internalSorter.enabled"
3. Set it to `true`

The status bar indicator will display "Tailwind Sorter (Internal)" when the internal sorter is active.

## Requirements

The extension can now function in two modes:

### Using RustyWind (Default)

🚨 **RustyWind is required but NOT included when using the default sorter.** If you're using the default sorting mode, you must install [RustyWind](https://github.com/avencera/rustywind) separately.

#### **Install RustyWind locally (Recommended)**

```sh
yarn add rustywind --dev
```

#### **Install RustyWind globally (For older Yarn versions)**

```sh
yarn add -g rustywind
```

### Using Internal Sorter (Experimental)

When the internal sorter is enabled, there are no external dependencies required. Simply enable it in the settings to start using it immediately.

## Extension Settings

This extension contributes the following settings:

### General Settings (Apply to Both Sorters)

```json
"tailwindSorter.enable": {
  "type": "boolean",
  "default": true,
  "description": "Enable/disable Tailwind class sorting"
},

"tailwindSorter.includeFiles": {
  "type": "array",
  "default": [
    "**/*.{js,jsx,ts,tsx,html}"
  ],
  "description": "Files to include for class sorting"
},

"tailwindSorter.languageIds": {
  "type": "array",
  "default": ["typescript", "typescriptreact", "javascript", "javascriptreact", "html"],
  "description": "Language IDs for which this extension should be active"
}
```

### RustyWind-Specific Settings (Only Apply When Using RustyWind)

```json
"tailwindSorter.customBinaryPath": {
  "type": "string",
  "default": "",
  "description": "Custom path to RustyWind binary. Supports workspace variables like ${workspaceFolder}."
},

"tailwindSorter.debug": {
  "type": "boolean",
  "default": false,
  "description": "Enable debug logging in the output channel"
},

"tailwindSorter.tailwindFunctions": {
  "type": "array",
  "default": ["cn", "cva", "clsx"],
  "description": "List of function names that contain Tailwind classes as arguments"
}
```

### Internal Sorter Settings (Only Apply When Using Internal Sorter)

```json
"tailwindSorter.internalSorter.enabled": {
  "type": "boolean",
  "default": false,
  "description": "Use the experimental internal Tailwind class sorter instead of Rustywind"
},

"tailwindSorter.internalSorter.debug": {
  "type": "boolean",
  "default": false,
  "description": "Enable additional debug logging for the internal sorter"
},

"tailwindSorter.internalSorter.removeDuplicateClasses": {
  "type": "boolean",
  "default": true,
  "description": "Remove duplicate Tailwind classes, keeping only the last occurrence (only applies when using internal sorter)"
},

"tailwindSorter.internalSorter.normalizeWhitespace": {
  "type": "boolean",
  "default": true,
  "description": "Normalize whitespace between classes (only applies when using internal sorter)"
}
```

## Commands

The extension provides the following commands:

- **Tailwind Sorter: Show Output** - Opens the output channel for debugging.
- **Tailwind Sorter: Test Formatter** - Tests if the current file would be formatted.

## RustyWind Pre-Check

If using the default RustyWind sorter and RustyWind is not installed, the extension will display an error message notifying you that it is required. Ensure that you have installed RustyWind either globally or locally in your project for this mode to function correctly.

## Development

### **Prerequisites:**

- Node.js and yarn
- Rust and Cargo
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) for WebAssembly compilation

### **Setup and build:**

```sh
# Clone the repository
git clone https://github.com/Digital-Magistery-Software/tailwind-class-sorter.git

# Install dependencies
yarn install

# Build the extension (WASM + TypeScript)
yarn build
```

The `yarn build` command will:

1. Run the `build:wasm` script which uses wasm-pack to compile the Rust code to WebAssembly
2. Move the compiled WASM files to the correct location in the out directory
3. Compile the TypeScript code

### **Development workflow:**

1. Make changes to the Rust code in the `wasm` directory
2. Run `yarn build:wasm` to rebuild the WebAssembly component
3. Make changes to the TypeScript code
4. Run `yarn compile` to rebuild the TypeScript code (or use `yarn watch` for continuous compilation)
5. Press `F5` in VS Code to launch the extension in debug mode

## License

This project is licensed under the MIT License.
