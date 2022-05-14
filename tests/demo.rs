#[cfg(all(feature = "serde", feature = "serde_json"))]
mod demo {
    use nonogram_rs::Layout;

    #[test]
    fn apple() {
        let json = include_str!("apple.json");
        let layout: Layout<()> = serde_json::from_str(json).unwrap();

        assert!(layout.solve().is_ok());
    }
}