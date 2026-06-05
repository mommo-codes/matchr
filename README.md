# matchr

[![PyPI](https://img.shields.io/pypi/v/matchr)](https://pypi.org/project/matchr/)

Fast fuzzy string matching — written in Rust, usable from Python.

## Install

```bash
pip install matchr
```

## Python quick start

```python
from matchr import best_match, rank_matches, batch_best_match

# find the closest match
best_match("oatly oat drink", ["Oatly Oat Drink 1L", "Oat Milk", "Oatly Barista"])
# → ('Oatly Oat Drink 1L', 0.902)

# filter weak matches with a threshold
best_match("xyz gibberish", ["Oatly Oat Drink 1L"], threshold=0.7)
# → None

# match many queries at once
batch_best_match(["oatly oat drink", "felix cat food"], catalog, threshold=0.7)
# → [('Oatly Oat Drink 1L', 0.902), ('Felix Cat Food 400g', 0.843)]
```

## Rust usage

```rust
use matchr::{levenshtein, jaro_winkler, trigram_similarity};

fn main() {
    println!("{}", levenshtein("cat", "bat"));           // 1
    println!("{}", jaro_winkler("martha", "marhta"));    // 0.961
    println!("{}", trigram_similarity("hello", "helo")); // 0.4
}
```

## Algorithms

- **Levenshtein** — minimum edit distance between two strings. Lower = more similar.
- **Jaro-Winkler** — similarity score from `0.0` to `1.0`, optimised for names and short strings. Gives a bonus for shared prefixes.
- **Trigram** — splits strings into overlapping 3-character chunks, scores overlap using the Dice coefficient. Good for longer strings and typo detection.
- **Token sort / token set** — order-invariant scorers built on top of `combined_score`. Sorts (and optionally deduplicates) whitespace-separated tokens before comparing, so `"Oat Drink Oatly 1L"` and `"Oatly Oat Drink 1L"` come out equal.
- **Combined score** — weighted blend of Levenshtein, Jaro-Winkler, and trigram. Used internally by `best_match`, `rank_matches`, and `batch_best_match`.

## Benchmarks

See [`benchmarks/`](benchmarks/) for the criterion suite (`cargo bench`) and a Python comparison vs `rapidfuzz`. There's also a [demo notebook](benchmarks/demo.ipynb) that walks through deduping a synthetic 10k-product catalog end-to-end.

## Notes

- All functions normalise input (trim + lowercase + strip diacritics) before comparing — `"Café"` and `"cafe"` are treated as equal
- `levenshtein` returns `usize` (edit distance), all others return `f64` (0.0–1.0)
- Python functions accept an optional `threshold` parameter — results below it are filtered out
- `batch_best_match` is parallelised via rayon and pre-normalises the candidate list once, so it scales with available cores