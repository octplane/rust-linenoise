#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
extern crate libc;

// pub type tcflag_t = libc::c_ulong;
// pub type cc_t = libc::c_uchar;
// pub type speed_t = libc::c_ulong;
// #[repr(C)]
// pub struct Struct_termios {
//     pub c_iflag: tcflag_t,
//     pub c_oflag: tcflag_t,
//     pub c_cflag: tcflag_t,
//     pub c_lflag: tcflag_t,
//     pub c_cc: [cc_t, ..20u],
//     pub c_ispeed: speed_t,
//     pub c_ospeed: speed_t,
// }
pub type __int8_t = libc::c_char;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __uint64_t = libc::c_ulonglong;
pub type __darwin_intptr_t = libc::c_long;
pub type __darwin_natural_t = libc::c_uint;
pub type __darwin_ct_rune_t = libc::c_int;
// #[repr(C)]
// pub struct Union_Unnamed1 {
//     pub data: [u64, ..16u],
// }
// impl Union_Unnamed1 {
//     pub fn __mbstate8(&mut self) -> *mut [libc::c_char, ..128u] {
//         unsafe { ::std::mem::transmute(self) }
//     }
//     pub fn _mbstateL(&mut self) -> *mut libc::c_longlong {
//         unsafe { ::std::mem::transmute(self) }
//     }
// }
// pub type __mbstate_t = Union_Unnamed1;
// pub type __darwin_mbstate_t = __mbstate_t;
pub type __darwin_ptrdiff_t = libc::c_long;
pub type __darwin_wchar_t = libc::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_wint_t = libc::c_int;
pub type __darwin_clock_t = libc::c_ulong;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_ssize_t = libc::c_long;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_fsblkcnt_t = libc::c_uint;
pub type __darwin_fsfilcnt_t = libc::c_uint;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_id_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_ino_t = __darwin_ino64_t;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_sigset_t = __uint32_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
pub type __darwin_useconds_t = __uint32_t;
// #[repr(C)]
// pub struct Struct___darwin_pthread_handler_rec {
//     pub __routine: ::std::option::Option<extern "C" fn
//                                              (arg1: *mut libc::c_void)>,
//     pub __arg: *mut libc::c_void,
//     pub __next: *mut Struct___darwin_pthread_handler_rec,
// }
// #[repr(C)]
// pub struct Struct__opaque_pthread_attr_t {
//     pub __sig: libc::c_long,
//     pub __opaque: [libc::c_char, ..56u],
// }
// #[repr(C)]
// pub struct Struct__opaque_pthread_cond_t {
//     pub __sig: libc::c_long,
//     pub __opaque: [libc::c_char, ..40u],
// }
// #[repr(C)]
// pub struct Struct__opaque_pthread_condattr_t {
//     pub __sig: libc::c_long,
//     pub __opaque: [libc::c_char, ..8u],
// }
// #[repr(C)]
// pub struct Struct__opaque_pthread_mutex_t {
//     pub __sig: libc::c_long,
//     pub __opaque: [libc::c_char, ..56u],
// }
// #[repr(C)]
// pub struct Struct__opaque_pthread_mutexattr_t {
//     pub __sig: libc::c_long,
//     pub __opaque: [libc::c_char, ..8u],
// }
// #[repr(C)]
// pub struct Struct__opaque_pthread_once_t {
//     pub __sig: libc::c_long,
//     pub __opaque: [libc::c_char, ..8u],
// }
// #[repr(C)]
// pub struct Struct__opaque_pthread_rwlock_t {
//     pub __sig: libc::c_long,
//     pub __opaque: [libc::c_char, ..192u],
// }
// #[repr(C)]
// pub struct Struct__opaque_pthread_rwlockattr_t {
//     pub __sig: libc::c_long,
//     pub __opaque: [libc::c_char, ..16u],
// }
// #[repr(C)]
// pub struct Struct__opaque_pthread_t {
//     pub __sig: libc::c_long,
//     pub __cleanup_stack: *mut Struct___darwin_pthread_handler_rec,
//     pub __opaque: [libc::c_char, ..8176u],
// }
// pub type __darwin_pthread_attr_t = Struct__opaque_pthread_attr_t;
// pub type __darwin_pthread_cond_t = Struct__opaque_pthread_cond_t;
// pub type __darwin_pthread_condattr_t = Struct__opaque_pthread_condattr_t;
// pub type __darwin_pthread_key_t = libc::c_ulong;
// pub type __darwin_pthread_mutex_t = Struct__opaque_pthread_mutex_t;
// pub type __darwin_pthread_mutexattr_t = Struct__opaque_pthread_mutexattr_t;
// pub type __darwin_pthread_once_t = Struct__opaque_pthread_once_t;
// pub type __darwin_pthread_rwlock_t = Struct__opaque_pthread_rwlock_t;
// pub type __darwin_pthread_rwlockattr_t = Struct__opaque_pthread_rwlockattr_t;
// pub type __darwin_pthread_t = *mut Struct__opaque_pthread_t;
// #[repr(C)]
// pub struct Struct_winsize {
//     pub ws_row: libc::c_ushort,
//     pub ws_col: libc::c_ushort,
//     pub ws_xpixel: libc::c_ushort,
//     pub ws_ypixel: libc::c_ushort,
// }
// pub type __darwin_nl_item = libc::c_int;
// pub type __darwin_wctrans_t = libc::c_int;
// pub type __darwin_wctype_t = __uint32_t;
// pub type pid_t = __darwin_pid_t;
// #[repr(C)]
// pub struct Struct_accessx_descriptor {
//     pub ad_name_offset: libc::c_uint,
//     pub ad_flags: libc::c_int,
//     pub ad_pad: [libc::c_int, ..2u],
// }
pub type uint64_t = libc::c_ulonglong;
// pub type ssize_t = __darwin_ssize_t;
// pub type uid_t = __darwin_uid_t;
// pub type gid_t = __darwin_gid_t;
// pub type intptr_t = __darwin_intptr_t;
// pub type off_t = __darwin_off_t;
// pub type useconds_t = __darwin_useconds_t;
// #[repr(C)]
// pub struct Struct_fd_set {
//     pub fds_bits: [__int32_t, ..32u],
// }
// pub type fd_set = Struct_fd_set;
// #[repr(C)]
// pub struct Struct_timespec {
//     pub tv_sec: __darwin_time_t,
//     pub tv_nsec: libc::c_long,
// }
// #[repr(C)]
// pub struct Struct_timeval {
//     pub tv_sec: __darwin_time_t,
//     pub tv_usec: __darwin_suseconds_t,
// }
// pub type time_t = __darwin_time_t;
// pub type suseconds_t = __darwin_suseconds_t;
// pub type sigset_t = __darwin_sigset_t;
// pub type dev_t = __darwin_dev_t;
// pub type mode_t = __darwin_mode_t;
// pub type uuid_t = __darwin_uuid_t;
// pub enum Struct_fssearchblock { }
// pub enum Struct_searchstate { }
// pub type Enum_Unnamed2 = libc::c_uint;
// pub const P_ALL: libc::c_uint = 0;
// pub const P_PID: libc::c_uint = 1;
// pub const P_PGID: libc::c_uint = 2;
// pub type idtype_t = Enum_Unnamed2;
// pub type id_t = __darwin_id_t;
// pub type sig_atomic_t = libc::c_int;
// #[repr(C)]
// pub struct Struct___darwin_i386_thread_state {
//     pub __eax: libc::c_uint,
//     pub __ebx: libc::c_uint,
//     pub __ecx: libc::c_uint,
//     pub __edx: libc::c_uint,
//     pub __edi: libc::c_uint,
//     pub __esi: libc::c_uint,
//     pub __ebp: libc::c_uint,
//     pub __esp: libc::c_uint,
//     pub __ss: libc::c_uint,
//     pub __eflags: libc::c_uint,
//     pub __eip: libc::c_uint,
//     pub __cs: libc::c_uint,
//     pub __ds: libc::c_uint,
//     pub __es: libc::c_uint,
//     pub __fs: libc::c_uint,
//     pub __gs: libc::c_uint,
// }
// #[repr(C)]
// pub struct Struct___darwin_fp_control {
//     pub __invalid: libc::c_ushort,
//     pub __denorm: libc::c_ushort,
//     pub __zdiv: libc::c_ushort,
//     pub __ovrfl: libc::c_ushort,
//     pub __undfl: libc::c_ushort,
//     pub __precis: libc::c_ushort,
//     pub unnamed_field1: libc::c_ushort,
//     pub __pc: libc::c_ushort,
//     pub __rc: libc::c_ushort,
//     pub unnamed_field2: libc::c_ushort,
//     pub unnamed_field3: libc::c_ushort,
// }
// pub type __darwin_fp_control_t = Struct___darwin_fp_control;
// #[repr(C)]
// pub struct Struct___darwin_fp_status {
//     pub __invalid: libc::c_ushort,
//     pub __denorm: libc::c_ushort,
//     pub __zdiv: libc::c_ushort,
//     pub __ovrfl: libc::c_ushort,
//     pub __undfl: libc::c_ushort,
//     pub __precis: libc::c_ushort,
//     pub __stkflt: libc::c_ushort,
//     pub __errsumm: libc::c_ushort,
//     pub __c0: libc::c_ushort,
//     pub __c1: libc::c_ushort,
//     pub __c2: libc::c_ushort,
//     pub __tos: libc::c_ushort,
//     pub __c3: libc::c_ushort,
//     pub __busy: libc::c_ushort,
// }
// pub type __darwin_fp_status_t = Struct___darwin_fp_status;
// #[repr(C)]
// pub struct Struct___darwin_mmst_reg {
//     pub __mmst_reg: [libc::c_char, ..10u],
//     pub __mmst_rsrv: [libc::c_char, ..6u],
// }
// #[repr(C)]
// pub struct Struct___darwin_xmm_reg {
//     pub __xmm_reg: [libc::c_char, ..16u],
// }
// #[repr(C)]
// pub struct Struct___darwin_i386_float_state {
//     pub __fpu_reserved: [libc::c_int, ..2u],
//     pub __fpu_fcw: Struct___darwin_fp_control,
//     pub __fpu_fsw: Struct___darwin_fp_status,
//     pub __fpu_ftw: __uint8_t,
//     pub __fpu_rsrv1: __uint8_t,
//     pub __fpu_fop: __uint16_t,
//     pub __fpu_ip: __uint32_t,
//     pub __fpu_cs: __uint16_t,
//     pub __fpu_rsrv2: __uint16_t,
//     pub __fpu_dp: __uint32_t,
//     pub __fpu_ds: __uint16_t,
//     pub __fpu_rsrv3: __uint16_t,
//     pub __fpu_mxcsr: __uint32_t,
//     pub __fpu_mxcsrmask: __uint32_t,
//     pub __fpu_stmm0: Struct___darwin_mmst_reg,
//     pub __fpu_stmm1: Struct___darwin_mmst_reg,
//     pub __fpu_stmm2: Struct___darwin_mmst_reg,
//     pub __fpu_stmm3: Struct___darwin_mmst_reg,
//     pub __fpu_stmm4: Struct___darwin_mmst_reg,
//     pub __fpu_stmm5: Struct___darwin_mmst_reg,
//     pub __fpu_stmm6: Struct___darwin_mmst_reg,
//     pub __fpu_stmm7: Struct___darwin_mmst_reg,
//     pub __fpu_xmm0: Struct___darwin_xmm_reg,
//     pub __fpu_xmm1: Struct___darwin_xmm_reg,
//     pub __fpu_xmm2: Struct___darwin_xmm_reg,
//     pub __fpu_xmm3: Struct___darwin_xmm_reg,
//     pub __fpu_xmm4: Struct___darwin_xmm_reg,
//     pub __fpu_xmm5: Struct___darwin_xmm_reg,
//     pub __fpu_xmm6: Struct___darwin_xmm_reg,
//     pub __fpu_xmm7: Struct___darwin_xmm_reg,
//     pub __fpu_rsrv4: [libc::c_char, ..224u],
//     pub __fpu_reserved1: libc::c_int,
// }
// #[repr(C)]
// pub struct Struct___darwin_i386_avx_state {
//     pub __fpu_reserved: [libc::c_int, ..2u],
//     pub __fpu_fcw: Struct___darwin_fp_control,
//     pub __fpu_fsw: Struct___darwin_fp_status,
//     pub __fpu_ftw: __uint8_t,
//     pub __fpu_rsrv1: __uint8_t,
//     pub __fpu_fop: __uint16_t,
//     pub __fpu_ip: __uint32_t,
//     pub __fpu_cs: __uint16_t,
//     pub __fpu_rsrv2: __uint16_t,
//     pub __fpu_dp: __uint32_t,
//     pub __fpu_ds: __uint16_t,
//     pub __fpu_rsrv3: __uint16_t,
//     pub __fpu_mxcsr: __uint32_t,
//     pub __fpu_mxcsrmask: __uint32_t,
//     pub __fpu_stmm0: Struct___darwin_mmst_reg,
//     pub __fpu_stmm1: Struct___darwin_mmst_reg,
//     pub __fpu_stmm2: Struct___darwin_mmst_reg,
//     pub __fpu_stmm3: Struct___darwin_mmst_reg,
//     pub __fpu_stmm4: Struct___darwin_mmst_reg,
//     pub __fpu_stmm5: Struct___darwin_mmst_reg,
//     pub __fpu_stmm6: Struct___darwin_mmst_reg,
//     pub __fpu_stmm7: Struct___darwin_mmst_reg,
//     pub __fpu_xmm0: Struct___darwin_xmm_reg,
//     pub __fpu_xmm1: Struct___darwin_xmm_reg,
//     pub __fpu_xmm2: Struct___darwin_xmm_reg,
//     pub __fpu_xmm3: Struct___darwin_xmm_reg,
//     pub __fpu_xmm4: Struct___darwin_xmm_reg,
//     pub __fpu_xmm5: Struct___darwin_xmm_reg,
//     pub __fpu_xmm6: Struct___darwin_xmm_reg,
//     pub __fpu_xmm7: Struct___darwin_xmm_reg,
//     pub __fpu_rsrv4: [libc::c_char, ..224u],
//     pub __fpu_reserved1: libc::c_int,
//     pub __avx_reserved1: [libc::c_char, ..64u],
//     pub __fpu_ymmh0: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh1: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh2: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh3: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh4: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh5: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh6: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh7: Struct___darwin_xmm_reg,
// }
// #[repr(C)]
// pub struct Struct___darwin_i386_exception_state {
//     pub __trapno: __uint16_t,
//     pub __cpu: __uint16_t,
//     pub __err: __uint32_t,
//     pub __faultvaddr: __uint32_t,
// }
// #[repr(C)]
// pub struct Struct___darwin_x86_debug_state32 {
//     pub __dr0: libc::c_uint,
//     pub __dr1: libc::c_uint,
//     pub __dr2: libc::c_uint,
//     pub __dr3: libc::c_uint,
//     pub __dr4: libc::c_uint,
//     pub __dr5: libc::c_uint,
//     pub __dr6: libc::c_uint,
//     pub __dr7: libc::c_uint,
// }
// #[repr(C)]
// pub struct Struct___darwin_x86_thread_state64 {
//     pub __rax: __uint64_t,
//     pub __rbx: __uint64_t,
//     pub __rcx: __uint64_t,
//     pub __rdx: __uint64_t,
//     pub __rdi: __uint64_t,
//     pub __rsi: __uint64_t,
//     pub __rbp: __uint64_t,
//     pub __rsp: __uint64_t,
//     pub __r8: __uint64_t,
//     pub __r9: __uint64_t,
//     pub __r10: __uint64_t,
//     pub __r11: __uint64_t,
//     pub __r12: __uint64_t,
//     pub __r13: __uint64_t,
//     pub __r14: __uint64_t,
//     pub __r15: __uint64_t,
//     pub __rip: __uint64_t,
//     pub __rflags: __uint64_t,
//     pub __cs: __uint64_t,
//     pub __fs: __uint64_t,
//     pub __gs: __uint64_t,
// }
// #[repr(C)]
// pub struct Struct___darwin_x86_float_state64 {
//     pub __fpu_reserved: [libc::c_int, ..2u],
//     pub __fpu_fcw: Struct___darwin_fp_control,
//     pub __fpu_fsw: Struct___darwin_fp_status,
//     pub __fpu_ftw: __uint8_t,
//     pub __fpu_rsrv1: __uint8_t,
//     pub __fpu_fop: __uint16_t,
//     pub __fpu_ip: __uint32_t,
//     pub __fpu_cs: __uint16_t,
//     pub __fpu_rsrv2: __uint16_t,
//     pub __fpu_dp: __uint32_t,
//     pub __fpu_ds: __uint16_t,
//     pub __fpu_rsrv3: __uint16_t,
//     pub __fpu_mxcsr: __uint32_t,
//     pub __fpu_mxcsrmask: __uint32_t,
//     pub __fpu_stmm0: Struct___darwin_mmst_reg,
//     pub __fpu_stmm1: Struct___darwin_mmst_reg,
//     pub __fpu_stmm2: Struct___darwin_mmst_reg,
//     pub __fpu_stmm3: Struct___darwin_mmst_reg,
//     pub __fpu_stmm4: Struct___darwin_mmst_reg,
//     pub __fpu_stmm5: Struct___darwin_mmst_reg,
//     pub __fpu_stmm6: Struct___darwin_mmst_reg,
//     pub __fpu_stmm7: Struct___darwin_mmst_reg,
//     pub __fpu_xmm0: Struct___darwin_xmm_reg,
//     pub __fpu_xmm1: Struct___darwin_xmm_reg,
//     pub __fpu_xmm2: Struct___darwin_xmm_reg,
//     pub __fpu_xmm3: Struct___darwin_xmm_reg,
//     pub __fpu_xmm4: Struct___darwin_xmm_reg,
//     pub __fpu_xmm5: Struct___darwin_xmm_reg,
//     pub __fpu_xmm6: Struct___darwin_xmm_reg,
//     pub __fpu_xmm7: Struct___darwin_xmm_reg,
//     pub __fpu_xmm8: Struct___darwin_xmm_reg,
//     pub __fpu_xmm9: Struct___darwin_xmm_reg,
//     pub __fpu_xmm10: Struct___darwin_xmm_reg,
//     pub __fpu_xmm11: Struct___darwin_xmm_reg,
//     pub __fpu_xmm12: Struct___darwin_xmm_reg,
//     pub __fpu_xmm13: Struct___darwin_xmm_reg,
//     pub __fpu_xmm14: Struct___darwin_xmm_reg,
//     pub __fpu_xmm15: Struct___darwin_xmm_reg,
//     pub __fpu_rsrv4: [libc::c_char, ..96u],
//     pub __fpu_reserved1: libc::c_int,
// }
// #[repr(C)]
// pub struct Struct___darwin_x86_avx_state64 {
//     pub __fpu_reserved: [libc::c_int, ..2u],
//     pub __fpu_fcw: Struct___darwin_fp_control,
//     pub __fpu_fsw: Struct___darwin_fp_status,
//     pub __fpu_ftw: __uint8_t,
//     pub __fpu_rsrv1: __uint8_t,
//     pub __fpu_fop: __uint16_t,
//     pub __fpu_ip: __uint32_t,
//     pub __fpu_cs: __uint16_t,
//     pub __fpu_rsrv2: __uint16_t,
//     pub __fpu_dp: __uint32_t,
//     pub __fpu_ds: __uint16_t,
//     pub __fpu_rsrv3: __uint16_t,
//     pub __fpu_mxcsr: __uint32_t,
//     pub __fpu_mxcsrmask: __uint32_t,
//     pub __fpu_stmm0: Struct___darwin_mmst_reg,
//     pub __fpu_stmm1: Struct___darwin_mmst_reg,
//     pub __fpu_stmm2: Struct___darwin_mmst_reg,
//     pub __fpu_stmm3: Struct___darwin_mmst_reg,
//     pub __fpu_stmm4: Struct___darwin_mmst_reg,
//     pub __fpu_stmm5: Struct___darwin_mmst_reg,
//     pub __fpu_stmm6: Struct___darwin_mmst_reg,
//     pub __fpu_stmm7: Struct___darwin_mmst_reg,
//     pub __fpu_xmm0: Struct___darwin_xmm_reg,
//     pub __fpu_xmm1: Struct___darwin_xmm_reg,
//     pub __fpu_xmm2: Struct___darwin_xmm_reg,
//     pub __fpu_xmm3: Struct___darwin_xmm_reg,
//     pub __fpu_xmm4: Struct___darwin_xmm_reg,
//     pub __fpu_xmm5: Struct___darwin_xmm_reg,
//     pub __fpu_xmm6: Struct___darwin_xmm_reg,
//     pub __fpu_xmm7: Struct___darwin_xmm_reg,
//     pub __fpu_xmm8: Struct___darwin_xmm_reg,
//     pub __fpu_xmm9: Struct___darwin_xmm_reg,
//     pub __fpu_xmm10: Struct___darwin_xmm_reg,
//     pub __fpu_xmm11: Struct___darwin_xmm_reg,
//     pub __fpu_xmm12: Struct___darwin_xmm_reg,
//     pub __fpu_xmm13: Struct___darwin_xmm_reg,
//     pub __fpu_xmm14: Struct___darwin_xmm_reg,
//     pub __fpu_xmm15: Struct___darwin_xmm_reg,
//     pub __fpu_rsrv4: [libc::c_char, ..96u],
//     pub __fpu_reserved1: libc::c_int,
//     pub __avx_reserved1: [libc::c_char, ..64u],
//     pub __fpu_ymmh0: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh1: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh2: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh3: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh4: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh5: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh6: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh7: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh8: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh9: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh10: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh11: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh12: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh13: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh14: Struct___darwin_xmm_reg,
//     pub __fpu_ymmh15: Struct___darwin_xmm_reg,
// }
// #[repr(C)]
// pub struct Struct___darwin_x86_exception_state64 {
//     pub __trapno: __uint16_t,
//     pub __cpu: __uint16_t,
//     pub __err: __uint32_t,
//     pub __faultvaddr: __uint64_t,
// }
// #[repr(C)]
// pub struct Struct___darwin_x86_debug_state64 {
//     pub __dr0: __uint64_t,
//     pub __dr1: __uint64_t,
//     pub __dr2: __uint64_t,
//     pub __dr3: __uint64_t,
//     pub __dr4: __uint64_t,
//     pub __dr5: __uint64_t,
//     pub __dr6: __uint64_t,
//     pub __dr7: __uint64_t,
// }
// #[repr(C)]
// pub struct Struct___darwin_mcontext32 {
//     pub __es: Struct___darwin_i386_exception_state,
//     pub __ss: Struct___darwin_i386_thread_state,
//     pub __fs: Struct___darwin_i386_float_state,
// }
// #[repr(C)]
// pub struct Struct___darwin_mcontext_avx32 {
//     pub __es: Struct___darwin_i386_exception_state,
//     pub __ss: Struct___darwin_i386_thread_state,
//     pub __fs: Struct___darwin_i386_avx_state,
// }
// #[repr(C)]
// pub struct Struct___darwin_mcontext64 {
//     pub __es: Struct___darwin_x86_exception_state64,
//     pub __ss: Struct___darwin_x86_thread_state64,
//     pub __fs: Struct___darwin_x86_float_state64,
// }
// #[repr(C)]
// pub struct Struct___darwin_mcontext_avx64 {
//     pub __es: Struct___darwin_x86_exception_state64,
//     pub __ss: Struct___darwin_x86_thread_state64,
//     pub __fs: Struct___darwin_x86_avx_state64,
// }
// pub type mcontext_t = *mut Struct___darwin_mcontext64;
// pub type pthread_attr_t = __darwin_pthread_attr_t;
// #[repr(C)]
// pub struct Struct___darwin_sigaltstack {
//     pub ss_sp: *mut libc::c_void,
//     pub ss_size: __darwin_size_t,
//     pub ss_flags: libc::c_int,
// }
// pub type stack_t = Struct___darwin_sigaltstack;
// #[repr(C)]
// pub struct Struct___darwin_ucontext {
//     pub uc_onstack: libc::c_int,
//     pub uc_sigmask: __darwin_sigset_t,
//     pub uc_stack: Struct___darwin_sigaltstack,
//     pub uc_link: *mut Struct___darwin_ucontext,
//     pub uc_mcsize: __darwin_size_t,
//     pub uc_mcontext: *mut Struct___darwin_mcontext64,
// }
// pub type ucontext_t = Struct___darwin_ucontext;
// #[repr(C)]
// pub struct Union_sigval {
//     pub data: [u64, ..1u],
// }
// impl Union_sigval {
//     pub fn sival_int(&mut self) -> *mut libc::c_int {
//         unsafe { ::std::mem::transmute(self) }
//     }
//     pub fn sival_ptr(&mut self) -> *mut *mut libc::c_void {
//         unsafe { ::std::mem::transmute(self) }
//     }
// }
// #[repr(C)]
// pub struct Struct_sigevent {
//     pub sigev_notify: libc::c_int,
//     pub sigev_signo: libc::c_int,
//     pub sigev_value: Union_sigval,
//     pub sigev_notify_function: ::std::option::Option<extern "C" fn
//                                                          (arg1:
//                                                               Union_sigval)>,
//     pub sigev_notify_attributes: *mut pthread_attr_t,
// }
// #[repr(C)]
// pub struct Struct___siginfo {
//     pub si_signo: libc::c_int,
//     pub si_errno: libc::c_int,
//     pub si_code: libc::c_int,
//     pub si_pid: pid_t,
//     pub si_uid: uid_t,
//     pub si_status: libc::c_int,
//     pub si_addr: *mut libc::c_void,
//     pub si_value: Union_sigval,
//     pub si_band: libc::c_long,
//     pub __pad: [libc::c_ulong, ..7u],
// }
// pub type siginfo_t = Struct___siginfo;
// #[repr(C)]
// pub struct Union___sigaction_u {
//     pub data: [u64, ..1u],
// }
// impl Union___sigaction_u {
//     pub fn __sa_handler(&mut self)
//      -> *mut ::std::option::Option<extern "C" fn(arg1: libc::c_int)> {
//         unsafe { ::std::mem::transmute(self) }
//     }
//     pub fn __sa_sigaction(&mut self)
//      ->
//          *mut ::std::option::Option<extern "C" fn
//                                         (arg1: libc::c_int,
//                                          arg2: *mut Struct___siginfo,
//                                          arg3: *mut libc::c_void)> {
//         unsafe { ::std::mem::transmute(self) }
//     }
// }
// #[repr(C)]
// pub struct Struct___sigaction {
//     pub __sigaction_u: Union___sigaction_u,
//     pub sa_tramp: ::std::option::Option<extern "C" fn
//                                             (arg1: *mut libc::c_void,
//                                              arg2: libc::c_int,
//                                              arg3: libc::c_int,
//                                              arg4: *mut siginfo_t,
//                                              arg5: *mut libc::c_void)>,
//     pub sa_mask: sigset_t,
//     pub sa_flags: libc::c_int,
// }
// #[repr(C)]
// pub struct Struct_sigaction {
//     pub __sigaction_u: Union___sigaction_u,
//     pub sa_mask: sigset_t,
//     pub sa_flags: libc::c_int,
// }
// pub type sig_t = ::std::option::Option<extern "C" fn(arg1: libc::c_int)>;
// #[repr(C)]
// pub struct Struct_sigvec {
//     pub sv_handler: ::std::option::Option<extern "C" fn(arg1: libc::c_int)>,
//     pub sv_mask: libc::c_int,
//     pub sv_flags: libc::c_int,
// }
// #[repr(C)]
// pub struct Struct_sigstack {
//     pub ss_sp: *mut libc::c_char,
//     pub ss_onstack: libc::c_int,
// }
pub type int8_t = libc::c_char;
pub type int16_t = libc::c_short;
pub type int32_t = libc::c_int;
pub type int64_t = libc::c_longlong;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type int_least8_t = int8_t;
pub type int_least16_t = int16_t;
pub type int_least32_t = int32_t;
pub type int_least64_t = int64_t;
pub type uint_least8_t = uint8_t;
pub type uint_least16_t = uint16_t;
pub type uint_least32_t = uint32_t;
pub type uint_least64_t = uint64_t;
pub type int_fast8_t = int8_t;
pub type int_fast16_t = int16_t;
pub type int_fast32_t = int32_t;
pub type int_fast64_t = int64_t;
pub type uint_fast8_t = uint8_t;
pub type uint_fast16_t = uint16_t;
pub type uint_fast32_t = uint32_t;
pub type uint_fast64_t = uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type intmax_t = libc::c_long;
pub type uintmax_t = libc::c_ulong;
pub type rlim_t = __uint64_t;
// #[repr(C)]
// pub struct Struct_rusage {
//     pub ru_utime: Struct_timeval,
//     pub ru_stime: Struct_timeval,
//     pub ru_maxrss: libc::c_long,
//     pub ru_ixrss: libc::c_long,
//     pub ru_idrss: libc::c_long,
//     pub ru_isrss: libc::c_long,
//     pub ru_minflt: libc::c_long,
//     pub ru_majflt: libc::c_long,
//     pub ru_nswap: libc::c_long,
//     pub ru_inblock: libc::c_long,
//     pub ru_oublock: libc::c_long,
//     pub ru_msgsnd: libc::c_long,
//     pub ru_msgrcv: libc::c_long,
//     pub ru_nsignals: libc::c_long,
//     pub ru_nvcsw: libc::c_long,
//     pub ru_nivcsw: libc::c_long,
// }
// pub type rusage_info_t = *mut libc::c_void;
// #[repr(C)]
// pub struct Struct_rusage_info_v0 {
//     pub ri_uuid: [uint8_t, ..16u],
//     pub ri_user_time: uint64_t,
//     pub ri_system_time: uint64_t,
//     pub ri_pkg_idle_wkups: uint64_t,
//     pub ri_interrupt_wkups: uint64_t,
//     pub ri_pageins: uint64_t,
//     pub ri_wired_size: uint64_t,
//     pub ri_resident_size: uint64_t,
//     pub ri_phys_footprint: uint64_t,
//     pub ri_proc_start_abstime: uint64_t,
//     pub ri_proc_exit_abstime: uint64_t,
// }
// #[repr(C)]
// pub struct Struct_rusage_info_v1 {
//     pub ri_uuid: [uint8_t, ..16u],
//     pub ri_user_time: uint64_t,
//     pub ri_system_time: uint64_t,
//     pub ri_pkg_idle_wkups: uint64_t,
//     pub ri_interrupt_wkups: uint64_t,
//     pub ri_pageins: uint64_t,
//     pub ri_wired_size: uint64_t,
//     pub ri_resident_size: uint64_t,
//     pub ri_phys_footprint: uint64_t,
//     pub ri_proc_start_abstime: uint64_t,
//     pub ri_proc_exit_abstime: uint64_t,
//     pub ri_child_user_time: uint64_t,
//     pub ri_child_system_time: uint64_t,
//     pub ri_child_pkg_idle_wkups: uint64_t,
//     pub ri_child_interrupt_wkups: uint64_t,
//     pub ri_child_pageins: uint64_t,
//     pub ri_child_elapsed_abstime: uint64_t,
// }
// #[repr(C)]
// pub struct Struct_rusage_info_v2 {
//     pub ri_uuid: [uint8_t, ..16u],
//     pub ri_user_time: uint64_t,
//     pub ri_system_time: uint64_t,
//     pub ri_pkg_idle_wkups: uint64_t,
//     pub ri_interrupt_wkups: uint64_t,
//     pub ri_pageins: uint64_t,
//     pub ri_wired_size: uint64_t,
//     pub ri_resident_size: uint64_t,
//     pub ri_phys_footprint: uint64_t,
//     pub ri_proc_start_abstime: uint64_t,
//     pub ri_proc_exit_abstime: uint64_t,
//     pub ri_child_user_time: uint64_t,
//     pub ri_child_system_time: uint64_t,
//     pub ri_child_pkg_idle_wkups: uint64_t,
//     pub ri_child_interrupt_wkups: uint64_t,
//     pub ri_child_pageins: uint64_t,
//     pub ri_child_elapsed_abstime: uint64_t,
//     pub ri_diskio_bytesread: uint64_t,
//     pub ri_diskio_byteswritten: uint64_t,
// }
// #[repr(C)]
// pub struct Struct_rusage_info_v3 {
//     pub ri_uuid: [uint8_t, ..16u],
//     pub ri_user_time: uint64_t,
//     pub ri_system_time: uint64_t,
//     pub ri_pkg_idle_wkups: uint64_t,
//     pub ri_interrupt_wkups: uint64_t,
//     pub ri_pageins: uint64_t,
//     pub ri_wired_size: uint64_t,
//     pub ri_resident_size: uint64_t,
//     pub ri_phys_footprint: uint64_t,
//     pub ri_proc_start_abstime: uint64_t,
//     pub ri_proc_exit_abstime: uint64_t,
//     pub ri_child_user_time: uint64_t,
//     pub ri_child_system_time: uint64_t,
//     pub ri_child_pkg_idle_wkups: uint64_t,
//     pub ri_child_interrupt_wkups: uint64_t,
//     pub ri_child_pageins: uint64_t,
//     pub ri_child_elapsed_abstime: uint64_t,
//     pub ri_diskio_bytesread: uint64_t,
//     pub ri_diskio_byteswritten: uint64_t,
//     pub ri_cpu_time_qos_default: uint64_t,
//     pub ri_cpu_time_qos_maintenance: uint64_t,
//     pub ri_cpu_time_qos_background: uint64_t,
//     pub ri_cpu_time_qos_utility: uint64_t,
//     pub ri_cpu_time_qos_legacy: uint64_t,
//     pub ri_cpu_time_qos_user_initiated: uint64_t,
//     pub ri_cpu_time_qos_user_interactive: uint64_t,
//     pub ri_billed_system_time: uint64_t,
//     pub ri_serviced_system_time: uint64_t,
// }
// pub type rusage_info_current = Struct_rusage_info_v3;
// #[repr(C)]
// pub struct Struct_rlimit {
//     pub rlim_cur: rlim_t,
//     pub rlim_max: rlim_t,
// }
// #[repr(C)]
// pub struct Struct_proc_rlimit_control_wakeupmon {
//     pub wm_flags: uint32_t,
//     pub wm_rate: int32_t,
// }
// #[repr(C)]
// pub struct Union_wait {
//     pub data: [u32, ..1u],
// }
// impl Union_wait {
//     pub fn w_status(&mut self) -> *mut libc::c_int {
//         unsafe { ::std::mem::transmute(self) }
//     }
//     pub fn w_T(&mut self) -> *mut Struct_Unnamed3 {
//         unsafe { ::std::mem::transmute(self) }
//     }
//     pub fn w_S(&mut self) -> *mut Struct_Unnamed4 {
//         unsafe { ::std::mem::transmute(self) }
//     }
// }
// #[repr(C)]
// pub struct Struct_Unnamed3 {
//     pub w_Termsig: libc::c_uint,
//     pub w_Coredump: libc::c_uint,
//     pub w_Retcode: libc::c_uint,
//     pub w_Filler: libc::c_uint,
// }
// #[repr(C)]
// pub struct Struct_Unnamed4 {
//     pub w_Stopval: libc::c_uint,
//     pub w_Stopsig: libc::c_uint,
//     pub w_Filler: libc::c_uint,
// }
// pub type ct_rune_t = __darwin_ct_rune_t;
// pub type rune_t = __darwin_rune_t;
// pub type wchar_t = __darwin_wchar_t;
// #[repr(C)]
// pub struct Struct_Unnamed5 {
//     pub quot: libc::c_int,
//     pub rem: libc::c_int,
// }
// pub type div_t = Struct_Unnamed5;
// #[repr(C)]
// pub struct Struct_Unnamed6 {
//     pub quot: libc::c_long,
//     pub rem: libc::c_long,
// }
// pub type ldiv_t = Struct_Unnamed6;
// #[repr(C)]
// pub struct Struct_Unnamed7 {
//     pub quot: libc::c_longlong,
//     pub rem: libc::c_longlong,
// }
// pub type lldiv_t = Struct_Unnamed7;
pub type u_int8_t = libc::c_uchar;
pub type u_int16_t = libc::c_ushort;
pub type u_int32_t = libc::c_uint;
pub type u_int64_t = libc::c_ulonglong;
pub type register_t = int64_t;
pub type user_addr_t = u_int64_t;
pub type user_size_t = u_int64_t;
pub type user_ssize_t = int64_t;
pub type user_long_t = int64_t;
pub type user_ulong_t = u_int64_t;
pub type user_time_t = int64_t;
pub type user_off_t = int64_t;
pub type syscall_arg_t = u_int64_t;
pub type fpos_t = __darwin_off_t;
#[repr(C)]
pub struct Struct___sbuf {
    pub _base: *mut libc::c_uchar,
    pub _size: libc::c_int,
}

