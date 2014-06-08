use frame;

#[deriving(Show, Eq)]
pub struct Message {
    frame: frame::Frame,
}

impl Message {
    pub fn new(destination: &str, body: &str) -> Message {
        let mut frame = frame::Frame::new(frame::Send, body);
        frame.add_header("destination", destination);
        Message {frame: frame}
    }

    pub fn from_frame(frame: frame::Frame) -> Message {
        Message {frame: frame}
    }

    pub fn add_header(&mut self, k: &str, v: &str) {
        self.frame.add_header(k, v);
    }

    pub fn to_string(&self) -> String {
        self.frame.to_string()
    }
}
