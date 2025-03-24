use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::sync::OnceLock;

use crate::debug_log;
use crate::prefixes::{TailwindPrefix, ValueType, find_order, get_tailwind_prefixes};

static REMOVE_DUPLICATES: OnceLock<Mutex<bool>> = OnceLock::new();
static DEBUG_MODE: OnceLock<Mutex<bool>> = OnceLock::new();
static NORMALIZE_WHITESPACE: OnceLock<Mutex<bool>> = OnceLock::new();

#[derive(Debug)]
enum TemplateChunk {
    Text(String),
    Expression(String),
}

pub fn set_remove_duplicates(remove: bool) {
    let mutex = REMOVE_DUPLICATES.get_or_init(|| Mutex::new(true));
    if let Ok(mut value) = mutex.lock() {
        *value = remove;
    }
}

pub fn set_debug_mode(debug: bool) {
    let mutex = DEBUG_MODE.get_or_init(|| Mutex::new(false));
    if let Ok(mut value) = mutex.lock() {
        *value = debug;
    }
}

pub fn set_normalize_whitespace(normalize: bool) {
    let mutex = NORMALIZE_WHITESPACE.get_or_init(|| Mutex::new(true));
    if let Ok(mut value) = mutex.lock() {
        *value = normalize;
    }
}

pub fn is_debug_enabled() -> bool {
    let mutex = DEBUG_MODE.get_or_init(|| Mutex::new(false));
    mutex.lock().map(|guard| *guard).unwrap_or(false)
}

fn is_normalize_whitespace_enabled() -> bool {
    let mutex = NORMALIZE_WHITESPACE.get_or_init(|| Mutex::new(true));
    mutex.lock().map(|guard| *guard).unwrap_or(true)
}

fn is_remove_duplicates_enabled() -> bool {
    let mutex = REMOVE_DUPLICATES.get_or_init(|| Mutex::new(true));
    mutex.lock().map(|guard| *guard).unwrap_or(true)
}

/// Check if a class contains arbitrary values with brackets or custom properties with parentheses
pub fn is_arbitrary_class(class: &str) -> bool {
    // Check for either bracket syntax (arbitrary values) or parentheses syntax (custom properties in v4)
    (class.contains('[') && class.contains(']')) || (class.contains('(') && class.contains(')'))
}

/// Check if a class is a variant (has a colon but is not an arbitrary property with a colon)
pub fn is_variant(class: &str) -> bool {
    // First, check if it's an arbitrary property - in that case, the colon might be within brackets
    if is_arbitrary_class(class) {
        // For arbitrary properties, we need to check if the colon is outside brackets/parentheses
        let mut inside_brackets = false;
        let mut inside_parens = false;
        let mut has_variant_colon = false;

        for c in class.chars() {
            match c {
                '[' => inside_brackets = true,
                ']' => inside_brackets = false,
                '(' => inside_parens = true,
                ')' => inside_parens = false,
                ':' => {
                    if !inside_brackets && !inside_parens {
                        has_variant_colon = true;
                    }
                }
                _ => {}
            }
        }

        return has_variant_colon;
    }

    // For normal classes, just check if it contains a colon
    class.contains(':')
}

/// Extract the property/attribute part from an arbitrary class or custom property
pub fn extract_arbitrary_attribute(class: &str) -> &str {
    // Full arbitrary property like "[color:red]"
    if class.starts_with('[') && class.ends_with(']') {
        return &class[1..class.len() - 1];
    }

    // CSS custom property with Tailwind v4 parentheses syntax like "(--color)"
    if class.starts_with('(') && class.ends_with(')') {
        return &class[1..class.len() - 1];
    }

    // Prefixed arbitrary value like "bg-[#ff0000]"
    if let Some(bracket_start) = class.find('[') {
        if let Some(bracket_end) = class.rfind(']') {
            return &class[bracket_start + 1..bracket_end];
        }
    }

    // Prefixed CSS custom property with Tailwind v4 syntax like "bg-(--color)"
    if let Some(paren_start) = class.find('(') {
        if let Some(paren_end) = class.rfind(')') {
            return &class[paren_start + 1..paren_end];
        }
    }

    // Default fallback
    ""
}

/// Split a class string into individual classes, preserving brackets and parentheses
pub fn split_preserving_brackets(class_string: &str) -> Vec<&str> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut bracket_depth = 0;
    let mut paren_depth = 0;

    for (i, c) in class_string.char_indices() {
        match c {
            '[' => bracket_depth += 1,
            ']' => bracket_depth -= 1,
            '(' => paren_depth += 1,
            ')' => paren_depth -= 1,
            ' ' => {
                // Only split on spaces outside of brackets/parentheses
                if bracket_depth == 0 && paren_depth == 0 {
                    let substring = class_string[start..i].trim();
                    if !substring.is_empty() {
                        result.push(substring);
                    }
                    start = i + 1; // Skip the space
                }
            }
            _ => {}
        }
    }

    // Add the last class
    let final_class = class_string[start..].trim();
    if !final_class.is_empty() {
        result.push(final_class);
    }

    result
}

fn remove_duplicates_from_sorted<'a>(sorted_classes: &[&'a str]) -> (Vec<&'a str>, HashSet<usize>) {
    let mut seen_classes = HashSet::new();
    let mut result = Vec::new();
    let mut removed_indices = HashSet::new();

    for (i, &class) in sorted_classes.iter().enumerate() {
        if is_tailwind_class(class) {
            if seen_classes.contains(class) {
                // This is a duplicate, mark it for removal
                removed_indices.insert(i);
            } else {
                // First occurrence, keep it
                seen_classes.insert(class);
                result.push(class);
            }
        } else {
            // Always keep non-Tailwind classes
            result.push(class);
        }
    }

    (result, removed_indices)
}

