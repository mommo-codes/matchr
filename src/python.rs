use crate::{
    best_match as bm_fn, combined_score as cs_fn, jaro_winkler as jw_fn, levenshtein as lev_fn,
    rank_matches as rm_fn, trigram_similarity as tri_fn,
};
use pyo3::prelude::*;

#[pyfunction]
fn levenshtein(a: &str, b: &str) -> usize {
    lev_fn(a, b)
}

#[pyfunction]
fn jaro_winkler(a: &str, b: &str) -> f64 {
    jw_fn(a, b)
}

#[pyfunction]
fn trigram_similarity(a: &str, b: &str) -> f64 {
    tri_fn(a, b)
}

#[pyfunction]
fn combined_score(a: &str, b: &str) -> f64 {
    cs_fn(a, b)
}

#[pyfunction]
#[pyo3(signature = (query, candidates, threshold=None))]
fn best_match(
    query: &str,
    candidates: Vec<String>,
    threshold: Option<f64>,
) -> Option<(String, f64)> {
    let refs: Vec<&str> = candidates.iter().map(|s| s.as_str()).collect();
    let min = threshold.unwrap_or(0.0);
    bm_fn(query, &refs)
        .filter(|(_, score)| *score >= min)
        .map(|(s, score)| (s.to_string(), score))
}

#[pyfunction]
#[pyo3(signature = (query, candidates, threshold=None))]
fn rank_matches(
    query: &str,
    candidates: Vec<String>,
    threshold: Option<f64>,
) -> Vec<(String, f64)> {
    let refs: Vec<&str> = candidates.iter().map(|s| s.as_str()).collect();
    let min = threshold.unwrap_or(0.0);
    rm_fn(query, &refs)
        .into_iter()
        .filter(|(_, score)| *score >= min)
        .map(|(s, score)| (s.to_string(), score))
        .collect()
}

#[pyfunction]
#[pyo3(signature = (queries, candidates, threshold=None))]
fn batch_best_match(
    queries: Vec<String>,
    candidates: Vec<String>,
    threshold: Option<f64>,
) -> Vec<Option<(String, f64)>> {
    let refs: Vec<&str> = candidates.iter().map(|s| s.as_str()).collect();
    let min = threshold.unwrap_or(0.0);

    queries
        .iter()
        .map(|q| {
            bm_fn(q, &refs)
                .filter(|(_, score)| *score >= min)
                .map(|(s, score)| (s.to_string(), score))
        })
        .collect()
}

#[pymodule]
fn matchr(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(levenshtein, m)?)?;
    m.add_function(wrap_pyfunction!(jaro_winkler, m)?)?;
    m.add_function(wrap_pyfunction!(trigram_similarity, m)?)?;
    m.add_function(wrap_pyfunction!(combined_score, m)?)?;
    m.add_function(wrap_pyfunction!(best_match, m)?)?;
    m.add_function(wrap_pyfunction!(rank_matches, m)?)?;
    m.add_function(wrap_pyfunction!(batch_best_match, m)?)?;
    Ok(())
}
