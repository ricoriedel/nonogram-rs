#[cfg(feature = "serde")]
mod demo {
    use nonogram_rs::Layout;

    #[test]
    fn apple() {
        let json = include_str!("../res/apple.json");
        let layout: Layout<char> = serde_json::from_str(json).unwrap();

        assert_eq!(1, layout.solve(usize::MAX, ()).collection.len());
    }

    #[test]
    fn apple_color() {
        let json = include_str!("../res/apple-color.json");
        let layout: Layout<char> = serde_json::from_str(json).unwrap();

        assert_eq!(3, layout.solve(usize::MAX, ()).collection.len());
    }

    #[test]
    fn palm() {
        let json = include_str!("../res/palm.json");
        let layout: Layout<char> = serde_json::from_str(json).unwrap();

        assert_eq!(1, layout.solve(usize::MAX, ()).collection.len());
    }

    #[test]
    fn palm_color() {
        let json = include_str!("../res/palm-color.json");
        let layout: Layout<char> = serde_json::from_str(json).unwrap();

        assert_eq!(2, layout.solve(usize::MAX, ()).collection.len());
    }

    #[test]
    fn colors() {
        let json = include_str!("../res/colors.json");
        let layout: Layout<char> = serde_json::from_str(json).unwrap();

        assert_eq!(1, layout.solve(usize::MAX, ()).collection.len());
    }

    #[test]
    fn flower() {
        let json = include_str!("../res/flower.json");
        let layout: Layout<char> = serde_json::from_str(json).unwrap();

        assert_eq!(1, layout.solve(usize::MAX, ()).collection.len());
    }
}
