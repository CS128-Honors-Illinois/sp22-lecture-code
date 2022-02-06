use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename: String = "src/main.txt".into();
    println!("Printing file {}\n", filename);

    let mut line_num: i32 = 1;

    // let file = File::open(filename)
    //     .expect("Something went wrong reading the file");
    
    let file = match File::open(filename.clone()) {
        Ok(f) => f,
        Err(_) => {
            File::create(filename).unwrap()
        }
    };

    let contents = BufReader::new(file);

    for line in contents.lines() {
        match line {
            Ok(line_str) => println!("{:2} {}", line_num, line_str),
            Err(_) => ()
        }
        line_num += 1;
    }
}