fn sort_with_preserved_whitespace(
    class_string: &str,
    sorted_classes: &[&str],
    removed_indices: &HashSet<usize>,
) -> String {
    // Determine if the string starts with whitespace
    let starts_with_whitespace = class_string
        .chars()
        .next()
        .is_some_and(|c| c.is_whitespace());

    // Parse the original input into alternating whitespace and class tokens
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_whitespace = starts_with_whitespace; // Start based on the first character
    let mut bracket_depth = 0;
    let mut paren_depth = 0;

    for c in class_string.chars() {
        // Update bracket and parenthesis depth
        match c {
            '[' => bracket_depth += 1,
            ']' => {
                if bracket_depth > 0 {
                    bracket_depth -= 1
                }
            }
            '(' => paren_depth += 1,
            ')' => {
                if paren_depth > 0 {
                    paren_depth -= 1
                }
            }
            _ => {}
        }

        let is_whitespace = c.is_whitespace() && bracket_depth == 0 && paren_depth == 0;

        if is_whitespace != in_whitespace {
            // We're switching between whitespace and class content
            if !current.is_empty() {
                tokens.push((current, in_whitespace));
                current = String::new();
            }
            in_whitespace = is_whitespace;
        }

        current.push(c);
    }

    // Add the last token
    if !current.is_empty() {
        tokens.push((current, in_whitespace));
    }

    // Extract class and whitespace tokens
    let mut whitespace_tokens: Vec<String> = Vec::new();
    let mut original_classes: Vec<String> = Vec::new();

    for (text, is_whitespace) in tokens {
        if is_whitespace {
            whitespace_tokens.push(text);
        } else {
            original_classes.push(text);
        }
    }

    // Filter out whitespace associated with removed classes
    let filtered_whitespace: Vec<String> = if !removed_indices.is_empty() {
        whitespace_tokens
            .into_iter()
            .enumerate()
            .filter(|(i, _)| {
                // Which class is after this whitespace?
                let class_index = if starts_with_whitespace {
                    *i // If string starts with whitespace, whitespace[i] is before class[i]
                } else {
                    *i + 1 // Otherwise, whitespace[i] is before class[i+1]
                };
                !removed_indices.contains(&class_index)
            })
            .map(|(_, ws)| ws)
            .collect()
    } else {
        whitespace_tokens
    };

    // Rebuild the string
    let mut output = String::new();

    // If the string starts with whitespace, add that first
    if starts_with_whitespace && !filtered_whitespace.is_empty() {
        output.push_str(&filtered_whitespace[0]);
    }

    // Add each class with its following whitespace
    for (i, class) in sorted_classes.iter().enumerate() {
        // Add the class
        output.push_str(class);

        // Find the index for whitespace - if string starts with whitespace,
        // we need to offset our index for the filtered whitespace tokens
        let whitespace_index = if starts_with_whitespace { i + 1 } else { i };

        // Add whitespace if not the last class
        if i < sorted_classes.len() - 1 && whitespace_index < filtered_whitespace.len() {
            output.push_str(&filtered_whitespace[whitespace_index]);
        }
    }

    // Add trailing whitespace if present
    let last_whitespace_index = if starts_with_whitespace {
        sorted_classes.len()
    } else {
        sorted_classes.len() - 1
    };
    if last_whitespace_index < filtered_whitespace.len() {
        output.push_str(&filtered_whitespace[last_whitespace_index]);
    }

    output
}

/// Detects if a string contains template expressions
fn contains_template_expr(s: &str) -> bool {
    s.contains("${")
}

/// Sort classes in a template literal while preserving expressions
fn sort_template_literal(class_string: &str) -> String {
    debug_log!("Sorting template literal: {}", class_string);

    let mut chunks = Vec::new();
    let mut current_pos = 0;

    // Parse the template string into alternating Text and Expression chunks
    while let Some(expr_start) = class_string[current_pos..].find("${") {
        let abs_start = current_pos + expr_start;

        // Add the text before the expression
        if abs_start > current_pos {
            chunks.push(TemplateChunk::Text(
                class_string[current_pos..abs_start].to_string(),
            ));
        }

        // Find the end of the expression
        if let Some(expr_end) = class_string[abs_start..].find("}") {
            let abs_end = abs_start + expr_end + 1;

            // Add the expression
            chunks.push(TemplateChunk::Expression(
                class_string[abs_start..abs_end].to_string(),
            ));

            current_pos = abs_end;
        } else {
            // No closing brace, treat the rest as text
            chunks.push(TemplateChunk::Text(class_string[abs_start..].to_string()));
            current_pos = class_string.len();
            break;
        }
    }

    // Add any remaining text
    if current_pos < class_string.len() {
        chunks.push(TemplateChunk::Text(class_string[current_pos..].to_string()));
    }

    if is_normalize_whitespace_enabled() {
        sort_template_with_normalized_whitespace(&chunks)
    } else {
        sort_template_with_preserved_whitespace(&chunks)
    }
}

// Helper function to sort template with normalized whitespace
fn sort_template_with_normalized_whitespace(chunks: &[TemplateChunk]) -> String {
    // Collect all classes from text chunks
    let mut all_classes = Vec::new();

    for chunk in chunks {
        if let TemplateChunk::Text(text) = chunk {
            let classes = split_preserving_brackets(text);
            for class in classes {
                if !class.trim().is_empty() && is_tailwind_class(class) {
                    all_classes.push(class);
                }
            }
        }
    }

    if all_classes.is_empty() {
        return chunks
            .iter()
            .map(|c| match c {
                TemplateChunk::Text(t) => t.clone(),
                TemplateChunk::Expression(e) => e.clone(),
            })
            .collect();
    }

    let sorted_classes = sort_tailwind_classes(&all_classes);

    // Handle duplicate removal if enabled
    let sorted_classes = if is_remove_duplicates_enabled() {
        let (classes, _) = remove_duplicates_from_sorted(&sorted_classes);
        classes
    } else {
        sorted_classes
    };

    // Build the result with normalized whitespace
    let mut result = String::new();
    let mut class_idx = 0;

    for chunk in chunks {
        match chunk {
            TemplateChunk::Expression(expr) => {
                // Add space before expression if needed
                if !result.is_empty() && !result.ends_with(' ') {
                    result.push(' ');
                }

                // Add the expression
                result.push_str(expr);

                // Add space after if more content follows
                if class_idx < sorted_classes.len() {
                    result.push(' ');
                }
            }
            TemplateChunk::Text(text) => {
                // Count valid classes in this chunk
                let classes = split_preserving_brackets(text);
                let valid_class_count = classes
                    .iter()
                    .filter(|&c| !c.trim().is_empty() && is_tailwind_class(c))
                    .count();

                // Add that many classes from the sorted list
                for _ in 0..valid_class_count {
                    if class_idx < sorted_classes.len() {
                        // Add space before if needed
                        if !result.is_empty() && !result.ends_with(' ') {
                            result.push(' ');
                        }

                        // Add the class
                        result.push_str(sorted_classes[class_idx]);
                        class_idx += 1;
                    }
                }
            }
        }
    }

    result.trim().to_string()
}