pub enum Struct___sFILEX { }


#[repr(C)]
pub struct Struct___sFILE {
    pub _p: *mut libc::c_uchar,
    pub _r: libc::c_int,
    pub _w: libc::c_int,
    pub _flags: libc::c_short,
    pub _file: libc::c_short,
    pub _bf: Struct___sbuf,
    pub _lbfsize: libc::c_int,
    pub _cookie: *mut libc::c_void,
    pub _close: ::std::option::Option<extern "C" fn(arg1: *mut libc::c_void)
                                          -> libc::c_int>,
    pub _read: ::std::option::Option<extern "C" fn
                                         (arg1: *mut libc::c_void,
                                          arg2: *mut libc::c_char,
                                          arg3: libc::c_int)
                                         -> libc::c_int>,
    pub _seek: ::std::option::Option<extern "C" fn
                                         (arg1: *mut libc::c_void,
                                          arg2: fpos_t, arg3: libc::c_int)
                                         -> fpos_t>,
    pub _write: ::std::option::Option<extern "C" fn
                                          (arg1: *mut libc::c_void,
                                           arg2: *const libc::c_char,
                                           arg3: libc::c_int)
                                          -> libc::c_int>,
    pub _ub: Struct___sbuf,
    pub _extra: *mut Struct___sFILEX,
    pub _ur: libc::c_int,
    pub _ubuf: [libc::c_uchar; 3],
    pub _nbuf: [libc::c_uchar; 1],
    pub _lb: Struct___sbuf,
    pub _blksize: libc::c_int,
    pub _offset: fpos_t,
}

