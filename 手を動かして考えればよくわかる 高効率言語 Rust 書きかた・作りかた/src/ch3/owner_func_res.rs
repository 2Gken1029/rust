fn main() {
  let g1 = String::from("過ちを見過ごす人は美しい");
  show_message(&g1); // 所有権ではなく参照を渡す
  println!("{}", g1); // g1に所有権がある
}

fn show_message(message: &String) {
  println!("{}", message);
}