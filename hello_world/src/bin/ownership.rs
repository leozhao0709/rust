fn main() {
    let s1 = String::from("rust");
    let s2 = s1.clone();
    let s3 = add_to_string(s2);

    println!("s1 is {s1}");
    // println!("s2 is {s2}");
    println!("s3 is {s3}");
}

fn add_to_string(mut p: String) -> String {
    p.push_str(" is awesome!");
    p
}
