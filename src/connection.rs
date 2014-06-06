use std::io::net::tcp::TcpStream;

use misc::*;
use frame;
use message;

pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    pub fn new(ip: &str, port: u16) -> Result<Connection, StompError> {
        let stream_err = TcpStream::connect(ip, port);
        if !stream_err.is_ok() {
            // use Result.err() to turn it into an Option.
            // could use Result.unwrap_err(), but that would require a show instance for TcpStream
            return Err(TcpError(stream_err.err().unwrap()));
        }
        let mut conn = Connection {stream: stream_err.unwrap()};
        let connect_frame = frame::Frame::new(frame::Connect, "");
        conn.send_frame(&connect_frame);

        // Check that the server sends back a CONNECTED frame
        let mut buf = [0, ..1024];
        conn.stream.read(buf);
        let response_frame = try!(frame::Frame::parse(buf));
        match response_frame.command {
            frame::Connected => Ok(conn),
            frame::Error     => Err(ConnectionRefused(format!("Server refused connection. Error was: {}", response_frame.body))),
            _                => Err(IncorrectResponse(format!(
                                    "Expected server to send a CONNECTED frame but didn't get one. Instead got a {} frame", 
                                    response_frame.command.to_str())))
        }
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

    pub fn send_message_and_wait(&mut self, msg: message::Message, timeout_ms: u64) -> Result<(), StompError> {
        self.send_message(msg);

        // Check the server did not send back an ERROR frame
        let mut buf = [0, ..1024];
        self.stream.set_read_timeout(Some(timeout_ms));
        let len = self.stream.read(buf);
        self.stream.set_read_timeout(None);
        match len {
            Ok(_) => {
                let response_frame = try!(frame::Frame::parse(buf));
                match response_frame.command {
                    frame::Error   => Err(MessageNotSent(format!("Could not send message. Error was: {}", response_frame.body))),
                    _              => Err(IncorrectResponse(format!(
                                        "Expected server to send no frame or an ERROR frame but instead got a {} frame",
                                        response_frame.command.to_str())))
                }
            },
            Err(_) => Ok(())
        }
    }
}
