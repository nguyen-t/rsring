use core::ffi::{c_void, c_size_t, c_int, c_uint};
use std::{io::Error, mem::size_of};
use libc::{syscall, sigset_t, SYS_io_uring_setup, SYS_io_uring_enter, SYS_io_uring_register};
use super::definitions::*;
use super::constants::*;

/* Rust wrapper for io_uring syscalls */
pub fn setup(entries: c_uint, p: *mut params) -> Result<c_int, Error> {
  let r = unsafe {
    syscall(SYS_io_uring_setup, entries, p)
  };

  if r < 0 { Err(Error::last_os_error()) } else { Ok(r as c_int) }
}

pub fn enter(fd: c_int, to_submit: c_uint, min_complete: c_uint, flags: c_uint, sig: *const sigset_t) -> Result<c_int, Error> {
  let r = unsafe {
    syscall(SYS_io_uring_enter, fd, to_submit, min_complete, flags, sig, size_of::<sigset_t>())
  };

  if r < 0 { Err(Error::last_os_error()) } else { Ok(r as c_int) }
}

pub fn enter2(fd: c_int, to_submit: c_uint, min_complete: c_uint, flags: c_uint, arg: *const c_void, argsz: c_size_t) -> Result<c_int, Error> {
  let r = unsafe {
    syscall(SYS_io_uring_enter, fd, to_submit, min_complete, flags, arg, argsz)
  };

  if r < 0 { Err(Error::last_os_error()) } else { Ok(r as c_int) }
}

pub fn register(fd: c_int, opcode: c_uint, arg: *mut c_void, nr_args: c_uint) -> Result<c_int, Error> {
  let r = unsafe {
    syscall(SYS_io_uring_register, fd, opcode, arg, nr_args)
  };

  if r < 0 { Err(Error::last_os_error()) } else { Ok(r as c_int) }
}

impl __kernel_timespec {
  pub fn from_ms(ms: i64) -> __kernel_timespec {
    __kernel_timespec {
      tv_sec:  (ms / 1000),
      tv_nsec: ((ms % 1000) * 1000000),
    }
  }
}

impl<T: Sized> sqe<T> {
  /* Sets user data field for pointer types */
  pub fn set_data_ptr(&mut self, data: *const c_void) -> &mut Self {
    self.user_data = data as u64;

    self
  }

  /* Sets user data field for u64 */
  pub fn set_data_u64(&mut self, data: u64) -> &mut Self {
    self.user_data = data;

    self
  }

  pub fn link(&mut self) -> &mut Self {
    self.flags |= IOSQE_IO_LINK as u8;

    self
  }

  pub fn hardlink(&mut self) -> &mut Self {
    self.flags |= IOSQE_IO_HARDLINK as u8;

    self
  }

  /* TODO: Prevent shooting yourself in the foot */
  pub fn direct(&mut self, file_index: u32) -> &mut Self {
    self.file_select = file_index + 1;

    self
  }

  /* Enables multishot on compatible ops */
  pub fn multishot(&mut self) -> &mut Self {
    self.ioprio |= match self.opcode as u32 {
      IORING_OP_RECVMSG  => IORING_RECV_MULTISHOT,
      IORING_OP_POLL_ADD => IORING_POLL_ADD_MULTI,
      IORING_OP_ACCEPT   => IORING_ACCEPT_MULTISHOT,
      IORING_OP_RECV     => IORING_RECV_MULTISHOT,
      _                  => 0,
    } as u16;

    self
  }

  /* Enables zerocopy on compatible ops */
  pub fn zerocopy(&mut self, ioprio: u16) -> &mut Self {
    self.ioprio |= match self.opcode as u32 {
      IORING_OP_SEND    => ioprio,
      IORING_OP_SENDMSG => ioprio,
      _                 => 0,
    };
    self.opcode = match self.opcode as u32 {
      IORING_OP_SEND    => IORING_OP_SEND_ZC as u8,
      IORING_OP_SENDMSG => IORING_OP_SENDMSG_ZC as u8,
      _                 => self.opcode,
    };

    self
  }
}

impl<T: Sized> cqe<T> {
  /* Gets user data field for pointer types */
  pub fn get_data_ptr(&self) -> *const c_void {
    self.user_data as *const c_void
  }

  /* Gets user data field for u64 */
  pub fn get_data_u64(&self) -> u64 {
    self.user_data
  }
}

impl params {
  pub fn new(flags: u32) -> params {
    params {
      sq_entries: 0,
      cq_entries: 0,
      flags: flags,
      sq_thread_cpu: 0,
      sq_thread_idle: 0,
      features: 0,
      wd_fd: 0,
      resv: [0, 0, 0],
      sq_off: sqring_offsets::default(),
      cq_off: cqring_offsets::default(),
    }
  }
}

impl getevents_arg {
  pub fn new(mask: *const sigset_t, ts: *const __kernel_timespec) -> getevents_arg {
    const _NSIG: u32 = 64;

    getevents_arg {
      sigmask:    mask as u64,
      sigmask_sz: (_NSIG) / 8,
      pad:        0,
      ts:         ts as u64,
    }
  }
}