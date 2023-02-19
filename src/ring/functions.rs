use libc::{c_void, sockaddr, socklen_t};

use crate::io_uring;
use crate::ring::Ring;

impl<T: Sized, U: Sized> Ring<T, U> {
  pub fn accept4(&mut self, sockfd: i32, addr: *mut sockaddr, addrlen: *mut socklen_t, flags: u32) -> Option<*mut io_uring::sqe<T>> {
    unsafe { 
      return self.prep(io_uring::IORING_OP_ACCEPT, sockfd, addr as *mut c_void, 0, addrlen as u64, flags);
    };
  }
}