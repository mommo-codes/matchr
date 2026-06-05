pub mod normalize;
pub mod levenshtein;
pub mod jaro_winkler;
pub mod trigram;

pub use normalize::normalize;
pub use levenshtein::levenshtein;
pub use jaro_winkler::jaro_winkler;
pub use trigram::trigram_similarity;

/// Blends all three algorithms into one similarity score.
///
/// Returns `0.0` (no similarity) to `1.0` (identical).
/// Levenshtein is normalised to 0–1 before blending.
///
/// # Examples
///
/// ```
/// use matchr::combined_score;
/// assert!((combined_score("cat", "cat") - 1.0).abs() < 0.001);
/// ```
pub fn combined_score(a: &str, b: &str) -> f64 {
    let lev = levenshtein(a, b);
    let max_len = a.chars().count().max(b.chars().count());

    let lev_sim = if max_len == 0 {
        1.0
    } else {
        1.0 - (lev as f64 / max_len as f64)
    };

    let jw  = jaro_winkler(a, b);
    let tri = trigram_similarity(a, b);

    lev_sim * 0.35 + jw * 0.40 + tri * 0.25
}

/// Returns the closest match from a list of candidates.
///
/// # Examples
///
/// ```
/// use matchr::best_match;
/// let candidates = vec!["apple", "grape", "mango"];
/// let (winner, score) = best_match("appel", &candidates).unwrap();
/// assert_eq!(winner, "apple");
/// ```
pub fn best_match<'a>(query: &str, candidates: &[&'a str]) -> Option<(&'a str, f64)> {
    candidates
        .iter()
        .map(|c| (*c, combined_score(query, c)))
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
}

/// Returns all candidates ranked by similarity score, highest first.
///
/// # Examples
///
/// ```
/// use matchr::rank_matches;
/// let candidates = vec!["apple", "grape", "mango"];
/// let ranked = rank_matches("appel", &candidates);
/// assert_eq!(ranked[0].0, "apple");
/// ```
pub fn rank_matches<'a>(query: &str, candidates: &[&'a str]) -> Vec<(&'a str, f64)> {
    let mut results: Vec<(&str, f64)> = candidates
        .iter()
        .map(|c| (*c, combined_score(query, c)))
        .collect();

    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    results
}

pub mod python;