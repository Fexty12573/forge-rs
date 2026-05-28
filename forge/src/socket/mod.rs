pub mod tcp_server;
pub mod tcp_stream;

pub use tcp_server::TcpServer;
pub use tcp_stream::{Shutdown, TcpStream};

mod consts {
    pub const AF_LOCAL: i32 = 1;
    pub const AF_UNIX: i32 = 1;
    pub const AF_INET: i32 = 2;
    pub const AF_INET6: i32 = 28;

    pub const SOCK_STREAM: i32 = 1;
    pub const SOCK_DGRAM: i32 = 2;
    pub const SOCK_RAW: i32 = 3;
    pub const SOCK_SEQPACKET: i32 = 5;

    pub const MSG_PEEK: i32 = 0x2;

    pub const SOL_SOCKET: i32 = 0xFFFF;
    pub const SO_REUSEADDR: i32 = 0x4;

    pub const IPPROTO_TCP: i32 = 6;
    pub const TCP_NODELAY: i32 = 1;
}
