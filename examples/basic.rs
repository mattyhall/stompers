extern crate stompers;

fn main() {
    let mut conn = stompers::Connection::new("127.0.0.1", 61613).unwrap();
    let msg = stompers::Message::new("a-queue", "hello world");
    println!("{}", conn.send_message_and_wait(msg, 100));
}
