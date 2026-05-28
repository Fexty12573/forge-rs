use core::{
    ffi::c_void,
    net::{SocketAddr, SocketAddrV4, SocketAddrV6},
};
use log::error;
use sys::socket::*;

use super::tcp_stream::TcpStream;

pub struct TcpServer {
    socket: Socket,
}

impl TcpServer {
    pub fn bind(addr: SocketAddr) -> Option<Self> {
        match addr {
            SocketAddr::V4(v4) => Self::bind_v4(v4),
            SocketAddr::V6(v6) => Self::bind_v6(v6),
        }
    }

    pub fn accept(&self) -> Option<TcpStream> {
        let mut peer_addr = SockAddrIn {
            sin_len: 0,
            sin_family: 0,
            sin_port: 0,
            sin_addr: [0; 4],
            sin_zero: [0; 8],
        };
        let mut addrlen = size_of::<SockAddrIn>();

        let socket =
            unsafe { forge_socket_accept(&self.socket, &mut peer_addr, &mut addrlen) };
        if socket.fd == -1 {
            error!("Failed to accept connection");
            return None;
        }

        Some(TcpStream::from_socket(socket))
    }

    pub fn close(&mut self) {
        unsafe { forge_socket_destroy(&mut self.socket) }
    }

    fn bind_v4(addr: SocketAddrV4) -> Option<Self> {
        let mut socket = unsafe {
            forge_socket_create(super::consts::AF_INET, super::consts::SOCK_STREAM, 0)
        };
        if socket.fd == -1 {
            error!("Failed to create socket");
            return None;
        }

        let reuse: i32 = 1;
        unsafe {
            forge_socket_setSockOpt(
                &socket,
                super::consts::SOL_SOCKET,
                super::consts::SO_REUSEADDR,
                &reuse as *const i32 as *const c_void,
                size_of::<i32>() as u32,
            )
        };

        let in_addr = SockAddrIn {
            sin_len: 0,
            sin_family: super::consts::AF_INET as u8,
            sin_port: addr.port().to_be(),
            sin_addr: addr.ip().octets(),
            sin_zero: [0; 8],
        };

        if unsafe { forge_socket_bind(&socket, &in_addr, size_of::<SockAddrIn>()) } == -1 {
            unsafe { forge_socket_destroy(&mut socket) };
            error!("Failed to bind to address");
            return None;
        }

        if unsafe { forge_socket_listen(&socket, 128) } == -1 {
            unsafe { forge_socket_destroy(&mut socket) };
            error!("Failed to listen on socket");
            return None;
        }

        Some(Self { socket })
    }

    fn bind_v6(_addr: SocketAddrV6) -> Option<Self> {
        error!("IPv6 not currently supported");
        None
    }
}

impl Drop for TcpServer {
    fn drop(&mut self) {
        unsafe { forge_socket_destroy(&mut self.socket) }
    }
}
