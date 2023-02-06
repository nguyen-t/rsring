#![allow(dead_code)]
mod io_uring;
mod constants;

pub use {
  io_uring::*,
  constants::*,
};

#[cfg(test)]
mod tests {
  use std::mem::size_of;
  use crate::io_uring::*;

  #[test]
  fn entry_size() {
    assert_eq!(size_of::<sqe<[u64; 2]>>(), 64);
    assert_eq!(size_of::<sqe<[u8; 80]>>(), 128);
    assert_eq!(size_of::<cqe<[u8; 0]>>(), 16);
    assert_eq!(size_of::<cqe<[u8; 16]>>(), 32);
  }

  #[test]
  fn offsets_size() {
    assert_eq!(size_of::<sqring_offsets>(), 40);
    assert_eq!(size_of::<cqring_offsets>(), 40);
  }

  #[test]
  fn params_size() {
    assert_eq!(size_of::<params>(), 120);
  }

  #[test]
  fn rsrc_size() {
    assert_eq!(size_of::<rsrc_register>(), 32);
    assert_eq!(size_of::<rsrc_update>(), 16);
    assert_eq!(size_of::<rsrc_update2>(), 32);
  }

  #[test]
  fn notification_size() {
    assert_eq!(size_of::<notification_slot>(), 32);
    assert_eq!(size_of::<notification_register>(), 32);
  }

  #[test]
  fn probe_size() {
    assert_eq!(size_of::<probe_op>(), 8);
    assert_eq!(size_of::<probe<[probe_op; 0]>>(), 16);
  }

  #[test]
  fn restriction_size() {
    assert_eq!(size_of::<restriction>(), 16);
  }

  #[test]
  fn buf_size() {
    assert_eq!(size_of::<buf>(), 16);
    assert_eq!(size_of::<buf_ring<[buf; 1]>>(), 16);
    assert_eq!(size_of::<buf_reg>(), 40);
  }

  #[test]
  fn events_size() {
    assert_eq!(size_of::<getevents_args>(), 24);
  }

  #[test]
  fn sync_cancel_size() {
    assert_eq!(size_of::<sync_cancel_reg>(), 64);
  }

  #[test]
  fn file_index_range_size() {
    assert_eq!(size_of::<file_index_range>(), 16);
  }

  #[test]
  fn recvmsg_test() {
    assert_eq!(size_of::<recvmsg_out>(), 16);
  }
}