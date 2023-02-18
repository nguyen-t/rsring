use core::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use crate::io_uring;

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
  pub fn get_khead(&self, order: Ordering) -> u32 {
    return unsafe { self.khead.read().load(order) };
  }

  #[inline]
  pub fn get_ktail(&self, order: Ordering) -> u32 {
    return unsafe { self.ktail.read().load(order) };
  }

  #[inline]
  pub fn get_kflags(&self, order: Ordering) -> u32 {
    return unsafe { self.kflags.read().load(order) };
  }

  #[inline]
  pub fn get_koverflow(&self, order: Ordering) -> u32 {
    return unsafe { self.koverflow.read().load(order) };
  }

  #[inline]
  pub fn set_khead(&self, data: u32, order: Ordering) {
    return unsafe { self.khead.read().store(data, order) };
  }

  #[inline]
  pub fn set_ktail(&self, data: u32, order: Ordering) {
    return unsafe { self.ktail.read().store(data, order) };
  }

  #[inline]
  pub fn set_kflags(&self, data: u32, order: Ordering) {
    return unsafe { self.kflags.read().store(data, order) };
  }

  #[inline]
  pub fn set_overflow(&self, data: u32, order: Ordering) {
    return unsafe { self.koverflow.read().store(data, order) };
  }

  pub fn needs_flush(&self) -> bool {
    let flags = io_uring::IORING_SQ_CQ_OVERFLOW | io_uring::IORING_SQ_TASKRUN;
    
    return (self.get_kflags(Ordering::Relaxed) & flags) > 0;
  }

  pub fn needs_enter(&self, flags: u32) -> bool {
    return (flags & io_uring::IORING_SETUP_IOPOLL) > 0 || self.needs_flush();
  }
}