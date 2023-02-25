use core::ffi::{c_void, c_size_t, c_int, c_uint};
use std::{io::Error, mem::size_of};
use libc::{syscall, sigset_t, SYS_io_uring_setup, SYS_io_uring_enter, SYS_io_uring_register};
use super::definitions;
use super::constants::*;

/* Rust wrapper for io_uring syscalls */
pub fn setup(entries: c_uint, p: *mut definitions::params) -> Result<c_int, Error> {
  let r = unsafe {
    syscall(SYS_io_uring_setup, entries, p)
  };

  return if r < 0 { Err(Error::last_os_error()) } else { Ok(r as c_int) };
}

pub fn enter(fd: c_int, to_submit: c_uint, min_complete: c_uint, flags: c_uint, sig: *mut sigset_t) -> Result<c_int, Error> {
  let r = unsafe {
    syscall(SYS_io_uring_enter, fd, to_submit, min_complete, flags, sig, size_of::<sigset_t>())
  };

  return if r < 0 { Err(Error::last_os_error()) } else { Ok(r as c_int) };
}

pub fn enter2(fd: c_int, to_submit: c_uint, min_complete: c_uint, flags: c_uint, arg: *mut c_void, argsz: c_size_t) -> Result<c_int, Error> {
  let r = unsafe {
    syscall(SYS_io_uring_enter, fd, to_submit, min_complete, flags, arg, argsz)
  };

  return if r < 0 { Err(Error::last_os_error()) } else { Ok(r as c_int) };
}

pub fn register(fd: c_int, opcode: c_uint, arg: *mut c_void, nr_args: c_uint) -> Result<c_int, Error> {
  let r = unsafe {
    syscall(SYS_io_uring_register, fd, opcode, arg, nr_args)
  };

  return if r < 0 { Err(Error::last_os_error()) } else { Ok(r as c_int) };
}

impl<T: Sized> definitions::sqe<T> {
  /* Sets user data field for pointer types */
  pub fn set_data_ptr(&mut self, data: *const c_void) -> &Self {
    self.user_data = data as u64;

    return self;
  }

  /* Sets user data field for u64 */
  pub fn set_data_u64(&mut self, data: u64) -> &Self {
    self.user_data = data;

    return self;
  }

  /* TODO: Prevent shooting yourself in the foot */
  pub fn direct(&mut self, file_index: u32) -> &Self {
    self.file_select = file_index + 1;

    return self;
  }

  /* Enables multishot on compatible ops */
  pub fn multishot(&mut self) -> &Self {
    self.ioprio |= match self.opcode as u32 {
      IORING_OP_RECVMSG  => IORING_RECV_MULTISHOT,
      IORING_OP_POLL_ADD => IORING_POLL_ADD_MULTI,
      IORING_OP_ACCEPT   => IORING_ACCEPT_MULTISHOT,
      IORING_OP_RECV     => IORING_RECV_MULTISHOT,
      _                  => 0,
    } as u16;

    return self;
  }

  /* Enables zeroshot on compatible ops */
  pub fn zeroshot(&mut self, ioprio: u16) -> &Self {
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

    return self;
  }
}

impl<T: Sized> definitions::cqe<T> {
  /* Gets user data field for pointer types */
  pub fn get_data_ptr(&self) -> *const c_void {
    return self.user_data as *const c_void;
  }

  /* Gets user data field for u64 */
  pub fn get_data_u64(&self) -> u64 {
    return self.user_data;
  }
}

impl definitions::params {
  pub fn new(flags: u32) -> definitions::params {
    return definitions::params {
      sq_entries: 0,
      cq_entries: 0,
      flags: flags,
      sq_thread_cpu: 0,
      sq_thread_idle: 0,
      features: 0,
      wd_fd: 0,
      resv: [0, 0, 0],
      sq_off: definitions::sqring_offsets::default(),
      cq_off: definitions::cqring_offsets::default(),
    };
  }
}