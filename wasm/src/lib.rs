use wasm_bindgen::prelude::*;

pub mod extractor;
pub mod sorter;
pub mod utils;

use extractor::extract_class_strings;
use sorter::{is_debug_enabled, set_debug_mode, set_remove_duplicates, sort_classes};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => {
        $crate::log(&format!($($t)*))
    }
}

#[macro_export]
macro_rules! debug_log {
    ($($t:tt)*) => {
        if is_debug_enabled() {
            $crate::log(&format!("[DEBUG] {}", format!($($t)*)))
        }
    }
}

#[wasm_bindgen]
pub fn configure_tailwind_sorter(remove_duplicates: bool, debug_mode: bool) {
    console_log!(
        "Configuring Tailwind sorter with remove_duplicates={}, debug_mode={}",
        remove_duplicates,
        debug_mode
    );
    set_remove_duplicates(remove_duplicates);
    set_debug_mode(debug_mode);
}

#[wasm_bindgen]
pub fn sort_tailwind_classes(document: &str, file_extension: &str) -> String {
    console_log!("Starting Tailwind class sorting for {}", file_extension);

    let class_matches = extract_class_strings(document, file_extension);

    // If no matches found, return the original document
    if class_matches.is_empty() {
        console_log!("No Tailwind classes found to sort");
        return document.to_string();
    }

    debug_log!("Found {} class matches to process", class_matches.len());

    let mut result = document.to_string();
    let mut replaced_count = 0;

    // Replace each class string with its sorted version
    // Process from end to start to avoid offset issues
    for class_match in &class_matches {
        let sorted_classes = sort_classes(&class_match.class_string);

        // Only replace if the order changed
        if sorted_classes != class_match.class_string {
            debug_log!("Original classes: \"{}\"", class_match.class_string);
            debug_log!("Sorted classes:   \"{}\"", sorted_classes);

            let (starts_with_quote, ends_with_quote) = if (class_match.original.starts_with('"')
                && class_match.original.ends_with('"'))
                || (class_match.original.starts_with('\'') && class_match.original.ends_with('\''))
                || (class_match.original.starts_with('`') && class_match.original.ends_with('`'))
            {
                (true, true)
            } else {
                (false, false)
            };

            let replacement = if starts_with_quote && ends_with_quote {
                // Replace the content between quotes
                let quote_char = class_match.original.chars().next().unwrap();
                format!("{}{}{}", quote_char, &sorted_classes, quote_char)
            } else {
                // No quotes, just replace the whole thing
                sorted_classes.clone()
            };

            // Double-check the replacement won't exceed boundaries
            let start = class_match.start;
            let end = class_match.end;

            if start < result.len() && end <= result.len() {
                // Replace the original with the sorted version
                result.replace_range(start..end, &replacement);
                replaced_count += 1;

                debug_log!(
                    "Sorted: \"{}\" → \"{}\"",
                    class_match.class_string,
                    sorted_classes
                );
            } else {
                console_log!(
                    "Warning: Could not replace class string at positions {}-{} (out of bounds)",
                    start,
                    end
                );
            }
        } else {
            debug_log!(
                "Classes already in correct order: \"{}\"",
                class_match.class_string
            );
        }
    }

    console_log!(
        "Completed sorting: {} class strings modified",
        replaced_count
    );

    result
}
