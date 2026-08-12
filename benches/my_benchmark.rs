use criterion::{criterion_group, criterion_main, Criterion};
use playfair::*;

/// Benchmark building the 5x5 matrix from a keyword.
#[inline]
fn keyword_generation() {
    let _ = Playfair::new("playfair example");
}

/// Benchmark a full encryption pass, including keyword generation.
#[inline]
fn encrypt() {
    let pf = Playfair::new("playfair example");
    pf.encrypt("hide the gold in the tree stump");
}

/// Benchmark a full decryption pass, including keyword generation.
#[inline]
fn decrypt() {
    let pf = Playfair::new("playfair example");
    pf.decrypt("bmodzbxdnabekudmuixmmouvif");
}

/// Register each benchmark with criterion.
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("keyword generation", |b| b.iter(keyword_generation));
    c.bench_function("encrypt", |b| b.iter(encrypt));
    c.bench_function("decrypt", |b| b.iter(decrypt));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
