extern crate stompers;

use std::io::timer::sleep;

fn main() {
    let mut conn = stompers::Connection::new("127.0.0.1", 61613).unwrap();
    conn.subscribe("a-queue");
    let msg = stompers::Message::new("a-queue", "hello world");
    conn.send_message(msg);
    println!("Message sent");
    // Sleep so that the Task has time to receive and print the message.
    sleep(50);
}
