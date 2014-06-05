extern crate stomp;

use stomp::connection;
use stomp::message;

fn main() {
    let mut conn = connection::Connection::new("127.0.0.1", 61613).unwrap();
    let msg = message::Message::new("a-queue", "hello world");
    println!("{}", conn.send_message(msg));
}
