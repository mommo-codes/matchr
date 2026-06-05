from typing import List, Optional, Tuple

def levenshtein(a: str, b: str) -> int:
    """Levenshtein edit distance between `a` and `b`.

    Input is normalised (trimmed, lowercased, diacritics stripped)
    before comparison. Returns the minimum number of single-character
    edits (insert, delete, substitute) to turn `a` into `b`.
    """
    ...

def jaro_winkler(a: str, b: str) -> float:
    """Jaro-Winkler similarity between `a` and `b`, in `[0.0, 1.0]`.

    Gives a bonus for shared prefixes — works well for names and short
    strings. Input is normalised before comparison.
    """
    ...

def trigram_similarity(a: str, b: str) -> float:
    """Trigram (Dice coefficient) similarity in `[0.0, 1.0]`.

    Splits both strings into overlapping 3-character chunks and scores
    overlap. Good for longer strings and typo detection.
    """
    ...

def combined_score(a: str, b: str) -> float:
    """Weighted blend of Levenshtein, Jaro-Winkler, and trigram in `[0.0, 1.0]`.

    Used internally by `best_match`, `rank_matches`, and
    `batch_best_match`. Higher is more similar.
    """
    ...

def token_sort_ratio(a: str, b: str) -> float:
    """Order-invariant similarity via `combined_score` over sorted tokens.

    Useful when word order varies, e.g. `"Oat Drink Oatly 1L"` vs
    `"Oatly Oat Drink 1L"`. Returns a score in `[0.0, 1.0]`.
    """
    ...

def token_set_ratio(a: str, b: str) -> float:
    """Like `token_sort_ratio` but also deduplicates tokens before comparing.

    Useful when one string repeats words the other doesn't.
    Returns a score in `[0.0, 1.0]`.
    """
    ...

def best_match(
    query: str,
    candidates: List[str],
    threshold: Optional[float] = ...,
) -> Optional[Tuple[str, float]]:
    """Return the best `(candidate, score)` for `query`, or `None`.

    If `threshold` is set and the best score is below it, returns `None`.
    """
    ...

def rank_matches(
    query: str,
    candidates: List[str],
    threshold: Optional[float] = ...,
) -> List[Tuple[str, float]]:
    """Return all candidates ranked by similarity, highest first.

    Candidates with a score below `threshold` are filtered out.
    """
    ...

def batch_best_match(
    queries: List[str],
    candidates: List[str],
    threshold: Optional[float] = ...,
) -> List[Optional[Tuple[str, float]]]:
    """Return the best match per query — parallelised via rayon.

    The candidate list is normalised once and shared across all queries.
    Each result is `None` if no candidate scored above `threshold`.
    """
    ...
