/* ADAPTED FROM LIBURING */

/*
 * sqe->file_index flags
 */
pub const IORING_FILE_INDEX_ALLOC: u32 = !0;

/*
 * sqe->flags
 */
pub const IOSQE_FIXED_FILE: u32	      = 1 << 0;
pub const IOSQE_IO_DRAIN: u32         = 1 << 1;
pub const IOSQE_IO_LINK: u32          = 1 << 2;
pub const IOSQE_IO_HARDLINK: u32      = 1 << 3;
pub const IOSQE_ASYNC: u32            = 1 << 4;
pub const IOSQE_BUFFER_SELECT: u32    = 1 << 5;
pub const IOSQE_CQE_SKIP_SUCCESS: u32 = 1 << 6;

/*
 * io_uring_setup() flags
 */
pub const IORING_SETUP_IOPOLL: u32        = 1 << 0;
pub const IORING_SETUP_SQPOLL: u32        = 1 << 1;
pub const IORING_SETUP_SQ_AFF: u32        = 1 << 2;
pub const IORING_SETUP_CQSIZE: u32        = 1 << 3;
pub const IORING_SETUP_CLAMP: u32         = 1 << 4;
pub const IORING_SETUP_ATTACH_WQ: u32     = 1 << 5;
pub const IORING_SETUP_R_DISABLED: u32    = 1 << 6;
pub const IORING_SETUP_SUBMIT_ALL: u32    = 1 << 7;
pub const IORING_SETUP_COOP_TASKRUN: u32  = 1 << 8;
pub const IORING_SETUP_TASKRUN_FLAG: u32  = 1 << 9;
pub const IORING_SETUP_SQE128: u32        = 1 << 10;
pub const IORING_SETUP_CQE32: u32         = 1 << 11;
pub const IORING_SETUP_SINGLE_ISSUER: u32 = 1 << 12;
pub const IORING_SETUP_DEFER_TASKRUN: u32 = 1 << 13;

/*
 * sqe->opcode
 */
pub const IORING_OP_NOP: u32             = 0;
pub const IORING_OP_READV: u32           = 1;
pub const IORING_OP_WRITEV: u32          = 2;
pub const IORING_OP_FSYNC: u32           = 3;
pub const IORING_OP_READ_FIXED: u32      = 4;
pub const IORING_OP_WRITE_FIXED: u32     = 5;
pub const IORING_OP_POLL_ADD: u32        = 6;
pub const IORING_OP_POLL_REMOVE: u32     = 7;
pub const IORING_OP_SYNC_FILE_RANGE: u32 = 8;
pub const IORING_OP_SENDMSG: u32         = 9;
pub const IORING_OP_RECVMSG: u32         = 10;
pub const IORING_OP_TIMEOUT: u32         = 11;
pub const IORING_OP_TIMEOUT_REMOVE: u32  = 12;
pub const IORING_OP_ACCEPT: u32          = 13;
pub const IORING_OP_ASYNC_CANCEL: u32    = 14;
pub const IORING_OP_LINK_TIMEOUT: u32    = 15;
pub const IORING_OP_CONNECT: u32         = 16;
pub const IORING_OP_FALLOCATE: u32       = 17;
pub const IORING_OP_OPENAT: u32          = 18;
pub const IORING_OP_CLOSE: u32           = 19;
pub const IORING_OP_FILES_UPDATE: u32    = 20;
pub const IORING_OP_STATX: u32           = 21;
pub const IORING_OP_READ: u32            = 22;
pub const IORING_OP_WRITE: u32           = 23;
pub const IORING_OP_FADVISE: u32         = 24;
pub const IORING_OP_MADVISE: u32         = 25;
pub const IORING_OP_SEND: u32            = 26;
pub const IORING_OP_RECV: u32            = 27;
pub const IORING_OP_OPENAT2: u32         = 28;
pub const IORING_OP_EPOLL_CTL: u32       = 29;
pub const IORING_OP_SPLICE: u32          = 30;
pub const IORING_OP_PROVIDE_BUFFERS: u32 = 31;
pub const IORING_OP_REMOVE_BUFFERS: u32  = 32;
pub const IORING_OP_TEE: u32             = 33;
pub const IORING_OP_SHUTDOWN: u32        = 34;
pub const IORING_OP_RENAMEAT: u32        = 35;
pub const IORING_OP_UNLINKAT: u32        = 36;
pub const IORING_OP_MKDIRAT: u32         = 37;
pub const IORING_OP_SYMLINKAT: u32       = 38;
pub const IORING_OP_LINKAT: u32          = 39;
pub const IORING_OP_MSG_RING: u32        = 40;
pub const IORING_OP_FSETXATTR: u32       = 41;
pub const IORING_OP_SETXATTR: u32        = 42;
pub const IORING_OP_FGETXATTR: u32       = 43;
pub const IORING_OP_GETXATTR: u32        = 44;
pub const IORING_OP_SOCKET: u32          = 45;
pub const IORING_OP_URING_CMD: u32       = 46;
pub const IORING_OP_SEND_ZC: u32         = 47;
pub const IORING_OP_SENDMSG_ZC: u32      = 48;
pub const IORING_OP_LAST: u32            = 49;