// Helper function to sort template with preserved whitespace
fn sort_template_with_preserved_whitespace(chunks: &[TemplateChunk]) -> String {
    // Collect all valid classes
    let mut all_classes: Vec<String> = Vec::new();
    let mut chunks_with_class_counts: Vec<(Option<String>, Vec<String>, Vec<String>)> = Vec::new();

    // Collect all classes and their metadata
    for chunk in chunks.iter() {
        match chunk {
            TemplateChunk::Expression(expr) => {
                // Track expressions
                chunks_with_class_counts.push((Some(expr.clone()), Vec::new(), Vec::new()));
            }
            TemplateChunk::Text(text) => {
                if text.trim().is_empty() {
                    // Track empty text chunks
                    chunks_with_class_counts.push((None, Vec::new(), Vec::new()));
                    continue;
                }

                let mut classes = Vec::new();
                let mut whitespace = Vec::new();

                // Parse the original text into alternating whitespace and class tokens
                let starts_with_whitespace = text.chars().next().is_some_and(|c| c.is_whitespace());

                // Recreate the whitespace/class sequence
                if starts_with_whitespace {
                    // Starts with whitespace
                    let mut in_whitespace = true;
                    let mut current = String::new();

                    for c in text.chars() {
                        let is_whitespace = c.is_whitespace();

                        if is_whitespace != in_whitespace {
                            // Switching between whitespace and class
                            if in_whitespace {
                                whitespace.push(current.clone());
                            } else {
                                classes.push(current.clone());
                            }
                            current.clear();
                            in_whitespace = is_whitespace;
                        }
                        current.push(c);
                    }

                    // Add the last token
                    if !current.is_empty() {
                        if in_whitespace {
                            whitespace.push(current.clone());
                        } else {
                            classes.push(current.clone());
                        }
                    }
                } else {
                    // Starts with a class
                    let mut in_whitespace = false;
                    let mut current = String::new();

                    for c in text.chars() {
                        let is_whitespace = c.is_whitespace();

                        if is_whitespace != in_whitespace {
                            // Switching between class and whitespace
                            if in_whitespace {
                                whitespace.push(current.clone());
                            } else {
                                classes.push(current.clone());
                            }
                            current.clear();
                            in_whitespace = is_whitespace;
                        }
                        current.push(c);
                    }

                    // Add the last token
                    if !current.is_empty() {
                        if in_whitespace {
                            whitespace.push(current.clone());
                        } else {
                            classes.push(current.clone());
                        }
                    }

                    // If classes/whitespace are not balanced, add empty whitespace
                    if classes.len() > whitespace.len() {
                        whitespace.push(String::new());
                    }
                }

                // Add the valid classes to our tracking list
                for class in classes.iter() {
                    if !class.trim().is_empty() && is_tailwind_class(class) {
                        all_classes.push(class.clone());
                    }
                }

                // Store both valid and invalid classes to maintain structure
                chunks_with_class_counts.push((None, classes, whitespace));
            }
        }
    }

    // No valid classes found, return original
    if all_classes.is_empty() {
        return chunks
            .iter()
            .map(|c| match c {
                TemplateChunk::Text(t) => t.clone(),
                TemplateChunk::Expression(e) => e.clone(),
            })
            .collect();
    }

    // Get the class strings for sorting
    let class_strings: Vec<&str> = all_classes.iter().map(|class| class.as_str()).collect();

    // Sort all classes together
    let sorted_classes = sort_tailwind_classes(&class_strings);

    // Remove duplicates if enabled
    let (final_classes, _) = if is_remove_duplicates_enabled() {
        remove_duplicates_from_sorted(&sorted_classes)
    } else {
        (sorted_classes, HashSet::new())
    };

    // Rebuild the template with sorted classes
    let mut result = String::new();
    let mut sorted_class_idx = 0;

    for (chunk_idx, chunk_info) in chunks_with_class_counts.into_iter().enumerate() {
        match chunk_info {
            (Some(expr), _, _) => {
                // Expression chunk - add as is
                result.push_str(&expr);
            }
            (None, classes, whitespace) => {
                // Text chunk - rebuild with sorted classes
                if classes.is_empty() {
                    // Empty text chunk, include any whitespace
                    if let Some(TemplateChunk::Text(text)) = chunks.get(chunk_idx) {
                        result.push_str(text);
                    }
                    continue;
                }

                let mut rebuilt = String::new();
                let starts_with_whitespace =
                    !whitespace.is_empty() && classes.len() <= whitespace.len();

                // Track which classes from this chunk we've already used
                let mut used_class_indices = HashSet::new();

                if starts_with_whitespace {
                    // Start with the first whitespace
                    rebuilt.push_str(&whitespace[0]);
                }

                // Add each class with its following whitespace
                for (local_idx, _) in classes.iter().enumerate() {
                    // Skip this position if it's not a valid class
                    // or if it's a duplicate that was removed
                    if used_class_indices.contains(&local_idx) {
                        continue;
                    }

                    // Find the next available sorted class from the global sorted list
                    if sorted_class_idx < final_classes.len() {
                        // Add the sorted class
                        rebuilt.push_str(final_classes[sorted_class_idx]);
                        sorted_class_idx += 1;

                        // Mark this position as used
                        used_class_indices.insert(local_idx);
                    }

                    // Add whitespace if available
                    let whitespace_idx = if starts_with_whitespace {
                        local_idx + 1
                    } else {
                        local_idx
                    };
                    if whitespace_idx < whitespace.len() {
                        rebuilt.push_str(&whitespace[whitespace_idx]);
                    }
                }

                result.push_str(&rebuilt);
            }
        }
    }

    result
}

/// Main function to sort Tailwind classes
pub fn sort_classes(class_string: &str) -> String {
    // Handle empty strings
    if class_string.is_empty() {
        return String::new();
    }

    // Check if this is a whitespace-only string
    if class_string.chars().all(|c| c.is_whitespace()) {
        // If normalization is enabled, return a single space
        if is_normalize_whitespace_enabled() {
            return " ".to_string();
        }
        // Otherwise preserve the original whitespace
        return class_string.to_string();
    }

    // Special case for strings that are empty when trimmed but not just whitespace
    if class_string.trim().is_empty() {
        return class_string.to_string();
    }

    // Don't sort class attributes containing `{{`, to match Prettier behavior
    if class_string.contains("{{") {
        if is_debug_enabled() {
            debug_log!("Not sorting classes containing '{{': {}", class_string);
        }
        return class_string.to_string();
    }

    // Check for template expressions
    if contains_template_expr(class_string) {
        if is_debug_enabled() {
            debug_log!("Sorting template literal expression: {}", class_string);
        }

        // Use special handling for template literals
        return sort_template_literal(class_string);
    }

    // Split the class string properly to handle spaces in arbitrary values
    let classes = split_preserving_brackets(class_string);

    // Handle ellipsis special case
    let has_ellipsis = classes.iter().any(|&c| c == "...");
    let has_unicode_ellipsis = classes.iter().any(|&c| c == "…");
    let classes_without_ellipsis: Vec<&str> = classes
        .iter()
        .filter(|&&c| c != "..." && c != "…")
        .copied()
        .collect();

    // Sort the classes
    let sorted_classes = sort_tailwind_classes(&classes_without_ellipsis);

    // If enabled, remove duplicates from the sorted list
    let (final_classes, removed_indices) = if is_remove_duplicates_enabled() {
        remove_duplicates_from_sorted(&sorted_classes)
    } else {
        (sorted_classes, HashSet::new())
    };

    // Add ellipsis at the end if it existed
    let mut result = final_classes;
    if has_ellipsis {
        result.push("...");
    } else if has_unicode_ellipsis {
        result.push("…");
    }

    if is_debug_enabled() {
        debug_log!("Original: {}", class_string);
        debug_log!("Sorted:   {}", result.join(" "));
    }

    // Normalize whitespace if enabled
    if is_normalize_whitespace_enabled() {
        result.join(" ")
    } else {
        // Preserve whitespace and handle duplicates
        sort_with_preserved_whitespace(class_string, &result, &removed_indices)
    }
}

