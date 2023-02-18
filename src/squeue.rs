use core::ffi::c_void;
use std::mem::size_of;
use std::sync::atomic::AtomicU32;
use libc::munmap;
use crate::io_uring;
use crate::util::memmap;

#[derive(Debug)]
pub struct SQueue<T: Sized> {
  pub khead:        *const AtomicU32,
  pub ktail:        *const AtomicU32,
  pub kflags:       *const AtomicU32,
  pub kdropped:     *const AtomicU32,
  pub array:        *mut AtomicU32,
  pub sqes:         *mut io_uring::sqe<T>,
  pub sqe_head:     u32,
  pub sqe_tail:     u32,
  pub ring_mask:    u32,
  pub ring_entries: u32,
  pub size:         usize,
}

impl<T: Sized> SQueue<T> {
  pub unsafe fn new(ring: *mut c_void, p: &io_uring::params, fd: i32) -> SQueue<T> {
    let size = size_of::<io_uring::sqe<T>>() * p.sq_entries as usize;
  
    return SQueue {
      khead: ring.add(p.sq_off.head as usize)            as *const AtomicU32,
      ktail: ring.add(p.sq_off.tail as usize)            as *const AtomicU32,
      kflags: ring.add(p.sq_off.tail as usize)           as *const AtomicU32,
      kdropped: ring.add(p.sq_off.dropped as usize)      as *const AtomicU32,
      array: ring.add(p.sq_off.array as usize)           as *mut AtomicU32,
      sqes: memmap(fd, size, io_uring::IORING_OFF_SQES) as *mut io_uring::sqe<T>,
      sqe_head: 0,
      sqe_tail : 0,
      ring_mask: ring.add(p.sq_off.ring_mask as usize).cast::<u32>().read(),
      ring_entries: ring.add(p.sq_off.ring_entries as usize).cast::<u32>().read(),
      size: size,
    };
  }
}

impl<T: Sized> Drop for SQueue<T> {
  fn drop(&mut self) {
    unsafe { 
      munmap(self.sqes as *mut c_void,  self.size) 
    };
  }
}