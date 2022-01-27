/// doc comment - notice the 3 slashes
fn main() {
  // this is an inline comment
  let cmd: &str = "cargo run --bin <filename without .rs>";

  /*
   * multi
   * line
   * comment
   */
  println!("Check out ./lecture2/src/bin/ for more code examples!");
  println!("You can run any file with a main() function using: `{}`", cmd);

  println!("Ex: run the int.rs file using `cargo run --bin int`");
}
