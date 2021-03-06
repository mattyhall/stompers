use std::io;

#[deriving(Show)]
pub enum StompError {
    TcpError(io::IoError),
    MalformedFrame(String),
    MalformedCommand(String),
    MalformedHeader(String),
    ConnectionRefused(String),
    IncorrectResponse(String),
    Other(String),
}
