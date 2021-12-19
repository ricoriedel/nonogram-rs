use std::fs;
use std::path::Path;
use nonogram_rs::algo;
use nonogram_rs::import::Layout;

#[test]
fn apple() {
    let json = fs::read_to_string(Path::new("res/apple.json")).unwrap();
    let layout = serde_json::from_str::<Layout>(&json).unwrap();
    let nonogram = algo::solve(&layout.cols, &layout.rows).unwrap();

    println!("{}", nonogram);
}

#[test]
fn palm_tree() {
    let json = fs::read_to_string(Path::new("res/palm-tree.json")).unwrap();
    let layout = serde_json::from_str::<Layout>(&json).unwrap();
    let nonogram = algo::solve(&layout.cols, &layout.rows).unwrap();

    println!("{}", nonogram);
}

#[test]
fn teal() {
    let json = fs::read_to_string(Path::new("res/teal.json")).unwrap();
    let layout = serde_json::from_str::<Layout>(&json).unwrap();
    let nonogram = algo::solve(&layout.cols, &layout.rows).unwrap();

    println!("{}", nonogram);
}