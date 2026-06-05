use crate::normalize;

/// Computes the Jaro-Winkler similarity between two strings.
///
/// Returns a score from `0.0` (no similarity) to `1.0` (identical).
/// Gives a bonus for shared prefixes — works well for names.
/// Input is normalised before comparison.
///
/// # Examples
///
/// ```
/// use matchr::jaro_winkler;
///
/// assert!((jaro_winkler("martha", "marhta") - 0.961).abs() < 0.001);
/// assert!((jaro_winkler("cat", "cat") - 1.0).abs() < 0.001);
/// ```

pub fn jaro_winkler(a: &str, b: &str) -> f64 {
    let a = normalize(a);
    let b = normalize(b);

    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    let a_len = a_chars.len();
    let b_len = b_chars.len();

    if a_len == 0 && b_len == 0 { return 1.0; }
    if a_len == 0 || b_len == 0 { return 0.0; }

    let match_window = (a_len.max(b_len) / 2).saturating_sub(1);

    let mut a_matches = vec![false; a_len];
    let mut b_matches = vec![false; b_len];

    let mut matches = 0usize;
    let mut transpositions = 0usize;

    for i in 0..a_len {
        let start = i.saturating_sub(match_window);
        let end = (i + match_window + 1).min(b_len);

        for j in start..end {
            if b_matches[j] || a_chars[i] != b_chars[j] { continue; }
            a_matches[i] = true;
            b_matches[j] = true;
            matches += 1;
            break;
        }
    }

    if matches == 0 { return 0.0; }

    let mut k = 0;
    for i in 0..a_len {
        if !a_matches[i] { continue; }
        while !b_matches[k] { k += 1; }
        if a_chars[i] != b_chars[k] { transpositions += 1; }
        k += 1;
    }

    let m = matches as f64;
    let jaro = (m / a_len as f64
              + m / b_len as f64
              + (m - transpositions as f64 / 2.0) / m) / 3.0;

    let prefix = a_chars.iter()
        .zip(b_chars.iter())
        .take(4)
        .take_while(|(x, y)| x == y)
        .count();

    jaro + prefix as f64 * 0.1 * (1.0 - jaro)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.001
    }

    #[test]
    fn test_identical() {
        assert!(approx(jaro_winkler("cat", "cat"), 1.0));
    }

    #[test]
    fn test_both_empty() {
        assert!(approx(jaro_winkler("", ""), 1.0));
    }

    #[test]
    fn test_no_match() {
        assert!(approx(jaro_winkler("cat", "dog"), 0.0));
    }

    #[test]
    fn test_classic() {
        assert!(approx(jaro_winkler("martha", "marhta"), 0.961));
    }

    #[test]
    fn test_case_insensitive() {
        assert!(approx(jaro_winkler("Martha", "marhta"), 0.961));
    }
}