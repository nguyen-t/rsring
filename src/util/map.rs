use std::io::Error;
use core::ffi::{c_void, c_int};
use libc::{mmap, munmap, MAP_FAILED, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_POPULATE};

#[derive(Debug)]
pub struct Map<T> {
  data: *mut T,
  size: usize,
}

impl<T> Map<T> {
  pub fn new(fd: c_int, len: usize, offset: i64) -> Result<Self, Error> {
    let map = unsafe {
      mmap(std::ptr::null_mut::<c_void>(), len, PROT_READ | PROT_WRITE, MAP_SHARED | MAP_POPULATE, fd, offset)
    };

    if map == MAP_FAILED {
      return Err(Error::last_os_error());
    }

    return Ok(Map {
      data: map as *mut T,
      size: len,
    });
  }

  pub fn raw(&self) -> *mut T {
    return self.data;
  }

  pub fn add(&self, count: usize) -> *mut T {
    return unsafe { self.data.add(count) };
  }

  pub fn sub(&self, count: usize) -> *mut T {
    return unsafe { self.data.sub(count) };
  }

  pub fn offset(&self, count: isize) -> *mut T {
    return unsafe { self.data.offset(count) };
  }
}

impl<T> Drop for Map<T> {
  fn drop(&mut self) {
    unsafe {
      munmap(self.data as *mut c_void, self.size);
    }
  }
}
