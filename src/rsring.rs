use std::io::Error;
use std::ptr;
use crate::squeue::SQueue;
use crate::cqueue::CQueue;
use crate::io_uring;

#[derive(Debug, Clone)]
pub struct RSRing {
  pub params: io_uring::params,
  pub fd:     i32,
  pub sq:     SQueue,
  pub cq:     CQueue,
}

impl RSRing {
  pub fn new(depth: u32) -> Result<RSRing, Error> {
    let mut params = Default::default();
    let fd = match io_uring::setup(depth, ptr::addr_of_mut!(params)) {
      Ok(fd) => fd,
      Err(e) => return Err(e)
    };
    let sq = SQueue::new(fd, &params);
    let cq = CQueue::new(fd, &params);

    return Ok(RSRing {
      params: params,
      fd: fd,
      sq: sq,
      cq: cq,
    });
  }

  pub fn submit(&mut self) {

  }
}

