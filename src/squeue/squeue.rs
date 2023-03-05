use core::ffi::c_void;
use std::mem::size_of;
use std::sync::atomic::{AtomicU32, Ordering};
use libc::{munmap, memset};
use crate::io_uring::{self, *};
use crate::util::memmap;

#[derive(Debug)]
pub struct SQueue<T: Sized> {
  pub khead:        *mut AtomicU32,
  pub ktail:        *mut AtomicU32,
  pub kflags:       *mut AtomicU32,
  pub kdropped:     *mut AtomicU32,
  pub array:        *mut u32,
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
      khead: ring.add(p.sq_off.head as usize)            as *mut AtomicU32,
      ktail: ring.add(p.sq_off.tail as usize)            as *mut AtomicU32,
      kflags: ring.add(p.sq_off.tail as usize)           as *mut AtomicU32,
      kdropped: ring.add(p.sq_off.dropped as usize)      as *mut AtomicU32,
      array: ring.add(p.sq_off.array as usize).cast::<u32>(),
      sqes: memmap(fd, size, io_uring::IORING_OFF_SQES) as *mut io_uring::sqe<T>,
      sqe_head: 0,
      sqe_tail : 0,
      ring_mask: ring.add(p.sq_off.ring_mask as usize).cast::<u32>().read(),
      ring_entries: ring.add(p.sq_off.ring_entries as usize).cast::<u32>().read(),
      size: size,
    };
  }

  #[inline]
  pub(crate) fn needs_wakeup(&self) -> bool {
    unsafe { 
      return ((*self.kflags).load(Ordering::Acquire) & IORING_SQ_NEED_WAKEUP) > 0;
    };
  }

  pub(crate) fn next(&mut self) -> Option<*mut io_uring::sqe<T>> {
    let shift = if size_of::<sqe<T>>() == 128 { 1 } else { 0 };
    let index = (self.sqe_tail & self.ring_mask) << shift;
    let next = self.sqe_tail + 1;
    let head = unsafe { (*self.khead).load(Ordering::Acquire) };

    if (next - head) > self.ring_entries {
      return None;
    }

    self.sqe_tail = next;

    return Some(unsafe { self.sqes.add(index as usize) });
  }

  pub(crate) fn flush(&mut self) -> u32 {
    let tail = self.sqe_tail;

    if self.sqe_head != tail {      
      self.sqe_head = tail;

      unsafe { (*self.ktail).store(tail, Ordering::Release) };
    }

    return tail - unsafe { (*self.khead).load(Ordering::Acquire) };
  }

  pub(crate) fn prep(&mut self, op: u32, fd: i32, addr: *const c_void, len: u32, offset: u64, flags: u32) -> Option<*mut io_uring::sqe<T>> {
    let sqe = self.next()?;

    unsafe {
      memset(sqe as *mut c_void, 0, size_of::<io_uring::sqe<T>>());
      (*sqe).opcode      = op as u8;
      (*sqe).fd          = fd;
      (*sqe).addr2       = offset;
      (*sqe).addr1       = addr as u64;
      (*sqe).len         = len;
      (*sqe).op_flags    = flags;
    };

    return Some(sqe);
  }

}

impl<T: Sized> Drop for SQueue<T> {
  fn drop(&mut self) {
    unsafe { 
      munmap(self.sqes as *mut c_void,  self.size) 
    };
  }
}