use criterion::{black_box, criterion_group, criterion_main, Criterion};
use matchr::{
    batch_best_match, best_match, combined_score, jaro_winkler, levenshtein, token_set_ratio,
    token_sort_ratio, trigram_similarity,
};

const SHORT_A: &str = "kitten";
const SHORT_B: &str = "sitting";

const PRODUCT_A: &str = "Oatly Oat Drink 1L";
const PRODUCT_B: &str = "Oatly Oat Drink Barista 1L";

const TOKEN_A: &str = "Oat Drink Oatly 1L";
const TOKEN_B: &str = "Oatly Oat Drink 1L";

fn synth_catalog(n: usize) -> Vec<&'static str> {
    let templates = [
        "Oatly Oat Drink 1L",
        "Felix Cat Food 400g",
        "Coca Cola 33cl",
        "Lay's Salted Chips 175g",
        "Kellogg's Corn Flakes 500g",
        "Nestle Aero Mint 95g",
        "Heinz Tomato Ketchup 460g",
        "Marabou Mjölkchoklad 200g",
    ];
    (0..n).map(|i| templates[i % templates.len()]).collect()
}

fn bench_levenshtein(c: &mut Criterion) {
    let mut group = c.benchmark_group("levenshtein");
    group.bench_function("short", |b| {
        b.iter(|| levenshtein(black_box(SHORT_A), black_box(SHORT_B)))
    });
    group.bench_function("product", |b| {
        b.iter(|| levenshtein(black_box(PRODUCT_A), black_box(PRODUCT_B)))
    });
    group.finish();
}

fn bench_jaro_winkler(c: &mut Criterion) {
    let mut group = c.benchmark_group("jaro_winkler");
    group.bench_function("short", |b| {
        b.iter(|| jaro_winkler(black_box(SHORT_A), black_box(SHORT_B)))
    });
    group.bench_function("product", |b| {
        b.iter(|| jaro_winkler(black_box(PRODUCT_A), black_box(PRODUCT_B)))
    });
    group.finish();
}

fn bench_trigram(c: &mut Criterion) {
    let mut group = c.benchmark_group("trigram_similarity");
    group.bench_function("short", |b| {
        b.iter(|| trigram_similarity(black_box(SHORT_A), black_box(SHORT_B)))
    });
    group.bench_function("product", |b| {
        b.iter(|| trigram_similarity(black_box(PRODUCT_A), black_box(PRODUCT_B)))
    });
    group.finish();
}

fn bench_combined_score(c: &mut Criterion) {
    c.bench_function("combined_score/product", |b| {
        b.iter(|| combined_score(black_box(PRODUCT_A), black_box(PRODUCT_B)))
    });
}

fn bench_token_scorers(c: &mut Criterion) {
    let mut group = c.benchmark_group("token_scorers");
    group.bench_function("sort_ratio", |bench| {
        bench.iter(|| token_sort_ratio(black_box(TOKEN_A), black_box(TOKEN_B)))
    });
    group.bench_function("set_ratio", |bench| {
        bench.iter(|| token_set_ratio(black_box(TOKEN_A), black_box(TOKEN_B)))
    });
    group.finish();
}

fn bench_best_match(c: &mut Criterion) {
    let mut group = c.benchmark_group("best_match");
    for size in [100, 1000, 10_000] {
        let candidates = synth_catalog(size);
        group.bench_function(format!("{}_candidates", size), |b| {
            b.iter(|| best_match(black_box("oatly oat drink"), black_box(&candidates)))
        });
    }
    group.finish();
}

fn bench_batch_best_match(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_best_match");
    let candidates = synth_catalog(1000);
    for n_queries in [10, 100, 1000] {
        let queries: Vec<&str> = (0..n_queries)
            .map(|i| match i % 4 {
                0 => "oatly oat drink",
                1 => "felix cat food",
                2 => "coca cola",
                _ => "lays chips",
            })
            .collect();
        group.bench_function(format!("{}_queries_x_1000_candidates", n_queries), |b| {
            b.iter(|| batch_best_match(black_box(&queries), black_box(&candidates)))
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_levenshtein,
    bench_jaro_winkler,
    bench_trigram,
    bench_combined_score,
    bench_token_scorers,
    bench_best_match,
    bench_batch_best_match,
);
criterion_main!(benches);
