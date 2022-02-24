#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None
}

impl<T> MyOption<T> {
    fn unwrap(&self) -> &T {
        match self {
            MyOption::Some(v) => v,
            MyOption::None => panic!()
        }
    }

    fn is_some(&self) -> bool {
        match self {
            MyOption::Some(_) => true,
            MyOption::None => false
        }
    }

    #[allow(dead_code)]
    fn is_none(&self) -> bool {
        !self.is_some()
    }
}

fn main() {
    let opt: MyOption<String> = MyOption::Some("CS 128 Honors".to_string());

    if opt.is_some() {
        println!("Found value in MyOption: {}", opt.unwrap());
    }

    let empty: MyOption<u8> = MyOption::None;
    empty.unwrap(); // WARNING: THIS WILL PANIC (!!!)
}
