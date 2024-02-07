use std::ops::Deref;

struct MyBox<T> {
    val: T,
}

impl<T> MyBox<T> {
    fn new(val: T) -> Self {
        return MyBox { val };
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

fn main() {
    let m = MyBox::new(String::from("abc"));
    let n = &m;
    hello(n);
}

fn hello(name: &str) {
    println!("Hello {name}");
}
