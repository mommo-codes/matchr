"""Benchmark matchr against rapidfuzz.

Usage:
    pip install matchr rapidfuzz
    python bench_python.py > results.json
"""

from __future__ import annotations

import json
import statistics
import sys
import time
from typing import Callable, Dict

try:
    from matchr import (
        batch_best_match,
        best_match,
        jaro_winkler,
        levenshtein,
        token_sort_ratio,
        trigram_similarity,
    )
except ImportError:
    print("matchr not installed — run: pip install matchr", file=sys.stderr)
    sys.exit(1)

try:
    from rapidfuzz import distance as rf_distance
    from rapidfuzz import fuzz as rf_fuzz
    from rapidfuzz import process as rf_process
except ImportError:
    print("rapidfuzz not installed — run: pip install rapidfuzz", file=sys.stderr)
    sys.exit(1)


def time_it(fn: Callable, *args, iterations: int = 10_000, repeats: int = 5) -> float:
    """Median seconds per call over `repeats` runs of `iterations` calls."""
    for _ in range(min(100, iterations)):
        fn(*args)

    timings = []
    for _ in range(repeats):
        start = time.perf_counter()
        for _ in range(iterations):
            fn(*args)
        elapsed = time.perf_counter() - start
        timings.append(elapsed / iterations)
    return statistics.median(timings)


SHORT_A = "kitten"
SHORT_B = "sitting"

PRODUCT_A = "Oatly Oat Drink 1L"
PRODUCT_B = "Oatly Oat Drink Barista 1L"

TOKEN_A = "Oat Drink Oatly 1L"
TOKEN_B = "Oatly Oat Drink 1L"


def build_catalog(size: int) -> list[str]:
    templates = [
        f"Oatly Oat Drink {flavor} 1L"
        for flavor in ("Original", "Barista", "Chocolate", "Strawberry")
    ] + [
        f"Felix Cat Food {weight}g" for weight in (200, 400, 800, 1200)
    ] + [
        f"Coca Cola {size_}cl" for size_ in (33, 50, 150, 200)
    ] + [
        f"Lay's {flavor} Chips 175g" for flavor in ("Salted", "Sour Cream", "BBQ", "Paprika")
    ]
    out = []
    while len(out) < size:
        out.extend(templates)
    return out[:size]


def main() -> None:
    results: Dict[str, Dict[str, float]] = {}

    # Single-pair scorers
    results["levenshtein (short)"] = {
        "matchr": time_it(levenshtein, SHORT_A, SHORT_B),
        "rapidfuzz": time_it(rf_distance.Levenshtein.distance, SHORT_A, SHORT_B),
    }
    results["levenshtein (product)"] = {
        "matchr": time_it(levenshtein, PRODUCT_A, PRODUCT_B),
        "rapidfuzz": time_it(rf_distance.Levenshtein.distance, PRODUCT_A, PRODUCT_B),
    }
    results["jaro-winkler (product)"] = {
        "matchr": time_it(jaro_winkler, PRODUCT_A, PRODUCT_B),
        "rapidfuzz": time_it(rf_distance.JaroWinkler.similarity, PRODUCT_A, PRODUCT_B),
    }
    results["trigram (product)"] = {
        "matchr": time_it(trigram_similarity, PRODUCT_A, PRODUCT_B),
        "rapidfuzz": time_it(rf_fuzz.QRatio, PRODUCT_A, PRODUCT_B),
    }
    results["token sort (reordered)"] = {
        "matchr": time_it(token_sort_ratio, TOKEN_A, TOKEN_B),
        "rapidfuzz": time_it(rf_fuzz.token_sort_ratio, TOKEN_A, TOKEN_B),
    }

    # best_match against larger catalogs
    catalog_1k = build_catalog(1000)
    query = "oatly oat drink"
    results["best_match (1k candidates)"] = {
        "matchr": time_it(best_match, query, catalog_1k, iterations=500),
        "rapidfuzz": time_it(
            rf_process.extractOne, query, catalog_1k, iterations=500
        ),
    }

    # Batch — 100 queries × 1k candidates
    catalog_1k = build_catalog(1000)
    queries = ["oatly oat drink", "felix cat food", "coca cola", "lays chips"] * 25

    def rapidfuzz_batch(qs, cs):
        return [rf_process.extractOne(q, cs) for q in qs]

    results["batch (100 queries × 1k)"] = {
        "matchr": time_it(batch_best_match, queries, catalog_1k, iterations=20),
        "rapidfuzz": time_it(rapidfuzz_batch, queries, catalog_1k, iterations=20),
    }

    json.dump(results, sys.stdout, indent=2)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
