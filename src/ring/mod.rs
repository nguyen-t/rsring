mod ring;
mod utility;
mod syscalls;

pub use {
  ring::Ring,
};

#[cfg(test)]
mod ring_tests {
  use crate::ring::Ring;
  use crate::io_uring::*;

  #[test]
  fn init_test() -> Result<(), String> {
    let ring = match Ring::<[u64; 2], [u8; 0]>::new(32) {
      Ok(ring) => ring,
      Err(err) => return Err(err.to_string())
    };

    println!("IORING_FEAT_SINGLE_MMAP     {}", ring.features & IORING_FEAT_SINGLE_MMAP     > 0);
    println!("IORING_FEAT_NODROP          {}", ring.features & IORING_FEAT_NODROP          > 0);
    println!("IORING_FEAT_SUBMIT_STABLE   {}", ring.features & IORING_FEAT_SUBMIT_STABLE   > 0);
    println!("IORING_FEAT_RW_CUR_POS      {}", ring.features & IORING_FEAT_RW_CUR_POS      > 0);
    println!("IORING_FEAT_CUR_PERSONALITY {}", ring.features & IORING_FEAT_CUR_PERSONALITY > 0);
    println!("IORING_FEAT_FAST_POLL       {}", ring.features & IORING_FEAT_FAST_POLL       > 0);
    println!("IORING_FEAT_POLL_32BITS     {}", ring.features & IORING_FEAT_POLL_32BITS     > 0);
    println!("IORING_FEAT_SQPOLL_NONFIXED {}", ring.features & IORING_FEAT_SQPOLL_NONFIXED > 0);
    println!("IORING_FEAT_EXT_ARG         {}", ring.features & IORING_FEAT_EXT_ARG         > 0);
    println!("IORING_FEAT_NATIVE_WORKERS  {}", ring.features & IORING_FEAT_NATIVE_WORKERS  > 0);
    println!("IORING_FEAT_RSRC_TAGS       {}", ring.features & IORING_FEAT_RSRC_TAGS       > 0);
    println!("IORING_FEAT_CQE_SKIP        {}", ring.features & IORING_FEAT_CQE_SKIP        > 0);
    println!("IORING_FEAT_LINKED_FILE     {}", ring.features & IORING_FEAT_LINKED_FILE     > 0);

    Ok(())
  }

  // #[test]
  // fn depth_size_test() {
  //   let ring = Ring::<[u64; 2], [u8; 0]>::new(32).unwrap();
  //   assert_eq!(ring.ring.len(), 388);
  
  //   let ring = Ring::<[u64; 2], [u8; 0]>::new(128).unwrap();
  //   assert_eq!(ring.ring.len(), 4928);

  //   let ring = Ring::<[u64; 2], [u8; 0]>::new(4096).unwrap();
  //   assert_eq!(ring.ring.len(), 147776);
  // }
}