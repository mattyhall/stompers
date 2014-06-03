use std::io;
use std::io::net::tcp::TcpStream;
use std::str;

use frame;

pub struct Connection {
    stream: TcpStream
}

impl Connection {
    pub fn new(ip: &str, port: u16) -> Result<Connection, frame::StompError> {
        let stream_err = TcpStream::connect(ip, port);
        if !stream_err.is_ok() {
            // use Result.err() to turn it into an Option.
            // could use Result.unwrap_err(), but that would require a show instance for TcpStream
            return Err(frame::TcpError(stream_err.err().unwrap()));
        }
        let mut conn = Connection {stream: stream_err.unwrap()};
        let mut connect_frame = frame::Frame::new(frame::Connect, "");
        conn.send_frame(connect_frame);

        // Check that the server sends back a CONNECTED frame
        let mut buf = [0, ..1024];
        conn.stream.read(buf);
        let response_frame = try!(frame::Frame::parse(buf));
        if response_frame.command != frame::Connected {
            return Err(frame::IncorrectResponse(format!(
                "Expected a CONNECTED frame but didn't get one. Instead got {}", response_frame.command.to_str())));
        }
        Ok(conn)
    }

    fn send_frame(&mut self, frame: frame::Frame) {
        let s = frame.to_string();
        let sb = s.as_bytes();
        self.stream.write(sb);
    }
}
