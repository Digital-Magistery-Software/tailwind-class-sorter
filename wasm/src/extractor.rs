use oxc::allocator::Allocator;
use oxc::ast::ast::StringLiteral;
use oxc::ast::visit::Visit;
use oxc::parser::Parser;
use oxc::span::SourceType;
use regex::Regex;
use std::path::Path;
use std::sync::OnceLock;

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

static TAILWIND_PATTERN: OnceLock<Regex> = OnceLock::new();

/// Heuristic to determine if a string looks like Tailwind classes
fn looks_like_class_string(value: &str) -> bool {
    // Must be non-empty and contain at least one space
    if value.trim().is_empty() || !value.contains(' ') {
        return false;
    }

    let tailwind_pattern = TAILWIND_PATTERN.get_or_init(|| {
        Regex::new(r"^(bg-|text-|p-|m-|px-|py-|mx-|my-|pt-|pr-|pb-|pl-|mt-|mr-|mb-|ml-|flex|grid|w-|h-|min-w-|min-h-|max-w-|max-h-|rounded|border|shadow|block|inline|inline-block|hidden|sm:|md:|lg:|xl:|2xl:|hover:|focus:|active:|disabled:|dark:|motion-safe:|motion-reduce:)").unwrap()
    });

    let has_tailwind_pattern = value
        .split_whitespace()
        .any(|class| tailwind_pattern.is_match(class));

    // Avoid strings that look like HTML or JSX
    let looks_like_markup = value.contains('<') || value.contains('>');

    has_tailwind_pattern && !looks_like_markup
}
