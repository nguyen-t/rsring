use std::io::Error;
use std::ptr;
use std::mem::size_of;
use libc::{c_void, sigset_t, munmap, memset, close};
use std::sync::atomic::Ordering;

use crate::io_uring;
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
  pub registered: bool,
  pub sq:         SQueue<T>,
  pub cq:         CQueue<U>,
}

impl<T: Sized, U: Sized> Ring<T, U> {
  pub fn new(flags: u32, depth: u32) -> Result<Ring<T, U>, Error> {
    let mut p = io_uring::params::new(flags);
    let fd = match io_uring::setup(depth, ptr::addr_of_mut!(p)) {
      Ok(fd) => fd,
      Err(e) => return Err(e)
    };
    let sq_size = p.sq_off.array as usize + p.sq_entries as usize * size_of::<u32>();
    let cq_size = p.cq_off.cqes as usize + p.cq_entries as usize * size_of::<io_uring::cqe<T>>();
    let size = core::cmp::max(sq_size, cq_size);
    let ring = memmap(fd, size, io_uring::IORING_OFF_SQ_RING);

    return Ok(Ring {
      ring: ring,
      size: size,
      ring_fd: fd,
      enter_fd: fd,
      flags: flags,
      registered: false,
      sq: unsafe { SQueue::<T>::new(ring, &p, fd) },
      cq: unsafe { CQueue::<U>::new(ring, &p) },
    });
  }

  pub fn submit(&mut self, submitted: u32, wait_nr: u32, getevents: bool) -> Result<i32, Error> {
    let cq_needs_enter = getevents || wait_nr > 0 || self.cq.needs_enter(self.flags);
    let mut flags: u32 = 0;
    
    if self.sq.needs_enter(submitted, self.flags, &mut flags) || cq_needs_enter {
      flags |= if cq_needs_enter { io_uring::IORING_ENTER_GET_EVENTS } else { 0 };
      flags |= if self.registered { io_uring::IORING_ENTER_REGISTERED_RING } else { 0 };

      return io_uring::enter(self.enter_fd, submitted, wait_nr, flags, ptr::null_mut::<sigset_t>());
    }

    return Ok(submitted as i32);
  }

  pub(crate) fn prep(&mut self, op: u32, fd: i32, addr: *const c_void, len: u32, offset: u64, flags: u32) -> Option<*mut io_uring::sqe<T>> {
    let next = self.sq.sqe_tail + 1;
    let shift = ((self.flags & io_uring::IORING_SETUP_SQE128) > 0) as u32;
    let index = ((self.sq.sqe_tail & self.sq.ring_mask) << shift) as usize;
    let sqpoll = (self.flags & io_uring::IORING_SETUP_SQPOLL) > 0;
    let head = self.sq.get_khead(if sqpoll { Ordering::Acquire } else { Ordering::Relaxed });
    let sqe = unsafe { self.sq.sqes.add(index) };

    if (next - head) > self.sq.ring_entries {
      return None;
    }
    
    unsafe {
      memset(sqe as *mut c_void, 0, size_of::<io_uring::sqe<T>>());
      (*sqe).opcode    = op as u8;
      (*sqe).fd        = fd;
      (*sqe).addr2     = offset;
      (*sqe).addr1     = addr as u64;
      (*sqe).len       = len;
      (*sqe).op_flags  = flags;
      self.sq.sqe_tail = next;
    }

    return Some(sqe);
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

