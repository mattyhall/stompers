extern crate stomp;

fn main() {
    let conn = stomp::connect::Connection::new("127.0.0.1", 61613).unwrap();
}

