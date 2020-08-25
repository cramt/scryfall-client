#[cfg(test)]
mod decklist {
    use crate::decklist::unpopulated_decklist::UnpopulatedDecklist;
    use tokio_test::block_on;

    #[test]
    fn fetch_novas_cube() {
        let cube = UnpopulatedDecklist::parse_raw(include_str!("polycube")).unwrap();
        let initial_size = cube.main.len();
        let cube = block_on(cube.populate()).unwrap();
        let new_size = cube.main.len();
        assert_eq!(initial_size, new_size);
    }
}
