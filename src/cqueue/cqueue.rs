use std::io::Error;
use std::mem::size_of;
use core::ffi::c_void;
use std::sync::atomic::{AtomicU32, Ordering};
use crate::io_uring::{self, *};

#[derive(Debug, Clone)]
pub struct CQueue<T: Sized> {
  pub khead:        *mut AtomicU32,
  pub ktail:        *mut AtomicU32,
  pub kflags:       *mut AtomicU32,
  pub koverflow:    *mut AtomicU32,
  pub cqes:         *mut io_uring::cqe<T>,
  pub ring_mask:    u32,
  pub ring_entries: u32,
  pub ext_arg:      bool,
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
      ext_arg: (p.features & IORING_FEAT_EXT_ARG) > 0,
    };
  }

  #[inline]
  pub fn overflowed(&self) -> bool {
    let flags = IORING_SQ_CQ_OVERFLOW;
    unsafe {
      return ((*self.kflags).load(Ordering::Acquire) & flags) > 0;
    };
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

  pub(crate) fn peek(&mut self) -> Result<(Option<*mut io_uring::cqe<T>>, u32), Error> {
    let mask = self.ring_mask;
    let shift = if size_of::<cqe<T>>() == 32 { 1 } else { 0 };

    loop {
      let tail = unsafe { (*self.ktail).load(Ordering::Acquire) };
      let head = unsafe { (*self.khead).load(Ordering::Acquire) };
      let available = tail - head;

      if available == 0 {
        return Ok((None, 0));
      }

      let index = (head & mask) << shift;
      let cqe = unsafe { self.cqes.add(index as usize) };
      let timeout = unsafe { (*cqe).user_data == !0 };

      if !self.ext_arg && timeout {
        let res = unsafe { (*cqe).res };
       
        self.advance(1);

        if res >= 0 {
          continue;
        }
        
        return Err(Error::from_raw_os_error(res));
      }

      return Ok((Some(cqe), available));
    };
  }

}