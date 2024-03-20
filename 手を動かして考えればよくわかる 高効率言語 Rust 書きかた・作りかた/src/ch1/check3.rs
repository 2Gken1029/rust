fn main() {
  for i in 1..51 {
    if (i % 3 == 0 || i % 10 == 30) || (i >= 30 && i <= 39) {
      println!("A");
    }else {
      println!("{}", i);
    }
  }
}