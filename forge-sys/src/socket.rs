use core::ffi::{c_char, c_void};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Socket {
    pub fd: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SockAddrIn {
    pub sin_len: u8,
    pub sin_family: u8,
    pub sin_port: u16,
    pub sin_addr: [u8; 4],
    pub sin_zero: [c_char; 8],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SockAddrIn6 {
    pub sin_len: u8,
    pub sin_family: i32,
    pub sin_port: u16,
    pub sin_addr: [u8; 4],
    pub sin_zero: [c_char; 8],
}

unsafe extern "C" {
    pub fn forge_socket_initDefault() -> bool;
    pub fn forge_socket_init(pool: *mut c_void, pool_size: usize, allocator_pool_size: usize, concurrency_count: i32) -> bool;
    pub fn forge_socket_deinit();

    pub fn forge_socket_create(domain: i32, ttype: i32, protocol: i32) -> Socket;
    pub fn forge_socket_destroy(socket: *mut Socket); // Can be called multiple times on the same socket

    pub fn forge_socket_bind(socket: *const Socket, addr: *const SockAddrIn, addrlen: usize) -> i32;
    pub fn forge_socket_connect(socket: *const Socket, addr: *const SockAddrIn, addrlen: usize) -> i32;
    pub fn forge_socket_listen(socket: *const Socket, backlog: i32) -> i32;
    pub fn forge_socket_accept(socket: *const Socket, addr: *mut SockAddrIn, addrlen: *mut usize) -> Socket;
    pub fn forge_socket_shutdown(socket: *const Socket, how: i32) -> i32;

    pub fn forge_socket_send(socket: *const Socket, buf: *const c_void, len: usize, flags: i32) -> isize;
    pub fn forge_socket_recv(socket: *const Socket, buf: *mut c_void, len: usize, flags: i32) -> isize;

    pub fn forge_socket_getSockOpt(socket: *const Socket, level: i32, name: i32, val: *mut c_void, len: *mut u32) -> i32;
    pub fn forge_socket_setSockOpt(socket: *const Socket, level: i32, name: i32, val: *const c_void, len: u32) -> i32;
}
