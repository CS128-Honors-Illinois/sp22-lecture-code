fn main() {
  let my_bool = true;

  if my_bool {
    println!("CS 128 Honors");
  }

  let x = -5;

  if x < 0 {
    println!("x = {} is a negative number", x);
  } else if x > 0 {
    println!("x = {} is a positive number", x);
  } else {
    println!("x = 0");
  }

  let y = if x < 0 {
    -x
  } else {
    x
  };

  println!("The absolute value of {} is {}", x, y);
}