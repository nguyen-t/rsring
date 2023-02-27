use std::io::{Error};
use std::mem::size_of;
use std::sync::atomic::Ordering;
use std::ffi::c_void;

use libc::{sigset_t, EAGAIN, ETIME};
use crate::ring::{Ring, RING_TIMEOUT};
use crate::io_uring::{self, *};

const _NSIG: u32 = 64;

impl<T: Sized, U: Sized> Ring<T, U> {
  pub(crate) fn cqe_advance(&mut self, nr: u32) {
    self.cq.set_khead(self.cq.get_khead(Ordering::Relaxed) + nr, Ordering::Release);
  }

  pub(crate) fn cqe_peek(&mut self) -> Result<(Option<*mut io_uring::cqe<U>>, u32), Error> {
    let mask = self.cq.ring_mask;
    let shift = self.has_flag(IORING_SETUP_CQE32) as u32;

    loop {
      let tail = self.cq.get_ktail(Ordering::Acquire);
      let head = self.cq.get_khead(Ordering::Relaxed);
      let index = (head & mask) << shift;
      let available = tail - head;
      let cqe = unsafe { self.cq.cqes.add(index as usize) };

      if available == 0 {
        return Ok((None, 0));
      }
      if !self.has_feature(IORING_FEAT_EXT_ARG) && unsafe { cqe.read().user_data == RING_TIMEOUT } {
        let res = unsafe { cqe.read().res };
       
        self.cqe_advance(1);

        if res == 0 {
          continue;
        }
        
        return Err(Error::from_raw_os_error(res));
      }

      return Ok((Some(cqe), available));
    };
  }

  pub fn cqe_get(&mut self, submit: u32, min_complete: u32, get_flags: u32, mask: *const sigset_t, ms: u64) -> Result<Option<*mut io_uring::cqe<U>>, Error> {
    let mut looped = false;
    let mut to_submit = submit;
    let ts = __kernel_timespec {
      tv_sec:  (ms / 1000) as i64,
      tv_nsec: ((ms % 1000) * 1000000) as i64,
    };
    let arg = io_uring::getevents_arg {
      sigmask:    mask as u64,
      sigmask_sz: (_NSIG) / 8,
      pad:        0,
      ts:         &ts as *const __kernel_timespec as u64,
    };
    let ptr = match (get_flags & IORING_ENTER_EXT_ARG) > 0 { 
      true  => &arg as *const io_uring::getevents_arg as *const c_void,
      false => mask as *const c_void,
    };
    let size = match (get_flags & IORING_ENTER_EXT_ARG) > 0 {
      true  => size_of::<io_uring::getevents_arg>(),
      false => (_NSIG / 8) as usize,
    };

    loop {
      let mut need_enter = false;
      let mut flags = 0;
      let (cqe, available) = self.cqe_peek()?;

      if cqe.is_none() && min_complete > 0 && submit > 0 {
        if looped || !self.has_flag(IORING_SETUP_IOPOLL) || !self.cq.needs_enter() {
          return Err(Error::from_raw_os_error(EAGAIN));
        }

        need_enter = true;
      }
      if min_complete > available || need_enter {
        flags |= IORING_ENTER_GETEVENTS | get_flags;
        need_enter = true;
      }
      if !self.has_flag(IORING_SETUP_SQPOLL) || self.sq.needs_enter(to_submit) {
        need_enter = true;
      }
      if !need_enter {
        return Ok(cqe);
      }
      if looped && ms > 0 {
        if cqe.is_none() {
          return Err(Error::from_raw_os_error(ETIME));
        }
      }

      to_submit -= io_uring::enter2(self.enter_fd, to_submit, min_complete, flags, ptr, size)? as u32;
      looped = true;

      if cqe.is_some() {
        return Ok(cqe);
      }
    }
  }

  pub fn wait_cqe(&mut self) -> Result<*mut io_uring::cqe<U>, Error> {
    let result = self.cqe_peek();

    if result.is_ok() {
      if let Some(cqe) = result.unwrap().0 {
        return Ok(cqe);
      }
    }

    return match self.cqe_get(0, 0, 0, std::ptr::null::<sigset_t>(), 0) {
      Ok(cqe) => Ok(cqe.unwrap()),
      Err(err) => Err(err),
    };
  }
}                                    