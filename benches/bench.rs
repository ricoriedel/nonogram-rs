use criterion::{criterion_group, criterion_main, Criterion};
use nonogram_rs::Layout;
use std::fs::read_to_string;

fn bench_res(c: &mut Criterion, name: &str) {
    let path = format!("res/{}.json", name);
    let json = read_to_string(path).unwrap();
    let layout: Layout<char> = serde_json::from_str(&json).unwrap();

    c.bench_function(name, |b| b.iter(|| layout.clone().solve(usize::MAX, ())));

    let swapped_name = format!("{}-swapped", name);
    let swapped = Layout {
        cols: layout.rows,
        rows: layout.cols,
    };
    c.bench_function(&swapped_name, |b| {
        b.iter(|| swapped.clone().solve(usize::MAX, ()))
    });
}

fn apple(c: &mut Criterion) {
    bench_res(c, "apple");
}

fn apple_color(c: &mut Criterion) {
    bench_res(c, "apple-color");
}

fn palm(c: &mut Criterion) {
    bench_res(c, "palm");
}

fn palm_color(c: &mut Criterion) {
    bench_res(c, "palm-color");
}

fn flower(c: &mut Criterion) {
    bench_res(c, "flower");
}

criterion_group!(res, apple, apple_color, palm, palm_color, flower);
criterion_main!(res);
