use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  let filename: String = "src/main.rs".into();
  println!("Printing file {}\n", filename);

  let mut line_num: i32 = 1;

  let file = File::open(filename)
    .expect("Something went wrong reading the file");
  let contents = BufReader::new(file);

  for line in contents.lines() {
    println!("{:2} {}", line_num, line.unwrap());
    line_num += 1;
  }
}