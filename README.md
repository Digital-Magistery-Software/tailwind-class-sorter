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
