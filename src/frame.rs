use std::str;
use std::collections::HashMap;
use misc::*;


#[deriving(Show, PartialEq, Eq, Clone)]
pub enum Command {
    // Client commands
    Connect,
    Send,
    Subscribe,
    // Server commands
    Connected,
    Receipt,
    Message,
    Error,
}

impl Command {
    pub fn to_str(&self) -> &str {
        let s = *self;
        match s {
            Connect   => "CONNECT",
            Send      => "SEND",
            Subscribe => "SUBSCRIBE",
            Receipt   => "RECEIPT",
            Message   => "MESSAGE",
            Connected => "CONNECTED",
            Error     => "ERROR",
        }
    }

    fn parse(s: &str) -> Result<Command, StompError> {
        match s {
            "CONNECTED" => Ok(Connected),
            "RECEIPT"   => Ok(Receipt),
            "MESSAGE"   => Ok(Message),
            "ERROR"     => Ok(Error),
            _           => Err(MalformedCommand(format!("Unknown command: {}", s)))
        }
    }
}

#[deriving(Show, PartialEq, Eq, Clone)]
pub struct Frame {
    pub command: Command,
    pub headers: HashMap<String,String>,
    pub body: String,
}

impl Frame {
    pub fn new(cmd: Command, bdy: &str) -> Frame {
        let body = String::from_str(bdy);
        let len = body.len();
        let mut frame = Frame {command: cmd, body: body, headers: HashMap::new()};
        frame.add_header("content-length", len.to_string().as_slice());
        frame
    }

    pub fn add_header(&mut self, k: &str, v: &str) {
        self.headers.insert(String::from_str(k), String::from_str(v));
    }

    pub fn to_string(&self) -> String {
        let command = self.command.to_str();
        let mut s = String::new();
        for (k, v) in self.headers.iter() {
            let h = format!("{}:{}\n", sanitise_header_text(k), sanitise_header_text(v));
            s.push_str(h.as_slice());
        }
        format!("{}\n{}\n{}\0", command, s, self.body)
    }

    pub fn parse(bytes: &[u8]) -> Result<Frame, StompError> {
        let s = str::from_utf8(bytes).unwrap();
        let lines: Vec<&str> = s.lines().collect();
        if lines.len() <= 1 {
            return Err(MalformedFrame(format!("Frame too short. Must have at least 2 lines. Frame was: {}", s)));
        }
        let cmd_str = lines[0];
        let cmd = try!(Command::parse(cmd_str));
        let mut frame = Frame::new(cmd, "");
        let mut iter = lines.iter().skip(1);
        
        for &line in iter {
            // a blank line means the body is next
            if line == "" {
                break;
            }
            let (k, v) = try!(parse_header(line));
            frame.headers.insert(k, v);
        }

        let mut body = String::new();
        // Getting a value from an iterator removes it from the iterator.
        // Therefore all values left are part of the body
        for &body_line in iter {
            body.push_str(body_line);
            body.push_str("\n");
        }
        body.pop_char();
        frame.body = body;

        return Ok(frame);
    }
}

fn parse_header<'a>(line: &'a str) -> Result<(String, String), StompError> {
    let parts: Vec<&str> = line.split_str(":").collect();
    if parts.len() != 2 {
        return Err(MalformedHeader(format!("Header does not have a key and a value. {}", line)));
    }
    let k = parse_header_text(parts[0]);
    let v = parse_header_text(parts[1]);
    Ok((k, v))
}

fn sanitise_header_text(s: &String) -> String {
    // replace backslash first, otherwise it will escape backslashes that we escaped
    s.replace("\\", "\\\\").replace("\r", "\\r")
     .replace("\n", "\\n").replace(":", "\\c")
}

fn parse_header_text(s: &str) -> String {
    let s = String::from_str(s);
    s.replace("\\c", ":").replace("\\\\", "\\").replace("\\r", "\r")
     .replace("\\n", "\n")
}
