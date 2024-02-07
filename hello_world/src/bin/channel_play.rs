use std::{sync::mpsc, thread};

enum WorkerMsg {
    PrintData(String),
    Sum(i32, i32),
    Quit,
}

enum MainMsg {
    SumResult(i32),
    WorkerQuit,
}

fn main() {
    let (worker_tx, worker_rx) = mpsc::channel::<WorkerMsg>();
    let (main_tx, main_rx) = mpsc::channel::<MainMsg>();

    let s = thread::spawn(move || loop {
        match worker_rx.recv() {
            Ok(msg) => match msg {
                WorkerMsg::PrintData(d) => println!("worker: {d}"),
                WorkerMsg::Sum(a, b) => {
                    println!("worker: calculating...");
                    main_tx.send(MainMsg::SumResult(a + b)).unwrap();
                }
                WorkerMsg::Quit => {
                    println!("worker: is terminating");
                    main_tx.send(MainMsg::WorkerQuit).unwrap();
                    break;
                }
            },
            Err(_) => {
                println!("worker: disconnected");
                main_tx.send(MainMsg::WorkerQuit).unwrap();
                break;
            }
        }
    });

    worker_tx
        .send(WorkerMsg::PrintData("hello from main".to_owned()))
        .unwrap();
    worker_tx.send(WorkerMsg::Sum(2, 2)).unwrap();
    worker_tx.send(WorkerMsg::Quit).unwrap();
    // drop(tx);

    while let Ok(msg) = main_rx.recv() {
        match msg {
            MainMsg::SumResult(r) => println!("Main: answer = {r}"),
            MainMsg::WorkerQuit => println!("Main: worker terminated"),
        }
    }

    let _ = s.join();
}
