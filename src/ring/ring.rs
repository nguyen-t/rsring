use std::io::Error;
use std::mem::size_of;
use std::ffi::c_void;
use libc::{close, EINVAL};

use crate::io_uring::{self, *};
use crate::util::Map;
use crate::squeue::SQueue;
use crate::cqueue::CQueue;

#[derive(Debug)]
pub struct Ring<T: Sized, U: Sized> {
  pub(crate) ring:       Map<c_void>,
  pub(crate) ring_fd:    i32,
  pub(crate) enter_fd:   i32,
  pub(crate) flags:      u32,
  pub(crate) features:   u32,
  pub(crate) sq:         SQueue<T>,
  pub(crate) cq:         CQueue<U>,
}

impl<T: Sized, U: Sized> Ring<T, U> {
  pub fn new(entries: u32) -> Result<Ring<T, U>, Error> {
    let mut p = io_uring::params::new(Ring::<T, U>::init_flags()?);
    let fd = match io_uring::setup(entries, &mut p) {
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
      ring_fd: fd,
      enter_fd: fd,
      flags: p.flags,
      features: p.features,
      sq: sq,
      cq: cq,
    });
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

