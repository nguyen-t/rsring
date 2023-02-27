use std::io::Error;
use std::ptr;
use std::mem::size_of;
use std::ffi::c_void;
use libc::{sigset_t, munmap, close};
use std::sync::atomic::Ordering;

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
    let mut sq = unsafe { SQueue::<T>::new(ring, &p, fd) };
    let cq = unsafe { CQueue::<U>::new(ring, &p) };

    for i in 0..sq.ring_entries {
      sq.set_array(i as usize, i as u32, Ordering::Relaxed);
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

  pub fn submit(&self, to_submit: u32, wait_nr: u32, getevents: bool) -> Result<i32, Error> {
    let sq_enter = self.sq.needs_enter(to_submit);
    let cq_enter = self.cq.needs_enter();
    let self_enter = getevents || wait_nr > 0 || self.has_flag(IORING_SETUP_IOPOLL) || !self.has_flag(IORING_SETUP_SQPOLL);

    if self_enter || sq_enter || cq_enter {
      let flags = 0 // TODO: self.int_flags & INT_FLAG_REG_RING
      | IORING_SQ_NEED_WAKEUP * (sq_enter as u32)
      | IORING_ENTER_GETEVENTS * (cq_enter as u32);
      
      return io_uring::enter(self.enter_fd, to_submit, wait_nr, flags, ptr::null_mut::<sigset_t>());
    }

    return Ok(to_submit as i32);
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
    // | IORING_SETUP_R_DISABLED
    | IORING_SETUP_SUBMIT_ALL
    | sqe_setup 
    | cqe_setup;
  }

  pub(crate) fn has_flag(&self, flag: u32) -> bool {
    return (self.flags & flag) > 0;
  }

  pub(crate) fn has_feature(&self, features: u32) -> bool {
    return (self.flags & features) > 0;
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

