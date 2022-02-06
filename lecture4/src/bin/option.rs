fn main() {
    let nums: Vec<i8> = vec![1, 2, 8, 1, 9, 9];

    let _demo: Option<&i8> = nums.get(0);

    for idx in 0..7 {
        let retrieval = nums.get(idx);

        // if retrieval.is_some() {
        //     println!("{}", retrieval.unwrap());
        // }

        // if retrieval.is_none() {
        //     println!("Value doesn't exist at index {}", idx);
        // }   
        
        match retrieval {
            Some(num) => println!("{}", num),
            None => ()
        }
    }
}

#[allow(dead_code)]
fn get_discord_staff_role(name: &str) -> Option<String> {
    match name {
        "Eustis" | "Neil" => Some("Instructor".into()),
        "Welby" | "Jonah" => Some("Admin".into()),
        "Arul" | "Liza" | "Harry" | "Cooper" => Some("Staff".into()),
        _ => None
    }
}