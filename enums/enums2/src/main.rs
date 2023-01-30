#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move { x: u8, y: u8 },
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo("hello world".to_owned()),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in messages {
        message.call();
    }
}

