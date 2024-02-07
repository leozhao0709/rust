enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

impl Message {
    fn print(&self) {
        match self {
            Self::Quit => println!("Quit"),
            Self::Move { x, y } => println!("Move x={x}, y={y}"),
            Self::Write(s) => println!("Write: {s}"),
        }
    }
}

fn main() {
    let quit = Message::Quit;
    quit.print();

    let moving = Message::Move { x: 2, y: 3 };
    moving.print();

    let write = Message::Write("abc".to_owned());
    write.print();
}
