# Tailwind Class Sorter (VS Code Extension)

A VS Code extension that automatically sorts Tailwind CSS classes using [RustyWind](https://github.com/avencera/rustywind).

[View Changelog](CHANGELOG.md)

## Features

- Automatically sorts Tailwind CSS classes on save
- Works with JavaScript, TypeScript, JSX, TSX, and HTML files
- Uses RustyWind for reliable and consistent class sorting
- Debug output for troubleshooting
- Test command to check if sorting would occur
- Configurable file and language support

## Tailwind Function Support

The extension can detect and sort Tailwind classes within specified function calls. By default, it supports:

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

## Requirements

🚨 **RustyWind is required but NOT included in this extension.** This extension uses [RustyWind](https://github.com/avencera/rustywind) to sort Tailwind CSS classes. You must install it separately.

### **Install RustyWind locally (Recommended)**

```sh
yarn add rustywind --dev
```

### **Install RustyWind globally (For older Yarn versions)**

If you are using an older version of Yarn that supports global installs, you can install RustyWind globally:

```sh
yarn add -g rustywind
```

## Extension Settings

This extension contributes the following settings:

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
},

"tailwindSorter.customBinaryPath": {
  "type": "string",
  "default": "",
  "description": "Custom path to RustyWind binary. Supports workspace variables like ${workspaceFolder}."
},

"tailwindSorter.debug": {
  "type": "boolean",
  "default": false,
  "description": "Enable debug logging in the output channel"
}

"tailwindSorter.tailwindFunctions": {
  "type": "array",
  "default": ["cn", "cva", "clsx"],
  "description": "List of function names that contain Tailwind classes as arguments"
}
```

## Commands

The extension provides the following commands:

- **Tailwind Sorter: Show Output** - Opens the output channel for debugging.
- **Tailwind Sorter: Test Formatter** - Tests if the current file would be formatted.

## RustyWind Pre-Check

If RustyWind is not installed, the extension will display an error message notifying you that it is required. Ensure that you have installed RustyWind either globally or locally in your project for this extension to function correctly.

## Development

### **Clone the repository:**

```sh
git clone https://github.com/Digital-Magistery-Software/tailwind-class-sorter.git
```

### **Install dependencies:**

```sh
yarn install
```

### **Open in VS Code & Debug:**

1. Open the project in VS Code.
2. Press `F5` to start debugging the extension.

## License

This project is licensed under the MIT License.
