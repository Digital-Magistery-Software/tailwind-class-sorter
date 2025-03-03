use std::collections::HashMap;
use std::sync::OnceLock;

pub fn sort_classes(class_string: &str) -> String {
    let classes: Vec<&str> = class_string.split_whitespace().collect();

    let mut categorized_classes: HashMap<usize, Vec<(usize, &str)>> = HashMap::new();

    for (idx, class) in classes.iter().enumerate() {
        let (_variant_prefix, base_class) = extract_variant(class);
        let category = get_class_category(base_class);

        categorized_classes
            .entry(category)
            .or_default()
            .push((idx, *class));
    }

    let mut ordered_categories: Vec<usize> = categorized_classes.keys().cloned().collect();
    ordered_categories.sort();

    let mut sorted_classes = Vec::new();

    for category in ordered_categories {
        let mut category_classes = categorized_classes.get(&category).unwrap().clone();

        category_classes.sort_by(|a, b| {
            let (a_variant, a_base) = extract_variant(a.1);
            let (b_variant, b_base) = extract_variant(b.1);

            let a_subcategory = get_subcategory(a_base);
            let b_subcategory = get_subcategory(b_base);

            if a_subcategory != b_subcategory {
                return a_subcategory.cmp(&b_subcategory);
            }

            if a_variant != b_variant {
                return get_variant_order(a_variant).cmp(&get_variant_order(b_variant));
            }

            a.0.cmp(&b.0)
        });

        for (_, class) in category_classes {
            sorted_classes.push(class);
        }
    }

    sorted_classes.join(" ")
}

/// Extract variant prefix (if any) from a class
/// Returns (variant_prefix, base_class)
fn extract_variant(class: &str) -> (&str, &str) {
    // Find the last colon in the class name
    if let Some(colon_pos) = class.rfind(':') {
        // Include the colon
        let variant = &class[0..=colon_pos];
        let base = &class[colon_pos + 1..];
        (variant, base)
    } else {
        // No variant
        ("", class)
    }
}

/// Get a sub-category for more granular sorting within a category
fn get_subcategory(class: &str) -> usize {
    // For padding, margin, width, etc., extract the side/direction info
    if class.starts_with("p-") || class.starts_with("m-") {
        return 1; // Base padding/margin
    } else if class.starts_with("px-") || class.starts_with("mx-") {
        return 2; // X-axis
    } else if class.starts_with("py-") || class.starts_with("my-") {
        return 3; // Y-axis
    } else if class.starts_with("pt-") || class.starts_with("mt-") {
        return 4; // Top
    } else if class.starts_with("pr-") || class.starts_with("mr-") {
        return 5; // Right
    } else if class.starts_with("pb-") || class.starts_with("mb-") {
        return 6; // Bottom
    } else if class.starts_with("pl-") || class.starts_with("ml-") {
        return 7; // Left
    }

    // For color utilities, try to extract the color weight
    if class.contains("-") {
        let parts: Vec<&str> = class.split('-').collect();
        if parts.len() >= 3 {
            if let Ok(weight) = parts.last().unwrap().parse::<usize>() {
                return weight;
            }
        }
    }

    // Default subcategory
    0
}

/// Get the category order for a class (lower is earlier)
fn get_class_category(class: &str) -> usize {
    // Special case for arbitrary classes like [...]
    if class.starts_with('[') && class.ends_with(']') {
        return 9999; // Place at the end
    }

    let category_map = get_category_map();

    for (prefix, category) in category_map {
        if class.starts_with(prefix) {
            return *category;
        }
    }

    for (exact, category) in get_exact_class_map() {
        if class == *exact {
            return *category;
        }
    }

    // Default to a high number for unknown classes (sort at the end)
    9000
}

/// Get the order for a variant prefix (lower is earlier)
fn get_variant_order(variant: &str) -> usize {
    if variant.is_empty() {
        return 0; // No variant comes first
    }

    // Remove trailing colon for lookup
    let variant_no_colon = variant.trim_end_matches(':');

    // Responsive variants in size order
    match variant_no_colon {
        "sm" => 100,
        "md" => 200,
        "lg" => 300,
        "xl" => 400,
        "2xl" => 500,

        // State variants
        "hover" => 1000,
        "focus" => 1100,
        "active" => 1200,
        "disabled" => 1300,
        "visited" => 1400,

        // Other variants
        "dark" => 2000,
        "group-hover" => 2100,
        "peer-hover" => 2200,

        // Unknown variants
        _ => 9000,
    }
}

