use std::ptr;
use std::ffi::{c_int, c_uint, c_char, c_void};
use libc::*;

use crate::io_uring::{self, *};
use crate::ring::Ring;

const NULL: *const c_void = ptr::null::<c_void>(); 

/* io_uring syscall equivalent ops */
impl<T: Sized, U: Sized> Ring<T, U> {
  #[inline]
	pub fn preadv2(&mut self, fd: c_int, iov: *const iovec, iovcnt: c_int, offset: off_t, flags: c_int) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_READV, fd, iov as *const c_void, iovcnt as u32, offset as u64, flags);
  }

  #[inline]
  pub fn preadv(&mut self, fd: c_int, iov: *const iovec, iovcnt: c_int, offset: off_t) -> Option<&mut io_uring::sqe<T>> {
    return self.preadv2(fd, iov, iovcnt, offset, 0);
  }

  #[inline]
  pub fn readv(&mut self, fd: c_int, iov: *const iovec, iovcnt: c_int) -> Option<&mut io_uring::sqe<T>> {
    return self.preadv(fd, iov, iovcnt, 0);
  }

  #[inline]
	pub fn pwritev2(&mut self, fd: c_int, iov: *const iovec, iovcnt: c_int, offset: off_t, flags: c_int) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_WRITEV, fd, iov as *const c_void, iovcnt as u32, offset as u64, flags);
  }

  #[inline]
  pub fn pwritev(&mut self, fd: c_int, iov: *const iovec, iovcnt: c_int, offset: off_t) -> Option<&mut io_uring::sqe<T>> {
    return self.pwritev2(fd, iov, iovcnt, offset, 0);
  }

  #[inline]
  pub fn writev(&mut self, fd: c_int, iov: *const iovec, iovcnt: c_int) -> Option<&mut io_uring::sqe<T>> {
    return self.pwritev(fd, iov, iovcnt, 0);
  }

  #[inline]
	pub fn fsync(&mut self, fd: c_int) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_FSYNC, fd, NULL, 0, 0, 0);
  }

  #[inline]
	pub fn fdatasync(&mut self, fd: c_int) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_FSYNC, fd, NULL, 0, 0, IORING_FSYNC_DATASYNC as i32);
  }

  #[inline]
	pub fn sync_file_range(&mut self, fd: c_int, offset: off64_t, nbytes: off64_t, flags: c_uint) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_SYNC_FILE_RANGE, fd, NULL, nbytes as u32, offset as u64, flags as i32);
  }

  #[inline]
	pub fn sendmsg(&mut self, sockfd: c_int, msg: *const msghdr, flags: c_int) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_SENDMSG, sockfd, msg as *const c_void, 1, 0, flags);
  }

  #[inline]
	pub fn recvmsg(&mut self, sockfd: c_int, msg: *const msghdr, flags: c_int) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_RECVMSG, sockfd, msg as *const c_void, 1, 0, flags);
  }

  #[inline]
	pub fn accept4(&mut self, sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t, flags: c_int) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_ACCEPT, sockfd, addr as *mut c_void, 0, addrlen as u64, flags);
  }

  #[inline]
  pub fn accept(&mut self, sockfd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> Option<&mut io_uring::sqe<T>> {
    return self.accept4(sockfd, addr, addrlen, 0);
  }

  #[inline]
	pub fn connect(&mut self, sockfd: c_int, addr: *const sockaddr, addrlen: socklen_t) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_CONNECT, sockfd, addr as *const c_void, 0, addrlen as u64, 0);
  }

  #[inline]
	pub fn fallocate(&mut self, fd: c_int, mode: c_int, offset: off_t, len: off_t) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_FALLOCATE, fd, len as *const c_void, mode as u32, offset as u64, 0);
  }

  #[inline]
	pub fn openat(&mut self, dirfd: c_int, pathname: *const c_char, flags: c_int, mode: mode_t) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_OPENAT, dirfd, pathname as *const c_void, mode as u32, 0, flags);
  }

  #[inline]
	pub fn open(&mut self, pathname: *const c_char, flags: c_int, mode: mode_t) -> Option<&mut io_uring::sqe<T>> {
    return self.openat(AT_FDCWD, pathname, flags, mode);
  }

  #[inline]
	pub fn creat(&mut self, pathname: *const c_char, mode: mode_t) -> Option<&mut io_uring::sqe<T>> {
    return self.open(pathname, O_CREAT | O_WRONLY | O_TRUNC, mode);
  }

  #[inline]
	pub fn close(&mut self, fd: c_int) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_CLOSE, fd, NULL, 0, 0, 0);
  }

  #[inline]
	pub fn statx(&mut self, dirfd: c_int, pathname: *const c_char, flags: c_int, mask: c_uint, statxbuf: *mut statx) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_STATX, dirfd, pathname as *const c_void, mask as u32, statxbuf as u64, flags);
  }

  #[inline]
	pub fn read(&mut self, fd: c_int, buf: *mut c_void, count: size_t) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_READ, fd, buf, count as u32, 0, 0);
  }

  #[inline]
	pub fn write(&mut self, fd: c_int, buf: *const c_void, count: size_t) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_WRITE, fd, buf, count as u32, 0, 0);
  }

  #[inline]
	pub fn posix_fadvise(&mut self, fd: c_int, offset: off_t, len: off_t, advice: c_int) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_FADVISE, fd, NULL, len as u32, offset as u64, advice);
  }

  #[inline]
	pub fn madvise(&mut self, addr: *mut c_void, length: size_t, advice: c_int) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_MADVISE, -1, addr, length as u32, 0, advice);
  }

  #[inline]
	pub fn send(&mut self, sockfd: c_int, buf: *const c_void, len: size_t, flags: c_int) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_SEND, sockfd, buf, len as u32, 0, flags);
  }

  #[inline]
	pub fn recv(&mut self, sockfd: c_int, buf: *mut c_void, len: size_t, flags: c_int) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_RECV, sockfd, buf, len as u32, 0, flags);
  }

  #[inline]
	pub fn openat2(&mut self, dirfd: c_int, pathname: *const c_char, flags: c_int, mode: mode_t) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_OPENAT, dirfd, pathname as *const c_void, mode, 0, flags);
  }

  #[inline]
	pub fn epoll_ctl(&mut self, epfd: c_int, op: c_int, fd: c_int, event: *mut epoll_event) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_EPOLL_CTL, epfd, event as *mut c_void, op as u32, fd as u64, 0);
  }

  #[inline]
	pub fn splice(&mut self, fd_in: c_int, off_in: *mut off64_t, fd_out: c_int, off_out: *mut off64_t, len: size_t, flags: c_uint) -> Option<&mut io_uring::sqe<T>> {
    let sqe = self.sq.prep(IORING_OP_SPLICE, fd_out, off_in as *const c_void, len as u32, off_out as u64, flags as i32)?;

    sqe.file_select = fd_in as u32;

    Some(sqe)
  }

  #[inline]
	pub fn tee(&mut self, fd_in: c_int, fd_out: c_int, len: size_t, flags: c_uint) -> Option<&mut io_uring::sqe<T>> {
    let sqe = self.sq.prep(IORING_OP_TEE, fd_out, NULL, len as u32, 0, flags as i32)?;

    sqe.file_select = fd_in as u32;

    Some(sqe)
  }

  #[inline]
	pub fn shutdown(&mut self, socket: c_int, how: c_int) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_SHUTDOWN, socket, NULL, how as u32, 0, 0);
  }

  #[inline]
	pub fn renameat(&mut self, olddirfd: c_int, oldpath: *const c_char, newdirfd: c_int, newpath: *const c_char) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_RENAMEAT, olddirfd, oldpath as *const c_void, newdirfd as u32, newpath as u64, 0);
  }

  #[inline]
  pub fn rename(&mut self, old: *const c_char, new: *const c_char) -> Option<&mut io_uring::sqe<T>> {
    return self.renameat(AT_FDCWD, old, AT_FDCWD, new);
  }

  #[inline]
	pub fn unlinkat(&mut self, dirfd: c_int, pathname: *const c_char, flags: c_int) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_UNLINKAT, dirfd, pathname as *const c_void, 0, 0, flags);
  }

  #[inline]
  pub fn unlink(&mut self, pathname: *const c_char) -> Option<&mut io_uring::sqe<T>> {
    return self.unlinkat(AT_FDCWD, pathname, 0);
  }

  #[inline]
	pub fn mkdirat(&mut self, dirfd: c_int, pathname: *const c_char, mode: mode_t) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_MKDIRAT, dirfd, pathname as *const c_void, mode as u32, 0, 0);
  }

  #[inline]
  pub fn mkdir(&mut self, path: *const c_char, mode: mode_t) -> Option<&mut io_uring::sqe<T>> {
    return self.mkdirat(AT_FDCWD, path, mode);
  }

  #[inline]
	pub fn symlinkat(&mut self, oldpath: *const c_char, newdirfd: c_int, newpath: *const c_char) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_SYMLINKAT, newdirfd, oldpath as *const c_void, 0, newpath as u64, 0);
  }

  #[inline]
  pub fn symlink(&mut self, path1: *const c_char, path2: *const c_char) -> Option<&mut io_uring::sqe<T>> {
    return self.symlinkat(path1, AT_FDCWD, path2);
  }

  #[inline]
	pub fn linkat(&mut self, olddirfd: c_int, oldpath: *const c_char, newdirfd: c_int, newpath: *const c_char, flags: c_int) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_LINKAT, olddirfd, oldpath as *const c_void, newdirfd as u32, newpath as u64, flags);
  }

  #[inline]
  pub fn link(&mut self, path1: *const c_char, path2: *const c_char) -> Option<&mut io_uring::sqe<T>> {
    return self.linkat(AT_FDCWD, path1, AT_FDCWD, path2, 0);
  }

  #[inline]
	pub fn fsetxattr(&mut self, fd: c_int, name: *const c_char, value: *const c_void, size: size_t, flags: c_int) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_FSETXATTR, fd, name as *const c_void, size as u32, value as u64, flags);
  }

  // #[inline]
	// pub fn setxattr(&mut self, fd: c_int) -> Option<&mut io_uring::sqe<T>> {
  //   return self.sq.prep();
  // }

  #[inline]
	pub fn fgetxattr(&mut self, fd: c_int, name: *const c_char, value: *mut c_void, size: size_t) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_FGETXATTR, fd, name as *const c_void, size as u32, value as u64, 0);
  }

  // #[inline]
	// pub fn getxattr(&mut self, fd: c_int) -> Option<&mut io_uring::sqe<T>> {
  //   return self.sq.prep();
  // }

  #[inline]
	pub fn socket(&mut self, domain: c_int, sock_type: c_int, protocol: c_int) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_SOCKET, domain, NULL, protocol as u32, sock_type as u64, 0);
  }
}