/// Sort Tailwind classes by their CSS property order
fn sort_tailwind_classes<'a>(classes: &[&'a str]) -> Vec<&'a str> {
    // Separate classes by type
    let mut custom_classes = Vec::new();
    let mut arbitrary_without_colon = Vec::new();
    let mut arbitrary_unknown_with_colon = Vec::new();
    let mut tailwind_classes = Vec::new();

    for &class in classes {
        // Handle fully arbitrary properties
        if (class.starts_with('[') && class.ends_with(']'))
            || (class.starts_with('(') && class.ends_with(')'))
        {
            let content = extract_arbitrary_attribute(class);
            if content.contains(':') {
                // Arbitrary with colon - check if it matches a known property
                let colon_idx = content.find(':').unwrap();
                let prop_name = &content[0..colon_idx];

                // Check if it's a recognized CSS property directly
                if find_order(prop_name) < 1000 {
                    // Any order less than 1000 is a known property
                    tailwind_classes.push(class);
                } else {
                    // Unknown property with colon, add to separate list for end placement
                    arbitrary_unknown_with_colon.push(class);
                }
            } else {
                // Arbitrary without colon - add with custom classes
                arbitrary_without_colon.push(class);
            }
        }
        // Handle all other classes
        else if is_tailwind_class(class) {
            tailwind_classes.push(class);
        } else {
            custom_classes.push(class);
        }
    }

    // Sort arbitrary without colon alphabetically
    arbitrary_without_colon.sort();

    // Sort unknown arbitrary with colon alphabetically
    arbitrary_unknown_with_colon.sort();

    // Process the remaining Tailwind classes
    let (container_class, remaining_tailwind): (Vec<&str>, Vec<&str>) = tailwind_classes
        .iter()
        .copied()
        .partition(|&c| c == "container");

    // Group utilities by parasite, base, state variants, responsive variants
    let (parasite_utilities, non_parasite): (Vec<&str>, Vec<&str>) = remaining_tailwind
        .iter()
        .copied()
        .partition(|&c| c == "group" || c == "peer");

    // Split variants (those with :) from base utilities
    let (variants, base_utilities): (Vec<&str>, Vec<&str>) =
        non_parasite.iter().copied().partition(|&c| is_variant(c));

    // Split responsive variants from state variants
    let (responsive_variants, state_variants): (Vec<&str>, Vec<&str>) =
        variants.iter().copied().partition(|&c| {
            let (variant, _) = split_variant(c);
            ["sm:", "md:", "lg:", "xl:", "2xl:"].contains(&variant)
        });

    // Sort the various categories
    let sorted_base = sort_basic_utilities(&base_utilities);
    let sorted_state = sort_state_variants(&state_variants);
    let sorted_responsive = sort_responsive_variants(&responsive_variants);

    // Combine all sorted categories in the correct order
    let mut result = Vec::new();
    result.extend(custom_classes); // Custom classes
    result.extend(arbitrary_without_colon); // Arbitrary without colons (alphabetical)
    result.extend(container_class); // Container
    result.extend(parasite_utilities); // Parasite utilities
    result.extend(sorted_base); // Base utilities (including matching arbitrary properties)
    result.extend(sorted_state); // State variants
    result.extend(sorted_responsive); // Responsive variants
    result.extend(arbitrary_unknown_with_colon); // Unknown arbitrary with colons

    result
}

/// Sort basic utilities (including arbitrary values)
fn sort_basic_utilities<'a>(base_utilities: &[&'a str]) -> Vec<&'a str> {
    let mut utilities_with_order: Vec<(&'a str, usize, String, bool, String)> = Vec::new();

    // Get property order for each utility
    for &class in base_utilities {
        let (order, sub_prop, is_negative, value) = get_property_order(class);
        utilities_with_order.push((class, order, sub_prop, is_negative, value));
    }

    // Sort by property order, then sub-property, then negative status, then value
    utilities_with_order.sort_by(
        |&(_, order_a, ref sub_a, neg_a, ref val_a), &(_, order_b, ref sub_b, neg_b, ref val_b)| {
            // Property order
            match order_a.cmp(&order_b) {
                std::cmp::Ordering::Equal => {
                    // Sub-property
                    match sub_a.cmp(sub_b) {
                        std::cmp::Ordering::Equal => {
                            // Negative status (negative first)
                            match neg_b.cmp(&neg_a) {
                                std::cmp::Ordering::Equal => {
                                    // Value
                                    val_a.cmp(val_b)
                                }
                                other => other,
                            }
                        }
                        other => other,
                    }
                }
                other => other,
            }
        },
    );

    // Extract just the class names
    utilities_with_order
        .into_iter()
        .map(|(class, _, _, _, _)| class)
        .collect()
}

/// Sort state variants (hover:, focus:, etc.)
fn sort_state_variants<'a>(variants: &[&'a str]) -> Vec<&'a str> {
    // Group variants by their base prefix (hover:, focus:, etc.)
    let mut grouped: HashMap<&str, Vec<&'a str>> = HashMap::new();

    for &class in variants {
        let (variant, _) = split_variant(class);
        grouped.entry(variant).or_default().push(class);
    }

    let mut variant_groups: Vec<(&str, usize)> = grouped
        .keys()
        .map(|&prefix| (prefix, get_state_variant_order(prefix)))
        .collect();

    // Sort by state variant order (hover before focus, etc.)
    variant_groups.sort_by_key(|&(_, order)| order);

    let mut result = Vec::new();

    for (prefix, _) in variant_groups {
        if let Some(classes) = grouped.get(prefix) {
            // Sort classes within this variant group by their base class order
            let mut variant_classes = classes.to_vec();

            variant_classes.sort_by(|&a, &b| {
                let (_, a_base) = split_variant(a);
                let (_, b_base) = split_variant(b);

                let (a_order, a_sub, a_neg, a_val) = get_property_order(a_base);
                let (b_order, b_sub, b_neg, b_val) = get_property_order(b_base);

                // Sort by property order, then sub-property, then negative status, then value
                match a_order.cmp(&b_order) {
                    std::cmp::Ordering::Equal => {
                        match a_sub.cmp(&b_sub) {
                            std::cmp::Ordering::Equal => {
                                match b_neg.cmp(&a_neg) {
                                    // Negative comes first
                                    std::cmp::Ordering::Equal => a_val.cmp(&b_val),
                                    other => other,
                                }
                            }
                            other => other,
                        }
                    }
                    other => other,
                }
            });

            result.extend(variant_classes);
        }
    }

    result
}

