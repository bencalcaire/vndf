pub use self::acceptor::Acceptor;
pub use self::connection::Connection;


mod acceptor;
mod connection;
mod ffi;

pub mod epoll;


pub type ConnId = u32;
