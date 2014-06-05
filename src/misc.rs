use std::io;

#[deriving(Show, Eq)]
pub enum StompError {
    TcpError(io::IoError),
    MalformedFrame(String),
    MalformedCommand(String),
    MalformedHeader(String),
    ConnectionRefused(String),
    IncorrectResponse(String),
    MessageNotSent(String),
    Other(String),
}
