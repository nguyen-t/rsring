use core::ffi::c_void;
use std::sync::atomic::AtomicU32;
use crate::io_uring;

#[derive(Debug, Clone)]
pub struct CQueue<T: Sized> {
  pub khead:     *const AtomicU32,
  pub ktail:     *const AtomicU32,
  pub kflags:    *const AtomicU32,
  pub koverflow: *const AtomicU32,
  pub cqes:      *mut io_uring::cqe<T>,
  pub ring_mask:    u32,
  pub ring_entries: u32,
}

impl<T: Sized> CQueue<T> {
  pub unsafe fn new(ring: *mut c_void, p: &io_uring::params) -> CQueue<T> {
    return CQueue {
      khead: ring.add(p.cq_off.head as usize)         as *const AtomicU32,
      ktail: ring.add(p.cq_off.tail as usize)         as *const AtomicU32,
      kflags: ring.add(p.cq_off.flags as usize)       as *const AtomicU32,
      koverflow: ring.add(p.cq_off.overflow as usize) as *const AtomicU32,
      cqes: ring.add(p.cq_off.cqes as usize)          as *mut io_uring::cqe<T>,
      ring_mask: ring.add(p.cq_off.ring_mask as usize).cast::<u32>().read(),
      ring_entries: ring.add(p.cq_off.ring_entries as usize).cast::<u32>().read(),
    };
  }
}