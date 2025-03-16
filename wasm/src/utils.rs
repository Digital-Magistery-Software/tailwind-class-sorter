/// Split a class string into individual classes
pub fn split_classes(class_string: &str) -> Vec<String> {
    class_string
        .split_whitespace()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// Join classes back into a string with proper spacing
pub fn join_classes(classes: &[String]) -> String {
    classes.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_classes() {
        let input = "flex items-center justify-between";
        let expected = vec![
            "flex".to_string(),
            "items-center".to_string(),
            "justify-between".to_string(),
        ];
        assert_eq!(split_classes(input), expected);
    }

    #[test]
    fn test_split_classes_with_extra_spaces() {
        let input = "  flex   items-center    justify-between  ";
        let expected = vec![
            "flex".to_string(),
            "items-center".to_string(),
            "justify-between".to_string(),
        ];
        assert_eq!(split_classes(input), expected);
    }

    #[test]
    fn test_join_classes() {
        let input = vec![
            "flex".to_string(),
            "items-center".to_string(),
            "justify-between".to_string(),
        ];
        let expected = "flex items-center justify-between";
        assert_eq!(join_classes(&input), expected);
    }
}
