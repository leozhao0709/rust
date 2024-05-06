pub fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let filter_number: Vec<_> = numbers
        .iter()
        .map(|num| num * 3)
        .filter(|&num| num > 10)
        .collect();
    println!("{filter_number:?}");

    for i in 1..=4 {
        println!("{i:?}");
    }
}
