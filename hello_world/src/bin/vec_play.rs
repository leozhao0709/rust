#[derive(Debug)]
struct People {
    name: String,
}

pub fn main() {
    let mut a = vec![1, 2, 3, 4, 5];
    let b = 6;
    a.push(b);
    println!("{b}");
    println!("arr is {a:?}");

    let mut b = vec![];
    for (i, val) in a.iter().enumerate() {
        println!("moving index {i} value {val} from a to b");
        b.push(val)
    }

    println!("a[0] is {}", a[0]);

    b.iter().for_each(|v| println!("{v}"));
    vec![1, 2, 3]
        .iter()
        .map(|&v| v * 2)
        .filter(|v| v % 2 == 1)
        .for_each(|v| println!("{v}"));

    let c = vec![
        People {
            name: "p1".to_owned(),
        },
        People {
            name: "p2".to_owned(),
        },
    ];

    for p in c.iter() {
        println!("{p:?}");
    }
    println!("{c:?}");

    println!("{}", vec![1, 2, 3] == vec![1, 2, 3]);

    // let mut v = vec![1, 2, 3, 4];

    // let first = {
    //     let temp_first = &v[0];
    //     *temp_first // Dereference to get the value
    // };

    // v.push(5);

    // println!("first = {first}");

    let mut a = vec![1, 2, 3, 4, 5];
    // for i in 0..a.len() {
    //     if a[i] < 3 {
    //         a[i] += 10
    //     }
    // }

    let b = &mut a;
    b[0] += 10;
    print!("{a:?}");
}
