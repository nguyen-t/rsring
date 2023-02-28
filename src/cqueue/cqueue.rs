use core::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use crate::io_uring::{self, *};

#[derive(Debug, Clone)]
pub struct CQueue<T: Sized> {
  pub khead:     *mut AtomicU32,
  pub ktail:     *mut AtomicU32,
  pub kflags:    *mut AtomicU32,
  pub koverflow: *mut AtomicU32,
  pub cqes:      *mut io_uring::cqe<T>,
  pub ring_mask:    u32,
  pub ring_entries: u32,
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
  pub fn overflowed(&self) -> bool {
    unsafe {
      return (self.kflags.read().load(Ordering::Relaxed) & (IORING_SQ_CQ_OVERFLOW)) > 0;
    };
  }

  #[inline]
  pub(crate) fn needs_flush(&self) -> bool {
    unsafe {
      return (self.kflags.read().load(Ordering::Relaxed) & (IORING_SQ_CQ_OVERFLOW | IORING_SQ_TASKRUN)) > 0;
    };
  }


}