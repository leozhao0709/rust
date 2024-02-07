fn main() {
    let mut a = [2, 3, 4, 5, 6, 7, 8];
    let stream = a.chunks(2);

    // println!("{:?}", stream.next());
    for chunk in stream {
        println!("{chunk:?}")
    }

    let a_slice = &mut a[..];
    a_slice[0] = 5;
    println!("{a_slice:?}");
    println!("{a:?}");
}
