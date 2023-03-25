/* TODO: Rewrite once anonymous unions and structs are in Rust-Nightly build */

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct __kernel_timespec {
  pub tv_sec:  i64,
  pub tv_nsec: i64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct sqe<T: Sized> {
  pub opcode:      u8,
  pub flags:       u8,
  pub ioprio:      u16,
  pub fd:          i32,
  pub addr2:       u64,
  pub addr1:       u64,
  pub len:         u32,
  pub op_flags:    u32,
  pub user_data:   u64,
  pub buf_select:  u16,
  pub personality: u16,
  pub file_select: u32,
  pub addr3:        T,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct cqe<T: Sized> {
  pub user_data: u64,
  pub res:       i32,
  pub flags:     u32,
  pub big_cqe:   T,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct sqring_offsets {
  pub head:         u32,
  pub tail:         u32,
  pub ring_mask:    u32,
  pub ring_entries: u32,
  pub flags:        u32,
  pub dropped:      u32,
  pub array:        u32,
  pub resv1:        u32,
  pub resv2:        u64,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub struct cqring_offsets {
  pub head:         u32,
  pub tail:         u32,
  pub ring_mask:    u32,
  pub ring_entries: u32,
  pub overflow:     u32,
  pub cqes:         u32,
  pub flags:        u32,
  pub resv1:        u32,
  pub resv2:        u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct params {
  pub sq_entries:     u32,
  pub cq_entries:     u32,
  pub flags:          u32,
  pub sq_thread_cpu:  u32,
  pub sq_thread_idle: u32,
  pub features:       u32,
  pub wd_fd:          u32,
  pub resv:           [u32; 3],
  pub sq_off:         sqring_offsets,
  pub cq_off:         cqring_offsets,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct rsrc_register {
  pub rn:    u32,
  pub flags: u32,
  pub resv:  u64,
  pub data:  u64,
  pub tags:  u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct rsrc_update {
  pub offset: u32,
  pub resv:   u32,
  pub data:   u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct rsrc_update2 {
  pub offset: u32,
  pub resv1:  u32,
  pub data:   u64,
  pub tags:   u64,
  pub nr:     u32,
  pub resv2:  u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct notification_slot {
  pub tag:  u64,
  pub resv: [u64; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct notification_register {
  pub nr_slots: u32,
  pub resv1:    u32,
  pub resv2:    u64,
  pub data:     u64,
  pub resv3:    u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct probe_op {
  pub op:    u8,
  pub resv1: u8,
  pub flags: u16,
  pub resv2: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct probe<const T: usize> {
  pub last_op: u8,
  pub ops_len: u8,
  pub resv1:   u16,
  pub resv2:   [u32; 3],
  pub ops:     [probe_op; T],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct restriction {
  pub opcode: u16,
  pub flags:  u8,
  pub resv1:  u8,
  pub resv2:  [u32; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct buf {
  pub addr: u64,
  pub len:  u32,
  pub bid:  u16,
  pub resv: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct buf_ring<const T: usize> {
  pub bufs: [buf; T],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct buf_reg {
  pub ring_addr:    u64,
  pub ring_entires: u32,
  pub bgid:         u16,
  pub pad:          u16,
  pub resv:         [u64; 3],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct getevents_arg {
  pub sigmask:    u64,
  pub sigmask_sz: u32,
  pub pad:        u32,
  pub ts:         u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct sync_cancel_reg {
  pub addr: u64,
  pub fd: i32,
  pub flags: u32,
  pub timeout: __kernel_timespec,
  pub pad: [u64; 4],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct file_index_range {
  pub off:  u32,
  pub len:  u32,
  pub resv: u64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct recvmsg_out {
  pub namelen:    u32,
  pub controllen: u32,
  pub payloadlen: u32,
  pub flags:      u32,
}