/// Sort responsive variants (sm:, md:, lg:, etc.)
fn sort_responsive_variants<'a>(variants: &[&'a str]) -> Vec<&'a str> {
    // Group variants by their base prefix (sm:, md:, etc.)
    let mut grouped: HashMap<&str, Vec<&'a str>> = HashMap::new();

    for &class in variants {
        let (variant, _) = split_variant(class);
        grouped.entry(variant).or_default().push(class);
    }

    let mut variant_groups: Vec<(&str, usize)> = grouped
        .keys()
        .map(|&prefix| (prefix, get_responsive_variant_order(prefix)))
        .collect();

    // Sort by responsive variant order (sm before md, etc.)
    variant_groups.sort_by_key(|&(_, order)| order);

    let mut result = Vec::new();

    for (prefix, _) in variant_groups {
        if let Some(classes) = grouped.get(prefix) {
            // Sort classes within this variant group by their base class order
            let mut variant_classes = classes.to_vec();

            variant_classes.sort_by(|&a, &b| {
                let (_, a_base) = split_variant(a);
                let (_, b_base) = split_variant(b);

                let (a_order, a_sub, a_neg, a_val) = get_property_order(a_base);
                let (b_order, b_sub, b_neg, b_val) = get_property_order(b_base);

                // Sort by property order, then sub-property, then negative status, then value
                match a_order.cmp(&b_order) {
                    std::cmp::Ordering::Equal => {
                        match a_sub.cmp(&b_sub) {
                            std::cmp::Ordering::Equal => {
                                match b_neg.cmp(&a_neg) {
                                    // Negative comes first
                                    std::cmp::Ordering::Equal => a_val.cmp(&b_val),
                                    other => other,
                                }
                            }
                            other => other,
                        }
                    }
                    other => other,
                }
            });

            result.extend(variant_classes);
        }
    }

    result
}

/// Split a class into variant and base parts
fn split_variant(class: &str) -> (&str, &str) {
    if let Some(pos) = class.find(':') {
        (&class[0..=pos], &class[pos + 1..])
    } else {
        ("", class)
    }
}

/// Get CSS property order information for a class
fn get_property_order(class: &str) -> (usize, String, bool, String) {
    // Handle special case for ellipsis
    if class == "..." || class == "…" {
        return (usize::MAX, String::new(), false, String::new());
    }

    // Handle negative values
    let is_negative = class.starts_with('-') && !class.starts_with("--");
    let lookup_class = if is_negative { &class[1..] } else { class };

    // Handle arbitrary values: both prefixed arbitrary values (bg-[red]) and full arbitrary properties ([color:red])
    if is_arbitrary_class(lookup_class) {
        // Full arbitrary property like "[margin:5px]"
        if (lookup_class.starts_with('[') && lookup_class.contains(':'))
            || (lookup_class.starts_with('(') && lookup_class.contains(':'))
        {
            let content = extract_arbitrary_attribute(lookup_class);
            let colon_idx = content.find(':').unwrap();
            let prop_name = &content[0..colon_idx];

            let order = find_order(prop_name);
            return (
                order,
                String::new(),
                is_negative,
                content[colon_idx + 1..].to_string(),
            );
        }

        // Prefixed arbitrary value like "bg-[red]"
        // Check if this is a prefixed arbitrary value
        for format in &["[", "("] {
            if let Some(prefix_end) = lookup_class.find(format) {
                let prefix = &lookup_class[0..prefix_end];

                if !prefix.is_empty() {
                    // This is a prefixed arbitrary value
                    let matching_prefixes: Vec<&TailwindPrefix> = get_tailwind_prefixes()
                        .iter()
                        .filter(|p| !p.is_standalone && prefix.starts_with(p.prefix))
                        .collect();

                    // Try to find an exact prefix match
                    if let Some(prefix_info) =
                        matching_prefixes.iter().find(|&&p| p.prefix == prefix)
                    {
                        return (
                            prefix_info.order,
                            String::new(),
                            is_negative,
                            class.to_string(),
                        );
                    }

                    // Then try prefixes by length (most specific first)
                    let mut sorted_prefixes = matching_prefixes.clone();
                    sorted_prefixes.sort_by_key(|p| -(p.prefix.len() as isize));

                    if let Some(prefix_info) = sorted_prefixes.first() {
                        return (
                            prefix_info.order,
                            String::new(),
                            is_negative,
                            class.to_string(),
                        );
                    }
                }
            }
        }

        // Default for unknown arbitrary properties
        return (1000, String::new(), is_negative, lookup_class.to_string());
    }

    // Find matching prefix for non-arbitrary classes
    if let Some(prefix) = find_matching_prefix(lookup_class) {
        // Extract the value part
        let mut sub_property = String::new();
        let mut value = String::new();

        if !prefix.is_standalone && prefix.prefix.ends_with('-') {
            let base_class = if lookup_class.contains(':') {
                lookup_class.split(':').last().unwrap_or(lookup_class)
            } else {
                lookup_class
            };

            if base_class.len() > prefix.prefix.len() {
                let value_part = &base_class[prefix.prefix.len()..];

                // Try to extract sub-property and value (e.g., from translate-x-4, get x and 4)
                if let Some(dash_pos) = value_part.find('-') {
                    sub_property = value_part[..dash_pos].to_string();
                    value = value_part[dash_pos + 1..].to_string();
                } else {
                    value = value_part.to_string();
                }
            }
        }

        return (prefix.order, sub_property, is_negative, value);
    }

    // Default for unknown classes
    (1000, String::new(), is_negative, class.to_string())
}