/* io_uring miscellaneous ops */
impl<T: Sized, U: Sized> Ring<T, U> {
  #[inline]
	pub fn nop(&mut self) -> Option<&mut io_uring::sqe<T>> {
    return self.sq.prep(IORING_OP_NOP, -1, NULL, 0, 0, 0);
  }

  // #[inline]
	// pub fn read_fixed(&mut self, fd: c_int) -> Option<&mut io_uring::sqe<T>> {
  //   return self.sq.prep();
  // }

  // #[inline]
	// pub fn write_fixed(&mut self, fd: c_int) -> Option<&mut io_uring::sqe<T>> {
  //   return self.sq.prep();
  // }

  // #[inline]
	// pub fn poll_add(&mut self, fd: c_int) -> Option<&mut io_uring::sqe<T>> {
  //   return self.sq.prep();
  // }

  // #[inline]
	// pub fn poll_remove(&mut self, fd: c_int) -> Option<&mut io_uring::sqe<T>> {
  //   return self.sq.prep();
  // }

  // #[inline]
	// pub fn timeout(&mut self, fd: c_int) -> Option<&mut io_uring::sqe<T>> {
  //   return self.sq.prep();
  // }

  // #[inline]
	// pub fn timeout_remove(&mut self, fd: c_int) -> Option<&mut io_uring::sqe<T>> {
  //   return self.sq.prep();
  // }

  // #[inline]
	// pub fn async_cancel(&mut self, fd: c_int) -> Option<&mut io_uring::sqe<T>> {
  //   return self.sq.prep();
  // }

  // #[inline]
	// pub fn link_timeout(&mut self, fd: c_int) -> Option<&mut io_uring::sqe<T>> {
  //   return self.sq.prep();
  // }

  // #[inline]
	// pub fn files_update(&mut self, fd: c_int) -> Option<&mut io_uring::sqe<T>> {
  //   return self.sq.prep();
  // }

  // #[inline]
	// pub fn provide_buffers(&mut self, fd: c_int) -> Option<&mut io_uring::sqe<T>> {
  //   return self.sq.prep();
  // }

  // #[inline]
	// pub fn remove_buffers(&mut self, fd: c_int) -> Option<&mut io_uring::sqe<T>> {
  //   return self.sq.prep();
  // }

  // #[inline]
	// pub fn msg_ring(&mut self, fd: c_int) -> Option<&mut io_uring::sqe<T>> {
  //   return self.sq.prep();
  // }

  // #[inline]
	// pub fn uring_cmd(&mut self, fd: c_int) -> Option<&mut io_uring::sqe<T>> {
  //   return self.sq.prep();
  // }
}