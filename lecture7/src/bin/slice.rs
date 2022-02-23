fn main() {
    let s = String::from("hello world");

    let hello = &s[..5];  // same as &s[..5]
    let world = &s[6..]; // same as &s[6..]
    let hello_world = &s;

    println!("{}", hello);
    println!("{}", world);
    println!("{}", hello_world);

    println!("---------");

    vector_slice();
}   

fn vector_slice() {
    let v: Vec<i32> = vec![1, 2, 8, 1, 9, 9];
    
    let my_slice: &[i32] = &v[..3];

    for val in my_slice {
        print!("{} ", val);
    }

    println!("");

    for val in &v[2..5] {
        print!("{} ", val);
    }

    println!("");
}