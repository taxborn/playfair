use criterion::{criterion_group, criterion_main, Criterion};
use playfair::*;

#[inline]
fn keyword_generation() {
    let _ = Playfair::new("playfair example");
}

#[inline]
fn encrypt() {
    let pf = Playfair::new("playfair example");
    pf.encrypt("hide the gold in the tree stump");
}

#[inline]
fn decrypt() {
    let pf = Playfair::new("playfair example");
    pf.decrypt("bmodzbxdnabekudmuixmmouvif");
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("keyword generation", |b| b.iter(|| keyword_generation()));
    c.bench_function("encrypt", |b| b.iter(|| encrypt()));
    c.bench_function("decrypt", |b| b.iter(|| decrypt()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
