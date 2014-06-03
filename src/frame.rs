#![crate_id = ""]
use std::io;
use std::str;
use collections::HashMap;

#[deriving(Show, Eq)]
pub enum StompError {
    TcpError(io::IoError),
    MalformedFrame(String),
    MalformedCommand(String),
    MalformedHeader(String),
    IncorrectResponse(String),
    Other(String),
}

#[deriving(Show, Eq)]
pub enum Command {
    Connect,
    Send,
    Error,
    Connected,
}

impl Command {
    pub fn to_str(&self) -> &str {
        let s = *self;
        match s {
            Connect => "CONNECT",
            Connected => "CONNECTED",
            Send => "SEND",
            Error => "ERROR",
        }
    }

    fn parse(s: &str) -> Result<Command, StompError> {
        match s {
            "CONNECTED" => Ok(Connected),
            "ERROR" => Ok(Error),
            _       => Err(MalformedCommand(format!("Unknown command: {}", s)))
        }
    }
}

pub struct Frame {
    pub command: Command,
    pub headers: HashMap<String,String>,
    pub body: String,
}

impl Frame {
    pub fn new(cmd: Command, bdy: &str) -> Frame {
        Frame {command: cmd, body: String::from_str(bdy), headers: HashMap::new()}
    }

    pub fn add_header(&mut self, k: &str, v: &str) {
        self.headers.insert(String::from_str(k), String::from_str(v));
    }

    pub fn to_string(&self) -> String {
        let command = self.command.to_str();
        let mut s = String::new();
        for (k, v) in self.headers.iter() {
            let h = format!("{}:{}\n", k, v);
            s.push_str(h.as_slice());
        }
        format!("{}\n{}\n\n{}\0", command, s.to_str(), self.body)
    }

    pub fn parse(bytes: &[u8]) -> Result<Frame, StompError> {
        let s = str::from_utf8(bytes).unwrap();
        let mut lines: Vec<&str> = s.lines().collect();
        if lines.len() <= 1 {
            return Err(MalformedFrame(String::from_str("Frame too short. Must have at least 2 lines")));
        }
        let cmd_str = *lines.get(0);
        let cmd = try!(Command::parse(cmd_str));
        let mut frame = Frame::new(cmd, "");
        
        for &line in lines.iter().skip(1) {
            let (k, v) = try!(parse_header(line));
            frame.add_header(k, v);
        }

        return Ok(frame);
    }
}

fn parse_header(line: &str) -> Result<(&str, &str), StompError> {
    Ok(("",""))
}
