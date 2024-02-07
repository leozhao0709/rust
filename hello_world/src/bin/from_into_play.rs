#[derive(Debug)]
struct Uppercase(String);

impl From<&str> for Uppercase {
    fn from(value: &str) -> Self {
        Uppercase(value.to_uppercase())
    }
}

impl From<String> for Uppercase {
    fn from(value: String) -> Self {
        Uppercase(value)
    }
}

fn main() {
    let a = Uppercase::from("1234");
    print!("{:?}", a.0);
    let b = Uppercase::from(String::from("123"));
}
