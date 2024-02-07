pub fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let plus_one: Vec<_> = numbers.iter().map(|num| num + 1).collect();
    print!("{plus_one:?}");
    println!("{numbers:?}");
}
