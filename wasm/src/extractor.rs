use oxc::allocator::Allocator;
use oxc::ast::ast::StringLiteral;
use oxc::ast::visit::Visit;
use oxc::parser::Parser;
use oxc::span::SourceType;
use std::path::Path;

use crate::console_log;

/// Represents a class string match in the document
#[derive(Debug, Clone)]
pub struct ClassMatch {
    /// Starting position in the document
    pub start: usize,
    /// Ending position in the document
    pub end: usize,
    /// The original class string as it appears in the code
    pub original: String,
    /// The extracted class string (might differ from original in template literals)
    pub class_string: String,
    /// Path to the node in the AST (for debugging)
    pub path: String,
}

/// Extract Tailwind class strings from a document
pub fn extract_class_strings(document: &str, file_extension: &str) -> Vec<ClassMatch> {
    let allocator = Allocator::default();

    // Create a fake path for source type detection
    let fake_path = format!("test.{}", file_extension);
    let source_type = SourceType::from_path(Path::new(&fake_path)).unwrap_or_default();

    let parser = Parser::new(&allocator, document, source_type);
    let ret = parser.parse();

    if !ret.errors.is_empty() {
        console_log!("Parsing errors: {:?}", ret.errors);
        return vec![];
    }

    let mut visitor = TailwindClassVisitor {
        document,
        class_matches: Vec::new(),
    };

    visitor.visit_program(&ret.program);

    // Sort by position from end to start to avoid offset issues when replacing
    visitor.class_matches.sort_by(|a, b| b.start.cmp(&a.start));

    visitor.class_matches
}

struct TailwindClassVisitor<'a> {
    document: &'a str,
    class_matches: Vec<ClassMatch>,
}

impl<'a> Visit<'a> for TailwindClassVisitor<'a> {
    fn visit_string_literal(&mut self, string_lit: &StringLiteral<'a>) {
        let value = string_lit.value.as_str();

        if looks_like_class_string(value) {
            let start = string_lit.span.start as usize;
            let end = string_lit.span.end as usize;

            // Extract the actual text from the document to preserve quotes, etc.
            let original_text = if start < self.document.len() && end <= self.document.len() {
                self.document[start..end].to_string()
            } else {
                value.to_string()
            };

            // Add to our collection of matches
            self.class_matches.push(ClassMatch {
                start,
                end,
                original: original_text,
                class_string: value.to_string(),
                path: "string_literal".to_string(),
            });

            console_log!("Found potential Tailwind classes: {}", value);
        }
    }
}

/// Heuristic to determine if a string looks like Tailwind classes
fn looks_like_class_string(value: &str) -> bool {
    // TODO: Improve heuristic to better identify Tailwind classes (probably use a regex or list of Tailwind classes)

    // Must be non-empty and contain at least one space
    if value.trim().is_empty() || !value.contains(' ') {
        return false;
    }

    // Check for common Tailwind class patterns
    let has_tailwind_pattern = value.split_whitespace().any(|class| {
        class.starts_with("bg-")
            || class.starts_with("text-")
            || class.starts_with("p-")
            || class.starts_with("m-")
            || class.starts_with("px-")
            || class.starts_with("py-")
            || class.starts_with("mx-")
            || class.starts_with("my-")
            || class.starts_with("flex")
            || class.starts_with("grid")
            || class.starts_with("w-")
            || class.starts_with("h-")
            || class.starts_with("rounded")
            || class.starts_with("border")
            || class.starts_with("shadow")
            || class == "block"
            || class == "inline"
            || class == "inline-block"
            || class == "hidden"
    });

    // Avoid strings that look like HTML or JSX
    let looks_like_markup = value.contains('<') || value.contains('>');

    has_tailwind_pattern && !looks_like_markup
}
