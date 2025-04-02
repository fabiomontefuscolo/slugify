pub fn slugify(input: String) -> String {
    let mut result = input.to_lowercase();

    result = result.replace(' ', "-");
    result = result
        .chars()
        .filter(|&c| c.is_alphanumeric() || c == '_' || c == '-')
        .collect();

    while result.contains("--") {
        result = result.replace("--", "-");
    }

    result = result
        .trim_matches(|c| c == ' ' || c == '-' || c == '_')
        .to_string();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowercase_conversion() {
        assert_eq!(slugify("Hello".to_string()), "hello");
        assert_eq!(slugify("WORLD".to_string()), "world");
        assert_eq!(slugify("MiXeD".to_string()), "mixed");
    }

    #[test]
    fn test_space_to_hyphen() {
        assert_eq!(slugify("hello world".to_string()), "hello-world");
        assert_eq!(slugify("multiple   spaces".to_string()), "multiple-spaces");
    }

    #[test]
    fn test_trim_trailing_spaces() {
        assert_eq!(
            slugify("  trailing     spaces    ".to_string()),
            "trailing-spaces"
        );
    }

    #[test]
    fn test_special_character_removal() {
        assert_eq!(slugify("hello!@#$%^&*()world".to_string()), "helloworld");
        assert_eq!(
            slugify("keep_underscore-hyphen".to_string()),
            "keep_underscore-hyphen"
        );
        assert_eq!(
            slugify("remove.comma,semicolon;".to_string()),
            "removecommasemicolon"
        );
    }

    #[test]
    fn test_consecutive_hyphen_replacement() {
        assert_eq!(slugify("hello--world".to_string()), "hello-world");
        assert_eq!(
            slugify("multiple---hyphens".to_string()),
            "multiple-hyphens"
        );
    }

    #[test]
    fn test_trim_special_characters() {
        assert_eq!(slugify("-hello-".to_string()), "hello");
        assert_eq!(slugify("_world_".to_string()), "world");
        assert_eq!(slugify(" spaces ".to_string()), "spaces");
        assert_eq!(slugify("-_-mixed_-_".to_string()), "mixed");
    }

    #[test]
    fn test_complex_cases() {
        assert_eq!(
            slugify("  Hello, World! This is a Test  ".to_string()),
            "hello-world-this-is-a-test"
        );
        assert_eq!(
            slugify("__Special-CHARS-123!@#".to_string()),
            "special-chars-123"
        );
        assert_eq!(
            slugify("Multiple   spaces and---hyphens".to_string()),
            "multiple-spaces-and-hyphens"
        );
        assert_eq!(slugify("".to_string()), "");
    }
}
