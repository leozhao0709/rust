use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

struct Station {
    tickets: i32,
}

fn main() {
    let station = Arc::new(Mutex::new(Station { tickets: 100 }));
    let mut threads = vec![];

    for i in 0..4 {
        let station_clone = station.clone();
        let th = thread::spawn(move || {
            loop {
                let mut station = station_clone.lock().unwrap();
                if station.tickets == 0 {
                    break;
                }
                station.tickets -= 1;
                println!(
                    "Thread {i} sold a ticket, tickets left: {}",
                    station.tickets
                );
                // Release the lock as soon as possible
                drop(station);
                // Introduce some delay to simulate work and give other threads a chance to work
                thread::sleep(Duration::from_millis(50));
            }
        });
        threads.push(th);
    }

    for th in threads {
        th.join().unwrap();
    }

    println!("All tickets sold out!");
}
