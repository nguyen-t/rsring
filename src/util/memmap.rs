use std::io::Error;
use core::ffi::{c_void, c_int};
use libc::{mmap, MAP_FAILED, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_POPULATE};

pub fn memmap(fd: c_int, len: usize, offset: i64) -> *mut c_void {
  let map = unsafe { 
    mmap(std::ptr::null_mut::<c_void>(), len, PROT_READ | PROT_WRITE, MAP_SHARED | MAP_POPULATE, fd, offset) 
  };

  if map == MAP_FAILED {
    panic!("{}", Error::last_os_error());
  }

  return map;
}