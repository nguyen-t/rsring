use std::ptr;
use libc::{c_int, c_void, off_t, off64_t, iovec, msghdr, sockaddr, socklen_t};

use crate::io_uring::{self, *};
use crate::ring::Ring;

const NULL: *const c_void = ptr::null::<c_void>(); 

/* Io_Uring wrappers for Linux Syscalls */
impl<T: Sized, U: Sized> Ring<T, U> {
  pub fn preadv2(&mut self, fd: c_int, iov: *const iovec, iovcnt: c_int, offset: off_t, flags: c_int) -> Option<*mut io_uring::sqe<T>> {
    return self.prep(IORING_OP_READV, fd, iov as *const c_void, iovcnt as u32, offset as u64, flags as u32);
  }

  pub fn pwritev2(&mut self, fd: c_int, iov: *const iovec, iovcnt: c_int, offset: off_t, flags: c_int) -> Option<*mut io_uring::sqe<T>> {
    return self.prep(IORING_OP_WRITEV, fd, iov as *const c_void, iovcnt as u32, offset as u64, flags as u32);
  }

  pub fn fsync(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
    return self.prep(IORING_OP_WRITEV, fd, NULL, 0, 0, 0);
  }

  pub fn fdatasync(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
    return self.prep(IORING_OP_WRITEV, fd, NULL, 0, 0, IORING_FSYNC_DATASYNC);
  }

  // pub fn poll(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  pub fn sync_file_range(&mut self, fd: c_int, offset: off64_t, nbytes: off64_t, flags: u32) -> Option<*mut io_uring::sqe<T>> {
    return self.prep(IORING_OP_SYNC_FILE_RANGE, fd, NULL, nbytes as u32, offset as u64, flags);
  }

  pub fn sendmsg(&mut self, sockfd: c_int, msg: *const msghdr, flags: c_int) -> Option<*mut io_uring::sqe<T>> {
    return self.prep(IORING_OP_SENDMSG, sockfd, msg as *const c_void, 1, 0, flags as u32);
  }

  pub fn recvmsg(&mut self, sockfd: c_int, msg: *const msghdr, flags: c_int) -> Option<*mut io_uring::sqe<T>> {
    return self.prep(IORING_OP_RECVMSG, sockfd, msg as *const c_void, 1, 0, flags as u32);
  }

  pub fn accept4(&mut self, sockfd: c_int, addr: *const sockaddr, addrlen: *const socklen_t, flags: c_int) -> Option<*mut io_uring::sqe<T>> {
    return self.prep(IORING_OP_ACCEPT, sockfd, addr as *mut c_void, 0, addrlen as u64, flags as u32);
  }

  pub fn connect(&mut self, sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> Option<*mut io_uring::sqe<T>> {
    return self.prep(IORING_OP_CONNECT, sockfd, addr as *const c_void, 0, addrlen as u64, 0);
  }

  pub fn fallocate(&mut self, fd: c_int, mode: c_int, offset: off_t, len: off_t) -> Option<*mut io_uring::sqe<T>> {
    return self.prep(IORING_OP_FALLOCATE, fd, len as *const c_void, mode as u32, offset as u64, 0);
  }

  // pub fn openat(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn close(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn statx(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn read(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn write(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn fadvise(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn madvise(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn send(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn recv(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn openat2(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn epoll_ctl(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn splice(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn tee(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn shutdown(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn renameat(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn unlinkat(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn mkdirat(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn symlinkat(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn linkat(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn fsetxattr(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn setxattr(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn fgetxattr(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn getxattr(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn socket(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }
}

impl<T: Sized, U: Sized> Ring<T, U> {
  pub fn nop(&mut self) -> Option<*mut io_uring::sqe<T>> {
    return self.prep(IORING_OP_NOP, -1, NULL, 0, 0, 0);
  }

  // pub fn read_fixed(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn write_fixed(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn timeout(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn timeout_remove(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn async_cancel(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn link_timeout(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn provide_buffers(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn remove_buffers(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn msg_ring(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn send_zc(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }

  // pub fn sendmsg_zc(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.prep();
  // }
}