#![feature(c_size_t)]
#![allow(dead_code)]
mod io_uring;
mod util;
mod squeue;
mod cqueue;
mod ring;

pub type RSRing  = ring::Ring<[u64; 2], [u8; 0]>;
// pub type RSRing = ring::Ring<[u8; 80], [u8; 16]>;