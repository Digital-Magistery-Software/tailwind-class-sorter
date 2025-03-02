use wasm_bindgen::prelude::*;

pub mod extractor;
pub mod utils;

use extractor::extract_class_strings;
use utils::{join_classes, split_classes};

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

#[wasm_bindgen]
pub fn sort_tailwind_classes(document: &str, file_extension: &str) -> String {
    console_log!("Starting Tailwind class sorting for {}", file_extension);

    let class_matches = extract_class_strings(document, file_extension);

    console_log!("Found {} class string matches", class_matches.len());

    for (i, class_match) in class_matches.iter().enumerate() {
        let classes = split_classes(&class_match.class_string);

        console_log!(
            "Match #{}: {} classes at positions {}-{}",
            i + 1,
            classes.len(),
            class_match.start,
            class_match.end
        );

        console_log!("  Original text: \"{}\"", class_match.original);
        console_log!("  Class string: \"{}\"", class_match.class_string);
        console_log!("  Individual classes: {:?}", classes);
    }

    // For now, just return the original document
    document.to_string()
}
