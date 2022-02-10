fn main() {
    let mut x: String = String::from("hello");

    let y = &mut x;
  
    // ERROR: value borrowed after move
    x.push_str(" world!");

    println!("x = {} and y = {}", x, y);
}