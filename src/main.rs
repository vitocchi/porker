// トランプのカード
#[derive(Debug)]
struct Card {
    suit: Suit, // マーク
    number: u8, //数字
}

// トランプのマーク
#[derive(Debug)]
enum Suit {
    Spade,
    Diamond,
    Club,
    Heart
}

impl Card {
    fn new(suit: Suit, number:u8) -> Result<Card, String> {
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


fn main() {
    println!("Hello, world!");
    let card = Card::new(Suit::Club, 0).unwrap();
    println!("{:?}", card);
}