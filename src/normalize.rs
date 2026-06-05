use unicode_normalization::char::is_combining_mark;
use unicode_normalization::UnicodeNormalization;

/// Trim, lowercase, and strip diacritics.
///
/// Steps:
/// 1. Trim surrounding whitespace
/// 2. Lowercase (Unicode-aware)
/// 3. Decompose to NFD
/// 4. Drop combining marks (Unicode category Mn)
/// 5. Recompose to NFC
///
/// # Examples
///
/// ```
/// use matchr::normalize;
///
/// assert_eq!(normalize("Café"), "cafe");
/// assert_eq!(normalize(" Hello "), "hello");
/// ```
pub fn normalize(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .nfd()
        .filter(|c| !is_combining_mark(*c))
        .nfc()
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lowercase() {
        assert_eq!(normalize("Hello"), "hello");
    }

    #[test]
    fn test_trim() {
        assert_eq!(normalize(" hello "), "hello");
    }

    #[test]
    fn test_both() {
        assert_eq!(normalize(" Hello World "), "hello world")
    }

    #[test]
    fn test_strips_acute_accent() {
        assert_eq!(normalize("café"), "cafe");
    }

    #[test]
    fn test_strips_tilde() {
        assert_eq!(normalize("Ñoño"), "nono");
    }

    #[test]
    fn test_strips_umlaut() {
        assert_eq!(normalize("Über"), "uber");
        assert_eq!(normalize("naïve"), "naive");
    }

    #[test]
    fn test_strips_multiple_diacritics() {
        assert_eq!(normalize("Crème Brûlée"), "creme brulee");
    }

    #[test]
    fn test_no_diacritics_unchanged() {
        assert_eq!(normalize("hello world"), "hello world");
    }

    #[test]
    fn test_precomposed_and_decomposed_match() {
        // "é" can be encoded as either U+00E9 (precomposed) or
        // "e" + U+0301 (decomposed). Both should normalise the same.
        let precomposed = "caf\u{00E9}";
        let decomposed = "cafe\u{0301}";
        assert_eq!(normalize(precomposed), normalize(decomposed));
        assert_eq!(normalize(precomposed), "cafe");
    }
}
