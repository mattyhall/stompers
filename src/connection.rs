use std::io::IoResult;
use std::io::net::tcp::TcpStream;
use sync::{Arc, Mutex};
use std::collections::HashMap;
use misc::*;
use frame;
use message;

#[deriving(Clone, Send)]
pub struct Connection {
    stream: TcpStream,
    subscriptions: Arc<Mutex<HashMap<int, Subscription>>>,
    subscription_num: int,
    kill_reader: Sender<bool>,
}

impl Connection {
    pub fn new(ip: &str, port: u16) -> Result<Connection, StompError> {
        let stream_err = TcpStream::connect(ip, port);
        if !stream_err.is_ok() {
            // use Result.err() to turn it into an Option.
            // could use Result.unwrap_err(), but that would require a show instance for TcpStream
            return Err(TcpError(stream_err.err().unwrap()));
        }
        let (tx, _) = channel();
        let mut conn = Connection {stream: stream_err.unwrap(), subscriptions: Arc::new(Mutex::new(HashMap::new())), 
            subscription_num: 0, kill_reader: tx};
        let connect_frame = frame::Frame::new(frame::Connect, "");
        conn.send_frame(&connect_frame);

        // Check that the server sends back a CONNECTED frame
        let (_, buf) = conn.read();
        let response_frame = try!(frame::Frame::parse(buf));
        match response_frame.command {
            frame::Connected => {
                conn.spawn_reader();
                Ok(conn)
            },
            frame::Error     => Err(ConnectionRefused(format!("Server refused connection. Error was: {}", response_frame.body))),
            _                => Err(IncorrectResponse(format!(
                                    "Expected server to send a CONNECTED frame but didn't get one. Instead got a {} frame", 
                                    response_frame.command.to_str())))
        }
    }

    fn read(&mut self) -> (IoResult<uint>, [u8, ..1024]) {
        let mut buf = [0, ..1024];
        let res = self.stream.read(buf);
        (res, buf)
    }

    fn spawn_reader(&mut self) {
        let stream = self.stream.clone();
        let (tx, rx): (Sender<bool>, Receiver<bool>) = channel();
        self.kill_reader = tx;
        let subs = self.subscriptions.clone();
        spawn(proc() {
            // as per https://github.com/mozilla/rust/issues/10617
            // I would have thought it would be fixed by now
            let mut stream = stream;
            // set timeout so that we can check if someone wants to kill this task
            stream.set_read_timeout(Some(50));
            let mut buf = [0, ..1024];
            loop {
                match stream.read(buf) {
                    Ok(_)  => {
                        let mut subs = subs.lock();
                        let frame = frame::Frame::parse(buf).unwrap();
                        let id_string = frame.headers.get(&String::from_str("subscription"));
                        let id = from_str(id_string.as_slice()).unwrap();
                        let msg = message::Message::from_frame(frame.clone());
                        let callback = subs.get(&id).callback;
                        callback(msg);
                    },
                    Err(_) => {}
                }

                match rx.try_recv() {
                    // if we receive something on the pipe break out of the loop
                    Ok(_)  => break,
                    Err(_) => {}
                }
            }
        });
    }

    fn send_frame(&mut self, frame: &frame::Frame) {
        let s = frame.to_string();
        self.send_string(s);
    }

    fn send_string(&mut self, s: String) {
        let sb = s.as_bytes();
        self.stream.write(sb);
    }

    pub fn send_message(&mut self, msg:message::Message) {
        self.send_string(msg.to_string());
    }

    pub fn subscribe(&mut self, queue: &str, callback: fn (message::Message)) {
        // lock borrows self/self.subscriptions so create a lifetime here
        // at the closing brace subs will die and will release self.subscriptions
        {
            let mut subs = self.subscriptions.lock();
            let is_queue = |sub: &&Subscription| -> bool {
                sub.name == String::from_str(queue)
            };
            let already_subbed = subs.values().find(is_queue);
            match already_subbed {
                // TODO: Decide whether to reassign callback or not
                Some(_) => {return;},
                None    => {}
            }
        }

        let mut subscribe_frame = frame::Frame::new(frame::Subscribe, "");
        subscribe_frame.add_header("id", self.subscription_num.to_string().as_slice());
        subscribe_frame.add_header("destination", queue);
        self.send_frame(&subscribe_frame);

        let mut subs = self.subscriptions.lock();
        let subscription = Subscription {name: String::from_str(queue), 
            id: self.subscription_num, callback: callback};
        subs.insert(self.subscription_num, subscription);
        self.subscription_num += 1;
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        // we want to kill the task that is reading off the stream
        self.kill_reader.send(true);
    }
}

#[deriving(Clone)]
struct Subscription {
    name: String,
    id: int,
    callback: fn (message::Message),
}
