# Benchmarks

## Rust (criterion)

```bash
cargo bench
```

HTML reports land in `target/criterion/`.

## Python (vs rapidfuzz)

```bash
pip install matchr rapidfuzz matplotlib numpy
python benchmarks/bench_python.py > benchmarks/results.json
python benchmarks/plot_results.py benchmarks/results.json benchmarks/benchmark_chart.png
```

The chart at `benchmarks/benchmark_chart.png` is embedded in the root README.

## What gets measured

| Benchmark | What it shows |
|---|---|
| `levenshtein (short/product)` | Edit-distance perf on short vs realistic-length strings |
| `jaro-winkler (product)` | JW perf vs `rapidfuzz.distance.JaroWinkler.similarity` |
| `trigram (product)` | Trigram vs `rapidfuzz.fuzz.QRatio` (closest analogue) |
| `token sort (reordered)` | Order-invariant matching for product names |
| `best_match (1k candidates)` | One query against a 1000-item catalog |
| `batch (100 queries × 1k)` | matchr's rayon-parallelised batch vs a Python loop |
