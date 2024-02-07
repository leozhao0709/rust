pub fn main() {
    let mut s1 = String::from("Rust");
    print_string(&s1);

    println!("s1 is {s1}");
    let s2 = add_to_string(&mut s1);
    println!("s2 is {s2}");

    let s3 = 0.1 + 0.2;
    println!("{s3}");
    let arr1 = [1, 2, 3, 4, 5];
    println!("{arr1:?}")
}

pub fn print_string(p1: &String) {
    println!("{p1}")
}

pub fn add_to_string(p1: &mut String) -> &String {
    p1.push_str(" is awesome!");
    p1
}
