# RSRing
A Rust-based io_uring library. <br>
Inspired by liburing (https://github.com/axboe/liburing) <br>

## Example TCP Server
```rust
use std::{net::TcpListener, os::fd::AsRawFd, mem::MaybeUninit, ffi::c_void};
use libc::{sockaddr, sockaddr_in, socklen_t};
use rsring::RSRing; 

fn main() {
  let msg = "Hello, RSRing!";
  let tcp = TcpListener::bind("127.0.0.1:3000").unwrap();
  let mut ring = RSRing::new(32).unwrap();
  let mut addr = MaybeUninit::<sockaddr_in>::uninit();
  let mut addr_len = MaybeUninit::<socklen_t>::uninit();
  
  ring.accept4(tcp.as_raw_fd(), addr.as_mut_ptr() as *mut sockaddr, addr_len.as_mut_ptr(), 0).unwrap()
    .set_data_u64(0);

  loop {
    let cqe = ring.submit_wait(1, 0).unwrap();
    let fd = cqe.res;

    if cqe.get_data_u64() == 0 {
      ring.accept4(tcp.as_raw_fd(), addr.as_mut_ptr() as *mut sockaddr, addr_len.as_mut_ptr(), 0).unwrap()
        .set_data_u64(0);
      ring.write(fd, msg.as_ptr() as *const c_void, msg.len()).unwrap()
        .set_data_u64(1)
        .link(false);
      ring.close(fd).unwrap()
        .set_data_u64(2);
    }

    ring.next();
  }
}
```