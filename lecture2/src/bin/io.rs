use std::io;

fn main() {
  let mut buffer = String::new();
  println!("What's your name?");

  io::stdin().read_line(&mut buffer)
    .expect("Unable to read from stdin");

  println!("Hello {}", buffer);
}
