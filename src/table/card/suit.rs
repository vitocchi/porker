use std::fmt;

pub enum Suit {
    Spade,
    Diamond,
    Club,
    Heart,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_symbol())
    }
}

impl Suit {
    fn get_symbol(&self) -> String {
        match self {
            Suit::Spade => String::from("♤"),
            Suit::Diamond => String::from("♢"),
            Suit::Club => String::from("♧"),
            Suit::Heart => String::from("♡"),
        }
    }
}
