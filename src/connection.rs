use std::io::IoResult;
use std::io::net::tcp::TcpStream;
use collections::HashMap;
use misc::*;
use frame;
use message;

pub struct Connection {
    stream: TcpStream,
    subscriptions: HashMap<String, int>,
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
        let mut conn = Connection {stream: stream_err.unwrap(), subscriptions: HashMap::new(), subscription_num: 0,
            kill_reader: tx};
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
        let mut stream = self.stream.clone();
        let (tx, rx): (Sender<bool>, Receiver<bool>) = channel();
        self.kill_reader = tx;
        spawn(proc() {
            // as per https://github.com/mozilla/rust/issues/10617
            // I would have thought it would be fixed by now
            let mut stream = stream;
            // set timeout so that we can check if someone wants to kill this task
            stream.set_read_timeout(Some(50));
            let mut buf = [0, ..1024];
            loop {
                match stream.read(buf) {
                    Ok(_)  => println!("From task: {}", frame::Frame::parse(buf)),
                    Err(_) => {}
                }

                match rx.try_recv() {
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

    pub fn subscribe(&mut self, queue: &str) {
        if !self.subscriptions.contains_key_equiv(&String::from_str(queue)) {
            let mut subscribe_frame = frame::Frame::new(frame::Subscribe, "");
            subscribe_frame.add_header("id", self.subscription_num.to_str().as_slice());
            subscribe_frame.add_header("destination", queue);
            self.send_frame(&subscribe_frame);

            self.subscriptions.insert(String::from_str(queue), self.subscription_num);
            self.subscription_num += 1;
        }
    }

    pub fn receive(&mut self) -> Result<message::Message, StompError> {
        let (_, buf) = self.read();
        let frame = try!(frame::Frame::parse(buf));
        match frame.command {
            frame::Message => Ok(message::Message::from_frame(frame)),
            frame::Error   => Err(Other(format!("There was an error: {}", frame.body))),
            _              => Err(IncorrectResponse(format!("Expected a MESSAGE frame but didn't get one. Instead got a {} frame",
                                   frame.command.to_str()))),
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        // we want to kill the task that is reading off the stream
        self.kill_reader.send(true);
    }
}

