use crate::normalize;
use std::collections::HashMap;

/// Computes trigram similarity between two strings.
///
/// Splits strings into overlapping 3-character chunks and scores
/// overlap using the Dice coefficient. Returns `0.0`–`1.0`.
/// Input is normalised before comparison.
///
/// # Examples
///
/// ```
/// use matchr::trigram_similarity;
///
/// assert!((trigram_similarity("hello", "hello") - 1.0).abs() < 0.001);
/// assert!((trigram_similarity("abc", "xyz") - 0.0).abs() < 0.001);
/// ```

pub fn trigram_similarity(a: &str, b: &str) -> f64 {
    let a = normalize(a);
    let b = normalize(b);

    let a_tri = trigrams(&a);
    let b_tri = trigrams(&b);

    if a_tri.is_empty() && b_tri.is_empty() { return 1.0; }
    if a_tri.is_empty() || b_tri.is_empty() { return 0.0; }

    let total = a_tri.len() + b_tri.len();
    let mut counts: HashMap<String, i32> = HashMap::new();
    
    for t in &a_tri {
        *counts.entry(t.clone()).or_insert(0) += 1;
    }

    let mut shared = 0;
    for t in &b_tri {
        if let Some(c) = counts.get_mut(t) {
            if *c > 0 {
                shared += 1;
                *c -= 1;
            }
        }
    }

    2.0 * shared as f64 / total as f64

}

fn trigrams(s: &str) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() < 3 { return vec![]; }
    chars.windows(3).map( |w| w.iter().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.001
    }

    #[test]
    fn test_identical() {
        assert!(approx(trigram_similarity("hello", "hello"), 1.0));
    }

    #[test]
    fn test_both_empty() {
        assert!(approx(trigram_similarity("", ""), 1.0));
    }

    #[test]
    fn test_no_overlap() {
        assert!(approx(trigram_similarity("abc", "xyz"), 0.0));
    }

    #[test]
    fn test_partial() {
        let score = trigram_similarity("hello", "helo");
        assert!(score > 0.0 && score < 1.0);
    }

    #[test]
    fn test_case_insensitive() {
        assert!(approx(trigram_similarity("Hello", "hello"), 1.0));
    }
}