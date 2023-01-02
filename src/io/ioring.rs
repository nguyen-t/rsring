#[allow(non_camel_case_types)]
pub enum SQE_FLAGS {
  FIXED_FILE  = 1,
  IO_DRAIN    = 2,
  IO_LINK     = 4,
  IO_HARDLINK = 8,
  ASYNC,
  BUFFER_SELECT,
  SKIP_SUCCESS,
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum SETUP {
  IOPOLL        = (1 << 0),
  SQPOLL        = (1 << 1),
  SQ_AFF        = (1 << 2),
  CQSIZE        = (1 << 3),
  CLAMP         = (1 << 4),
  ATTACH_WQ     = (1 << 5),
  R_DISABLED    = (1 << 6),
  SUBMIT_ALL    = (1 << 7),
  COOP_TASKRUN  = (1 << 8),
  TASKRUN_FLAG  = (1 << 9),
  SQE128        = (1 << 10),
  CQE32         = (1 << 11),
  SINGLE_ISSUER = (1 << 12),
  DEFER_TASKRUN = (1 << 13),
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum OP {
	NOP             = 0,
	READV           = 1,
	WRITEV          = 2,
	FSYNC           = 3,
	READ_FIXED      = 4,
	WRITE_FIXED     = 5,
	POLL_ADD        = 6,
	POLL_REMOVE     = 7,
	SYNC_FILE_RANGE = 8,
	SENDMSG         = 9,
	RECVMSG         = 10,
	TIMEOUT         = 11,
	TIMEOUT_REMOVE  = 12,
	ACCEPT          = 13,
	ASYNC_CANCEL    = 14,
	LINK_TIMEOUT    = 15,
	CONNECT         = 16,
	FALLOCATE       = 17,
	OPENAT          = 18,
	CLOSE           = 19,
	FILES_UPDATE    = 20,
	STATX           = 21,
	READ            = 22,
	WRITE           = 23,
	FADVISE         = 24,
	MADVISE         = 25,
	SEND            = 26,
	RECV            = 27,
	OPENAT2         = 28,
	EPOLL_CTL       = 29,
	SPLICE          = 30,
	PROVIDE_BUFFERS = 31,
	REMOVE_BUFFERS  = 32,
	TEE             = 33,
	SHUTDOWN        = 34,
	RENAMEAT        = 35,
	UNLINKAT        = 36,
	MKDIRAT         = 37,
	SYMLINKAT       = 38,
	LINKAT          = 39,
	MSG_RING        = 40,
	FSETXATTR       = 41,
	SETXATTR        = 42,
	FGETXATTR       = 43,
	GETXATTR        = 44,
	SOCKET          = 45,
	URING_CMD       = 46,
	SEND_ZC         = 47,
	SENDMSG_ZC      = 48,
	LAST            = 49,
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum CMD {
  FIXED = (1 << 0),
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum FSYNC {
  DATASYNC = (1 << 0),
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum TIMEOUT {
  ABS                 = (1 << 0),
  UPDATE              = (1 << 1),
  BOOTTIME            = (1 << 2),
  REALTIME            = (1 << 3),
  LINK_TIMEOUT_UPDATE = (1 << 4),
  ETIME_SUCCESS       = (1 << 5),
  CLOCK_MASK          = (TIMEOUT::BOOTTIME as u32 | TIMEOUT::REALTIME as u32),
  UPDATE_MASK         = (TIMEOUT::UPDATE as u32 | TIMEOUT::LINK_TIMEOUT_UPDATE as u32),
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum SPLICE {
  F_FD_IN_FIXED = (1 << 31),
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum POLL {
  ADD_MULTI        = (1 << 0),
  UPDATE_EVENTS    = (1 << 1),
  UPDATE_USER_DATA = (1 << 2),
  ADD_LEVEL        = (1 << 3),
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum ASYNC_CANCEL {
  ALL   = (1 << 0),
  FD    = (1 << 1),
  ANY   = (1 << 2),
  FIXED = (1 << 3),
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum IOPRIO {
  RECVSEND_POLL_FIRST  = (1 << 0),
  RECV_MULTISHOT       = (1 << 1),
  RECVSEND_FIXED_BUF   = (1 << 2),
  SEND_ZC_REPORT_USAGE = (1 << 3),
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum NOTIF {
  USAGE_ZC_COPIED = (1 << 31),
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum ACCEPT {
  MULTISHOT = (1 << 0),
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum MSG {
  DATA,
  SEND_FD,
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum MSG_RING {
  CQE_SKIP = (1 << 0),
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum CQE_F {
  BUFFER       = (1 << 0),
  MORE         = (1 << 1),
  SOCK_NOEMPTY = (1 << 2),
  NOTIF        = (1 << 3),
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum CQE {
  BUFFER_SHIFT = 16,
}

#[allow(non_camel_case_types)]
#[repr(u64)]
pub enum OFF {
  SQ_RING = 0x00000000,
  CQ_RING = 0x08000000,
  SQES    = 0x10000000,
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum SQ {
  NEED_WAKEUP = (1 << 0),
  OVERFLOW    = (1 << 1),
  TASKRUN     = (1 << 2),
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum CQ {
  EVENTFD_DISABLED = (1 << 0),
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum ENTER {
  GET_EVENTS      = (1 << 0),
  SQ_WAKEUP       = (1 << 1),
  SQ_WAIT         = (1 << 2),
  EXT_ARG         = (1 << 3),
  REGISTERED_RING = (1 << 4),
}

#[allow(non_camel_case_types)]
#[repr(u32)]
pub enum FEAT {
  SINGLE_MMAP		  = (1 << 0),
  NODROP		      = (1 << 1),
  SUBMIT_STABLE	  = (1 << 2),
  RW_CUR_POS		  = (1 << 3),
  CUR_PERSONALITY	= (1 << 4),
  FAST_POLL		    = (1 << 5),
  POLL_32BITS 	  = (1 << 6),
  SQPOLL_NONFIXED	= (1 << 7),
  EXT_ARG		      = (1 << 8),
  NATIVE_WORKERS	= (1 << 9),
  RSRC_TAGS		    = (1 << 10),
  CQE_SKIP		    = (1 << 11),
  LINKED_FILE		  = (1 << 12),
}

#[allow(non_camel_case_types)]
pub enum REGISTER {
	REGISTER_BUFFERS			    = 0,
	UNREGISTER_BUFFERS		    = 1,
	REGISTER_FILES			      = 2,
	UNREGISTER_FILES			    = 3,
	REGISTER_EVENTFD			    = 4,
	UNREGISTER_EVENTFD		    = 5,
	REGISTER_FILES_UPDATE		  = 6,
	REGISTER_EVENTFD_ASYNC		= 7,
	REGISTER_PROBE			      = 8,
	REGISTER_PERSONALITY		  = 9,
	UNREGISTER_PERSONALITY		= 10,
	REGISTER_RESTRICTIONS		  = 11,
	REGISTER_ENABLE_RINGS		  = 12,
	REGISTER_FILES2			      = 13,
	REGISTER_FILES_UPDATE2		= 14,
	REGISTER_BUFFERS2		      = 15,
	REGISTER_BUFFERS_UPDATE		= 16,
	REGISTER_IOWQ_AFF		      = 17,
	UNREGISTER_IOWQ_AFF		    = 18,
	REGISTER_IOWQ_MAX_WORKERS	= 19,
	REGISTER_RING_FDS		      = 20,
	UNREGISTER_RING_FDS		    = 21,
	REGISTER_PBUF_RING		    = 22,
	UNREGISTER_PBUF_RING		  = 23,
	REGISTER_SYNC_CANCEL		  = 24,
	REGISTER_FILE_ALLOC_RANGE	= 25,
	REGISTER_LAST             = 26,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum IO_WQ {
  BOUND,
  UNBOUND,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum RESTRICTION {
  REGISTER_OP        = 0,
  SQE_OP             = 1,
  SQE_FLAGS_ALLOWED  = 2,
  SQE_FLAGS_REQUIRED = 3,
  LAST               = 4,
}