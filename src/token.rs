use crate::{combined_score_raw, normalize};
use std::collections::BTreeSet;

/// Token-sort similarity between two strings.
///
/// Splits each string into whitespace-separated tokens, sorts them
/// alphabetically, joins them back, and runs `combined_score` on
/// the result. Order-invariant — useful when word order varies,
/// e.g. `"Oat Drink Oatly 1L"` vs `"Oatly Oat Drink 1L"`.
///
/// Returns `0.0`–`1.0`. Input is normalised before tokenising.
///
/// # Examples
///
/// ```
/// use matchr::token_sort_ratio;
///
/// let score = token_sort_ratio("Oat Drink Oatly", "Oatly Oat Drink");
/// assert!((score - 1.0).abs() < 0.001);
/// ```
pub fn token_sort_ratio(a: &str, b: &str) -> f64 {
    let a_sorted = sort_tokens(&normalize(a));
    let b_sorted = sort_tokens(&normalize(b));
    combined_score_raw(&a_sorted, &b_sorted)
}

/// Token-set similarity between two strings.
///
/// Like `token_sort_ratio` but also deduplicates tokens before
/// comparing — useful when one string repeats words the other
/// doesn't, e.g. `"apple apple banana"` vs `"banana apple"`.
///
/// Returns `0.0`–`1.0`. Input is normalised before tokenising.
///
/// # Examples
///
/// ```
/// use matchr::token_set_ratio;
///
/// let score = token_set_ratio("apple apple banana", "banana apple");
/// assert!((score - 1.0).abs() < 0.001);
/// ```
pub fn token_set_ratio(a: &str, b: &str) -> f64 {
    let a_set = set_tokens(&normalize(a));
    let b_set = set_tokens(&normalize(b));
    combined_score_raw(&a_set, &b_set)
}

fn sort_tokens(s: &str) -> String {
    let mut tokens: Vec<&str> = s.split_whitespace().collect();
    tokens.sort();
    tokens.join(" ")
}

fn set_tokens(s: &str) -> String {
    let tokens: BTreeSet<&str> = s.split_whitespace().collect();
    tokens.into_iter().collect::<Vec<&str>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.001
    }

    #[test]
    fn test_sort_identical() {
        assert!(approx(token_sort_ratio("hello world", "hello world"), 1.0));
    }

    #[test]
    fn test_sort_reordered() {
        assert!(approx(token_sort_ratio("hello world", "world hello"), 1.0));
    }

    #[test]
    fn test_sort_product_example() {
        let score = token_sort_ratio("Oat Drink Oatly 1L", "Oatly Oat Drink 1L");
        assert!(approx(score, 1.0));
    }

    #[test]
    fn test_sort_case_insensitive() {
        assert!(approx(token_sort_ratio("HELLO World", "world hello"), 1.0));
    }

    #[test]
    fn test_sort_partial_overlap() {
        let score = token_sort_ratio("apple banana", "apple cherry");
        assert!(score > 0.0 && score < 1.0);
    }

    #[test]
    fn test_sort_both_empty() {
        assert!(approx(token_sort_ratio("", ""), 1.0));
    }

    #[test]
    fn test_set_deduplicates() {
        assert!(approx(
            token_set_ratio("apple apple banana", "banana apple"),
            1.0,
        ));
    }

    #[test]
    fn test_set_identical() {
        assert!(approx(token_set_ratio("hello world", "hello world"), 1.0));
    }

    #[test]
    fn test_set_reordered_and_repeated() {
        let score = token_set_ratio("oat oat drink oatly", "oatly drink oat");
        assert!(approx(score, 1.0));
    }

    #[test]
    fn test_set_case_insensitive() {
        assert!(approx(token_set_ratio("Apple BANANA", "banana apple"), 1.0));
    }

    #[test]
    fn test_set_partial_overlap() {
        let score = token_set_ratio("apple banana", "apple cherry");
        assert!(score > 0.0 && score < 1.0);
    }

    #[test]
    fn test_set_both_empty() {
        assert!(approx(token_set_ratio("", ""), 1.0));
    }
}
