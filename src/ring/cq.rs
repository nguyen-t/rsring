use std::io::{Error};
use std::mem::size_of;
use std::ffi::c_void;

use libc::{sigset_t, EAGAIN, ETIME};
use crate::ring::{Ring};
use crate::io_uring::{self, *};

/* TODO: Rewrite */

fn submitter(fd: i32, to_submit: u32, min_complete: u32, flags: u32, timeout: u64, sig: *const sigset_t) -> Result<i32, Error> {
  let ts = __kernel_timespec::from_ms(timeout as i64);
  let arg = io_uring::getevents_arg::new(sig, &ts);
  let ptr = match (flags & IORING_ENTER_EXT_ARG) > 0 { 
    true  => &arg as *const io_uring::getevents_arg as *const c_void,
    false => sig as *const c_void,
  };
  let size = match (flags & IORING_ENTER_EXT_ARG) > 0 {
    true  => size_of::<io_uring::getevents_arg>(),
    false => arg.sigmask_sz as usize,
  };

  return io_uring::enter2(fd, to_submit, min_complete, flags, ptr, size);
}

impl<T: Sized, U: Sized> Ring<T, U> {
  pub(crate) fn cqe_get(&mut self, to_submit: u32, min_complete: u32, get_flags: u32, sig: *const sigset_t, timeout: u64) -> Result<Option<*mut io_uring::cqe<U>>, Error> {
    let mut looped = false;
    let mut error = 0;
    let mut submit = to_submit;

    loop {
      let mut need_enter = false;
      let mut flags = 0;
      let (cqe, available) = match self.cq.peek() {
        Ok((cqe, available)) => (cqe, available),
        Err(err) => return Err(if error == 0 { err } else { Error::from_raw_os_error(error) }),
      };

      if cqe.is_none() && min_complete == 0 && submit == 0 {
        let iopoll = (self.flags & IORING_SETUP_IOPOLL) > 0;
        let flush = self.cq.needs_flush();
        let cq_enter = iopoll || flush;

        if looped || !cq_enter {
          if error == 0 {
            return Err(Error::from_raw_os_error(EAGAIN));
          }
          
          return Err(Error::from_raw_os_error(error));
        }

        need_enter = true;
      }
      if min_complete > available || need_enter {
        flags |= IORING_ENTER_GETEVENTS | get_flags;
        need_enter = true;
      }

      let sqpoll = (self.flags & IORING_SETUP_SQPOLL) > 0;
      let wakeup = self.sq.needs_wakeup();
      let sq_enter = ((submit != 0) && !sqpoll) || ((submit != 0) && wakeup);

      flags |= if wakeup { IORING_ENTER_SQ_WAKEUP } else { 0 };

      if sq_enter {
        need_enter = true;
      }
      if !need_enter {
        return Ok(cqe);
      }
      if looped && timeout > 0 {
        if cqe.is_none() && error == 0 {
          return Err(Error::from_raw_os_error(ETIME));
        }

        return Err(Error::from_raw_os_error(error));
      }
      if false {
        flags |= IORING_ENTER_REGISTERED_RING;
      }

      let ret = match submitter(self.enter_fd, submit, min_complete, flags, timeout, sig) {
        Ok(ret) => ret,
        Err(err) => return Err(if error == 0 { Error::from_raw_os_error(error) } else { err }),
      };

      submit -= ret as u32;

      if cqe.is_some() {
        return Ok(cqe);
      }
      if !looped {
        looped = true;
        error = ret;
      }
    }
  }

  pub fn wait_cqe(&mut self) -> Result<*mut io_uring::cqe<U>, Error> {
    let result = self.cq.peek();

    if result.is_ok() {
      if let Some(cqe) = result.unwrap().0 {
        return Ok(cqe);
      }
    }

    return match self.cqe_get(0, 1, 0, std::ptr::null::<sigset_t>(), 0) {
      Ok(cqe) => match cqe {
        Some(cqe) => Ok(cqe),
        None => Err(Error::last_os_error()),
      },
      Err(err) => Err(err),
    };
  }
}                                    