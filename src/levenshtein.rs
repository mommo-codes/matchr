use crate::normalize;

/// Computes the Levenshtein edit distance between two strings.
///
/// Returns the minimum number of single-character edits (insert,
/// delete, substitute) to transform `a` into `b`. Input is
/// normalised (lowercased, trimmed) before comparison.
///
/// # Examples
///
/// ```
/// use matchr::levenshtein;
///
/// assert_eq!(levenshtein("cat", "bat"), 1);
/// assert_eq!(levenshtein("cat", "cat"), 0);
/// ```

pub fn levenshtein(a: &str, b: &str) -> usize {
    let a = normalize(a);
    let b = normalize(b);

    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();

    let m = a_chars.len();
    let n = b_chars.len();

    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for i in 0..=m { dp[i][0] = i; }
    for j in 0..=n { dp[0][j] = j; }

    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if a_chars[i - 1] == b_chars[j - 1] {
                dp[i - 1][j - 1]
            } else {
                1 + dp[i - 1][j - 1]
                    .min(dp[i - 1][j])
                    .min(dp[i][j - 1])
            };
        }
    }

    dp[m][n]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identical() {
        assert_eq!(levenshtein("cat", "cat"), 0);
    }

    #[test]
    fn test_one_substitution() {
        assert_eq!(levenshtein("cat", "bat"), 1);
    }

    #[test]
    fn test_one_insertion() {
        assert_eq!(levenshtein("cat", "cart"), 1);
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(levenshtein("Cat", "cat"), 0);
    }

    #[test]
    fn test_completely_different() {
        assert_eq!(levenshtein("cat", "dog"), 3);
    }
}