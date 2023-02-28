use std::mem::size_of;
use std::ffi::c_void;
use libc::{memset};
use std::sync::atomic::Ordering;

use crate::io_uring::{self, *};
use crate::ring::Ring;

impl<T: Sized, U: Sized> Ring<T, U> {
  pub(crate) fn sqe_get(&mut self) -> Option<*mut io_uring::sqe<T>> {
    let next = self.sq.sqe_tail + 1;
    let head = self.sq.get_khead(Ordering::Acquire);
    let shift = self.has_flag(IORING_SETUP_SQE128) as u32;
    let index = (self.sq.sqe_tail & self.sq.ring_mask) << shift;

    if (next - head) > self.sq.ring_entries {
      return None;
    }

    self.sq.sqe_tail = next;

    return Some(unsafe { self.sq.sqes.add(index as usize) });
  }

  pub(crate) fn sqe_flush(&mut self) -> u32 {
    let tail = self.sq.sqe_tail;

    if self.sq.sqe_head != tail {
      self.sq.sqe_head = tail;
      self.sq.set_ktail(tail, if self.has_flag(IORING_SETUP_SQPOLL) { Ordering::Release } else { Ordering::Relaxed });
    }

    return tail - self.sq.get_khead(Ordering::Relaxed);
  }

  pub(crate) fn sqe_prep(&mut self, op: u32, fd: i32, addr: *const c_void, len: u32, offset: u64, flags: u32) -> Option<*mut io_uring::sqe<T>> {
    let sqe = self.sqe_get()?;

    unsafe {
      memset(sqe as *mut c_void, 0, size_of::<io_uring::sqe<T>>());
      (*sqe).opcode    = op as u8;
      (*sqe).fd        = fd;
      (*sqe).addr2     = offset;
      (*sqe).addr1     = addr as u64;
      (*sqe).len       = len;
      (*sqe).op_flags  = flags;
    };

    return Some(sqe);
  }
}