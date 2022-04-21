#[cfg(all(feature = "serde", feature = "serde_json"))]
mod serde {
    use nonogram_rs::serde::Layout;

    #[test]
    fn apple() {
        let json = include_str!("../res/apple.json");
        let layout: Layout = serde_json::from_str(json).unwrap();

        layout.solve().unwrap();
    }

    #[test]
    fn palm_tree() {
        let json = include_str!("../res/palm-tree.json");
        let layout: Layout = serde_json::from_str(json).unwrap();

        layout.solve().unwrap();
    }

    #[test]
    fn teal() {
        let json = include_str!("../res/teal.json");
        let layout: Layout = serde_json::from_str(json).unwrap();

        layout.solve().unwrap();
    }
}