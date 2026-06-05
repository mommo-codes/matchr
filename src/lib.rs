pub mod jaro_winkler;
pub mod levenshtein;
pub mod normalize;
pub mod token;
pub mod trigram;

pub use jaro_winkler::jaro_winkler;
pub use levenshtein::levenshtein;
pub use normalize::normalize;
pub use token::{token_set_ratio, token_sort_ratio};
pub use trigram::trigram_similarity;

use jaro_winkler::jaro_winkler_raw;
use levenshtein::levenshtein_raw;
use rayon::prelude::*;
use trigram::trigram_similarity_raw;

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
    let a = normalize(a);
    let b = normalize(b);
    combined_score_raw(&a, &b)
}

pub(crate) fn combined_score_raw(a: &str, b: &str) -> f64 {
    let lev = levenshtein_raw(a, b);
    let max_len = a.chars().count().max(b.chars().count());

    let lev_sim = if max_len == 0 {
        1.0
    } else {
        1.0 - (lev as f64 / max_len as f64)
    };

    let jw = jaro_winkler_raw(a, b);
    let tri = trigram_similarity_raw(a, b);

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
    let q = normalize(query);
    candidates
        .iter()
        .map(|c| {
            let cn = normalize(c);
            (*c, combined_score_raw(&q, &cn))
        })
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
    let q = normalize(query);
    let mut results: Vec<(&str, f64)> = candidates
        .iter()
        .map(|c| {
            let cn = normalize(c);
            (*c, combined_score_raw(&q, &cn))
        })
        .collect();

    results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    results
}

/// Returns the best match for each query against the candidate list.
///
/// Candidates are normalised once (not per query). Queries are
/// processed in parallel via rayon — scales with available cores.
///
/// # Examples
///
/// ```
/// use matchr::batch_best_match;
/// let candidates = ["apple", "grape", "mango"];
/// let queries = ["appel", "mngo"];
/// let results = batch_best_match(&queries, &candidates);
/// assert_eq!(results[0].unwrap().0, "apple");
/// assert_eq!(results[1].unwrap().0, "mango");
/// ```
pub fn batch_best_match<'a>(
    queries: &[&str],
    candidates: &[&'a str],
) -> Vec<Option<(&'a str, f64)>> {
    let cand_norm: Vec<String> = candidates.iter().map(|c| normalize(c)).collect();

    queries
        .par_iter()
        .map(|&q| {
            let q_norm = normalize(q);
            candidates
                .iter()
                .zip(cand_norm.iter())
                .map(|(orig, norm)| (*orig, combined_score_raw(&q_norm, norm)))
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.001
    }

    #[test]
    fn test_combined_score_identical() {
        assert!(approx(combined_score("cat", "cat"), 1.0));
    }

    #[test]
    fn test_combined_score_case_insensitive() {
        assert!(approx(combined_score("CAT", "cat"), 1.0));
    }

    #[test]
    fn test_combined_score_raw_skips_normalization() {
        assert!(combined_score_raw("CAT", "cat") < 1.0);
    }

    #[test]
    fn test_combined_score_raw_matches_public_on_normalized_input() {
        assert!(approx(
            combined_score_raw("cat", "bat"),
            combined_score("cat", "bat"),
        ));
    }

    #[test]
    fn test_best_match_handles_normalized_query() {
        let candidates = vec!["apple", "grape", "mango"];
        let (winner, _) = best_match("APPEL", &candidates).unwrap();
        assert_eq!(winner, "apple");
    }

    #[test]
    fn test_rank_matches_handles_normalized_query() {
        let candidates = vec!["apple", "grape", "mango"];
        let ranked = rank_matches(" APPEL ", &candidates);
        assert_eq!(ranked[0].0, "apple");
        assert_eq!(ranked.len(), 3);
    }

    #[test]
    fn test_batch_best_match_single_query() {
        let candidates = ["apple", "grape", "mango"];
        let queries = ["appel"];
        let results = batch_best_match(&queries, &candidates);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].unwrap().0, "apple");
    }

    #[test]
    fn test_batch_best_match_multiple_queries() {
        let candidates = ["apple", "grape", "mango"];
        let queries = ["appel", "grpe", "mngo"];
        let results = batch_best_match(&queries, &candidates);
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].unwrap().0, "apple");
        assert_eq!(results[1].unwrap().0, "grape");
        assert_eq!(results[2].unwrap().0, "mango");
    }

    #[test]
    fn test_batch_best_match_no_candidates() {
        let candidates: [&str; 0] = [];
        let queries = ["apple"];
        let results = batch_best_match(&queries, &candidates);
        assert_eq!(results.len(), 1);
        assert!(results[0].is_none());
    }

    #[test]
    fn test_batch_best_match_no_queries() {
        let candidates = ["apple", "grape"];
        let queries: [&str; 0] = [];
        let results = batch_best_match(&queries, &candidates);
        assert!(results.is_empty());
    }

    #[test]
    fn test_batch_best_match_agrees_with_best_match() {
        let candidates = ["apple", "grape", "mango"];
        let queries = ["appel", "grpe"];
        let batch = batch_best_match(&queries, &candidates);
        for (i, &q) in queries.iter().enumerate() {
            let serial = best_match(q, &candidates);
            assert_eq!(batch[i], serial);
        }
    }
}

pub mod python;
