use core::ffi::c_void;
use std::mem::size_of;
use std::sync::atomic::AtomicU32;
use libc::munmap;
use crate::util::ring_map;
use crate::io_uring;

#[derive(Debug, Clone)]
pub struct CQueue {
  pub ring:     *mut c_void,
  pub size:     usize,
  pub head:     *mut AtomicU32,
  pub tail:     *mut AtomicU32,
  pub mask:     *mut AtomicU32,
  pub entries:  *mut AtomicU32,
  pub overflow: *mut AtomicU32,
  pub cqes:     *mut AtomicU32,
}

impl CQueue {
  pub fn new(fd: i32, params: &io_uring::params) -> CQueue {
    let cqe_size = if (params.flags & io_uring::IORING_SETUP_CQE32) > 0 { 
      size_of::<io_uring::cqe<[u64; 2]>>() 
    }
    else {
      size_of::<io_uring::cqe<[u64; 0]>>() 
    };
    let size = params.cq_off.cqes as usize
      + params.cq_entries as usize
      * cqe_size as usize;
    let ring = ring_map(fd, size, io_uring::IORING_OFF_CQ_RING);

    return CQueue {
      ring: ring,
      size: size,
      head: unsafe { ring.add(params.cq_off.head as usize) } as *mut AtomicU32,
      tail: unsafe { ring.add(params.cq_off.tail as usize) } as *mut AtomicU32,
      mask: unsafe { ring.add(params.cq_off.ring_mask as usize) } as *mut AtomicU32,
      entries: unsafe { ring.add(params.cq_off.ring_entires as usize) } as *mut AtomicU32,
      overflow: unsafe { ring.add(params.cq_off.overflow as usize) } as *mut AtomicU32,
      cqes: unsafe { ring.add(params.cq_off.cqes as usize) } as * mut AtomicU32,
    };
  }
}

impl Drop for CQueue {
  fn drop(&mut self) {
    unsafe { munmap(self.ring, self.size) };
  }
}