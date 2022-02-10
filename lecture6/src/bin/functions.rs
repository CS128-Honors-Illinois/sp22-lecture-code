fn main() {
    let class = "CS 128 Honors".to_string();

    say_hello(&class);

    // ERROR: value used here after move
    say_hello(&class);

    // class still has ownership
}

fn say_hello(name: &String) {
    println!("Hello {}!", name);
} // name is dropped
