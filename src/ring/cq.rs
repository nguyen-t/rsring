use std::io::{Error};
use std::mem::size_of;
use std::sync::atomic::Ordering;
use std::ffi::c_void;

use libc::{sigset_t, EAGAIN, ETIME};
use crate::ring::{Ring, RING_TIMEOUT};
use crate::io_uring::{self, *};

impl<T: Sized, U: Sized> Ring<T, U> {
  pub(crate) fn cqe_advance(&mut self, nr: u32) {
    unsafe {
      (*self.cq.khead).store((*self.cq.khead).load(Ordering::Acquire) + nr, Ordering::Release);
    };
  }

  pub(crate) fn cqe_peek(&mut self) -> Result<(Option<*mut io_uring::cqe<U>>, u32), Error> {
    let mask = self.cq.ring_mask;
    let shift = self.has_flag(IORING_SETUP_CQE32) as u32;

    loop {
      let tail = unsafe { (*self.cq.ktail).load(Ordering::Acquire) };
      let head = unsafe { (*self.cq.khead).load(Ordering::Acquire) };
      let available = tail - head;

      if available == 0 {
        return Ok((None, 0));
      }

      let index = (head & mask) << shift;
      let cqe = unsafe { self.cq.cqes.add(index as usize) };
      let ext_arg = self.has_feature(IORING_FEAT_EXT_ARG);
      let timeout = unsafe { (*cqe).user_data == RING_TIMEOUT };

      if !ext_arg && timeout {
        let res = unsafe { (*cqe).res };
       
        self.cqe_advance(1);

        if res >= 0 {
          continue;
        }
        
        return Err(Error::from_raw_os_error(res));
      }

      return Ok((Some(cqe), available));
    };
  }

  pub(crate) fn cqe_get(&mut self, to_submit: u32, wait_nr: u32, get_flags: u32, mask: *const sigset_t, ms: u64) -> Result<Option<*mut io_uring::cqe<U>>, Error> {
    let mut looped = false;
    let mut error = 0;
    let mut submit = to_submit;
    let ts = __kernel_timespec::from_ms(ms as i64);
    let arg = io_uring::getevents_arg::new(mask, &ts);
    let ptr = match (get_flags & IORING_ENTER_EXT_ARG) > 0 { 
      true  => &arg as *const io_uring::getevents_arg as *const c_void,
      false => mask as *const c_void,
    };
    let size = match (get_flags & IORING_ENTER_EXT_ARG) > 0 {
      true  => size_of::<io_uring::getevents_arg>(),
      false => arg.sigmask_sz as usize,
    };

    loop {
      let mut need_enter = false;
      let mut flags = 0;
      let (cqe, available) = match self.cqe_peek() {
        Ok((cqe, available)) => (cqe, available),
        Err(err) => return Err(if error == 0 { err } else { Error::from_raw_os_error(error) }),
      };

      if cqe.is_none() && wait_nr == 0 && submit == 0 {
        let iopoll = self.has_flag(IORING_SETUP_IOPOLL);
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
      if wait_nr > available || need_enter {
        flags |= IORING_ENTER_GETEVENTS | get_flags;
        need_enter = true;
      }

      let sqpoll = self.has_flag(IORING_SETUP_SQPOLL);
      let wakeup = self.sq.needs_wakeup();
      let sq_enter = ((submit != 0) && !sqpoll) || ((submit != 0) && wakeup);

      flags |= if wakeup { IORING_ENTER_SQ_WAKEUP } else { 0 };

      if sq_enter {
        need_enter = true;
      }
      if !need_enter {
        return Ok(cqe);
      }
      if looped && ms > 0 {
        if cqe.is_none() && error == 0 {
          return Err(Error::from_raw_os_error(ETIME));
        }

        return Err(Error::from_raw_os_error(error));
      }
      if false {
        flags |= IORING_ENTER_REGISTERED_RING;
      }

      let ret = match io_uring::enter2(self.enter_fd, submit, wait_nr, flags, ptr, size) {
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
    let result = self.cqe_peek();

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