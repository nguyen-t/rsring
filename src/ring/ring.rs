use std::io::Error;
use std::ptr;
use std::mem::size_of;
use std::ffi::c_void;
use libc::{sigset_t, close, EINVAL};

use crate::io_uring::{self, *};
use crate::util::Map;
use crate::squeue::SQueue;
use crate::cqueue::CQueue;

#[derive(Debug)]
pub struct Ring<T: Sized, U: Sized> {
  pub ring:       Map<c_void>,
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
    let mut p = io_uring::params::new(Ring::<T, U>::init_flags()?);
    let fd = match io_uring::setup(depth, ptr::addr_of_mut!(p)) {
      Ok(fd) => fd,
      Err(e) => return Err(e)
    };
    let sq_size = p.sq_off.array as usize + p.sq_entries as usize * size_of::<u32>();
    let cq_size = p.cq_off.cqes as usize + p.cq_entries as usize * size_of::<io_uring::cqe<T>>();
    let size = core::cmp::max(sq_size, cq_size);
    let ring = Map::new(fd, size, IORING_OFF_SQ_RING)?;
    let sqes = Map::new(fd, size, io_uring::IORING_OFF_SQES)?;
    let sq = unsafe { SQueue::<T>::new(ring.raw(), &p, sqes) };
    let cq = unsafe { CQueue::<U>::new(ring.raw(), &p) };

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

  pub fn submit_wait(&mut self, min_complete: u32) -> Result<i32, Error> {
    let to_submit = self.sq.flush();
    let submit = to_submit != 0;
    let sqpoll = (self.flags & IORING_SETUP_SQPOLL) > 0;
    let iopoll = (self.flags & IORING_SETUP_IOPOLL) > 0;
    let wakeup = self.sq.needs_wakeup();
    let flush = self.cq.needs_flush();
    let sq_enter = (submit && !sqpoll) || (submit && wakeup);
    let cq_enter = iopoll || flush;

    if min_complete > 0 || sq_enter || cq_enter {
      let register = false;
      let flags = if register { IORING_ENTER_REGISTERED_RING } else { 0 }
        | if sq_enter { IORING_SQ_NEED_WAKEUP } else { 0 }
        | if cq_enter { IORING_ENTER_GETEVENTS } else { 0 };

      return io_uring::enter(self.enter_fd, to_submit, min_complete, flags, ptr::null_mut::<sigset_t>());
    }

    return Ok(to_submit as i32);
  }

  pub fn submit(&mut self) -> Result<i32, Error> {
    return self.submit_wait(0);
  }

  pub fn next(&mut self) {
    self.cq.advance(1);
  }

  pub(crate) fn init_flags() -> Result<u32, Error> {
    let sqe_setup = match size_of::<sqe<T>>() {
      64  => 0,
      128 => IORING_SETUP_SQE128,
      _   => return Err(Error::from_raw_os_error(EINVAL)),
    };
    let cqe_setup = match size_of::<cqe<U>>() {
      16 => 0,
      32 => IORING_SETUP_CQE32,
      _  => return Err(Error::from_raw_os_error(EINVAL)),
    };

    return Ok(
        IORING_SETUP_SQPOLL
      | IORING_SETUP_SUBMIT_ALL
      | sqe_setup 
      | cqe_setup
    );
  }
}

impl<T: Sized, U: Sized> Drop for Ring<T, U> {
  fn drop(&mut self) {
    unsafe { 
      close(self.ring_fd);
      close(self.enter_fd);
    };
  }
}

