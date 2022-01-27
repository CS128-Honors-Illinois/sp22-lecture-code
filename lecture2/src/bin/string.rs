fn main() {
  let _empty = String::new();
  let ill: String = String::from("I-L-L");
  let a_str: &str = "Fighting";

  let converted_str: &str = ill.as_str();
  let converted_string: String = a_str.to_string();

  let ini: String = "I-N-I".to_string();
  let illini: String = "Illini".into();

  let rally: String = format!("When I say {}, you say {}!", ill, ini);

  println!("{} {}!", a_str, illini);
  print!("{}\n", rally);

  println!();

  println!("ill (String) is formatted the same as converted_str (&str): '{}' vs '{}'", ill, converted_str);
  println!("a_str (&str) is formatted the same as converted_string (string): '{}' vs '{}'", a_str, converted_string);
}