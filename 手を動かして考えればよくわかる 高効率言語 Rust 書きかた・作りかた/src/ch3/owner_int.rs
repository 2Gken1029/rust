fn main() {
  let g1 = 30;
  let g2 = g1;
  println!("{}", g1); // プリミディブは所有権システムがない
  println!("{}", g2);
}