/// Singleton for category map (for performance)
static CATEGORY_MAP: OnceLock<Vec<(&'static str, usize)>> = OnceLock::new();

fn get_category_map() -> &'static Vec<(&'static str, usize)> {
    CATEGORY_MAP.get_or_init(|| {
        // The order here is meant to follow Tailwind's property order
        // as seen in the file property-order.ts in the TailwindCss Repo
        // Lower numbers come first
        vec![
            // Layout (1xx)
            ("container", 100),
            ("display", 101),
            ("block", 102),  // Exact match handled separately
            ("inline", 103), // Exact match handled separately
            ("flex", 104),   // Exact match handled separately
            ("grid", 105),   // Exact match handled separately
            ("table", 106),
            ("hidden", 107), // Exact match handled separately
            // Positioning (2xx)
            ("position", 200),
            ("static", 201),   // Exact match handled separately
            ("fixed", 202),    // Exact match handled separately
            ("absolute", 203), // Exact match handled separately
            ("relative", 204), // Exact match handled separately
            ("sticky", 205),   // Exact match handled separately
            ("inset-", 206),
            ("top-", 210),
            ("right-", 211),
            ("bottom-", 212),
            ("left-", 213),
            ("z-", 220),
            // Box Model - Size (3xx)
            ("w-", 300),
            ("h-", 301),
            ("min-w-", 310),
            ("min-h-", 311),
            ("max-w-", 320),
            ("max-h-", 321),
            // Box Model - Spacing (4xx)
            ("p-", 400),
            ("px-", 401),
            ("py-", 402),
            ("pt-", 403),
            ("pr-", 404),
            ("pb-", 405),
            ("pl-", 406),
            ("m-", 410),
            ("mx-", 411),
            ("my-", 412),
            ("mt-", 413),
            ("mr-", 414),
            ("mb-", 415),
            ("ml-", 416),
            ("space-", 420),
            // Flexbox & Grid (5xx)
            ("flex-", 500),
            ("flex-grow", 501),
            ("flex-shrink", 502),
            ("flex-basis", 503),
            ("order-", 510),
            ("grid-", 520),
            ("grid-cols-", 521),
            ("grid-rows-", 522),
            ("grid-flow-", 523),
            ("col-", 524),
            ("row-", 525),
            ("gap-", 530),
            ("gap-x-", 531),
            ("gap-y-", 532),
            // Alignment (6xx)
            ("justify-", 600),
            ("content-", 610),
            ("items-", 620),
            ("self-", 630),
            ("place-", 640),
            // Typography (7xx)
            ("font-", 700),
            ("text-", 710),
            ("tracking-", 720),
            ("leading-", 730),
            ("list-", 740),
            ("placeholder-", 750),
            ("whitespace-", 760),
            ("break-", 770),
            ("truncate", 780), // Exact match handled separately
            // Visual styles (8xx)
            ("bg-", 800),
            ("border", 810), // Special case handled separately
            ("border-", 811),
            ("rounded", 820), // Exact match handled separately
            ("rounded-", 821),
            ("shadow", 830), // Exact match handled separately
            ("shadow-", 831),
            ("opacity-", 840),
            // Effects (9xx)
            ("transform", 900), // Exact match handled separately
            ("transform-", 901),
            ("rotate-", 902),
            ("scale-", 903),
            ("translate-", 904),
            ("skew-", 905),
            // Transitions & Animation (10xx)
            ("transition", 1000), // Exact match handled separately
            ("transition-", 1001),
            ("duration-", 1002),
            ("ease-", 1003),
            ("delay-", 1004),
            ("animate-", 1005),
            // Misc (11xx)
            ("cursor-", 1100),
            ("select-", 1110),
            ("resize", 1120), // Exact match handled separately
            ("resize-", 1121),
            ("ring-", 1130),
            ("focus-ring", 1131),
            ("fill-", 1140),
            ("stroke-", 1150),
            ("outline-", 1160),
            ("object-", 1170),
            ("overflow-", 1180),
            ("overscroll-", 1190),
        ]
    })
}

/// Singleton for exact class matches (for performance)
static EXACT_CLASS_MAP: OnceLock<Vec<(&'static str, usize)>> = OnceLock::new();

fn get_exact_class_map() -> &'static Vec<(&'static str, usize)> {
    EXACT_CLASS_MAP.get_or_init(|| {
        vec![
            // Layout
            ("block", 102),
            ("inline", 103),
            ("flex", 104),
            ("grid", 105),
            ("hidden", 107),
            // Positioning
            ("static", 201),
            ("fixed", 202),
            ("absolute", 203),
            ("relative", 204),
            ("sticky", 205),
            // Typography
            ("truncate", 780),
            ("italic", 781),
            ("underline", 782),
            ("uppercase", 783),
            ("lowercase", 784),
            ("capitalize", 785),
            // Visual styles
            ("rounded", 820),
            ("shadow", 830),
            // Effects
            ("transform", 900),
            // Transitions
            ("transition", 1000),
            // Misc
            ("resize", 1120),
        ]
    })
}
