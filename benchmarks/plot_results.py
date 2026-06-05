"""Plot benchmark results as a bar chart.

Usage:
    python bench_python.py > results.json
    python plot_results.py results.json benchmark_chart.png
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

import matplotlib.pyplot as plt
import numpy as np


def plot(input_path: Path, output_path: Path) -> None:
    with input_path.open() as f:
        results = json.load(f)

    labels = list(results.keys())
    matchr_us = [r.get("matchr", 0) * 1e6 for r in results.values()]
    rapidfuzz_us = [r.get("rapidfuzz", 0) * 1e6 for r in results.values()]

    x = np.arange(len(labels))
    width = 0.38

    fig, ax = plt.subplots(figsize=(12, 6))
    ax.bar(x - width / 2, matchr_us, width, label="matchr", color="#d97757")
    ax.bar(x + width / 2, rapidfuzz_us, width, label="rapidfuzz", color="#555555")
    ax.set_ylabel("Time per call (µs, log scale)")
    ax.set_yscale("log")
    ax.set_title("matchr vs rapidfuzz — lower is better")
    ax.set_xticks(x)
    ax.set_xticklabels(labels, rotation=18, ha="right")
    ax.legend(loc="upper left")
    ax.grid(axis="y", linestyle="--", alpha=0.4, which="both")
    fig.tight_layout()
    fig.savefig(output_path, dpi=150)
    print(f"Saved chart to {output_path}", file=sys.stderr)


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print(
            "Usage: python plot_results.py <results.json> <output.png>",
            file=sys.stderr,
        )
        sys.exit(1)
    plot(Path(sys.argv[1]), Path(sys.argv[2]))
