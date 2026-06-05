pub mod normalize;
pub mod levenshtein;
pub mod jaro_winkler;
pub mod trigram;

pub use normalize::normalize;
pub use levenshtein::levenshtein;
pub use jaro_winkler::jaro_winkler;
pub use trigram::trigram_similarity;