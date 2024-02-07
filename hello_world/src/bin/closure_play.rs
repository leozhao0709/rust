fn main() {
    let factor = 5;
    println!("{}", math(2, 2, |a, b| (a + b) * factor));
    // let op1 = Box::new(|a, b| (a - b) * factor); // not working
    // println!("{}", math2(2, 2, op1));
    println!("{factor}");
}
fn math<T>(a: i32, b: i32, op: T) -> i32
where
    T: Fn(i32, i32) -> i32,
{
    op(a, b)
}
fn math2(a: i32, b: i32, op: impl Fn(i32, i32) -> i32) -> i32 {
    op(a, b)
}
fn math3(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
}
