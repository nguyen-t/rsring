use std::io::Error;
use std::ptr;
use std::mem::size_of;
use std::ffi::c_void;
use libc::{sigset_t, EAGAIN};

use crate::io_uring::{self, *};
use crate::ring::Ring;

impl<T: Sized, U: Sized> Ring<T, U> {
  pub fn next(&mut self) {
    self.cq.advance(1);
  }

  pub fn wait(&mut self) -> Result<&mut io_uring::cqe<U>, Error> {
    loop {
      if let Some(cqe) = self.cq.next() {
        return Ok(unsafe { cqe.as_mut().unwrap() });
      };
      match self.ready(0, 1, ptr::null::<sigset_t>(), 0) {
        Ok(_) => (),
        Err(err) => return Err(err),
      };
    }
  }

  pub fn submit(&mut self) -> Result<i32, Error> {
    let to_submit = self.sq.remaining();

    self.sq.update();

    return self.ready(to_submit, 0, ptr::null_mut::<sigset_t>(), 0);
  }

  pub fn submit_wait(&mut self) -> Result<&mut io_uring::cqe<U>, Error> {
    let to_submit = self.sq.remaining();

    self.sq.update();
    self.ready(to_submit, 1, ptr::null::<sigset_t>(), 0)?;

    return match self.cq.next() {
      Some(cqe) => Ok(unsafe { cqe.as_mut().unwrap() }),
      None => Err(Error::last_os_error()),
    };
  }

  pub fn ready(&mut self, to_submit: u32, min_complete: u32, sig: *const sigset_t, timeout: u32) -> Result<i32, Error> {
    let sqpoll = (self.flags & IORING_SETUP_SQPOLL) > 0;
    let iopoll = (self.flags & IORING_SETUP_IOPOLL) > 0;
    let wakeup = self.sq.needs_wakeup();
    let flush = self.cq.needs_flush();
    let sq_enter = (wakeup || !sqpoll) && (to_submit > 0);
    let cq_enter = iopoll || flush;

    if to_submit == 0 && min_complete == 0 && !sq_enter && !cq_enter {
      return Err(Error::from_raw_os_error(EAGAIN));
    }
    if min_complete > 0 || sq_enter || cq_enter {
      let mut flags = 0;
      let available = self.cq.available();

      if min_complete > available {
        flags |= IORING_ENTER_GETEVENTS
      }
      if wakeup {
        flags |= IORING_ENTER_SQ_WAKEUP;
      }
      if (self.features & IORING_FEAT_EXT_ARG) > 0 && timeout > 0 {
        let ts = __kernel_timespec::from_ms(timeout as i64);
        let arg = io_uring::getevents_arg::new(sig, &ts);
        let ptr = &arg as *const io_uring::getevents_arg as *const c_void;
        let size = size_of::<io_uring::getevents_arg>();
  
        flags |= IORING_ENTER_EXT_ARG;
  
        return io_uring::enter2(self.enter_fd, 0, min_complete, flags, ptr, size);
      }

      return io_uring::enter(self.enter_fd, to_submit, min_complete, flags, sig);
    }

    return Ok(to_submit as i32);
  }
}