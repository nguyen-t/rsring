use core::ffi::{c_void, c_uint};
use std::mem::size_of;
use std::sync::atomic::AtomicU32;
use libc::munmap;
use crate::io::{ring_map, ioring, io_uring};

#[derive(Debug)]
pub struct SQueue {
  pub ring: *mut c_void,
  pub size: usize,
  pub head: *mut AtomicU32,
  pub tail: *mut AtomicU32,
  pub mask: *mut AtomicU32,
  pub entries: *mut AtomicU32,
  pub flags: *mut AtomicU32,
  pub array: *mut AtomicU32,
}

impl SQueue {
  pub fn new(fd: i32, params: &io_uring::params) -> SQueue {
    let size = params.sq_off.array as usize 
      + params.sq_entries as usize
      * size_of::<c_uint>() as usize;
    let ring = ring_map(fd, size, ioring::OFF::SQ_RING as i64);

    return SQueue {
      ring: ring,
      size: size,
      head: unsafe { ring.add(params.cq_off.head as usize) } as *mut AtomicU32,
      tail: unsafe { ring.add(params.cq_off.tail as usize) } as *mut AtomicU32,
      mask: unsafe { ring.add(params.cq_off.ring_mask as usize) } as *mut AtomicU32,
      entries: unsafe { ring.add(params.cq_off.ring_entires as usize) } as *mut AtomicU32,
      flags: unsafe { ring.add(params.sq_off.flags as usize) } as *mut AtomicU32,
      array: unsafe { ring.add(params.sq_off.array as usize) } as *mut AtomicU32,
    };
  }
}

impl Drop for SQueue {
  fn drop(&mut self) {
    unsafe { munmap(self.ring, self.size) };
  }
}