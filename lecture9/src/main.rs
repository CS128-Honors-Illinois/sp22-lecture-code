#[derive(Clone, Default, Debug, PartialEq)]
struct Student {
    name: String,
    netid: String,
    major: String
}

impl Student {
    fn new(name: String, netid: String, major: String) -> Student {
        Student {
            name,
            netid,
            major
        }
    }

    fn introduce(&self) {
        println!("Hi, I'm {} and I'm am {} major!", self.name, self.major);
    }

    fn change_major(&mut self, new_major: String) {
        self.major = new_major;
    }
}

fn main() {
    // let mut s: Student = Student {
    //     name: "Neil".to_string(),
    //     netid: "neilk3".to_string(),
    //     major: "CS".to_string()
    // };
    let mut s = Student::new("Neil".to_string(), "neilk3".to_string(), "CS".to_string());
    let mut x= s.clone();
    x.change_major("ECE".into());
    if x == s {
        println!("equal");
    }

    format!("{:?}", x);

    s.introduce();
    s.change_major("CS + Stats".into());
    s.introduce();
}

// fn iter() {
//     let mut v: Vec<u8> = vec![1, 2, 3, 4];

//     let mut sum = 0;

//     for idx in 2..v.len()  {
//         sum += v[idx];
//     }
// }