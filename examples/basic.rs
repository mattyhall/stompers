extern crate stomp;

use stomp::connect;
use stomp::message;

fn main() {
    let mut conn = connect::Connection::new("127.0.0.1", 61613).unwrap();
    let msg = message::Message::new("a-queue", "hello world");
    conn.send_message(msg);
}

