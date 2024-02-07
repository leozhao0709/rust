fn main() {
    let a: Result<&str, &str> = Err("some error");
    match a {
        Err(e) => println!("{e:?}"),
        _ => (),
    }
}
