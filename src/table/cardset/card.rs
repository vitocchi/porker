use super::suit::Suit;
use std::fmt;

// トランプのカード
pub const MIN_NUMBER: u8 = 1;
pub const MAX_NUMBER: u8 = 13;

pub struct Card {
    suit: Suit, // マーク
    number: u8, //数字
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0} : {1: >2}", self.suit, self.number)
    }
}
impl Card {
    pub fn new(suit: Suit, number: u8) -> Result<Card, String> {
        Card::check_number(number)?;
        Ok(Card { suit, number })
    }

    fn check_number(number: u8) -> Result<(), String> {
        if number > MAX_NUMBER || number < MIN_NUMBER {
            return Err(number.to_string() + " is to large as number of card!!");
        }
        return Ok(());
    }

}

#[test]
#[should_panic]
fn deny_larger_number() {
    Card::check_number(14).unwrap();
}

#[test]
#[should_panic]
fn deny_smaller_number() {
    Card::check_number(0).unwrap();
}