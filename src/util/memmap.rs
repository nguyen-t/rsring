use core::ffi::{c_void, c_int};
use libc::{mmap, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_POPULATE};

pub fn memmap(fd: c_int, size: usize, offset: i64) -> *mut c_void {
  return unsafe { 
    mmap(std::ptr::null_mut(), size, PROT_READ | PROT_WRITE, MAP_SHARED | MAP_POPULATE, fd, offset) 
  };
}