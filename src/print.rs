pub fn run() {
  // Print to console
  println!("hello from the print.rs file.");

  println!("Number: {0}, {1} {1}", 1, "yeet");

  println!("Number: {num}, {word} {word}", num = 1, word = "yeet");

  println!("Stuff: {:?}", (true, false, "left", 232));

  println!("10+11={}", 10+11);
}