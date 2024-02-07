use std::thread;

fn main() {
    let data = vec!["a", "b", "c"];
    let cap = thread::spawn(move || {
        let data: Vec<_> = data.iter().map(|d| d.to_ascii_uppercase()).collect();
        data
    });

    // println!("main finishing running {data:?}");

    match cap.join() {
        Ok(data) => println!("cap is joining now. data is {data:?}"),
        Err(_) => println!("Error when join"),
    }
}
