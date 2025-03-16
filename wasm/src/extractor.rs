use oxc::allocator::Allocator;
use oxc::ast::ast::{StringLiteral, TemplateLiteral};
use oxc::ast::visit::Visit;
use oxc::parser::Parser;
use oxc::span::SourceType;
use std::path::Path;

use crate::console_log;
use crate::sorter::{is_arbitrary_class, is_tailwind_class};

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

/// Heuristic to determine if a string looks like Tailwind classes
fn looks_like_class_string(value: &str) -> bool {
    // Must be non-empty
    if value.trim().is_empty() {
        return false;
    }

    // Ignore strings that are just one word
    let parts: Vec<&str> = value.split_whitespace().collect();
    if parts.len() <= 1 {
        return false;
    }

    // Count how many words look like Tailwind classes
    let tailwind_class_count = parts
        .iter()
        .filter(|&&class| is_tailwind_class(class) || is_arbitrary_class(class))
        .count();

    // Require at least two words that look like Tailwind classes
    // and at least 40% of the words should look like Tailwind classes
    let min_tailwind_count = 2;
    let min_tailwind_percentage = 0.4;
    let has_enough_tailwind_classes = tailwind_class_count >= min_tailwind_count
        && (tailwind_class_count as f64 / parts.len() as f64) >= min_tailwind_percentage;

    // Reject patterns that are very unlikely to be Tailwind class lists
    let looks_like_markup = value.contains('<') || value.contains('>');
    let looks_like_assignment = value.contains('=') && !value.contains('[') && !value.contains('(');

    let looks_like_url = value.contains("://")
        || (value.matches('.').count() > 1
            && value.contains('/')
            && !value.split_whitespace().any(|w| w.contains('-')));

    let looks_like_json = value.starts_with('{') || value.starts_with('[');

    let looks_like_path = value.contains('/')
        && !value.contains(" / ")
        && !value.split_whitespace().any(|word| {
            // Check for Tailwind fraction pattern (e.g., w-1/2, h-3/4)
            word.matches('/').count() == 1
                && word.split('/').all(|part| !part.is_empty())
                && (word.contains('-') || word.contains(':'))
        });

    let looks_like_template = value.contains("${");

    let has_javascript_operators = value.contains("&&")
        || value.contains("||")
        || value.contains(" * ")
        || value.contains(" + ")
        || value.contains(" +")
        || value.contains("+ ");

    let has_semicolon = value.contains(';');

    has_enough_tailwind_classes
        && !looks_like_markup
        && !looks_like_assignment
        && !looks_like_url
        && !looks_like_json
        && !looks_like_path
        && !looks_like_template
        && !has_javascript_operators
        && !has_semicolon
}
