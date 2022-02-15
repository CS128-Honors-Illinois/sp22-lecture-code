fn main() {
    let mut my_int: i32 = 124;

    println!("my_int before deref: {}", my_int);
    deref_int(&mut my_int);
    println!("my_int after deref: {}", my_int);

    let mut my_string: String = "hello".to_string();
    println!("my_string before deref: {}", my_string);
    deref_string(&mut my_string);
    println!("my_string after deref: {}", my_string);
}

fn deref_int(num: &mut i32) {
    *num += 4;
}

fn deref_string(s: &mut String) {
    *s = "goodbye".to_string();
    // s.push_str(" world");
}