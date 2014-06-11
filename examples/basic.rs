extern crate stompers;

use std::io::timer::sleep;

fn listener(msg: stompers::Message) {
    println!("Listener got message: {}", msg);
}

fn main() {
    let mut conn = stompers::Connection::new("127.0.0.1", 61613).unwrap();
    conn.subscribe("a-queue", listener);
    let mut msg = stompers::Message::new("a-queue", "hello\nworld");
    // this library will escape characters in the header when needed
    msg.add_header("nasty\n:", "header");
    conn.send_message(msg);
    println!("Message sent");
    // Sleep so that the Task has time to receive and print the message.
    sleep(50);
}
