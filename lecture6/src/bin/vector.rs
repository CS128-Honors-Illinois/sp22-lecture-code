fn main() {
    let mut names: Vec<String> = vec!["Eustis".into(), "Welby".into(), "Neil".into()];

    // let eustis = &mut names[0];
    // eustis.push_str("!!!");
    // println!("{}", names[0]);

    for name in names.iter() {
        println!("{}", name);
    }
}