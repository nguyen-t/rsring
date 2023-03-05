use std::mem::size_of;
use core::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use crate::io_uring::{self, *};

#[derive(Debug, Clone)]
pub struct CQueue<T: Sized> {
  pub(crate) khead:        *mut AtomicU32,
  pub(crate) ktail:        *mut AtomicU32,
  pub(crate) kflags:       *mut AtomicU32,
  pub(crate) koverflow:    *mut AtomicU32,
  pub(crate) cqes:         *mut io_uring::cqe<T>,
  pub(crate) ring_mask:    u32,
  pub(crate) ring_entries: u32,
}

impl<T: Sized> CQueue<T> {
  pub unsafe fn new(ring: *mut c_void, p: &io_uring::params) -> CQueue<T> {
    return CQueue {
      khead: ring.add(p.cq_off.head as usize)         as *mut AtomicU32,
      ktail: ring.add(p.cq_off.tail as usize)         as *mut AtomicU32,
      kflags: ring.add(p.cq_off.flags as usize)       as *mut AtomicU32,
      koverflow: ring.add(p.cq_off.overflow as usize) as *mut AtomicU32,
      cqes: ring.add(p.cq_off.cqes as usize)          as *mut io_uring::cqe<T>,
      ring_mask: ring.add(p.cq_off.ring_mask as usize).cast::<u32>().read(),
      ring_entries: ring.add(p.cq_off.ring_entries as usize).cast::<u32>().read(),
    };
  }

  #[inline]
  pub(crate) fn available(&self) -> u32 {
    let tail = unsafe { (*self.ktail).load(Ordering::Acquire) };
    let head = unsafe { (*self.khead).load(Ordering::Acquire) };

    return tail - head;
  }

  #[inline]
  pub(crate) fn needs_flush(&self) -> bool {
    let flags = IORING_SQ_CQ_OVERFLOW | IORING_SQ_TASKRUN;

    unsafe {
      return ((*self.kflags).load(Ordering::Acquire) & flags) > 0;
    };
  }

  #[inline]
  pub(crate) fn advance(&mut self, nr: u32) {
    unsafe {
      (*self.khead).store((*self.khead).load(Ordering::Acquire) + nr, Ordering::Release);
    };
  }

  pub(crate) fn next(&mut self) -> Option<*mut io_uring::cqe<T>> {
    let shift = if size_of::<cqe<T>>() == 32 { 1 } else { 0 };
    let tail = unsafe { (*self.ktail).load(Ordering::Acquire) };
    let head = unsafe { (*self.khead).load(Ordering::Acquire) };
    let index = (head & self.ring_mask) << shift;
    
    if tail - head == 0 {
      return None;
    }

    return Some(unsafe { self.cqes.add(index as usize) });
  }
}