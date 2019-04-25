extern crate porkerlib;

use porkerlib::table;
fn main() {
    // ゲームを初期化する (デッキに整列されたカード52枚、手札0枚)
    let mut table = table::Table::new();

    // デッキをシャッフルする
    table.shuffle_deck();

    // カードを5枚引く
    table.pick_from_deck().unwrap();
    table.pick_from_deck().unwrap();
    table.pick_from_deck().unwrap();
    table.pick_from_deck().unwrap();
    table.pick_from_deck().unwrap();

    // 出力
    println!("{}", table);
}
