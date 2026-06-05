use pyo3::prelude::*;
use crate::{levenshtein, jaro_winkler, trigram_similarity, combined_score};
use crate::{best_match, rank_matches};

#[pyfunction]
fn py_levenshtein(a: &str, b: &str) -> usize {
    levenshtein(a, b)
}

#[pyfunction]
fn py_jaro_winkler(a: &str, b: &str) -> f64 {
    jaro_winkler(a, b)
}

#[pyfunction]
fn py_trigram_similarity(a: &str, b: &str) -> f64 {
    trigram_similarity(a, b)
}

#[pyfunction]
fn py_combined_score(a: &str, b: &str) -> f64 {
    combined_score(a, b)
}

#[pyfunction]
fn py_best_match(query: &str, candidates: Vec<String>) -> Option<(String, f64)> {
    let refs: Vec<&str> = candidates.iter().map(|s| s.as_str()).collect();
    best_match(query, &refs).map(|(s, score)| (s.to_string(), score))
}

#[pyfunction]
fn py_rank_matches(query: &str, candidates: Vec<String>) -> Vec<(String, f64)> {
    let refs: Vec<&str> = candidates.iter().map(|s| s.as_str()).collect();
    rank_matches(query, &refs)
        .into_iter()
        .map(|(s, score)| (s.to_string(), score))
        .collect()
}

#[pymodule]
fn matchr(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(py_levenshtein, m)?)?;
    m.add_function(wrap_pyfunction!(py_jaro_winkler, m)?)?;
    m.add_function(wrap_pyfunction!(py_trigram_similarity, m)?)?;
    m.add_function(wrap_pyfunction!(py_combined_score, m)?)?;
    m.add_function(wrap_pyfunction!(py_best_match, m)?)?;
    m.add_function(wrap_pyfunction!(py_rank_matches, m)?)?;
    Ok(())
}