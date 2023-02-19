use core::ffi::{c_void, c_size_t, c_int, c_uint};
use std::{io::Error, mem::size_of};
use libc::{syscall, sigset_t, SYS_io_uring_setup, SYS_io_uring_enter, SYS_io_uring_register};
use super::definitions;

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
  pub fn get_data<U>(&self) -> *const U {
    return self.user_data as *const U;
  }

  pub fn set_data<U>(&mut self, data: *const U) {
    self.user_data = data as u64;
  }

  pub fn get_data_u64(&self) -> u64 { 
    return self.user_data;
  }

  pub fn set_data_u64(&mut self, data: u64) {
    self.user_data = data;
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