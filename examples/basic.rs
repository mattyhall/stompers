extern crate stompers;

fn main() {
    let mut conn = stompers::Connection::new("127.0.0.1", 61613).unwrap();
    conn.subscribe("a-queue");
    let msg = stompers::Message::new("a-queue", "hello world");
    conn.send_message(msg);
    println!("Message sent");
    println!("Received: {}", conn.receive());
}
