# Change Log

## [1.0.0] - 2025-02-19

### Added

- Initial release
- Automatic Tailwind CSS class sorting using RustyWind
- Support for JavaScript, TypeScript, JSX, TSX, and HTML files
- Format on save functionality
- Debug output and test command
- Configurable file and language support

## [1.1.0] - 2025-02-20

### Added

- Support for Tailwind classes within function calls (cn, cva, clsx)
- Configurable list of function names via `tailwindFunctions` setting

## [1.2.0] - 2025-03-01

### Added

- AST-based Tailwind class extraction using Oxc parser
- Support for complex class usage patterns:
  - Template literals
  - Function calls with multiple arguments
  - Conditional expressions
  - Object properties
- Proper handling of Tailwind v4 custom property syntax

### Changed

- Replaced regex-based extraction with more accurate AST parsing
