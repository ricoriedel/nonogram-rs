#![feature(test)]

extern crate test;

use test::Bencher;
use nonogram_rs::solve;

#[bench]
fn hundred_layers_of_recursion(b: &mut Bencher) {
    let data = vec![vec![1]; 100];

    b.iter(|| {
        let cols = data.clone();
        let rows = data.clone();

       solve(cols, rows).unwrap();
    });
}