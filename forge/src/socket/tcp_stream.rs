use core::{
    ffi::c_void,
    fmt,
    net::{SocketAddr, SocketAddrV4, SocketAddrV6},
};
use log::error;
use sys::socket::*;

pub struct TcpStream {
    pub(super) socket: Socket,
}

pub enum Shutdown {
    Read = 0,
    Write = 1,
    Both = 2,
}

impl TcpStream {
    pub fn connect(addr: SocketAddr) -> Option<Self> {
        match addr {
            SocketAddr::V4(v4) => Self::connect_v4(v4),
            SocketAddr::V6(v6) => Self::connect_v6(v6),
        }
    }

    pub(super) fn from_socket(socket: Socket) -> Self {
        Self { socket }
    }

    pub fn close(&mut self) {
        unsafe { forge_socket_destroy(&mut self.socket) }
    }

    pub fn read(&self, buf: &mut [u8]) -> Option<usize> {
        let n = unsafe { forge_socket_recv(&self.socket, buf.as_mut_ptr() as *mut c_void, buf.len(), 0) };
        if n < 0 {
            error!("Failed to receive data");
            return None;
        }
        Some(n as usize)
    }

    pub fn write(&self, buf: &[u8]) -> Option<usize> {
        let n = unsafe { forge_socket_send(&self.socket, buf.as_ptr() as *const c_void, buf.len(), 0) };
        if n < 0 {
            error!("Failed to send data");
            return None;
        }
        Some(n as usize)
    }

    pub fn peek(&self, buf: &mut [u8]) -> Option<usize> {
        let n = unsafe {
            forge_socket_recv(
                &self.socket,
                buf.as_mut_ptr() as *mut c_void,
                buf.len(),
                super::consts::MSG_PEEK,
            )
        };
        if n < 0 {
            error!("Failed to peek data");
            return None;
        }
        Some(n as usize)
    }

    pub fn shutdown(&self, how: Shutdown) -> bool {
        let result = unsafe { forge_socket_shutdown(&self.socket, how as i32) };
        if result == -1 {
            error!("Failed to shutdown socket");
        }
        result == 0
    }

    pub fn set_nodelay(&self, nodelay: bool) -> bool {
        let val: i32 = nodelay as i32;
        let result = unsafe {
            forge_socket_setSockOpt(
                &self.socket,
                super::consts::IPPROTO_TCP,
                super::consts::TCP_NODELAY,
                &val as *const i32 as *const c_void,
                size_of::<i32>() as u32,
            )
        };
        if result == -1 {
            error!("Failed to set TCP_NODELAY");
        }
        result == 0
    }

    fn connect_v4(addr: SocketAddrV4) -> Option<Self> {
        let mut socket = unsafe { forge_socket_create(super::consts::AF_INET, super::consts::SOCK_STREAM, 0) };
        if socket.fd == -1 {
            error!("Failed to create socket");
            return None;
        }

        let in_addr = SockAddrIn {
            sin_len: 0,
            sin_family: super::consts::AF_INET as u8,
            sin_port: addr.port().to_be(),
            sin_addr: addr.ip().octets(),
            sin_zero: [0; 8],
        };

        let result = unsafe { forge_socket_connect(&socket, &in_addr, size_of::<SockAddrIn>()) };
        if result == -1 {
            unsafe { forge_socket_destroy(&mut socket) };
            error!("Failed to connect to address");
            return None;
        }

        Some(Self { socket })
    }

    fn connect_v6(_addr: SocketAddrV6) -> Option<Self> {
        error!("IPv6 not currently supported");
        None
    }
}

impl Drop for TcpStream {
    fn drop(&mut self) {
        unsafe { forge_socket_destroy(&mut self.socket) }
    }
}

impl fmt::Write for TcpStream {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        match self.write(s.as_bytes()) {
            Some(_) => Ok(()),
            None => Err(fmt::Error {}),
        }
    }
}
