use core::ffi::{c_void, c_size_t, c_int, c_uint};
use std::{io::Error};
use libc::{syscall, SYS_io_uring_setup, SYS_io_uring_enter, SYS_io_uring_register};

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct cqe {
  pub user_data: u64,
  pub res: i64,
  pub flags: u32,
  pub big_cqe: [u64; 4],
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct sqe {
  pub opcode: u8,
  pub flags: u8,
  pub ioprio: u16,
  pub fd: i32,
  pub addr1: u64,
  pub addr2: u64,
  pub len: u32,
  pub op_flags: u64,
  pub user_data: u64,
  pub buf_select: u16,
  pub personality: u16,
  pub file_select: u32,
  pub addr3: u64,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct sqring_offsets {
  pub head: u32,
  pub tail: u32,
  pub ring_mask: u32,
  pub ring_entries: u32,
  pub flags: u32,
  pub dropped: u32,
  pub array: u32,
  pub resv: [u32; 3],
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct cqring_offsets {
  pub head: u32,
  pub tail: u32,
  pub ring_mask: u32,
  pub ring_entires: u32,
  pub overflow: u32,
  pub cqes: u32,
  pub flags: u32,
  pub resv: [u32; 3],
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct params {
  pub sq_entries: u32,
  pub cq_entries: u32,
  pub flags: u32,
  pub sq_thread_cpu: u32,
  pub sq_thread_idle: u32,
  pub resv: [u32; 5],
  pub sq_off: sqring_offsets,
  pub cq_off: cqring_offsets,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct getevents_args {
  pub sigmask: u64,
  pub sigmask_sz: u32,
  pub pad: u32,
  pub ts: u64,
}

pub fn setup(entries: c_uint, p: *mut params) -> Result<c_int, Error> {
  let r = unsafe {
    syscall(SYS_io_uring_setup, entries, p)
  };

  return if r < 0 { Err(Error::last_os_error()) } else { Ok(r as c_int) };
}

pub fn enter(fd: c_int, to_submit: c_uint, min_complete: c_uint, flags: c_uint, arg: *mut c_void, argsz: c_size_t) -> Result<c_int, Error> {
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