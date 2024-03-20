// Item構造体（タプル）を定義
struct Item(String, i64);
fn main() {
  // タプルを作る
  let banana = Item("バナナ".to_string(), 300);
  let apple = Item("リンゴ".to_string(), 200);
  let mango = Item("マンゴー".to_string(), 500);

  // Itemをベクターに追加
  let items = vec![banana, apple, mango];

  // 合計金額を求める
  let total = print_and_sum_items(&items);
  println!("合計{}円です", total);
}

// タプルを表示する関数
fn print_tuple(item: &Item){
  println!("{}を{}円で購入", item.0, item.1);
}

// アイテムを順に表示し合計金額を求める
fn print_and_sum_items(items: &Vec<Item>) -> i64 {
  let mut total = 0;
  for it in items {
    print_tuple(&it);
    total += it.1;
  }
  return total;
}