# RSRing
A low level Rust io_uring library. <br>
Inspired by liburing (https://github.com/axboe/liburing) <br>

## TODO
 - Resolve max ring size issue
 - Add more comprehensive tests
 - Add more syscalls
 - Add registered buffers
 - Async support?

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
  
  ring.accept(tcp.as_raw_fd(), addr.as_mut_ptr() as *mut sockaddr, addr_len.as_mut_ptr()).unwrap()
    .set_data_u64(0);

  loop {
    let cqe = ring.submit_wait().unwrap();
    let fd = cqe.res;

    if cqe.get_data_u64() == 0 {
      ring.accept(tcp.as_raw_fd(), addr.as_mut_ptr() as *mut sockaddr, addr_len.as_mut_ptr()).unwrap()
        .set_data_u64(0);
      ring.write(fd, msg.as_ptr() as *const c_void, msg.len()).unwrap()
        .set_data_u64(1)
        .link();
      ring.close(fd).unwrap()
        .set_data_u64(2);
    }

    ring.next();
  }
}
```