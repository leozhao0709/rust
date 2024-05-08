use std::thread;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    // let data = ["a", "b", "c"];
    // let cap = thread::spawn(move || {
    //     let data: Vec<_> = data.iter().map(|d| d.to_ascii_uppercase()).collect();
    //     data
    // });

    // // println!("main finishing running {data:?}");

    // match cap.join() {
    //     Ok(data) => println!("cap is joining now. data is {data:?}"),
    //     Err(_) => println!("Error when join"),
    // }

    let p1 = Person {
        name: "some one".to_owned(),
        age: 18,
    };

    // let _ = thread::spawn(move || println!("within thread: {:?}", &p1)).join();
    // println!("p1 age is {:?}", p1);

    thread::scope(|s| {
        s.spawn(|| println!("within thread: {:?}", p1));
    });
    println!("p1 age is {:?}", p1);
}
