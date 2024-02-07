fn main() {
    let a = Some(100);
    match a {
        Some(s @ 10 | s @ 9) => println!("a is 10 or 11: {s}"),
        Some(s @ 11..=20) => println!("a is within 11..20: {s}"),
        None => println!("None"),
        _ => println!("a is another number"),
    }
}
