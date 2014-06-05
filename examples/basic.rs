extern crate stomp;

fn main() {
    let mut conn = stomp::Connection::new("127.0.0.1", 61613).unwrap();
    let msg = stomp::Message::new("a-queue", "hello world");
    println!("{}", conn.send_message(msg));
}