/// Find matching Tailwind prefix for a class
fn find_matching_prefix(class: &str) -> Option<&'static TailwindPrefix> {
    // Extract base class without variants
    let base_class = if class.contains(':') {
        class.split(':').last().unwrap_or(class)
    } else {
        class
    };

    // Handle container special case
    if base_class == "container" {
        return get_tailwind_prefixes()
            .iter()
            .find(|&p| p.prefix == "container");
    }

    // Handle arbitrary values
    if base_class.contains('[') || base_class.contains('(') {
        return None;
    }

    // Check for standalone utilities (exact matches)
    if let Some(prefix) = get_tailwind_prefixes()
        .iter()
        .find(|&p| p.is_standalone && p.prefix == base_class)
    {
        return Some(prefix);
    }

    // Check for prefixed utilities
    let mut matches: Vec<&'static TailwindPrefix> = get_tailwind_prefixes()
        .iter()
        .filter(|&p| !p.is_standalone && base_class.starts_with(p.prefix))
        .collect();

    // Sort by prefix length (descending) to find most specific match
    matches.sort_by_key(|p| -(p.prefix.len() as isize));

    // Try to find the longest match where the value is valid
    if let Some(longest_prefix) = matches.first() {
        let longest_len = longest_prefix.prefix.len();
        let value_part = if base_class.len() > longest_len {
            &base_class[longest_len..]
        } else {
            ""
        };

        // Check all prefixes of the same length and find one that accepts the value
        for &prefix in matches.iter() {
            if prefix.prefix.len() == longest_len && is_valid_value_for_prefix(prefix, value_part) {
                return Some(prefix);
            }
        }
    }

    // Fall back to the longest match if no valid prefix was found
    matches.into_iter().next()
}

/// Get state variant order for sorting
fn get_state_variant_order(variant: &str) -> usize {
    match variant {
        "hover:" => 100,
        "focus:" => 200,
        "focus-visible:" => 210,
        "focus-within:" => 220,
        "active:" => 300,
        "visited:" => 400,
        "checked:" => 500,
        "disabled:" => 600,
        "group-hover:" => 700,
        "group-focus:" => 710,
        "peer-hover:" => 800,
        "peer-focus:" => 810,
        "dark:" => 900,
        "first:" => 1000,
        "last:" => 1100,
        "only:" => 1200,
        "odd:" => 1300,
        "even:" => 1400,
        _ => 1500,
    }
}

/// Get responsive variant order for sorting
fn get_responsive_variant_order(variant: &str) -> usize {
    match variant {
        "sm:" => 100,
        "md:" => 200,
        "lg:" => 300,
        "xl:" => 400,
        "2xl:" => 500,
        _ => 600,
    }
}

/// Check if a value is valid for a given prefix
fn is_valid_value_for_prefix(prefix: &TailwindPrefix, value: &str) -> bool {
    // For standalone utilities (like "flex"), the value should be empty
    if prefix.is_standalone {
        return value.is_empty();
    }

    // Check the list of allowed values first
    if prefix.allowed_values.contains(&value) {
        return true;
    }

    // Then check for value types
    for &value_type in prefix.value_types {
        match value_type {
            ValueType::None => {
                // None type means no value is needed
                return value.is_empty();
            }
            ValueType::Number => {
                // Check if value is a number (integer or decimal)
                if value.parse::<f64>().is_ok() {
                    return true;
                }
            }
            ValueType::Fraction => {
                // Check for fraction format (e.g., 1/2, 2/3)
                if value.contains('/') {
                    let parts: Vec<&str> = value.split('/').collect();
                    if parts.len() == 2
                        && parts[0].parse::<f64>().is_ok()
                        && parts[1].parse::<f64>().is_ok()
                    {
                        return true;
                    }
                }
            }
            ValueType::Color => {
                // Check for basic colors
                if ["white", "black", "transparent", "current", "inherit"].contains(&value) {
                    return true;
                }
            }
            ValueType::ColorPalette => {
                // Check for color-number pattern (e.g., red-500)
                let parts: Vec<&str> = value.split('-').collect();
                if parts.len() == 2 {
                    let color = parts[0];
                    let number = parts[1];

                    if [
                        "slate", "gray", "zinc", "neutral", "stone", "red", "orange", "amber",
                        "yellow", "lime", "green", "emerald", "teal", "cyan", "sky", "blue",
                        "indigo", "violet", "purple", "fuchsia", "pink", "rose",
                    ]
                    .contains(&color)
                        && number.parse::<usize>().is_ok()
                    {
                        return true;
                    }
                }
            }
            ValueType::CustomProperty => {
                // Check for custom property format (--var)
                if value.starts_with("--")
                    || (value.starts_with('(')
                        && value.ends_with(')')
                        && value[1..value.len() - 1].starts_with("--"))
                {
                    return true;
                }
            }
            ValueType::ArbitraryValue => {
                // Check for arbitrary value format [value]
                if (value.starts_with('[') && value.ends_with(']'))
                    || (value.starts_with('(') && value.ends_with(')'))
                {
                    return true;
                }
            }
            ValueType::Length => {
                // Check common length values
                if ["px", "auto", "full", "screen"].contains(&value) || value.parse::<f64>().is_ok()
                {
                    return true;
                }
            }
            ValueType::Scale => {
                // Check common scale values
                if [
                    "xs", "sm", "md", "lg", "xl", "2xl", "3xl", "4xl", "5xl", "6xl", "7xl", "8xl",
                    "9xl", "none",
                ]
                .contains(&value)
                {
                    return true;
                }
            }
        }
    }

    false
}

/// Determine if a class is a valid Tailwind utility
pub fn is_tailwind_class(class: &str) -> bool {
    // Special cases
    if class == "..." || class == "…" || class == "container" || class == "group" || class == "peer"
    {
        return true;
    }

    // Apply rejection rules first
    if should_reject_candidate(class) {
        return false;
    }

    // Handle variants (hover:, sm:, etc.) FIRST - MOVED UP!
    if is_variant(class) {
        let (_, base) = split_variant(class);
        return is_tailwind_class(base);
    }

    // Handle negative classes
    if class.starts_with('-') && !class.starts_with("--") {
        return is_tailwind_class(&class[1..]);
    }

    // Check for arbitrary values with prefixes (like bg-[red]) - MOVED DOWN!
    if is_arbitrary_class(class) {
        // If it's an arbitrary class like [color:red], check for a valid property
        if class.starts_with('[') || class.starts_with('(') {
            // This is a completely arbitrary property+value
            return true;
        }

        // For prefixed arbitrary values like bg-[red], check if the prefix is valid
        for marker in &["[", "("] {
            if let Some(idx) = class.find(marker) {
                let prefix = &class[0..idx];

                // Class needs a non-empty prefix
                if prefix.is_empty() {
                    return false;
                }

                // Check if this prefix matches any Tailwind prefix
                return get_tailwind_prefixes()
                    .iter()
                    .any(|p| !p.is_standalone && prefix.starts_with(p.prefix));
            }
        }

        return false;
    }

    // Find matching prefix
    if let Some(prefix) = find_matching_prefix(class) {
        if prefix.is_standalone {
            // For standalone prefixes like "flex", the class name should exactly match
            return class == prefix.prefix;
        } else {
            // For prefixed utilities like "px-4", extract the value part and check if it's valid
            let value_part = &class[prefix.prefix.len()..];
            return is_valid_value_for_prefix(prefix, value_part);
        }
    }

    false
}

