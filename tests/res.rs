use std::fs;
use std::path::Path;
use nonogram_rs::import::import;

#[test]
fn apple() {
    let json = fs::read_to_string(Path::new("res/apple.json")).unwrap();
    let layout = import(&json).unwrap();
    let nonogram = layout.solve().unwrap();

    println!("{}", nonogram);
}

#[test]
fn palm_tree() {
    let json = fs::read_to_string(Path::new("res/palm-tree.json")).unwrap();
    let layout = import(&json).unwrap();
    let nonogram = layout.solve().unwrap();

    println!("{}", nonogram);
}

#[test]
fn teal() {
    let json = fs::read_to_string(Path::new("res/teal.json")).unwrap();
    let layout = import(&json).unwrap();
    let nonogram = layout.solve().unwrap();

    println!("{}", nonogram);
}