/*
 * sqe->uring_cmd_flags
 */
pub const IORING_URING_CMD_FIXED: u32 = 1 << 0;

/*
 * sqe->fsync_flags
 */
pub const IORING_FSYNC_DATASYNC: u32 = 1 << 0;

/*
 * sqe->timeout_flags
 */
pub const IORING_TIMEOUT_ABS: u32           = 1 << 0;
pub const IORING_TIMEOUT_UPDATE: u32        = 1 << 1;
pub const IORING_TIMEOUT_BOOTTIME: u32      = 1 << 2;
pub const IORING_TIMEOUT_REALTIME: u32      = 1 << 3;
pub const IORING_LINK_TIMEOUT_UPDATE: u32   = 1 << 4;
pub const IORING_TIMEOUT_ETIME_SUCCESS: u32 = 1 << 5;
pub const IORING_TIMEOUT_CLOCK_MASK: u32    = IORING_TIMEOUT_BOOTTIME | IORING_TIMEOUT_REALTIME;
pub const IORING_TIMEOUT_UPDATE_MASK: u32   = IORING_TIMEOUT_UPDATE | IORING_LINK_TIMEOUT_UPDATE;

/*
 * sqe->splice_flags 
 */
pub const SPLICE_F_FD_IN_FIXED: u32 = 1 << 31;

/*
 * sqe->poll_events flags
 */
pub const IORING_POLL_ADD_MULTI: u32        = 1 << 0;
pub const IORING_POLL_UPDATE_EVENTS: u32    = 1 << 1;
pub const IORING_POLL_UPDATE_USER_DATA: u32 = 1 << 2;
pub const IORING_POLL_ADD_LEVEL: u32        = 1 << 3;

/*
 * sqe->async_cancel flags
 */
pub const IORING_ASYNC_CANCEL_ALL: u32   = 1 << 0;
pub const IORING_ASYNC_CANCEL_FD: u32    = 1 << 1;
pub const IORING_ASYNC_CANCEL_ANY: u32   = 1 << 2;
pub const IORING_ASYNC_CANCEL_FIXED: u32 = 1 << 3;

/*
 * sqe->ioprio send/sendmsg and recv/recvmsg flags 
 */
pub const IORING_RECVSEND_POLL_FIRST: u32  = 1 << 0;
pub const IORING_RECV_MULTISHOT: u32       = 1 << 1;
pub const IORING_RECVSEND_FIXED_BUF: u32   = 1 << 2;
pub const IORING_SEND_ZC_REPORT_USAGE: u32 = 1 << 3;

/*
 * cqe->res flags
 */
pub const IORING_NOTIF_USAGE_ZC_COPIED: u32 = 1 << 31;

/*
 * sqe->ioprio accept flags
 */
pub const IORING_ACCEPT_MULTISHOT: u32 = 1 << 0;

/*
 * sqe->addr command types
 */
pub const IORING_MSG_DATA: u32    = 0;
pub const IORING_MSG_SEND_FD: u32 = 1;

/*
 * sqe->msg_ring_flags flags
 */
pub const IORING_MSG_RING_CQE_SKIP: u32 = 1 << 0;

/*
 * cqe->flags flags
 */
pub const IORING_CQE_F_BUFFER: u32       = 1 << 0;
pub const IORING_CQE_F_MORE: u32         = 1 << 1;
pub const IORING_CQE_F_SOCK_NOEMPTY: u32 = 1 << 2;
pub const IORING_CQE_F_NOTIF: u32        = 1 << 3;

/*
 *
 */
pub const IORING_CQE_BUFFER_SHIFT: u32   = 1 << 4;

/*
 * mmap address offsets
 */
pub const IORING_OFF_SQ_RING: u64 = 0x00000000;
pub const IORING_OFF_CQ_RING: u64 = 0x08000000;
pub const IORING_OFF_SQES: u64    = 0x10000000;

/*
 * sqring_offsets->flags flags
 */
pub const IORING_SQ_NEED_WAKEUP: u32 = 1 << 0;
pub const IORING_SQ_CQ_OVERFLOW: u32 = 1 << 1;
pub const IORING_SQ_TASKRUN: u32     = 1 << 2;

/*
 * cqring_offsets->flags flags
 */

/*
 * Disable eventfd notifications
 */
pub const IORING_CQ_EVENTFD_DISABLED: u32 = 1 << 0;

/*
 * io_uring_enter2() flags
 */