/// Rejection rules for classes that definitely aren't Tailwind utilities
fn should_reject_candidate(class: &str) -> bool {
    // Empty class is invalid
    if class.is_empty() {
        return true;
    }

    // Reject candidates that start with a capital letter
    if class.starts_with(|c: char| c.is_ascii_uppercase()) {
        return true;
    }

    // Reject candidates that end with "-" or "_"
    if class.ends_with('-') || class.ends_with('_') {
        return true;
    }

    // Reject candidates that are single camelCase words
    if class.chars().all(|c| c.is_ascii_alphanumeric())
        && class.chars().any(|c| c.is_ascii_uppercase())
    {
        return true;
    }

    // Reject candidates that look like SVG path data
    if !class.contains('-') && !class.contains(':') && class.contains('.') {
        return true;
    }

    // Reject candidates that look like version constraints or email addresses
    if class
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '@')
        && class.chars().skip(1).any(|c| c == '@')
    {
        return true;
    }

    // Reject candidates that look like URLs
    if class.starts_with("http://") || class.starts_with("https://") {
        return true;
    }

    // Reject candidates that look like markdown links
    if class.starts_with("[http://") || class.starts_with("[https://") {
        return true;
    }

    // Reject candidates that look like imports with path aliases
    if class.len() > 1 && class.starts_with('@') && class.chars().nth(1) == Some('/') {
        return true;
    }

    // Reject candidates that look like paths
    if !class.contains(':') && !class.contains('[') {
        let slash_count = class.chars().filter(|&c| c == '/').count();
        if slash_count > 1 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        (fn $name:ident() $body:block) => {
            #[test]
            fn $name() {
                set_normalize_whitespace(true);
                set_remove_duplicates(true);
                $body;
            }
        };
    }

    test! {
        fn test_blog_post_example() {
            let input = "text-white px-4 sm:px-8 py-2 sm:py-3 bg-sky-700 hover:bg-sky-800";
            let expected = "bg-sky-700 px-4 py-2 text-white hover:bg-sky-800 sm:px-8 sm:py-3";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_position_utilities() {
            let input = "py-2 px-4 my-2 mx-4 top-0 right-0 bottom-0 left-0 inset-0";
            let expected = "inset-0 top-0 right-0 bottom-0 left-0 mx-4 my-2 px-4 py-2";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_container_with_utilities() {
            let input = "p-4 container mx-auto md:w-1/2 bg-white";
            let expected = "container mx-auto bg-white p-4 md:w-1/2";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_padding_margin_order() {
            let input = "pl-4 p-2 m-4 mb-8 mx-2 py-3";
            let expected = "m-4 mx-2 mb-8 p-2 py-3 pl-4";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_tailwind_custom_properties() {
            let input = "bg-(--main-color) text-(--text-color) border-(--border-color) p-4 m-2";
            let expected = "m-2 border-(--border-color) bg-(--main-color) p-4 text-(--text-color)";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_custom_classes() {
            let input = "custom-class p-4 another-custom bg-blue-500 text-white my-class";
            let expected = "custom-class another-custom my-class bg-blue-500 p-4 text-white";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_ascii_ellipsis() {
            let input = "p-4 ... bg-blue-500";
            let expected = "bg-blue-500 p-4 ...";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_unicode_ellipsis() {
            let input = "p-4 … bg-blue-500";
            let expected = "bg-blue-500 p-4 …";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_multiple_responsive_variants() {
            let input = "flex md:grid lg:flex xl:grid 2xl:flex sm:block";
            let expected = "flex sm:block md:grid lg:flex xl:grid 2xl:flex";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_parasite_utilities() {
            let input = "p-4 group peer flex";
            let expected = "group peer flex p-4";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_complex_variants() {
            let input = "hover:opacity-75 focus:outline-none opacity-50 hover:scale-150 scale-125 sm:flex md:block lg:hidden sm:p-4 p-2 md:m-6";
            let expected = "scale-125 p-2 opacity-50 hover:scale-150 hover:opacity-75 focus:outline-none sm:flex sm:p-4 md:m-6 md:block lg:hidden";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_arbitrary_values() {
            let input = "bg-[#ff0000] text-[16px] p-[10px] m-[5px]";
            let expected = "m-[5px] bg-[#ff0000] p-[10px] text-[16px]";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_variant_consistent_with_base() {
            let input1 = "opacity-50 pointer-events-none";
            let expected1 = "pointer-events-none opacity-50";

            let input2 = "disabled:opacity-50 disabled:pointer-events-none";
            let expected2 = "disabled:pointer-events-none disabled:opacity-50";

            assert_eq!(sort_classes(input1), expected1);
            assert_eq!(sort_classes(input2), expected2);
        }
    }

    test! {
        fn test_camel_case_arbitrary_properties() {
            let input = "[backgroundColor:red] p-4 [marginTop:5px]";
            let expected = "p-4 [backgroundColor:red] [marginTop:5px]";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_transform_utilities() {
            let input = "scale-110 rotate-45 translate-x-4 -translate-y-2 skew-x-12";
            let expected = "translate-x-4 -translate-y-2 scale-110 rotate-45 skew-x-12";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_negative_values() {
            let input = "-translate-x-4 -mt-2 -mx-3 -mb-12";
            let expected = "-mx-3 -mt-2 -mb-12 -translate-x-4";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_arbitrary_properties() {
            let input = "[color:red] [margin:10px] [transform:rotate(45deg)]";
            let expected = "[margin:10px] [transform:rotate(45deg)] [color:red]";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_mixed_arbitrary_and_standard() {
            let input = "text-blue-500 [fontSize:14px] p-4 [padding-top:20px]";
            let expected = "p-4 [padding-top:20px] text-blue-500 [fontSize:14px]";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_complex_state_variants() {
            let input = "active:bg-blue-700 hover:bg-blue-500 focus:outline-none focus:ring-2 focus:ring-blue-300";
            let expected = "hover:bg-blue-500 focus:ring-2 focus:ring-blue-300 focus:outline-none active:bg-blue-700";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_group_peer_variants() {
            let input = "group-hover:bg-blue-400 peer-hover:bg-green-400 hover:bg-red-400";
            let expected = "hover:bg-red-400 group-hover:bg-blue-400 peer-hover:bg-green-400";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_nested_variants() {
            let input = "sm:hover:bg-blue-500 hover:sm:bg-blue-500";
            let expected = "hover:sm:bg-blue-500 sm:hover:bg-blue-500";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_empty_string() {
            let input = "";
            let expected = "";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_empty_spaces_only() {
            let input = "   ";
            let expected = " ";
            assert_eq!(sort_classes(input), expected);
        }
    }

    test! {
        fn test_interpolation() {
            let input = "bg-blue-500 {{ dynamicClass }} p-4";
            let expected = "bg-blue-500 {{ dynamicClass }} p-4";
            assert_eq!(sort_classes(input), expected);
        }
    }
}

#[cfg(test)]
mod preserve_whitespace_tests {
    use super::*;

    macro_rules! preserve_whitespace_test {
        (fn $name:ident() $body:block) => {
            #[test]
            fn $name() {
                set_normalize_whitespace(false);
                set_remove_duplicates(true);
                $body;
            }
        };
    }

    preserve_whitespace_test! {
        fn test_preserve_whitespace() {
            let input = "  p-4    flex   mt-2  ";
            let expected = "  mt-2    flex   p-4  ";

            assert_eq!(sort_classes(input), expected);
        }
    }

    preserve_whitespace_test! {
        fn test_preserve_whitespace_different_spacing() {
            let input = "  bg-blue-500     text-white   p-4  ";
            let expected = "  bg-blue-500     p-4   text-white  ";

            assert_eq!(sort_classes(input), expected);
        }
    }

    preserve_whitespace_test! {
        fn test_preserve_whitespace_leading_trailing() {
            let input = "   flex      ";
            let expected = "   flex      ";

            assert_eq!(sort_classes(input), expected);
        }
    }

    preserve_whitespace_test! {
        fn test_preserve_whitespace_with_many_classes() {
            let input = " p-2  m-4    bg-red-500  text-sm   hover:bg-red-600  sm:text-lg ";
            let expected = " m-4  bg-red-500    p-2  text-sm   hover:bg-red-600  sm:text-lg ";

            assert_eq!(sort_classes(input), expected);
        }
    }

    preserve_whitespace_test! {
        fn test_preserve_whitespace_complex_mixed() {
            let input = "   flex    xl:block    lg:flex     md:hidden     ";
            let expected = "   flex    md:hidden    lg:flex     xl:block     ";

            assert_eq!(sort_classes(input), expected);
        }
    }

    preserve_whitespace_test! {
        fn test_preserve_whitespace_with_arbitrary_values() {
            let input = "  p-[20px]   m-[10px]    bg-[#ff0000]  ";
            let expected = "  m-[10px]   bg-[#ff0000]    p-[20px]  ";

            assert_eq!(sort_classes(input), expected);
        }
    }

    preserve_whitespace_test! {
        fn test_preserve_whitespace_with_duplicates_removed() {
            let input = "p-4   m-2     p-4  bg-blue-500 m-2   ";
            let expected = "m-2     bg-blue-500  p-4   ";

            assert_eq!(sort_classes(input), expected);
        }
    }
}

#[cfg(test)]
mod normalize_whitespace_tests {
    use super::*;

    macro_rules! normalize_whitespace_test {
        (fn $name:ident() $body:block) => {
            #[test]
            fn $name() {
                set_normalize_whitespace(true);
                set_remove_duplicates(true);
                $body;
            }
        };
    }

    normalize_whitespace_test! {
        fn test_normalize_whitespace() {
            let input = "  p-4    flex   mt-2  ";
            let expected = "mt-2 flex p-4";

            assert_eq!(sort_classes(input), expected);
        }
    }

    normalize_whitespace_test! {
        fn test_normalize_whitespace_different_spacing() {
            let input = "  bg-blue-500     text-white   p-4  ";
            let expected = "bg-blue-500 p-4 text-white";

            assert_eq!(sort_classes(input), expected);
        }
    }

    normalize_whitespace_test! {
        fn test_normalize_whitespace_leading_trailing() {
            let input = "   flex      ";
            let expected = "flex";

            assert_eq!(sort_classes(input), expected);
        }
    }

    normalize_whitespace_test! {
        fn test_normalize_whitespace_with_many_classes() {
            let input = " p-2  m-4    bg-red-500  text-sm   hover:bg-red-600  sm:text-lg ";
            let expected = "m-4 bg-red-500 p-2 text-sm hover:bg-red-600 sm:text-lg";

            assert_eq!(sort_classes(input), expected);
        }
    }

    normalize_whitespace_test! {
        fn test_normalize_whitespace_complex_mixed() {
            let input = "   flex    xl:block    lg:flex     md:hidden     ";
            let expected = "flex md:hidden lg:flex xl:block";
            assert_eq!(sort_classes(input), expected);
        }
    }

    normalize_whitespace_test! {
        fn test_normalize_whitespace_with_arbitrary_values() {
            let input = "  p-[20px]   m-[10px]    bg-[#ff0000]  ";
            let expected = "m-[10px] bg-[#ff0000] p-[20px]";

            assert_eq!(sort_classes(input), expected);
        }
    }
}

#[cfg(test)]
mod remove_duplicates_tests {
    use super::*;

    macro_rules! remove_duplicates_test {
        (fn $name:ident() $body:block) => {
            #[test]
            fn $name() {
                set_normalize_whitespace(true);
                set_remove_duplicates(true);
                $body;
            }
        };
    }

    remove_duplicates_test! {
        fn test_remove_duplicates() {
            let input = "p-4 m-2 p-4 bg-blue-500 m-2";
            let expected = "m-2 bg-blue-500 p-4";

            assert_eq!(sort_classes(input), expected);
        }
    }

    remove_duplicates_test! {
        fn test_remove_duplicates_with_custom_classes() {
            let input = "p-4 my-class p-4 bg-blue-500 custom my-class custom";
            let expected = "my-class custom my-class custom bg-blue-500 p-4";

            assert_eq!(sort_classes(input), expected);
        }
    }

    remove_duplicates_test! {
        fn test_remove_duplicates_with_arbitrary_values() {
            let input = "p-[20px] m-[10px] p-[20px] bg-[#ff0000] m-[10px]";
            let expected = "m-[10px] bg-[#ff0000] p-[20px]";

            assert_eq!(sort_classes(input), expected);
        }
    }

    remove_duplicates_test! {
        fn test_remove_duplicates_with_whitespace() {
            let input = "  p-4    flex   mt-2  ";
            let expected = "mt-2 flex p-4";

            assert_eq!(sort_classes(input), expected);
        }
    }
}
