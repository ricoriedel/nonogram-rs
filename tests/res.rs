#[cfg(all(feature = "serde", feature = "serde_json"))]
mod demo {
    use nonogram_rs::Layout;

    #[test]
    fn apple() {
        let json = include_str!("../res/apple.json");
        let layout: Layout<char> = serde_json::from_str(json).unwrap();

        assert!(layout.solve().is_ok());
    }

    #[test]
    fn apple_color() {
        let json = include_str!("../res/apple-color.json");
        let layout: Layout<char> = serde_json::from_str(json).unwrap();

        assert!(layout.solve().is_ok());
    }

    #[test]
    fn palm() {
        let json = include_str!("../res/palm.json");
        let layout: Layout<char> = serde_json::from_str(json).unwrap();

        assert!(layout.solve().is_ok());
    }

    #[test]
    fn palm_color() {
        let json = include_str!("../res/palm-color.json");
        let layout: Layout<char> = serde_json::from_str(json).unwrap();

        assert!(layout.solve().is_ok());
    }

    #[test]
    fn colors() {
        let json = include_str!("../res/colors.json");
        let layout: Layout<char> = serde_json::from_str(json).unwrap();

        assert!(layout.solve().is_ok());
    }
}