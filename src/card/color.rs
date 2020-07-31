#[derive(Copy, Clone, Debug)]
enum Color {
    Green,
    White,
    Blue,
    Black,
    Red,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::Green => "G",
            Color::White => "W",
            Color::Black => "B",
            Color::Blue => "U",
            Color::Red => "R",
        }.to_string()
    }
}

impl Color {
    pub fn from(s: &str) -> Result<Color, ()> {
        match s {
            "G" => Ok(Color::Green),
            "W" => Ok(Color::White),
            "B" => Ok(Color::Black),
            "U" => Ok(Color::Blue),
            "R" => Ok(Color::Red),
            _ => Err(())
        }
    }
}