pub type FILE = Struct___sFILE;
// pub type errno_t = libc::c_int;
// pub type rsize_t = __darwin_size_t;
// pub type wint_t = __darwin_wint_t;
// #[repr(C)]
// pub struct Struct_Unnamed8 {
//     pub __min: __darwin_rune_t,
//     pub __max: __darwin_rune_t,
//     pub __map: __darwin_rune_t,
//     pub __types: *mut __uint32_t,
// }
// pub type _RuneEntry = Struct_Unnamed8;
// #[repr(C)]
// pub struct Struct_Unnamed9 {
//     pub __nranges: libc::c_int,
//     pub __ranges: *mut _RuneEntry,
// }
// pub type _RuneRange = Struct_Unnamed9;
#[repr(C)]
pub struct Struct_Unnamed10 {
    pub __name: [libc::c_char; 14],
    pub __mask: __uint32_t,
}

pub type _RuneCharClass = Struct_Unnamed10;
// #[repr(C)]
// pub struct Struct_Unnamed11 {
//     pub __magic: [libc::c_char, ..8u],
//     pub __encoding: [libc::c_char, ..32u],
//     pub __sgetrune: ::std::option::Option<extern "C" fn
//                                               (arg1: *const libc::c_char,
//                                                arg2: __darwin_size_t,
//                                                arg3:
//                                                    *mut *const libc::c_char)
//                                               -> __darwin_rune_t>,
//     pub __sputrune: ::std::option::Option<extern "C" fn
//                                               (arg1: __darwin_rune_t,
//                                                arg2: *mut libc::c_char,
//                                                arg3: __darwin_size_t,
//                                                arg4: *mut *mut libc::c_char)
//                                               -> libc::c_int>,
//     pub __invalid_rune: __darwin_rune_t,
//     pub __runetype: [__uint32_t, ..256u],
//     pub __maplower: [__darwin_rune_t, ..256u],
//     pub __mapupper: [__darwin_rune_t, ..256u],
//     pub __runetype_ext: _RuneRange,
//     pub __maplower_ext: _RuneRange,
//     pub __mapupper_ext: _RuneRange,
//     pub __variable: *mut libc::c_void,
//     pub __variable_len: libc::c_int,
//     pub __ncharclasses: libc::c_int,
//     pub __charclasses: *mut _RuneCharClass,
// }
// pub type _RuneLocale = Struct_Unnamed11;
pub type u_char = libc::c_uchar;
pub type u_short = libc::c_ushort;
pub type u_int = libc::c_uint;
pub type u_long = libc::c_ulong;
pub type ushort = libc::c_ushort;
pub type _uint = libc::c_uint;
pub type u_quad_t = u_int64_t;
pub type quad_t = int64_t;
pub type qaddr_t = *mut quad_t;
pub type caddr_t = *mut libc::c_char;
pub type daddr_t = int32_t;
pub type fixpt_t = u_int32_t;
pub type blkcnt_t = __darwin_blkcnt_t;
pub type blksize_t = __darwin_blksize_t;
pub type in_addr_t = __uint32_t;
pub type in_port_t = __uint16_t;
pub type ino_t = __darwin_ino_t;
pub type ino64_t = __darwin_ino64_t;
pub type key_t = __int32_t;
pub type nlink_t = __uint16_t;
pub type segsz_t = int32_t;
pub type swblk_t = int32_t;
pub type clock_t = __darwin_clock_t;
pub type fd_mask = __int32_t;
// pub type pthread_cond_t = __darwin_pthread_cond_t;
// pub type pthread_condattr_t = __darwin_pthread_condattr_t;
// pub type pthread_mutex_t = __darwin_pthread_mutex_t;
// pub type pthread_mutexattr_t = __darwin_pthread_mutexattr_t;
// pub type pthread_once_t = __darwin_pthread_once_t;
// pub type pthread_rwlock_t = __darwin_pthread_rwlock_t;
// pub type pthread_rwlockattr_t = __darwin_pthread_rwlockattr_t;
// pub type pthread_t = __darwin_pthread_t;
// pub type pthread_key_t = __darwin_pthread_key_t;
// pub type fsblkcnt_t = __darwin_fsblkcnt_t;
// pub type fsfilcnt_t = __darwin_fsfilcnt_t;
#[repr(C)]
pub struct Struct_ttysize {
    pub ts_lines: libc::c_ushort,
    pub ts_cols: libc::c_ushort,
    pub ts_xxx: libc::c_ushort,
    pub ts_yyy: libc::c_ushort,
}

