use oxc::allocator::Allocator;
use oxc::ast::ast::{StringLiteral, TemplateLiteral};
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

    console_log!(
        "Found {} Tailwind class matches",
        visitor.class_matches.len()
    );

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

            console_log!("Found Tailwind classes in string literal: {}", value);
        }
    }

    fn visit_template_literal(&mut self, template_lit: &TemplateLiteral<'a>) {
        // Process each static part (quasi) of the template literal
        for (i, quasi) in template_lit.quasis.iter().enumerate() {
            // Only process if the cooked value is available
            if let Some(cooked) = &quasi.value.cooked {
                let value = cooked.as_str();

                if looks_like_class_string(value) {
                    let start = quasi.span.start as usize;
                    let end = quasi.span.end as usize;

                    // Extract the actual text from the document
                    let original_text = if start < self.document.len() && end <= self.document.len()
                    {
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
                        path: format!("template_literal_quasi[{}]", i),
                    });

                    console_log!("Found Tailwind classes in template literal: {}", value);
                }
            }
        }

        // Explicitly visit each expression in the template literal
        // This is so we find string literals inside template expressions
        for expr in &template_lit.expressions {
            self.visit_expression(expr);
        }
    }
}

static TAILWIND_PATTERN: OnceLock<Regex> = OnceLock::new();

/// Heuristic to determine if a string looks like Tailwind classes
fn looks_like_class_string(value: &str) -> bool {
    // Must be non-empty
    if value.trim().is_empty() {
        return false;
    }

    // Ignore strings that are just one word since they obviously don't need sorted
    let parts: Vec<&str> = value.split_whitespace().collect();
    if parts.len() <= 1 {
        return false;
    }

    let has_tailwind_pattern = parts
        .iter()
        .any(|&class| get_tailwind_pattern().is_match(class));

    // Avoid strings that look like HTML or JSX
    let looks_like_markup = value.contains('<') || value.contains('>');

    has_tailwind_pattern && !looks_like_markup
}

fn get_tailwind_pattern() -> &'static Regex {
    TAILWIND_PATTERN.get_or_init(|| {
        Regex::new(r"^(bg-|text-|font-|p-|m-|px-|py-|pt-|pb-|pr-|pl-|mx-|my-|mt-|mb-|mr-|ml-|w-|h-|min-w-|min-h-|max-w-|max-h-|rounded|border|shadow|outline-|ring-|flex-|grid-|gap-|space-|divide-|opacity-|z-|top-|bottom-|left-|right-|inset-|align-|justify-|items-|content-|self-|order-|transform|rotate-|scale-|skew-|translate-|transition-|duration-|ease-|delay-|animate-|cursor-|overflow-|overscroll-|scroll-|whitespace-|break-|object-|float-|clear-|fill-|stroke-|sr-|appearance-|isolation-|backdrop-|pointer-events-|resize-|select-|snap-|touch-|table-|list-|line-|placeholder-|tracking-|leading-|contrast-|grayscale-|hue-rotate-|invert-|saturate-|sepia-|brightness-|blur-|sm:|md:|lg:|xl:|2xl:|hover:|focus:|active:|disabled:|focus-visible:|focus-within:|visited:|checked:|dark:|odd:|even:|first:|last:|only:|motion-safe:|motion-reduce:|portrait:|landscape:|focus:|hover:)").unwrap()
    })
}
