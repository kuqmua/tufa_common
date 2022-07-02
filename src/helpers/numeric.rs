#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Numeric {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl std::fmt::Display for Numeric {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Numeric::Zero => write!(f, "0"),
            Numeric::One => write!(f, "1"),
            Numeric::Two => write!(f, "2"),
            Numeric::Three => write!(f, "3"),
            Numeric::Four => write!(f, "4"),
            Numeric::Five => write!(f, "5"),
            Numeric::Six => write!(f, "6"),
            Numeric::Seven => write!(f, "7"),
            Numeric::Eight => write!(f, "8"),
            Numeric::Nine => write!(f, "9"),
        }
    }
}
