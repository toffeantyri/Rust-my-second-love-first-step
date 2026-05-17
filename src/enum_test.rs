pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

pub fn process_message(msg: &Message) {
    match msg {
        Message::Quit => println!("Programm is ended"),
        Message::Move { x, y } => println!("Move to {} - {} ", x, y),
        Message::Write(text) => println!("Message: {}", text),
    }
}
