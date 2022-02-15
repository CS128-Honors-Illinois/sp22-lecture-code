fn main() {
    let my_result: Result<i32, String> = get_result_ok();

    match my_result {
        Ok(num) => {
            let mut x = num;
            add_10(&mut x);
            println!("Got {}", x);
        },
        Err(msg) => {
            println!("Got an error with message: {}", msg);
        },
    }

    let my_result: Result<i32, String> = get_result_err();

    match my_result {
        Ok(num) => {
            let mut x = num;
            add_10(&mut x);
            println!("Got {}", x);
        },
        Err(msg) => {
            println!("Got an error with message: {}", msg);
            println!("DEBUG printing `msg`: {:?}", msg);
        },
    }
}

fn add_10(v: &mut i32) {
    *v += 10;
}

fn get_result_ok() -> Result<i32, String> {
    // do some computation...
    Ok(128)
}

fn get_result_err() -> Result<i32, String> {
    // do some computation...
    Err("Something went wrong in the computation".into())
}