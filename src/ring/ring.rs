use std::io::Error;
use std::ptr;
use std::mem::size_of;
use std::ffi::c_void;
use libc::{sigset_t, munmap, close};

use crate::io_uring::{self, *};
use crate::util::memmap;
use crate::squeue::SQueue;
use crate::cqueue::CQueue;

#[derive(Debug)]
pub struct Ring<T: Sized, U: Sized> {
  pub ring:       *mut c_void,
  pub size:       usize,
  pub ring_fd:    i32,
  pub enter_fd:   i32,
  pub flags:      u32,
  pub features:   u32,
  pub registered: bool,
  pub sq:         SQueue<T>,
  pub cq:         CQueue<U>,
}

impl<T: Sized, U: Sized> Ring<T, U> {
  pub fn new(depth: u32) -> Result<Ring<T, U>, Error> {
    let mut p = io_uring::params::new(Ring::<T, U>::init_flags());
    let fd = match io_uring::setup(depth, ptr::addr_of_mut!(p)) {
      Ok(fd) => fd,
      Err(e) => return Err(e)
    };
    let sq_size = p.sq_off.array as usize + p.sq_entries as usize * size_of::<u32>();
    let cq_size = p.cq_off.cqes as usize + p.cq_entries as usize * size_of::<io_uring::cqe<T>>();
    let size = core::cmp::max(sq_size, cq_size);
    let ring = memmap(fd, size, IORING_OFF_SQ_RING);
    let sq = unsafe { SQueue::<T>::new(ring, &p, fd) };
    let cq = unsafe { CQueue::<U>::new(ring, &p) };

    for i in 0..sq.ring_entries {
      unsafe {
        *sq.array.add(i as usize) = i as u32;
      };
    }

    return Ok(Ring {
      ring: ring,
      size: size,
      ring_fd: fd,
      enter_fd: fd,
      flags: p.flags,
      features: p.features,
      registered: false,
      sq: sq,
      cq: cq,
    });
  }

  pub fn submit(&mut self, wait_nr: u32) -> Result<i32, Error> {
    let to_submit = self.sq.flush();
    let submit = to_submit != 0;
    let sqpoll = self.has_flag(IORING_SETUP_SQPOLL);
    let iopoll = self.has_flag(IORING_SETUP_IOPOLL);
    let wakeup = self.sq.needs_wakeup();
    let flush = self.cq.needs_flush();
    let sq_enter = (submit && !sqpoll) || (submit && wakeup);
    let cq_enter = iopoll || flush;

    if wait_nr > 0 || sq_enter || cq_enter {
      let register = false;
      let flags = if register { IORING_ENTER_REGISTERED_RING } else { 0 }
        | if sq_enter { IORING_SQ_NEED_WAKEUP } else { 0 }
        | if cq_enter { IORING_ENTER_GETEVENTS } else { 0 };

      return io_uring::enter(self.enter_fd, to_submit, wait_nr, flags, ptr::null_mut::<sigset_t>());
    }

    return Ok(to_submit as i32);
  }

  pub fn next(&mut self) {
    self.cq.advance(1);
  }

  pub(crate) fn init_flags() -> u32 {
    let sqe_setup = match size_of::<SQueue<T>>() {
      64  => 0,
      128 => IORING_SETUP_SQE128,
      _   => 0,
    };
    let cqe_setup = match size_of::<CQueue<T>>() {
      16 => 0,
      32 => IORING_SETUP_CQE32,
      _  => 0,
    };

    return IORING_SETUP_SQPOLL
    | IORING_SETUP_SUBMIT_ALL
    | sqe_setup 
    | cqe_setup;
  }

  pub(crate) fn has_flag(&self, flag: u32) -> bool {
    return (self.flags & flag) > 0;
  }
}

impl<T: Sized, U: Sized> Drop for Ring<T, U> {
  fn drop(&mut self) {
    unsafe { 
      munmap(self.ring, self.size);
      close(self.ring_fd);
      close(self.enter_fd);
    };
  }
}

