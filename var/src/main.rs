use time;
const MAX: usize = usize::max_value();

fn main() {
    let a = 1;
    println!("a = {}", a);

    let mut b: u32 = 2;
    println!("b = {}", b);
    b = a;
    println!("b = {}", b);

    let b = 1.1;
    println!("b = {}", b);

    println!("{}", MAX);

    println!("{}", time::now().ctime());
}
