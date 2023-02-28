use std::ptr;
use std::ffi::{c_int, c_uint, c_char, c_void};
use libc::{size_t, off_t, off64_t, mode_t, socklen_t, iovec, msghdr, sockaddr, statx};

use crate::io_uring::{self, *};
use crate::ring::Ring;

const NULL: *const c_void = ptr::null::<c_void>(); 

/* io_uring syscall equivalent ops */
impl<T: Sized, U: Sized> Ring<T, U> {
  #[inline]
	pub fn preadv2(&mut self, fd: c_int, iov: *const iovec, iovcnt: c_int, offset: off_t, flags: c_int) -> Option<*mut io_uring::sqe<T>> {
    return self.sqe_prep(IORING_OP_READV, fd, iov as *const c_void, iovcnt as u32, offset as u64, flags as u32);
  }

  #[inline]
	pub fn pwritev2(&mut self, fd: c_int, iov: *const iovec, iovcnt: c_int, offset: off_t, flags: c_int) -> Option<*mut io_uring::sqe<T>> {
    return self.sqe_prep(IORING_OP_WRITEV, fd, iov as *const c_void, iovcnt as u32, offset as u64, flags as u32);
  }

  #[inline]
	pub fn fsync(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
    return self.sqe_prep(IORING_OP_WRITEV, fd, NULL, 0, 0, 0);
  }

  #[inline]
	pub fn fdatasync(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
    return self.sqe_prep(IORING_OP_WRITEV, fd, NULL, 0, 0, IORING_FSYNC_DATASYNC);
  }

  #[inline]
	pub fn sync_file_range(&mut self, fd: c_int, offset: off64_t, nbytes: off64_t, flags: u32) -> Option<*mut io_uring::sqe<T>> {
    return self.sqe_prep(IORING_OP_SYNC_FILE_RANGE, fd, NULL, nbytes as u32, offset as u64, flags);
  }

  #[inline]
	pub fn sendmsg(&mut self, sockfd: c_int, msg: *const msghdr, flags: c_int) -> Option<*mut io_uring::sqe<T>> {
    return self.sqe_prep(IORING_OP_SENDMSG, sockfd, msg as *const c_void, 1, 0, flags as u32);
  }

  #[inline]
	pub fn recvmsg(&mut self, sockfd: c_int, msg: *const msghdr, flags: c_int) -> Option<*mut io_uring::sqe<T>> {
    return self.sqe_prep(IORING_OP_RECVMSG, sockfd, msg as *const c_void, 1, 0, flags as u32);
  }

  #[inline]
	pub fn accept4(&mut self, sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t, flags: c_int) -> Option<*mut io_uring::sqe<T>> {
    return self.sqe_prep(IORING_OP_ACCEPT, sockfd, addr as *mut c_void, 0, addrlen as u64, flags as u32);
  }

  #[inline]
	pub fn connect(&mut self, sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> Option<*mut io_uring::sqe<T>> {
    return self.sqe_prep(IORING_OP_CONNECT, sockfd, addr as *const c_void, 0, addrlen as u64, 0);
  }

  #[inline]
	pub fn fallocate(&mut self, fd: c_int, mode: c_int, offset: off_t, len: off_t) -> Option<*mut io_uring::sqe<T>> {
    return self.sqe_prep(IORING_OP_FALLOCATE, fd, len as *const c_void, mode as u32, offset as u64, 0);
  }

  #[inline]
	pub fn openat(&mut self, dirfd: c_int, pathname: *const c_char, flags: c_int, mode: mode_t) -> Option<*mut io_uring::sqe<T>> {
    return self.sqe_prep(IORING_OP_OPENAT, dirfd, pathname as *const c_void, mode as u32, 0, flags as u32);
  }

  #[inline]
	pub fn close(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
    return self.sqe_prep(IORING_OP_CLOSE, fd, NULL, 0, 0, 0);
  }

  #[inline]
	pub fn statx(&mut self, dirfd: c_int, pathname: *const c_char, flags: c_int, mask: c_uint, statxbuf: *mut statx) -> Option<*mut io_uring::sqe<T>> {
    return self.sqe_prep(IORING_OP_STATX, dirfd, pathname as *const c_void, mask as u32, statxbuf as u64, flags as u32);
  }

  #[inline]
	pub fn read(&mut self, fd: c_int, buf: *mut c_void, count: size_t) -> Option<*mut io_uring::sqe<T>> {
    return self.sqe_prep(IORING_OP_READ, fd, buf, count as u32, 0, 0);
  }

  #[inline]
	pub fn write(&mut self, fd: c_int, buf: *const c_void, count: size_t) -> Option<*mut io_uring::sqe<T>> {
    return self.sqe_prep(IORING_OP_WRITE, fd, buf, count as u32, 0, 0);
  }

  #[inline]
	pub fn posix_fadvise(&mut self, fd: c_int, offset: off_t, len: off_t, advice: c_int) -> Option<*mut io_uring::sqe<T>> {
    return self.sqe_prep(IORING_OP_FADVISE, fd, NULL, len as u32, offset as u64, advice as u32);
  }

  #[inline]
	pub fn madvise(&mut self, addr: *mut c_void, length: size_t, advice: c_int) -> Option<*mut io_uring::sqe<T>> {
    return self.sqe_prep(IORING_OP_MADVISE, -1, addr, length as u32, 0, advice as u32);
  }

  #[inline]
	pub fn send(&mut self, sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> Option<*mut io_uring::sqe<T>> {
    return self.sqe_prep(IORING_OP_SEND, sockfd, buf, len as u32, 0, flags as u32);
  }

  #[inline]
	pub fn recv(&mut self, sockfd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> Option<*mut io_uring::sqe<T>> {
    return self.sqe_prep(IORING_OP_RECV, sockfd, buf, len as u32, 0, flags as u32);
  }

  // #[inline]
	// pub fn openat2(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn epoll_ctl(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn splice(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn tee(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn shutdown(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn renameat(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn unlinkat(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn mkdirat(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn symlinkat(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
    // return self.call();
  // }

  // #[inline]
	// pub fn linkat(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn fsetxattr(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn setxattr(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn fgetxattr(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn getxattr(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  #[inline]
	pub fn socket(&mut self, domain: c_int, sock_type: c_int, protocol: c_int) -> Option<*mut io_uring::sqe<T>> {
    return self.sqe_prep(IORING_OP_SOCKET, domain as i32, NULL, protocol as u32, sock_type as u64, 0);
  }
}

/* io_uring miscellaneous ops */
impl<T: Sized, U: Sized> Ring<T, U> {
  #[inline]
	pub fn nop(&mut self) -> Option<*mut io_uring::sqe<T>> {
    return self.sqe_prep(IORING_OP_NOP, -1, NULL, 0, 0, 0);
  }

  // #[inline]
	// pub fn read_fixed(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn write_fixed(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn poll_add(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn poll_remove(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn timeout(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn timeout_remove(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn async_cancel(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn link_timeout(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn files_update(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn provide_buffers(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn remove_buffers(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn msg_ring(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }

  // #[inline]
	// pub fn uring_cmd(&mut self, fd: c_int) -> Option<*mut io_uring::sqe<T>> {
  //   return self.call();
  // }
}