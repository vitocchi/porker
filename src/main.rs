extern crate porkerlib;

use porkerlib::table;
fn main() {
    // ゲームを初期化する (デッキに整列されたカード52枚、手札0枚)
    let mut table = table::Table::new();

    // デッキをシャッフルする
    table.shuffle_deck();

    // カードを5枚引く
    table.init_hand().unwrap();

    // 出力
    print!("{}", table.hand);
    println!("{}", table.get_variant().unwrap());
}
