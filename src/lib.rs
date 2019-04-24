// トランプのカード
#[derive(Debug)]
pub struct Card {
    suit: Suit, // マーク
    number: u8, //数字
}

// トランプのマーク
#[derive(Debug)]
pub enum Suit {
    Spade,
    Diamond,
    Club,
    Heart
}

impl Card {
    pub fn new(suit: Suit, number:u8) -> Result<Card, String> {
        Card::check_number(number)?;
        Ok(Card{suit, number})
    }

    fn check_number(number:u8) -> Result<(), String> {
        if number > 13 || number == 0 {
            return Err(number.to_string() + " is to large as number of card!!");
        }
        return Ok(())
    }
}