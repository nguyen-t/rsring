use core::ffi::c_void;
use std::mem::size_of;
use std::sync::atomic::{AtomicU32, Ordering, fence};
use libc::munmap;
use crate::io_uring;
use crate::util::memmap;

#[derive(Debug)]
pub struct SQueue<T: Sized> {
  pub khead:        *mut AtomicU32,
  pub ktail:        *mut AtomicU32,
  pub kflags:       *mut AtomicU32,
  pub kdropped:     *mut AtomicU32,
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
      khead: ring.add(p.sq_off.head as usize)            as *mut AtomicU32,
      ktail: ring.add(p.sq_off.tail as usize)            as *mut AtomicU32,
      kflags: ring.add(p.sq_off.tail as usize)           as *mut AtomicU32,
      kdropped: ring.add(p.sq_off.dropped as usize)      as *mut AtomicU32,
      array: ring.add(p.sq_off.array as usize)           as *mut AtomicU32,
      sqes: memmap(fd, size, io_uring::IORING_OFF_SQES) as *mut io_uring::sqe<T>,
      sqe_head: 0,
      sqe_tail : 0,
      ring_mask: ring.add(p.sq_off.ring_mask as usize).cast::<u32>().read(),
      ring_entries: ring.add(p.sq_off.ring_entries as usize).cast::<u32>().read(),
      size: size,
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
  pub fn get_kdropped(&self, order: Ordering) -> u32 {
    return unsafe { self.kdropped.read().load(order) };
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
  pub fn set_kdropped(&self, data: u32, order: Ordering) {
    return unsafe { self.kdropped.read().store(data, order) };
  }

  pub fn needs_enter(&self, submit: u32, flags: u32, wakeup: &mut u32) -> bool {
    if submit == 0 {
      return false;
    }
    if (flags & io_uring::IORING_SETUP_SQPOLL) == 0 {
      return true;
    }

    fence(Ordering::SeqCst);

    if (self.get_kflags(Ordering::Relaxed) & io_uring::IORING_SQ_NEED_WAKEUP) == 0 {
      *wakeup |= io_uring::IORING_ENTER_SQ_WAKEUP;

      return true;
    }

    return false;
  }

  pub unsafe fn flush_sq(&mut self, flags: u32) -> u32 {
    let tail = self.sqe_tail;
    let sqpoll = (flags & io_uring::IORING_SETUP_SQPOLL) > 0;

    if self.sqe_head != tail {
      self.sqe_head = tail;
      self.ktail.read().store(tail, if sqpoll { Ordering::Release } else { Ordering::Relaxed });
    }

    return tail - self.khead.read().load(Ordering::Relaxed);
  }
}

impl<T: Sized> Drop for SQueue<T> {
  fn drop(&mut self) {
    unsafe { 
      munmap(self.sqes as *mut c_void,  self.size) 
    };
  }
}