fn main() {
    let mut vec: Vec<i32> = vec![];
    n_result(32, &mut vec);
    println!("{vec:?}");
}

fn n_result(n: i32, vec: &mut Vec<i32>) {
    if n == 0 {
        return;
    }
    vec.push(n);
    n_result(n - 1, vec);
}
