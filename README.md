# matchr

Fast fizzy string matching in Rust.

Algorithms:
- **Levenshtein**: minimum edit distance between two strings
- **Jaro-Winkler**: similarity score optimised for names and short strings
- **Trigram**: character n-gram overlap, for typo detection

## Usage

```rust
use matchr::{levenshtein, jaro_winkler, trigram_similarity};

fn main() {
    println!("{}", levenshtein("cat", "bat"));           // 1
    println!("{}", jaro_winkler("martha", "marhta"));    // 0.961
    println!("{}", trigram_similarity("hello", "helo")); // 0.4
}
```

## Algorithm

### Levenshtein
Counts the minimum single-character edits (insert, delete, substitute)
to transform one string into another. Lower = more similar.

### Jaro-Winkler
Returns a score from `0.0` (no similarity) to `1.0` (identical).
Gives a bonus when strings share a common prefix - works well for names.

### Trigram Similarity
Splits strings into overlapping 3-character chunks and scores overlap
using the Dice coefficient. Returns `0.0`–`1.0`. Good for longer strings.

## Notes
- All functions normalise input (lowercase + trim) before comparing
- `levenshtein` returns `usize`, similarity functions return `f64`