pub type __darwin_size_t = libc::c_ulong;
pub type size_t = __darwin_size_t;

#[repr(C)]
#[allow(missing_copy_implementations)]
pub struct Struct_linenoiseCompletions {
    pub len: size_t,
    pub cvec: *mut *mut libc::c_char,
}

pub type linenoiseCompletionCallback = extern fn(*mut libc::c_char, *mut Struct_linenoiseCompletions);

pub type linenoiseCompletions = Struct_linenoiseCompletions;
// pub type linenoiseCompletionCallback = libc::c_void;
extern "C" {
    pub static mut optarg: *mut libc::c_char;
    pub static mut optind: libc::c_int;
    pub static mut opterr: libc::c_int;
    pub static mut optopt: libc::c_int;
    pub static mut suboptarg: *mut libc::c_char;
    pub static mut optreset: libc::c_int;
    pub static mut __mb_cur_max: libc::c_int;
    pub static mut __stdinp: *mut FILE;
    pub static mut __stdoutp: *mut FILE;
    pub static mut __stderrp: *mut FILE;
    pub static sys_nerr: libc::c_int;
    pub static mut sys_errlist: *const *const libc::c_char;
    // pub static mut _DefaultRuneLocale: _RuneLocale;
    // pub static mut _CurrentRuneLocale: *mut _RuneLocale;
}
extern "C" {
    pub fn linenoiseSetCompletionCallback(arg1:
                                              *mut linenoiseCompletionCallback);
    pub fn linenoiseAddCompletion(arg1: *mut linenoiseCompletions,
                                  arg2: *const libc::c_char);
    pub fn linenoise(prompt: *const libc::c_char) -> *mut libc::c_char;
    pub fn linenoiseHistoryAdd(line: *const libc::c_char) -> libc::c_int;
    pub fn linenoiseHistorySetMaxLen(len: libc::c_int) -> libc::c_int;
    pub fn linenoiseHistorySave(filename: *const libc::c_char)
     -> libc::c_int;
    pub fn linenoiseHistoryLoad(filename: *const libc::c_char)
     -> libc::c_int;
    pub fn linenoiseClearScreen();
    pub fn linenoiseSetMultiLine(ml: libc::c_int);
    pub fn linenoisePrintKeyCodes();
}
