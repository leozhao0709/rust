fn main() {
    let num: i64 = 1234567890;
    let mut sum = 0;
    for i in 0..=num {
        sum += i;
    }
    println!("sum is {sum}");
}