pub const IORING_ENTER_GETEVENTS: u32       = 1 << 0;
pub const IORING_ENTER_SQ_WAKEUP: u32       = 1 << 1;
pub const IORING_ENTER_SQ_WAIT: u32         = 1 << 2;
pub const IORING_ENTER_EXT_ARG: u32         = 1 << 3;
pub const IORING_ENTER_REGISTERED_RING: u32 = 1 << 4;

/*
 * io_uring_params->features flags
 */
pub const IORING_FEAT_SINGLE_MMAP: u32     = 1 << 0;
pub const IORING_FEAT_NODROP: u32          = 1 << 1;
pub const IORING_FEAT_SUBMIT_STABLE: u32   = 1 << 2;
pub const IORING_FEAT_RW_CUR_POS: u32      = 1 << 3;
pub const IORING_FEAT_CUR_PERSONALITY: u32 = 1 << 4;
pub const IORING_FEAT_FAST_POLL: u32       = 1 << 5;
pub const IORING_FEAT_POLL_32BITS: u32     = 1 << 6;
pub const IORING_FEAT_SQPOLL_NONFIXED: u32 = 1 << 7;
pub const IORING_FEAT_EXT_ARG: u32         = 1 << 8;
pub const IORING_FEAT_NATIVE_WORKERS: u32  = 1 << 9;
pub const IORING_FEAT_RSRC_TAGS: u32       = 1 << 10;
pub const IORING_FEAT_CQE_SKIP: u32        = 1 << 11;
pub const IORING_FEAT_LINKED_FILE: u32     = 1 << 12;

/*
 * io_uring_register() opcodes and arguments
 */
pub const IORING_FEAT_REGISTER_BUFFERS: u32          = 0;
pub const IORING_FEAT_UNREGISTER_BUFFERS: u32        = 1;
pub const IORING_FEAT_REGISTER_FILES: u32            = 2;
pub const IORING_FEAT_UNREGISTER_FILES: u32          = 3;
pub const IORING_FEAT_REGISTER_EVENTFD: u32          = 4;
pub const IORING_FEAT_UNREGISTER_EVENTFD: u32        = 5;
pub const IORING_FEAT_REGISTER_FILES_UPDATE: u32     = 6;
pub const IORING_FEAT_REGISTER_EVENTFD_ASYNC: u32    = 7;
pub const IORING_FEAT_REGISTER_PROBE: u32            = 8;
pub const IORING_FEAT_REGISTER_PERSONALITY: u32      = 9;
pub const IORING_FEAT_UNREGISTER_PERSONALITY: u32    = 10;
pub const IORING_FEAT_REGISTER_RESTRICTIONS: u32     = 11;
pub const IORING_FEAT_REGISTER_ENABLE_RINGS: u32     = 12;
pub const IORING_FEAT_REGISTER_FILES2: u32           = 13;
pub const IORING_FEAT_REGISTER_FILES_UPDATE2: u32    = 14;
pub const IORING_FEAT_REGISTER_BUFFERS2: u32         = 15;
pub const IORING_FEAT_REGISTER_BUFFERS_UPDATE: u32   = 16;
pub const IORING_FEAT_REGISTER_IOWQ_AFF: u32         = 17;
pub const IORING_FEAT_UNREGISTER_IOWQ_AFF: u32       = 18;
pub const IORING_FEAT_REGISTER_IOWQ_MAX_WORKERS: u32 = 19;
pub const IORING_FEAT_REGISTER_RING_FDS: u32         = 20;
pub const IORING_FEAT_UNREGISTER_RING_FDS: u32       = 21;
pub const IORING_FEAT_REGISTER_PBUF_RING: u32        = 22;
pub const IORING_FEAT_UNREGISTER_PBUF_RING: u32      = 23;
pub const IORING_FEAT_REGISTER_SYNC_CANCEL: u32      = 24;
pub const IORING_FEAT_REGISTER_FILE_ALLOC_RANGE: u32 = 25;
pub const IORING_FEAT_REGISTER_LAST: u32             = 26;

/*
 * io-wq worker categories
 */
pub const IO_WQ_BOUND: u32   = 0;
pub const IO_WQ_UNBOUND: u32 = 1;

/*
 * Register a fully sparse file space, rather than pass in an array of all - 1 file descriptors
 */
pub const IORING_RSRC_REGISTER_SPARSE: u32 = 1 << 0;

/*
 * Skip updating fd indexes set to this value in the fd table
 */
pub const IORING_REGISTER_FILES_SKIP: i32 = -2;

/*
 *
 */
pub const IO_URING_OP_SUPPORTED: u32 = 1 << 0;

/*
 * io_uring_restriction->opcode values
 */
pub const IORING_RESTRICTION_REGISTER_OP: u32        = 0;
pub const IORING_RESTRICTION_SQE_OP: u32             = 1;
pub const IORING_RESTRICTION_SQE_FLAGS_ALLOWED: u32  = 2;
pub const IORING_RESTRICTION_SQE_FLAGS_REQUIRED: u32 = 3;
pub const IORING_RESTRICTION_LAST: u32               = 4;