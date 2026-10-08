#[repr(C)]
pub struct toks_gen { _opaque: [u8; 0] }
#[repr(C)]
pub struct toks_uni { _opaque: [u8; 0] }
#[repr(C)]
pub struct toks_wp_tables { _opaque: [u8; 0] }
#[repr(C)]
pub struct toks_spm_op { _opaque: [u8; 0] }
#[repr(C)]
pub struct toks_spm { _opaque: [u8; 0] }
use ::libc;
#[cfg(target_arch = "x86")]
pub use ::core::arch::x86::_mm_pause;
#[cfg(target_arch = "x86_64")]
pub use ::core::arch::x86_64::_mm_pause;
extern "C" {
    fn toks_scratch_bytes(
        ctx: *const toks_ctx,
        max_len: uint64_t,
        flags: uint32_t,
    ) -> uint64_t;
    fn toks_scratch_init(
        ctx: *const toks_ctx,
        scr: *mut ::core::ffi::c_void,
        bytes: uint64_t,
        flags: uint32_t,
    ) -> int64_t;
    fn toks_encode(
        ctx: *const toks_ctx,
        text: *const ::core::ffi::c_void,
        len: uint64_t,
        flags: uint32_t,
        out: *mut uint32_t,
        cap: uint64_t,
        scr: *mut ::core::ffi::c_void,
    ) -> int64_t;
    fn toks_split_points(
        ctx: *const toks_ctx,
        text: *const ::core::ffi::c_void,
        len: uint64_t,
        flags: uint32_t,
        n_want: uint32_t,
        offs: *mut uint64_t,
        cap: uint64_t,
        scr: *mut ::core::ffi::c_void,
    ) -> int64_t;
    fn memcpy(
        dst: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        dst: *mut ::core::ffi::c_void,
        c: ::core::ffi::c_int,
        n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn toks_plat_alloc(n: uint64_t) -> *mut ::core::ffi::c_void;
    fn toks_plat_free(p: *mut ::core::ffi::c_void, n: uint64_t);
    fn toks_plat_arena(n: uint64_t) -> *mut uint8_t;
    fn toks_plat_arena_free(p: *mut uint8_t, n: uint64_t);
    fn toks_plat_getenv(
        name: *const ::core::ffi::c_char,
        buf: *mut ::core::ffi::c_char,
        cap: uint64_t,
    ) -> int64_t;
    fn toks_pp_ids(
        ctx: *const toks_ctx,
        flags: uint32_t,
        pre: *mut *const uint32_t,
        n_pre: *mut uint32_t,
        suf: *mut *const uint32_t,
        n_suf: *mut uint32_t,
    );
    fn __sched_cpucount(
        __setsize: size_t,
        __setp: *const cpu_set_t,
    ) -> ::core::ffi::c_int;
    fn sched_yield() -> ::core::ffi::c_int;
    fn sched_getaffinity(
        __pid: __pid_t,
        __cpusetsize: size_t,
        __cpuset: *mut cpu_set_t,
    ) -> ::core::ffi::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_attr_init(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    fn pthread_attr_destroy(__attr: *mut pthread_attr_t) -> ::core::ffi::c_int;
    fn pthread_attr_setaffinity_np(
        __attr: *mut pthread_attr_t,
        __cpusetsize: size_t,
        __cpuset: *const cpu_set_t,
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __nbytes: size_t,
    ) -> ssize_t;
    fn sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn syscall(__sysno: ::core::ffi::c_long, ...) -> ::core::ffi::c_long;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_ctx {
    pub t: toks_tables,
    pub tier: uint32_t,
    pub dec_byte_level: uint32_t,
    pub pp_ids: [uint32_t; 64],
    pub n_pp_prefix: uint32_t,
    pub n_pp_suffix: uint32_t,
    pub n_nonspecial: uint32_t,
    pub dec_max: uint32_t,
    pub identity: uint64_t,
    pub cpu_features: uint64_t,
    pub special_ids: *mut uint32_t,
    pub dec_slot: *mut uint8_t,
    pub dec_len: *mut uint8_t,
    pub voc_slots: *const uint32_t,
    pub voc_mask: uint64_t,
    pub voc_add: *const uint32_t,
    pub voc_pool: *const uint8_t,
    pub voc_added: *const uint32_t,
    pub voc_special: *const uint32_t,
    pub voc_n_add: uint32_t,
    pub voc_bf: uint32_t,
    pub mem_voc: *mut uint8_t,
    pub mem_voc_len: uint64_t,
    pub mem_tables: *mut uint8_t,
    pub mem_tables_len: uint64_t,
    pub mem_bpe: *mut uint8_t,
    pub mem_bpe_len: uint64_t,
    pub name: [::core::ffi::c_char; 64],
    pub source_sha256: [uint8_t; 32],
    pub nfc: uint32_t,
    pub spm: *const toks_spm,
    pub mem_spm: *mut uint8_t,
    pub mem_spm_len: uint64_t,
    pub dc: toks_dchain,
    pub cut_chunk: uint32_t,
    pub cut_run: uint32_t,
    pub has_drop: uint32_t,
    pub drop: [uint8_t; 32],
    pub drop_unk: uint32_t,
    pub drop_fuse: uint32_t,
    pub wp: *const toks_wp_tables,
    pub mem_wp: *mut uint8_t,
    pub mem_wp_len: uint64_t,
    pub scr_extra: uint64_t,
    pub wp_mat_cap: uint64_t,
    pub o: toks_opts,
    pub uni: *const toks_uni,
    pub mem_uni: *mut uint8_t,
    pub mem_uni_len: uint64_t,
    pub gen: *const toks_gen,
    pub mem_gen: *mut uint8_t,
    pub mem_gen_len: uint64_t,
    pub bound_num: uint32_t,
    pub bound_den: uint32_t,
    pub bound_g: uint32_t,
    pub bound_rsv: uint32_t,
    pub pp_type: [uint32_t; 64],
    pub pp_seq_type: uint32_t,
    pub voc_n_dec: uint32_t,
    pub voc_dec: *const uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_opts {
    pub trunc_on: uint32_t,
    pub trunc_max: uint32_t,
    pub trunc_stride: uint32_t,
    pub pad_on: uint32_t,
    pub pad_fixed: uint32_t,
    pub pad_len: uint32_t,
    pub pad_multiple: uint32_t,
    pub pad_id: uint32_t,
    pub pad_left: uint32_t,
    pub wp_win: uint32_t,
    pub dec_wordpiece: uint32_t,
    pub dec_cleanup: uint32_t,
    pub dec_prefix_len: uint32_t,
    pub dec_prefix: [uint8_t; 16],
    pub pad_file: uint32_t,
    pub pad_type_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_dchain {
    pub dec: *const toks_spm_op,
    pub n_dec: uint32_t,
    pub has_decoder: uint32_t,
    pub bf_first: uint32_t,
    pub on: uint32_t,
    pub holes: *const uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_tables {
    pub magic: uint32_t,
    pub version: uint32_t,
    pub algo: uint32_t,
    pub tmpl: uint32_t,
    pub tmpl_params: uint32_t,
    pub flags: uint32_t,
    pub n_ids: uint32_t,
    pub n_merges: uint32_t,
    pub cls_ascii: *const uint8_t,
    pub cls_stage1: *const uint16_t,
    pub cls_stage2: *const uint8_t,
    pub cls_nblocks: uint64_t,
    pub byte2id: *const uint32_t,
    pub bytepair: *const uint32_t,
    pub merge_slots: *const uint64_t,
    pub merge_mask: uint64_t,
    pub merge_shift: uint64_t,
    pub merge_maxprobe: uint64_t,
    pub rank2id: *const uint32_t,
    pub premerge: *const uint8_t,
    pub words: *const uint8_t,
    pub words_mask: uint64_t,
    pub vhash: *const uint64_t,
    pub vhash_mask: uint64_t,
    pub tok_off: *const uint32_t,
    pub tok_bytes: *const uint8_t,
    pub add_shufti: *const uint8_t,
    pub add_index: *const uint64_t,
    pub add_single: *const uint32_t,
    pub add_cand: *const uint32_t,
    pub add_entries: *const toks_added_entry,
    pub add_bytes: *const uint8_t,
    pub add_n: uint64_t,
    pub add_phases: uint64_t,
    pub apm: *const uint8_t,
    pub pairf: *const uint64_t,
    pub rsv: [uint64_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_added_entry {
    pub off: uint32_t,
    pub len: uint16_t,
    pub flags: uint8_t,
    pub phase: uint8_t,
    pub id: uint32_t,
    pub rsv: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C, align(128))]
pub struct toks_par {
    pub busy: line32,
    pub open: line32,
    pub active: line32,
    pub seq: line32,
    pub quit: line32,
    pub next: line64,
    pub fin: line64,
    pub ctx: *const toks_ctx,
    pub n: uint32_t,
    pub n_fast: uint32_t,
    pub eager: uint32_t,
    pub last_k: uint32_t,
    pub flags: uint32_t,
    pub scr_flags: uint32_t,
    pub spin_ns: uint64_t,
    pub c_ps: uint64_t,
    pub c_ser: uint64_t,
    pub o_wake: uint64_t,
    pub o_join: uint64_t,
    pub lat_wake: uint64_t,
    pub wake0: uint64_t,
    pub join0: uint64_t,
    pub call_bytes: uint64_t,
    pub eps: uint64_t,
    pub tail_len: uint64_t,
    pub t_go: uint64_t,
    pub t_last: uint64_t,
    pub slots: *mut slot,
    pub thr: *mut thr_t,
    pub mem: *mut uint8_t,
    pub mem_bytes: uint64_t,
    pub items: *mut toks_par_item,
    pub n_items: uint64_t,
    pub pre: *const uint32_t,
    pub suf: *const uint32_t,
    pub n_pre: uint32_t,
    pub n_suf: uint32_t,
    pub units: *mut unit,
    pub n_units: uint64_t,
    pub units_cap: uint64_t,
    pub parts: *mut part,
    pub n_parts: uint64_t,
    pub parts_cap: uint64_t,
    pub cuts: *mut uint64_t,
    pub cuts_cap: uint64_t,
    pub stage: *mut uint32_t,
    pub stage_ids: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct part {
    pub item: uint64_t,
    pub a: uint64_t,
    pub len: uint64_t,
    pub slab: uint64_t,
    pub nxt: uint64_t,
    pub cnt: int64_t,
    pub first: uint32_t,
    pub done: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unit {
    pub a: uint64_t,
    pub b: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_par_item {
    pub text: *const ::core::ffi::c_void,
    pub len: uint64_t,
    pub out: *mut uint32_t,
    pub cap: uint64_t,
    pub n: int64_t,
}
pub type thr_t = pthread_t;
pub type pthread_t = libc::pthread_t;
#[derive(Copy, Clone)]
#[repr(C, align(128))]
pub struct slot {
    pub word: uint32_t,
    pub asleep: uint32_t,
    _pad: [u8; 120],
    pub scr: *mut uint8_t,
    pub scr_bytes: uint64_t,
    pub scr_len: uint64_t,
    pub t_join: uint64_t,
    pub busy_ns: uint64_t,
    pub busy_bytes: uint64_t,
    pub grew: uint32_t,
    pub joined: uint32_t,
    pub was_asleep: uint32_t,
    pub par: *mut toks_par,
    pub id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C, align(128))]
pub struct line64 {
    pub v: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C, align(128))]
pub struct line32 {
    pub v: uint32_t,
}
pub const memory_order_relaxed: memory_order = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
pub const memory_order_acquire: memory_order = 2;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct topo {
    pub n_cpu: uint32_t,
    pub n_fast: uint32_t,
    pub fast: cpu_set_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16],
}
pub type __cpu_mask = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cursor {
    pub k: uint64_t,
    pub sum: uint64_t,
    pub head: uint64_t,
    pub tail: uint64_t,
    pub bad: ::core::ffi::c_int,
}
pub const memory_order_release: memory_order = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
pub type ssize_t = __ssize_t;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct toks_par_info {
    pub size: uint32_t,
    pub threads: uint32_t,
    pub fast: uint32_t,
    pub last: uint32_t,
    pub ns_per_mib: uint64_t,
    pub wake_ns: uint64_t,
    pub join_ns: uint64_t,
    pub min_bytes: uint64_t,
}
pub type memory_order = ::core::ffi::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_consume: memory_order = 1;
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _SC_SIGSTKSZ: C2RustUnnamed = 250;
pub const _SC_MINSIGSTKSZ: C2RustUnnamed = 249;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed = 236;
pub const _SC_IPV6: C2RustUnnamed = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed = 182;
pub const _SC_TRACE: C2RustUnnamed = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed = 175;
pub const _SC_STREAMS: C2RustUnnamed = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed = 169;
pub const _SC_2_PBS: C2RustUnnamed = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed = 160;
pub const _SC_SPAWN: C2RustUnnamed = 159;
pub const _SC_SIGNALS: C2RustUnnamed = 158;
pub const _SC_SHELL: C2RustUnnamed = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed = 156;
pub const _SC_REGEXP: C2RustUnnamed = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed = 153;
pub const _SC_NETWORKING: C2RustUnnamed = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed = 146;
pub const _SC_PIPE: C2RustUnnamed = 145;
pub const _SC_FIFO: C2RustUnnamed = 144;
pub const _SC_FD_MGMT: C2RustUnnamed = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed = 139;
pub const _SC_CPUTIME: C2RustUnnamed = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed = 135;
pub const _SC_BASE: C2RustUnnamed = 134;
pub const _SC_BARRIERS: C2RustUnnamed = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed = 123;
pub const _SC_NL_NMAX: C2RustUnnamed = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed = 117;
pub const _SC_UINT_MAX: C2RustUnnamed = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed = 110;
pub const _SC_NZERO: C2RustUnnamed = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed = 108;
pub const _SC_WORD_BIT: C2RustUnnamed = 107;
pub const _SC_LONG_BIT: C2RustUnnamed = 106;
pub const _SC_INT_MIN: C2RustUnnamed = 105;
pub const _SC_INT_MAX: C2RustUnnamed = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed = 98;
pub const _SC_2_UPE: C2RustUnnamed = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed = 89;
pub const _SC_PASS_MAX: C2RustUnnamed = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed = 85;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed = 68;
pub const _SC_THREADS: C2RustUnnamed = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed = 61;
pub const _SC_IOV_MAX: C2RustUnnamed = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed = 60;
pub const _SC_SELECT: C2RustUnnamed = 59;
pub const _SC_POLL: C2RustUnnamed = 58;
pub const _SC_PII_OSI: C2RustUnnamed = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed = 55;
pub const _SC_PII_XTI: C2RustUnnamed = 54;
pub const _SC_PII: C2RustUnnamed = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed = 49;
pub const _SC_2_C_DEV: C2RustUnnamed = 48;
pub const _SC_2_C_BIND: C2RustUnnamed = 47;
pub const _SC_2_VERSION: C2RustUnnamed = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed = 44;
pub const _SC_LINE_MAX: C2RustUnnamed = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed = 31;
pub const _SC_PAGESIZE: C2RustUnnamed = 30;
pub const _SC_VERSION: C2RustUnnamed = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed = 25;
pub const _SC_AIO_MAX: C2RustUnnamed = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed = 18;
pub const _SC_MEMLOCK: C2RustUnnamed = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed = 16;
pub const _SC_FSYNC: C2RustUnnamed = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed = 12;
pub const _SC_TIMERS: C2RustUnnamed = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed = 3;
pub const _SC_CLK_TCK: C2RustUnnamed = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed = 1;
pub const _SC_ARG_MAX: C2RustUnnamed = 0;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615
    as ::core::ffi::c_ulong;
pub const TOKS_E_ARG: ::core::ffi::c_int = -(10 as ::core::ffi::c_int);
pub const TOKS_E_NOMEM: ::core::ffi::c_int = -(11 as ::core::ffi::c_int);
pub const TOKS_MAX_TEXT: ::core::ffi::c_ulonglong = (1 as ::core::ffi::c_ulonglong)
    << 29 as ::core::ffi::c_int;
pub const TOKS_SCRATCH_MEMO_SET: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
    << 20 as ::core::ffi::c_int;
pub const TOKS_ADDED_MASK: ::core::ffi::c_uint = 3 as ::core::ffi::c_uint;
pub const TOKS_NO_POSTPROCESS: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const TOKS_CONTINUATION: ::core::ffi::c_uint = 8 as ::core::ffi::c_uint;
pub const TOKS_NO_TRUNCATE: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
pub const TOKS_NO_PAD: ::core::ffi::c_uint = 32 as ::core::ffi::c_uint;
#[inline]
unsafe extern "C" fn toks_st32(mut p: *mut ::core::ffi::c_void, mut v: uint32_t) {
    memcpy(p, &raw mut v as *const ::core::ffi::c_void, 4 as size_t);
}
#[inline]
unsafe extern "C" fn toks_cpy(
    mut dst: *mut ::core::ffi::c_void,
    mut src: *const ::core::ffi::c_void,
    mut n: uint64_t,
) {
    memcpy(dst, src, n as size_t);
}
#[inline]
unsafe extern "C" fn toks_scr_cache_mib(mut flags: uint32_t) -> uint64_t {
    return flags as uint64_t >> 12 as ::core::ffi::c_int & 0xff as uint64_t;
}
#[inline]
unsafe extern "C" fn toks_scr_cache_ok(mut n: uint64_t) -> ::core::ffi::c_int {
    return (n == 0 as uint64_t
        || n >= 4 as uint64_t && n <= 128 as uint64_t
            && n & n.wrapping_sub(1 as uint64_t) == 0 as uint64_t) as ::core::ffi::c_int;
}
pub const __CPU_SETSIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const __NCPUBITS: usize = (8 as usize)
    .wrapping_mul(::core::mem::size_of::<__cpu_mask>() as usize);
pub const CPU_SETSIZE: ::core::ffi::c_int = __CPU_SETSIZE;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const __NR_futex: ::core::ffi::c_int = 202 as ::core::ffi::c_int;
pub const SYS_futex: ::core::ffi::c_int = __NR_futex;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const FUTEX_WAIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FUTEX_WAKE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FUTEX_PRIVATE_FLAG: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const FUTEX_WAIT_PRIVATE: ::core::ffi::c_int = FUTEX_WAIT | FUTEX_PRIVATE_FLAG;
pub const FUTEX_WAKE_PRIVATE: ::core::ffi::c_int = FUTEX_WAKE | FUTEX_PRIVATE_FLAG;
pub const PAR_DEFAULT_N: ::core::ffi::c_uint = 4 as ::core::ffi::c_uint;
pub const PAR_MAX_N: ::core::ffi::c_uint = 1024 as ::core::ffi::c_uint;
pub const PAR_MIN_BYTES: ::core::ffi::c_uint = (16 as ::core::ffi::c_uint)
    << 10 as ::core::ffi::c_int;
pub const PAR_MIN_UNIT: ::core::ffi::c_uint = (8 as ::core::ffi::c_uint)
    << 10 as ::core::ffi::c_int;
pub const PAR_MAX_UNIT: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
    << 20 as ::core::ffi::c_int;
pub const PAR_UNITS_PER: ::core::ffi::c_uint = 8 as ::core::ffi::c_uint;
pub const PAR_EFF: ::core::ffi::c_uint = 768 as ::core::ffi::c_uint;
pub const PAR_UNIT_NS: ::core::ffi::c_uint = 200 as ::core::ffi::c_uint;
pub const PAR_SCR_MIN: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
    << 20 as ::core::ffi::c_int;
pub const PAR_C0: ::core::ffi::c_uint = 2500 as ::core::ffi::c_uint;
pub const PAR_EPS0: ::core::ffi::c_uint = 102 as ::core::ffi::c_uint;
pub const PAR_WAKE0: ::core::ffi::c_uint = 30000 as ::core::ffi::c_uint;
pub const PAR_JOIN0: ::core::ffi::c_uint = 10000 as ::core::ffi::c_uint;
pub const PAR_SPIN_LO: ::core::ffi::c_uint = 10000 as ::core::ffi::c_uint;
pub const PAR_SPIN_HI: ::core::ffi::c_uint = 200000 as ::core::ffi::c_uint;
pub const PAR_LINE: ::core::ffi::c_uint = 128 as ::core::ffi::c_uint;
pub const PAR_OVERFLOW: ::core::ffi::c_int = -(1000 as ::core::ffi::c_int);
pub const PAR_PART: ::core::ffi::c_ulong = UINT64_MAX;
unsafe extern "C" fn relax() {
    std::hint::spin_loop();
}
unsafe extern "C" fn now_ns() -> uint64_t {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(libc::CLOCK_MONOTONIC as i32, &raw mut ts);
    return (ts.tv_sec as uint64_t)
        .wrapping_mul(1000000000 as uint64_t)
        .wrapping_add(ts.tv_nsec as uint64_t);
}
#[cfg(target_os = "linux")]
unsafe extern "C" fn addr_wait(mut a: *mut uint32_t, mut v: uint32_t) {
    syscall(
        libc::SYS_futex as ::core::ffi::c_long,
        a as *mut uint32_t,
        FUTEX_WAIT_PRIVATE,
        v,
        NULL,
        NULL,
        0 as ::core::ffi::c_int,
    );
}
#[cfg(target_os = "linux")]
unsafe extern "C" fn addr_wake(mut a: *mut uint32_t) {
    syscall(
        libc::SYS_futex as ::core::ffi::c_long,
        a as *mut uint32_t,
        FUTEX_WAKE_PRIVATE,
        1 as ::core::ffi::c_int,
        NULL,
        NULL,
        0 as ::core::ffi::c_int,
    );
}
#[cfg(target_os = "linux")]
unsafe extern "C" fn sys_num(
    mut cpu: uint32_t,
    mut leaf: *const ::core::ffi::c_char,
) -> uint64_t {
    static mut pfx: [::core::ffi::c_char; 28] = unsafe {
        ::core::mem::transmute::<
            [u8; 28],
            [::core::ffi::c_char; 28],
        >(*b"/sys/devices/system/cpu/cpu\0")
    };
    let mut path: [::core::ffi::c_char; 96] = [0; 96];
    let mut b: [::core::ffi::c_char; 32] = [0; 32];
    let mut n: uint32_t = 0 as uint32_t;
    let mut d: uint32_t = 1 as uint32_t;
    let mut q: *const ::core::ffi::c_char = &raw const pfx as *const ::core::ffi::c_char;
    while *q as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        let fresh1 = n;
        n = n.wrapping_add(1);
        path[fresh1 as usize] = *q;
        q = q.offset(1);
    }
    while d.wrapping_mul(10 as uint32_t) <= cpu {
        d = (d as ::core::ffi::c_uint).wrapping_mul(10 as ::core::ffi::c_uint)
            as uint32_t as uint32_t;
    }
    while d != 0 as uint32_t {
        let fresh2 = n;
        n = n.wrapping_add(1);
        path[fresh2 as usize] = ('0' as i32 as uint32_t)
            .wrapping_add(cpu.wrapping_div(d).wrapping_rem(10 as uint32_t))
            as ::core::ffi::c_char;
        d = (d as ::core::ffi::c_uint).wrapping_div(10 as ::core::ffi::c_uint)
            as uint32_t as uint32_t;
    }
    let fresh3 = n;
    n = n.wrapping_add(1);
    path[fresh3 as usize] = '/' as i32 as ::core::ffi::c_char;
    let mut q_0: *const ::core::ffi::c_char = leaf;
    while *q_0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        && (n.wrapping_add(1 as uint32_t) as usize)
            < ::core::mem::size_of::<[::core::ffi::c_char; 96]>() as usize
    {
        let fresh4 = n;
        n = n.wrapping_add(1);
        path[fresh4 as usize] = *q_0;
        q_0 = q_0.offset(1);
    }
    path[n as usize] = 0 as ::core::ffi::c_char;
    let mut fd: ::core::ffi::c_int = open(
        &raw mut path as *mut ::core::ffi::c_char,
        O_RDONLY | O_CLOEXEC,
    );
    if fd < 0 as ::core::ffi::c_int {
        return 0 as uint64_t;
    }
    let mut r: ssize_t = read(
        fd,
        &raw mut b as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 32]>() as size_t)
            .wrapping_sub(1 as size_t),
    );
    close(fd);
    let mut v: uint64_t = 0 as uint64_t;
    let mut i: ssize_t = 0 as ssize_t;
    while i < r && b[i as usize] as ::core::ffi::c_int >= '0' as i32
        && b[i as usize] as ::core::ffi::c_int <= '9' as i32
    {
        v = v
            .wrapping_mul(10 as uint64_t)
            .wrapping_add(
                (b[i as usize] as ::core::ffi::c_int - '0' as i32) as uint64_t,
            );
        i += 1;
    }
    return v;
}
#[cfg(target_os = "linux")]
unsafe extern "C" fn topo_read(mut t: *mut topo) {
    let mut n: ::core::ffi::c_long = 1 as ::core::ffi::c_long;
    let mut all: cpu_set_t = cpu_set_t { __bits: [0; 16] };
    ::libc::memset(
        &raw mut all as *mut ::core::ffi::c_void,
        '\0' as i32,
        ::core::mem::size_of::<cpu_set_t>() as ::libc::size_t,
    );
    ::libc::memset(
        &raw mut (*t).fast as *mut ::core::ffi::c_void,
        '\0' as i32,
        ::core::mem::size_of::<cpu_set_t>() as ::libc::size_t,
    );
    if sched_getaffinity(
        0 as __pid_t,
        ::core::mem::size_of::<cpu_set_t>() as size_t,
        &raw mut all,
    ) != 0 as ::core::ffi::c_int
    {
        n = sysconf(_SC_NPROCESSORS_ONLN as ::core::ffi::c_int);
        (*t).n_fast = (if n < 1 as ::core::ffi::c_long {
            1 as ::core::ffi::c_long
        } else {
            n
        }) as uint32_t;
    } else {
        static mut LEAF: [*const ::core::ffi::c_char; 2] = [
            b"cpu_capacity\0" as *const u8 as *const ::core::ffi::c_char,
            b"cpufreq/cpuinfo_max_freq\0" as *const u8 as *const ::core::ffi::c_char,
        ];
        n = __sched_cpucount(::core::mem::size_of::<cpu_set_t>() as size_t, &raw mut all)
            as ::core::ffi::c_long;
        (*t).n_fast = n as uint32_t;
        let mut l: uint32_t = 0 as uint32_t;
        while l < 2 as uint32_t {
            let mut hi: uint64_t = 0 as uint64_t;
            let mut lo: uint64_t = UINT64_MAX as uint64_t;
            let mut c: uint32_t = 0 as uint32_t;
            while c < CPU_SETSIZE as uint32_t {
                if !(({
                    let mut __cpu: size_t = c as size_t;
                    if __cpu.wrapping_div(8 as size_t)
                        < ::core::mem::size_of::<cpu_set_t>() as usize
                    {
                        (*(&raw mut all.__bits as *mut __cpu_mask as *const __cpu_mask)
                            .offset(__cpu.wrapping_div(__NCPUBITS) as isize)
                            & (1 as ::core::ffi::c_int as __cpu_mask)
                                << __cpu.wrapping_rem(__NCPUBITS) != 0 as __cpu_mask)
                            as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }
                }) == 0)
                {
                    let mut v: uint64_t = sys_num(c, LEAF[l as usize]);
                    if v > hi {
                        hi = v;
                    }
                    if v < lo {
                        lo = v;
                    }
                }
                c = c.wrapping_add(1);
            }
            if hi == 0 as uint64_t
                || lo.wrapping_mul(10 as uint64_t) >= hi.wrapping_mul(9 as uint64_t)
            {
                l = l.wrapping_add(1);
            } else {
                let mut f: uint32_t = 0 as uint32_t;
                let mut c_0: uint32_t = 0 as uint32_t;
                while c_0 < CPU_SETSIZE as uint32_t {
                    if ({
                        let mut __cpu: size_t = c_0 as size_t;
                        (if __cpu.wrapping_div(8 as size_t)
                            < ::core::mem::size_of::<cpu_set_t>() as usize
                        {
                            (*(&raw mut all.__bits as *mut __cpu_mask
                                as *const __cpu_mask)
                                .offset(__cpu.wrapping_div(__NCPUBITS) as isize)
                                & (1 as ::core::ffi::c_int as __cpu_mask)
                                    << __cpu.wrapping_rem(__NCPUBITS) != 0 as __cpu_mask)
                                as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        })
                    }) != 0
                        && sys_num(c_0, LEAF[l as usize]).wrapping_mul(10 as uint64_t)
                            >= hi.wrapping_mul(9 as uint64_t)
                    {
                        let mut __cpu: size_t = c_0 as size_t;
                        if __cpu.wrapping_div(8 as size_t)
                            < ::core::mem::size_of::<cpu_set_t>() as usize
                        {
                            *(&raw mut (*t).fast.__bits as *mut __cpu_mask)
                                .offset(__cpu.wrapping_div(__NCPUBITS) as isize)
                                |= (1 as ::core::ffi::c_int as __cpu_mask)
                                    << __cpu.wrapping_rem(__NCPUBITS);
                        } else {};
                        f = f.wrapping_add(1);
                    }
                    c_0 = c_0.wrapping_add(1);
                }
                (*t).n_fast = f;
                break;
            }
        }
    }
    if n < 1 as ::core::ffi::c_long {
        n = 1 as ::core::ffi::c_long;
    }
    if n > PAR_MAX_N as ::core::ffi::c_long {
        n = PAR_MAX_N as ::core::ffi::c_long;
    }
    (*t).n_cpu = n as uint32_t;
    if (*t).n_fast < 1 as uint32_t || (*t).n_fast > (*t).n_cpu {
        (*t).n_fast = (*t).n_cpu;
    }
}
unsafe extern "C" fn thr_main(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    worker_loop(arg as *mut slot);
    return NULL;
}
#[cfg(target_os = "linux")]
unsafe extern "C" fn thr_start(
    mut t: *mut thr_t,
    mut s: *mut slot,
    mut tp: *const topo,
    mut fast: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut at: pthread_attr_t = pthread_attr_t { __size: [0; 56] };
    if pthread_attr_init(&raw mut at) != 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if fast != 0 && (*tp).n_fast < (*tp).n_cpu {
        pthread_attr_setaffinity_np(
            &raw mut at,
            ::core::mem::size_of::<cpu_set_t>() as size_t,
            &raw const (*tp).fast,
        );
    }
    let mut r: ::core::ffi::c_int = pthread_create(
        t as *mut pthread_t,
        &raw mut at,
        Some(
            thr_main
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                ) -> *mut ::core::ffi::c_void,
        ),
        s as *mut ::core::ffi::c_void,
    );
    pthread_attr_destroy(&raw mut at);
    return r;
}
unsafe extern "C" fn thr_join(mut t: thr_t) {
    pthread_join(t as pthread_t, ::core::ptr::null_mut::<*mut ::core::ffi::c_void>());
}
unsafe extern "C" fn scratch_fit(
    mut p: *const toks_par,
    mut s: *mut slot,
    mut len: uint64_t,
    mut grew: *mut ::core::ffi::c_int,
) -> int64_t {
    if !(*s).scr.is_null() && len <= (*s).scr_len {
        return 0 as int64_t;
    }
    let mut want: uint64_t = (*s).scr_len.wrapping_mul(2 as uint64_t);
    if want < len {
        want = len;
    }
    if want < PAR_SCR_MIN as uint64_t {
        want = PAR_SCR_MIN as uint64_t;
    }
    if want as ::core::ffi::c_ulonglong > TOKS_MAX_TEXT {
        want = TOKS_MAX_TEXT as uint64_t;
    }
    let mut bytes: uint64_t = toks_scratch_bytes((*p).ctx, want, (*p).scr_flags);
    let mut m: *mut uint8_t = toks_plat_arena(bytes);
    if m.is_null() {
        return TOKS_E_NOMEM as int64_t;
    }
    if toks_scratch_init((*p).ctx, m as *mut ::core::ffi::c_void, bytes, (*p).scr_flags)
        != 0 as int64_t
    {
        toks_plat_arena_free(m, bytes);
        return TOKS_E_NOMEM as int64_t;
    }
    toks_plat_arena_free((*s).scr, (*s).scr_bytes);
    (*s).scr = m;
    (*s).scr_bytes = bytes;
    (*s).scr_len = want;
    *grew = 1 as ::core::ffi::c_int;
    return 0 as int64_t;
}
unsafe extern "C" fn grow(
    mut a: *mut *mut ::core::ffi::c_void,
    mut cap: *mut uint64_t,
    mut want: uint64_t,
    mut z: uint64_t,
) -> int64_t {
    if want <= *cap {
        return 0 as int64_t;
    }
    let mut c: uint64_t = want
        .wrapping_add(want.wrapping_div(2 as uint64_t))
        .wrapping_add(64 as uint64_t);
    let mut m: *mut ::core::ffi::c_void = toks_plat_alloc(c.wrapping_mul(z));
    if m.is_null() {
        return TOKS_E_NOMEM as int64_t;
    }
    if !(*a).is_null() {
        memcpy(m, *a, (*cap).wrapping_mul(z) as size_t);
        toks_plat_free(*a, (*cap).wrapping_mul(z));
    }
    *a = m;
    *cap = c;
    return 0 as int64_t;
}
unsafe extern "C" fn item_ok(mut it: *const toks_par_item) -> ::core::ffi::c_int {
    return ((*it).len as ::core::ffi::c_ulonglong <= TOKS_MAX_TEXT
        && (!(*it).text.is_null() || (*it).len == 0 as uint64_t)) as ::core::ffi::c_int;
}
unsafe extern "C" fn item_bytes(mut it: *const toks_par_item) -> uint64_t {
    return if item_ok(it) != 0 { (*it).len } else { 0 as uint64_t };
}
unsafe extern "C" fn run_group(
    mut p: *mut toks_par,
    mut s: *mut slot,
    mut a: uint64_t,
    mut b: uint64_t,
) {
    let mut grew: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut t0: uint64_t = now_ns();
    let mut bytes: uint64_t = 0 as uint64_t;
    let mut i: uint64_t = a;
    while i < b {
        let mut it: *mut toks_par_item = (*p).items.offset(i as isize)
            as *mut toks_par_item;
        let mut r: int64_t = if item_ok(it) != 0 {
            scratch_fit(p, s, (*it).len, &raw mut grew)
        } else {
            0 as int64_t
        };
        (*it).n = if r < 0 as int64_t {
            r
        } else {
            toks_encode(
                (*p).ctx,
                (*it).text,
                (*it).len,
                (*p).flags,
                (*it).out,
                (*it).cap,
                (*s).scr as *mut ::core::ffi::c_void,
            )
        };
        bytes = bytes.wrapping_add(item_bytes(it));
        i = i.wrapping_add(1);
    }
    if grew == 0 {
        (*s).busy_ns = (*s).busy_ns.wrapping_add(now_ns().wrapping_sub(t0));
        (*s).busy_bytes = (*s).busy_bytes.wrapping_add(bytes);
    }
    (*s).grew |= grew as uint32_t;
}
unsafe extern "C" fn run_part(mut p: *mut toks_par, mut s: *mut slot, mut i: uint64_t) {
    let mut q: *mut part = (*p).parts.offset(i as isize) as *mut part;
    let mut it: *mut toks_par_item = (*p).items.offset((*q).item as isize)
        as *mut toks_par_item;
    let mut fl: uint32_t = (*p).flags | TOKS_NO_POSTPROCESS as uint32_t
        | (if (*q).first != 0 { 0 as uint32_t } else { TOKS_CONTINUATION as uint32_t });
    let mut grew: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut r: int64_t = scratch_fit(p, s, (*q).len, &raw mut grew);
    let mut t0: uint64_t = now_ns();
    if r == 0 as int64_t && (*q).first != 0 {
        let mut c0: uint64_t = if (*it).cap > (*p).n_pre as uint64_t {
            (*it).cap.wrapping_sub((*p).n_pre as uint64_t)
        } else {
            0 as uint64_t
        };
        r = toks_encode(
            (*p).ctx,
            (*it).text,
            (*q).len,
            fl,
            if c0 != 0 as uint64_t {
                (*it).out.offset((*p).n_pre as isize)
            } else {
                ::core::ptr::null_mut::<uint32_t>()
            },
            c0,
            (*s).scr as *mut ::core::ffi::c_void,
        );
    } else if r == 0 as int64_t {
        r = toks_encode(
            (*p).ctx,
            ((*it).text as *const uint8_t).offset((*q).a as isize)
                as *const ::core::ffi::c_void,
            (*q).len,
            fl,
            (*p).stage.offset((*q).slab as isize),
            (*q).len.wrapping_add(4 as uint64_t),
            (*s).scr as *mut ::core::ffi::c_void,
        );
        if r > (*q).len.wrapping_add(4 as uint64_t) as int64_t {
            r = PAR_OVERFLOW as int64_t;
        }
    }
    if grew == 0 {
        (*s).busy_ns = (*s).busy_ns.wrapping_add(now_ns().wrapping_sub(t0));
        (*s).busy_bytes = (*s).busy_bytes.wrapping_add((*q).len);
    }
    (*s).grew |= grew as uint32_t;
    (*q).cnt = r;
    crate::atomic::atomic_store_release(
        &raw mut (*q).done,
        1 as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn place(
    mut p: *mut toks_par,
    mut i: uint64_t,
    mut at: uint64_t,
    mut bad: ::core::ffi::c_int,
) {
    let mut q: *mut part = (*p).parts.offset(i as isize) as *mut part;
    let mut it: *mut toks_par_item = (*p).items.offset((*q).item as isize)
        as *mut toks_par_item;
    if (*q).first == 0 && bad == 0 && (*q).cnt > 0 as int64_t && at < (*it).cap {
        let mut m: uint64_t = (*q).cnt as uint64_t;
        if m > (*it).cap.wrapping_sub(at) {
            m = (*it).cap.wrapping_sub(at);
        }
        toks_cpy(
            (*it).out.offset(at as isize) as *mut ::core::ffi::c_void,
            (*p).stage.offset((*q).slab as isize) as *const ::core::ffi::c_void,
            m.wrapping_mul(4 as uint64_t),
        );
    }
    crate::atomic::atomic_xadd_release(
        &raw mut (*p).fin.v,
        1 as ::core::ffi::c_uint as uint64_t,
    );
}
unsafe extern "C" fn advance(
    mut p: *mut toks_par,
    mut c: *mut cursor,
    mut upto: uint64_t,
    mut wait: ::core::ffi::c_int,
) {
    while (*c).k < upto {
        let mut q: *mut part = (*p).parts.offset((*c).k as isize) as *mut part;
        if crate::atomic::atomic_load_acquire(&raw mut (*q).done) == 0 as uint32_t {
            if wait == 0 {
                return;
            }
            relax();
        } else {
            if (*q).first != 0 {
                (*c).sum = 0 as uint64_t;
                (*c).bad = 0 as ::core::ffi::c_int;
            }
            if (*q).cnt < 0 as int64_t {
                (*c).bad = 1 as ::core::ffi::c_int;
            }
            if (*c).k == (*c).head {
                place(
                    p,
                    (*c).k,
                    ((*p).n_pre as uint64_t).wrapping_add((*c).sum),
                    (*c).bad,
                );
                (*c).head = (*q).nxt;
            }
            (*c).sum = (*c)
                .sum
                .wrapping_add(
                    if (*q).cnt > 0 as int64_t {
                        (*q).cnt as uint64_t
                    } else {
                        0 as uint64_t
                    },
                );
            (*c).k = (*c).k.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn run_job(mut p: *mut toks_par, mut s: *mut slot) {
    let mut c: cursor = cursor {
        k: 0 as uint64_t,
        sum: 0 as uint64_t,
        head: UINT64_MAX as uint64_t,
        tail: UINT64_MAX as uint64_t,
        bad: 0 as ::core::ffi::c_int,
    };
    loop {
        let mut u: uint64_t = crate::atomic::atomic_xadd_relaxed(
            &raw mut (*p).next.v,
            1 as ::core::ffi::c_uint as uint64_t,
        );
        if u >= (*p).n_units {
            break;
        }
        let mut x: *const unit = (*p).units.offset(u as isize) as *mut unit;
        if (*x).b != PAR_PART as uint64_t {
            run_group(p, s, (*x).a, (*x).b);
            crate::atomic::atomic_xadd_release(
                &raw mut (*p).fin.v,
                1 as ::core::ffi::c_uint as uint64_t,
            );
        } else {
            let mut i: uint64_t = (*x).a;
            run_part(p, s, i);
            (*(*p).parts.offset(i as isize)).nxt = UINT64_MAX as uint64_t;
            if c.head == UINT64_MAX as uint64_t {
                c.head = i;
            } else {
                (*(*p).parts.offset(c.tail as isize)).nxt = i;
            }
            c.tail = i;
            advance(
                p,
                &raw mut c,
                i.wrapping_add(1 as uint64_t),
                0 as ::core::ffi::c_int,
            );
        }
    }
    if c.head != UINT64_MAX as uint64_t {
        advance(
            p,
            &raw mut c,
            c.tail.wrapping_add(1 as uint64_t),
            1 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn unit_add(
    mut p: *mut toks_par,
    mut a: uint64_t,
    mut b: uint64_t,
) -> int64_t {
    if grow(
        &raw mut (*p).units as *mut *mut ::core::ffi::c_void,
        &raw mut (*p).units_cap,
        (*p).n_units.wrapping_add(1 as uint64_t),
        ::core::mem::size_of::<unit>() as uint64_t,
    ) != 0 as int64_t
    {
        return TOKS_E_NOMEM as int64_t;
    }
    (*(*p).units.offset((*p).n_units as isize)).a = a;
    (*(*p).units.offset((*p).n_units as isize)).b = b;
    (*p).n_units = (*p).n_units.wrapping_add(1);
    return 0 as int64_t;
}
unsafe extern "C" fn part_add(
    mut p: *mut toks_par,
    mut i: uint64_t,
    mut a: uint64_t,
    mut b: uint64_t,
    mut ids: *mut uint64_t,
) -> int64_t {
    if grow(
        &raw mut (*p).parts as *mut *mut ::core::ffi::c_void,
        &raw mut (*p).parts_cap,
        (*p).n_parts.wrapping_add(1 as uint64_t),
        ::core::mem::size_of::<part>() as uint64_t,
    ) != 0 as int64_t
        || grow(
            &raw mut (*p).units as *mut *mut ::core::ffi::c_void,
            &raw mut (*p).units_cap,
            (*p).n_units.wrapping_add(1 as uint64_t),
            ::core::mem::size_of::<unit>() as uint64_t,
        ) != 0 as int64_t
    {
        return TOKS_E_NOMEM as int64_t;
    }
    let mut q: *mut part = (*p).parts.offset((*p).n_parts as isize) as *mut part;
    (*q).item = i;
    (*q).a = a;
    (*q).len = b.wrapping_sub(a);
    (*q).first = (a == 0 as uint64_t) as ::core::ffi::c_int as uint32_t;
    (*q).slab = *ids;
    (*q).cnt = 0 as int64_t;
    crate::atomic::atomic_store_relaxed(
        &raw mut (*q).done,
        0 as ::core::ffi::c_uint,
    );
    if a != 0 as uint64_t {
        *ids = (*ids).wrapping_add((*q).len.wrapping_add(4 as uint64_t));
    }
    (*(*p).units.offset((*p).n_units as isize)).a = (*p).n_parts;
    (*(*p).units.offset((*p).n_units as isize)).b = PAR_PART as uint64_t;
    (*p).n_units = (*p).n_units.wrapping_add(1);
    (*p).n_parts = (*p).n_parts.wrapping_add(1);
    return 0 as int64_t;
}
unsafe extern "C" fn split(
    mut p: *mut toks_par,
    mut i: uint64_t,
    mut len: uint64_t,
    mut t: uint64_t,
    mut tail: uint64_t,
    mut at: uint64_t,
    mut head: uint64_t,
    mut ids: *mut uint64_t,
) -> int64_t {
    let mut it: *const toks_par_item = (*p).items.offset(i as isize)
        as *mut toks_par_item;
    let mut want: uint64_t = len
        .wrapping_add(tail)
        .wrapping_sub(1 as uint64_t)
        .wrapping_div(tail);
    if want > 0xffffffff as uint64_t {
        want = 0xffffffff as uint64_t;
    }
    if grow(
        &raw mut (*p).cuts as *mut *mut ::core::ffi::c_void,
        &raw mut (*p).cuts_cap,
        want.wrapping_add(1 as uint64_t),
        8 as uint64_t,
    ) != 0 as int64_t
    {
        return TOKS_E_NOMEM as int64_t;
    }
    let mut nc: int64_t = toks_split_points(
        (*p).ctx,
        (*it).text,
        len,
        (*p).flags,
        want as uint32_t,
        (*p).cuts,
        want.wrapping_sub(1 as uint64_t),
        NULL,
    );
    if nc <= 0 as int64_t {
        return unit_add(p, i, i.wrapping_add(1 as uint64_t));
    }
    *(*p).cuts.offset(nc as isize) = len;
    let mut a: uint64_t = 0 as uint64_t;
    let mut j: uint64_t = 0 as uint64_t;
    while j <= nc as uint64_t {
        let mut b: uint64_t = *(*p).cuts.offset(j as isize);
        if !(j < nc as uint64_t && at.wrapping_add(b) <= head && b.wrapping_sub(a) < t) {
            if part_add(p, i, a, b, ids) != 0 as int64_t {
                return TOKS_E_NOMEM as int64_t;
            }
            a = b;
        }
        j = j.wrapping_add(1);
    }
    return 0 as int64_t;
}
unsafe extern "C" fn plan(
    mut p: *mut toks_par,
    mut bytes: uint64_t,
    mut k: uint32_t,
) -> int64_t {
    let mut t: uint64_t = unit_len(bytes, k);
    let mut tail: uint64_t = tail_len(t);
    let mut head: uint64_t = bytes.wrapping_sub(bytes.wrapping_div(4 as uint64_t));
    let mut at: uint64_t = 0 as uint64_t;
    let mut g: uint64_t = 0 as uint64_t;
    let mut acc: uint64_t = 0 as uint64_t;
    let mut ids: uint64_t = 0 as uint64_t;
    (*p).tail_len = tail;
    (*p).n_units = 0 as uint64_t;
    (*p).n_parts = 0 as uint64_t;
    let mut i: uint64_t = 0 as uint64_t;
    while i < (*p).n_items {
        let mut len: uint64_t = item_bytes(
            (*p).items.offset(i as isize) as *mut toks_par_item,
        );
        if len >= (2 as uint64_t).wrapping_mul(t) {
            if g < i && unit_add(p, g, i) != 0 as int64_t
                || split(p, i, len, t, tail, at, head, &raw mut ids) != 0 as int64_t
            {
                return TOKS_E_NOMEM as int64_t;
            }
            at = at.wrapping_add(len);
            g = i.wrapping_add(1 as uint64_t);
            acc = 0 as uint64_t;
        } else {
            acc = acc.wrapping_add(len);
            at = at.wrapping_add(len);
            if acc >= (if at <= head { t } else { tail }) {
                if unit_add(p, g, i.wrapping_add(1 as uint64_t)) != 0 as int64_t {
                    return TOKS_E_NOMEM as int64_t;
                }
                g = i.wrapping_add(1 as uint64_t);
                acc = 0 as uint64_t;
            }
        }
        i = i.wrapping_add(1);
    }
    if g < (*p).n_items && unit_add(p, g, (*p).n_items) != 0 as int64_t {
        return TOKS_E_NOMEM as int64_t;
    }
    if ids > (*p).stage_ids {
        let mut z: uint64_t = ids.wrapping_add(ids.wrapping_div(2 as uint64_t));
        let mut st: *mut uint32_t = toks_plat_arena(z.wrapping_mul(4 as uint64_t))
            as *mut ::core::ffi::c_void as *mut uint32_t;
        if st.is_null() {
            return TOKS_E_NOMEM as int64_t;
        }
        toks_plat_arena_free(
            (*p).stage as *mut uint8_t,
            (*p).stage_ids.wrapping_mul(4 as uint64_t),
        );
        (*p).stage = st;
        (*p).stage_ids = z;
    }
    return 0 as int64_t;
}
unsafe extern "C" fn finish_parts(mut p: *mut toks_par) {
    let mut j: uint64_t = 0 as uint64_t;
    while j < (*p).n_parts {
        let mut it: *mut toks_par_item = (*p)
            .items
            .offset((*(*p).parts.offset(j as isize)).item as isize)
            as *mut toks_par_item;
        let mut n: uint64_t = (*p).n_pre as uint64_t;
        let mut bad: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            let mut r: int64_t = (*(*p).parts.offset(j as isize)).cnt;
            bad |= (r < 0 as int64_t) as ::core::ffi::c_int;
            n = n
                .wrapping_add(
                    if r > 0 as int64_t { r as uint64_t } else { 0 as uint64_t },
                );
            j = j.wrapping_add(1);
            if !(j < (*p).n_parts && (*(*p).parts.offset(j as isize)).first == 0) {
                break;
            }
        }
        if bad == 0 {
            let mut i: uint64_t = 0 as uint64_t;
            while i < (*p).n_pre as uint64_t && i < (*it).cap {
                toks_st32(
                    (*it).out.offset(i as isize) as *mut ::core::ffi::c_void,
                    *(*p).pre.offset(i as isize),
                );
                i = i.wrapping_add(1);
            }
            let mut i_0: uint64_t = 0 as uint64_t;
            while i_0 < (*p).n_suf as uint64_t && n.wrapping_add(i_0) < (*it).cap {
                toks_st32(
                    (*it).out.offset(n as isize).offset(i_0 as isize)
                        as *mut ::core::ffi::c_void,
                    *(*p).suf.offset(i_0 as isize),
                );
                i_0 = i_0.wrapping_add(1);
            }
            (*it).n = n.wrapping_add((*p).n_suf as uint64_t) as int64_t;
        } else {
            let mut s0: *mut slot = (*p).slots.offset(0 as ::core::ffi::c_int as isize)
                as *mut slot;
            let mut grew: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut r_0: int64_t = scratch_fit(p, s0, (*it).len, &raw mut grew);
            (*it).n = if r_0 < 0 as int64_t {
                r_0
            } else {
                toks_encode(
                    (*p).ctx,
                    (*it).text,
                    (*it).len,
                    (*p).flags,
                    (*it).out,
                    (*it).cap,
                    (*s0).scr as *mut ::core::ffi::c_void,
                )
            };
        }
    }
}
unsafe extern "C" fn ewma(mut x: *mut uint64_t, mut v: uint64_t) {
    let mut hi: uint64_t = (*x)
        .wrapping_mul(4 as uint64_t)
        .wrapping_add(20000 as uint64_t);
    if v > hi {
        v = hi;
    }
    *x = if v >= *x {
        (*x).wrapping_add(v.wrapping_sub(*x).wrapping_div(4 as uint64_t))
    } else {
        (*x).wrapping_sub((*x).wrapping_sub(v).wrapping_div(4 as uint64_t))
    };
}
unsafe extern "C" fn ewma_cost(mut x: *mut uint64_t, mut v: uint64_t) {
    let mut hi: uint64_t = (*x)
        .wrapping_mul(4 as uint64_t)
        .wrapping_add(20000 as uint64_t);
    if v > hi {
        v = hi;
    }
    *x = if v >= *x {
        (*x).wrapping_add(v.wrapping_sub(*x).wrapping_div(2 as uint64_t))
    } else {
        (*x).wrapping_sub((*x).wrapping_sub(v).wrapping_div(8 as uint64_t))
    };
}
unsafe extern "C" fn unit_len(mut bytes: uint64_t, mut k: uint32_t) -> uint64_t {
    let mut t: uint64_t = bytes
        .wrapping_div((k as uint64_t).wrapping_mul(PAR_UNITS_PER as uint64_t));
    return if t < PAR_MIN_UNIT as uint64_t {
        PAR_MIN_UNIT as uint64_t
    } else if t > PAR_MAX_UNIT as uint64_t {
        PAR_MAX_UNIT as uint64_t
    } else {
        t
    };
}
unsafe extern "C" fn tail_len(mut t: uint64_t) -> uint64_t {
    return if t.wrapping_div(4 as uint64_t) < PAR_MIN_UNIT as uint64_t {
        PAR_MIN_UNIT as uint64_t
    } else {
        t.wrapping_div(4 as uint64_t)
    };
}
unsafe extern "C" fn extra(
    mut p: *const toks_par,
    mut bytes: uint64_t,
    mut k: uint32_t,
    mut est: uint64_t,
    mut c: uint64_t,
) -> uint64_t {
    let mut t: uint64_t = unit_len(bytes, k);
    let mut tl: uint64_t = tail_len(t);
    let mut units: uint64_t = bytes
        .wrapping_sub(bytes.wrapping_div(4 as uint64_t))
        .wrapping_div(t)
        .wrapping_add(bytes.wrapping_div(4 as uint64_t).wrapping_div(tl))
        .wrapping_add(1 as uint64_t);
    return est
        .wrapping_div(1024 as uint64_t)
        .wrapping_mul((*p).eps)
        .wrapping_add(
            (k.wrapping_sub(1 as uint32_t) as uint64_t)
                .wrapping_mul(
                    tl
                        .wrapping_div(2 as uint64_t)
                        .wrapping_mul(c)
                        .wrapping_div(1000 as uint64_t),
                ),
        )
        .wrapping_add(units.wrapping_mul(PAR_UNIT_NS as uint64_t));
}
unsafe extern "C" fn c_dec(mut p: *const toks_par) -> uint64_t {
    return if (*p).c_ser < (*p).c_ps { (*p).c_ser } else { (*p).c_ps };
}
unsafe extern "C" fn choose_k(
    mut p: *const toks_par,
    mut bytes: uint64_t,
    mut now: uint64_t,
) -> uint32_t {
    if (*p).n == 1 as uint32_t || bytes < PAR_MIN_BYTES as uint64_t {
        return 1 as uint32_t;
    }
    let mut kmax: uint64_t = bytes.wrapping_div(PAR_MIN_UNIT as uint64_t);
    if kmax > (*p).n as uint64_t {
        kmax = (*p).n as uint64_t;
    }
    if (*p).eager != 0 {
        return kmax as uint32_t;
    }
    let mut c: uint64_t = c_dec(p);
    let mut est: uint64_t = bytes.wrapping_div(1000 as uint64_t).wrapping_mul(c);
    let mut delays: uint64_t = 0 as uint64_t;
    let mut hot: ::core::ffi::c_int = ((*p).t_last != 0 as uint64_t
        && now.wrapping_sub((*p).t_last)
            < crate::atomic::atomic_load_relaxed(&raw const (*p).spin_ns))
        as ::core::ffi::c_int;
    let mut k: uint32_t = 1 as uint32_t;
    while (k as uint64_t) < kmax {
        let mut asleep: ::core::ffi::c_int = (crate::atomic::atomic_load_relaxed(
            &raw mut (*(*p).slots.offset(k as isize)).asleep,
        ) != 0 as uint32_t) as ::core::ffi::c_int;
        delays = delays
            .wrapping_add(
                if asleep != 0 && hot == 0 { (*p).o_wake } else { (*p).o_join },
            );
        let mut all: uint64_t = est
            .wrapping_add(extra(p, bytes, k.wrapping_add(1 as uint32_t), est, c))
            .wrapping_add(delays);
        if est.wrapping_div(1024 as uint64_t).wrapping_mul(1024 as uint64_t)
            < all.wrapping_div(1024 as uint64_t).wrapping_mul(PAR_EFF as uint64_t)
        {
            break;
        }
        k = k.wrapping_add(1);
    }
    return k;
}
unsafe extern "C" fn set_spin(mut p: *mut toks_par) {
    let mut v: uint64_t = if (*p).lat_wake < PAR_SPIN_LO as uint64_t {
        PAR_SPIN_LO as uint64_t
    } else if (*p).lat_wake > PAR_SPIN_HI as uint64_t {
        PAR_SPIN_HI as uint64_t
    } else {
        (*p).lat_wake
    };
    crate::atomic::atomic_store_relaxed(&raw mut (*p).spin_ns, v);
}
unsafe extern "C" fn learn(
    mut p: *mut toks_par,
    mut k: uint32_t,
    mut q: uint32_t,
    mut t0: uint64_t,
    mut t_end: uint64_t,
) {
    let mut ns: uint64_t = 0 as uint64_t;
    let mut bytes: uint64_t = 0 as uint64_t;
    let mut delays: uint64_t = 0 as uint64_t;
    let mut grew: uint32_t = 0 as uint32_t;
    let mut current_block_6: u64;
    let mut j: uint32_t = 0 as uint32_t;
    while j < k {
        let mut s: *mut slot = (*p).slots.offset(j as isize) as *mut slot;
        if j != 0 as uint32_t {
            let mut in_0: ::core::ffi::c_int = (crate::atomic::atomic_load_acquire(
                &raw mut (*s).joined,
            ) == q && (*s).t_join >= (*p).t_go) as ::core::ffi::c_int;
            let mut d: uint64_t = (if in_0 != 0 { (*s).t_join } else { t_end })
                .wrapping_sub((*p).t_go);
            if (*s).was_asleep != 0 {
                ewma(&raw mut (*p).lat_wake, d);
            }
            delays = delays.wrapping_add(d);
            if in_0 == 0 {
                current_block_6 = 16668937799742929182;
            } else {
                current_block_6 = 3276175668257526147;
            }
        } else {
            current_block_6 = 3276175668257526147;
        }
        match current_block_6 {
            3276175668257526147 => {
                ns = ns.wrapping_add((*s).busy_ns);
                bytes = bytes.wrapping_add((*s).busy_bytes);
                grew |= (*s).grew;
            }
            _ => {}
        }
        j = j.wrapping_add(1);
    }
    if bytes >= PAR_MIN_BYTES as uint64_t {
        ewma(&raw mut (*p).c_ps, ns.wrapping_mul(1000 as uint64_t).wrapping_div(bytes));
    }
    set_spin(p);
    if grew != 0 {
        return;
    }
    let mut est: uint64_t = (*p)
        .call_bytes
        .wrapping_div(1000 as uint64_t)
        .wrapping_mul((*p).c_ps);
    let mut all: uint64_t = (k as uint64_t).wrapping_mul(t_end.wrapping_sub(t0));
    let mut idle: uint64_t = (k.wrapping_sub(1 as uint32_t) as uint64_t)
        .wrapping_mul(
            (*p)
                .tail_len
                .wrapping_div(2 as uint64_t)
                .wrapping_mul((*p).c_ps)
                .wrapping_div(1000 as uint64_t),
        )
        .wrapping_add((*p).n_units.wrapping_mul(PAR_UNIT_NS as uint64_t));
    if ns >= 2000000 as uint64_t {
        let mut fixed: uint64_t = ns.wrapping_add(delays).wrapping_add(idle);
        let mut e: uint64_t = if all > fixed {
            all.wrapping_sub(fixed).wrapping_mul(1024 as uint64_t).wrapping_div(ns)
        } else {
            0 as uint64_t
        };
        ewma_cost(
            &raw mut (*p).eps,
            if e > 512 as uint64_t { 512 as uint64_t } else { e },
        );
    } else {
        let mut fixed_0: uint64_t = est
            .wrapping_add(est.wrapping_div(1024 as uint64_t).wrapping_mul((*p).eps))
            .wrapping_add(idle);
        let mut o: uint64_t = if all > fixed_0 {
            all.wrapping_sub(fixed_0)
                .wrapping_div(k.wrapping_sub(1 as uint32_t) as uint64_t)
        } else {
            0 as uint64_t
        };
        ewma_cost(
            if (*(*p).slots.offset(1 as ::core::ffi::c_int as isize)).was_asleep != 0 {
                &raw mut (*p).o_wake
            } else {
                &raw mut (*p).o_join
            },
            if o < 100 as uint64_t { 100 as uint64_t } else { o },
        );
        if (*p).o_join > (*p).o_wake {
            (*p).o_join = (*p).o_wake;
        }
    };
}
unsafe extern "C" fn worker_loop(mut s: *mut slot) {
    let mut p: *mut toks_par = (*s).par;
    let mut seen: uint32_t = 0 as uint32_t;
    loop {
        let mut w: uint32_t = crate::atomic::atomic_load_acquire(
            &raw mut (*s).word,
        );
        if w == seen {
            let mut t0: uint64_t = now_ns();
            let mut i: uint32_t = 1 as uint32_t;
            loop {
                w = crate::atomic::atomic_load_acquire(&raw mut (*s).word);
                if !(w == seen) {
                    break;
                }
                relax();
                if i & 63 as uint32_t != 0 as uint32_t
                    || now_ns().wrapping_sub(t0)
                        < crate::atomic::atomic_load_relaxed(&raw mut (*p).spin_ns)
                {
                    i = i.wrapping_add(1);
                } else {
                    crate::atomic::atomic_store_seqcst(
                        &raw mut (*s).asleep,
                        1 as ::core::ffi::c_uint,
                    );
                    loop {
                        w = crate::atomic::atomic_load_seqcst(&raw mut (*s).word);
                        if !(w == seen) {
                            break;
                        }
                        addr_wait(&raw mut (*s).word, seen);
                    }
                    crate::atomic::atomic_store_seqcst(
                        &raw mut (*s).asleep,
                        0 as ::core::ffi::c_uint,
                    );
                    break;
                }
            }
        }
        seen = w;
        if crate::atomic::atomic_load_seqcst(&raw mut (*p).quit.v) != 0 as uint32_t
        {
            return;
        }
        crate::atomic::atomic_xadd_seqcst(
            &raw mut (*p).active.v,
            1 as ::core::ffi::c_uint,
        );
        if crate::atomic::atomic_load_seqcst(&raw mut (*p).open.v) != 0 as uint32_t
            && crate::atomic::atomic_load_seqcst(&raw mut (*p).seq.v) == w
        {
            (*s).t_join = now_ns();
            (*s).busy_ns = 0 as uint64_t;
            (*s).busy_bytes = 0 as uint64_t;
            (*s).grew = 0 as ::core::ffi::c_uint as uint32_t;
            crate::atomic::atomic_store_release(&raw mut (*s).joined, w);
            run_job(p, s);
        }
        crate::atomic::atomic_xsub_seqcst(
            &raw mut (*p).active.v,
            1 as ::core::ffi::c_uint,
        );
    };
}
unsafe extern "C" fn lock(mut p: *mut toks_par) {
    let mut z: uint32_t = 0 as uint32_t;
    loop {
        let fresh5 = crate::atomic::atomic_cxchgweak_seqcst_seqcst(
            &raw mut (*p).busy.v,
            *&raw mut z,
            1 as ::core::ffi::c_uint,
        );
        *&raw mut z = fresh5.0;
        if fresh5.1 {
            break;
        }
        z = 0 as ::core::ffi::c_uint as uint32_t;
        sched_yield();
    };
}
unsafe extern "C" fn unlock(mut p: *mut toks_par) {
    crate::atomic::atomic_store_seqcst(
        &raw mut (*p).busy.v,
        0 as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn open_job(mut p: *mut toks_par, mut k: uint32_t) -> uint32_t {
    let mut q: uint32_t = crate::atomic::atomic_load_seqcst(&raw mut (*p).seq.v)
        .wrapping_add(1 as uint32_t);
    if q == 0 as uint32_t {
        q = 1 as ::core::ffi::c_uint as uint32_t;
    }
    crate::atomic::atomic_store_relaxed(
        &raw mut (*p).next.v,
        0 as ::core::ffi::c_uint as uint64_t,
    );
    crate::atomic::atomic_store_relaxed(
        &raw mut (*p).fin.v,
        0 as ::core::ffi::c_uint as uint64_t,
    );
    crate::atomic::atomic_store_seqcst(&raw mut (*p).seq.v, q);
    crate::atomic::atomic_store_seqcst(
        &raw mut (*p).open.v,
        1 as ::core::ffi::c_uint,
    );
    (*p).t_go = now_ns();
    let mut j: uint32_t = 1 as uint32_t;
    while j < k {
        let mut s: *mut slot = (*p).slots.offset(j as isize) as *mut slot;
        crate::atomic::atomic_store_seqcst(&raw mut (*s).word, q);
        (*s).was_asleep = crate::atomic::atomic_load_seqcst(&raw mut (*s).asleep);
        if (*s).was_asleep != 0 as uint32_t {
            addr_wake(&raw mut (*s).word);
        }
        j = j.wrapping_add(1);
    }
    return q;
}
unsafe extern "C" fn close_job(mut p: *mut toks_par) {
    crate::atomic::atomic_store_seqcst(
        &raw mut (*p).open.v,
        0 as ::core::ffi::c_uint,
    );
    while crate::atomic::atomic_load_seqcst(&raw mut (*p).active.v) != 0 as uint32_t
    {
        relax();
    }
}
unsafe extern "C" fn dispatch(mut p: *mut toks_par, mut k: uint32_t, mut t0: uint64_t) {
    let mut s0: *mut slot = (*p).slots.offset(0 as ::core::ffi::c_int as isize)
        as *mut slot;
    (*s0).busy_ns = 0 as uint64_t;
    (*s0).busy_bytes = 0 as uint64_t;
    (*s0).grew = 0 as ::core::ffi::c_uint as uint32_t;
    let mut q: uint32_t = open_job(p, k);
    run_job(p, s0);
    while crate::atomic::atomic_load_acquire(&raw mut (*p).fin.v) < (*p).n_units {
        relax();
    }
    let mut t_end: uint64_t = now_ns();
    close_job(p);
    learn(p, k, q, t0, t_end);
}
unsafe extern "C" fn calibrate(mut p: *mut toks_par) {
    let mut wake: uint64_t = 0 as uint64_t;
    let mut join: uint64_t = UINT64_MAX as uint64_t;
    let mut r: uint32_t = 0 as uint32_t;
    while r < 6 as uint32_t {
        crate::atomic::atomic_store_relaxed(
            &raw mut (*p).spin_ns,
            (if r < 4 as uint32_t {
                20000000 as ::core::ffi::c_uint
            } else {
                0 as ::core::ffi::c_uint
            }) as uint64_t,
        );
        if r >= 4 as uint32_t {
            let mut t: uint64_t = now_ns();
            let mut j: uint32_t = 1 as uint32_t;
            while j < (*p).n && now_ns().wrapping_sub(t) < 20000000 as uint64_t {
                if crate::atomic::atomic_load_seqcst(
                    &raw mut (*(*p).slots.offset(j as isize)).asleep,
                ) != 0 as uint32_t
                {
                    j = j.wrapping_add(1);
                } else {
                    relax();
                }
            }
            t = now_ns();
            while now_ns().wrapping_sub(t) < 2000000 as uint64_t {
                relax();
            }
        }
        (*p).n_units = 0 as uint64_t;
        let mut q: uint32_t = open_job(p, (*p).n);
        let mut t_0: uint64_t = (*p).t_go;
        let mut sum: uint64_t = 0 as uint64_t;
        let mut cnt: uint64_t = 0 as uint64_t;
        let mut j_0: uint32_t = 1 as uint32_t;
        while j_0 < (*p).n {
            let mut s: *mut slot = (*p).slots.offset(j_0 as isize) as *mut slot;
            while crate::atomic::atomic_load_acquire(&raw mut (*s).joined) != q
                && now_ns().wrapping_sub(t_0) < 20000000 as uint64_t
            {
                relax();
            }
            if crate::atomic::atomic_load_acquire(&raw mut (*s).joined) == q
                && (*s).t_join >= t_0
            {
                sum = sum.wrapping_add((*s).t_join.wrapping_sub(t_0));
                cnt = cnt.wrapping_add(1);
            }
            j_0 = j_0.wrapping_add(1);
        }
        close_job(p);
        if cnt != 0 as uint64_t && r >= 1 as uint32_t && r < 4 as uint32_t
            && sum.wrapping_div(cnt) < join
        {
            join = sum.wrapping_div(cnt);
        }
        if cnt != 0 as uint64_t && r >= 4 as uint32_t && sum.wrapping_div(cnt) > wake {
            wake = sum.wrapping_div(cnt);
        }
        r = r.wrapping_add(1);
    }
    if wake != 0 as uint64_t {
        (*p).lat_wake = if wake < 1000 as uint64_t { 1000 as uint64_t } else { wake };
        (*p).o_wake = (*p).lat_wake;
    }
    if join != UINT64_MAX as uint64_t && join > (*p).o_join {
        (*p).o_join = join;
    }
    if (*p).o_wake < (*p).o_join {
        (*p).o_wake = (*p).o_join;
    }
    (*p).wake0 = (*p).o_wake;
    (*p).join0 = (*p).o_join;
    set_spin(p);
}
unsafe extern "C" fn run_items(mut p: *mut toks_par, mut bytes: uint64_t) {
    let mut timed: ::core::ffi::c_int = ((*p).n > 1 as uint32_t
        && bytes >= PAR_MIN_BYTES as uint64_t) as ::core::ffi::c_int;
    let mut t0: uint64_t = if timed != 0 { now_ns() } else { 0 as uint64_t };
    (*p).call_bytes = bytes;
    let mut k: uint32_t = if timed != 0 {
        choose_k(p, bytes, t0)
    } else {
        1 as uint32_t
    };
    if k > 1 as uint32_t && plan(p, bytes, k) == 0 as int64_t
        && (*p).n_units > 1 as uint64_t
    {
        if k as uint64_t > (*p).n_units {
            k = (*p).n_units as uint32_t;
        }
        (*p).last_k = k;
        dispatch(p, k, t0);
        finish_parts(p);
        (*p).t_last = now_ns();
        return;
    }
    let mut s0: *mut slot = (*p).slots.offset(0 as ::core::ffi::c_int as isize)
        as *mut slot;
    (*p).last_k = 1 as ::core::ffi::c_uint as uint32_t;
    if timed != 0 {
        (*s0).busy_ns = 0 as uint64_t;
        (*s0).busy_bytes = 0 as uint64_t;
        (*s0).grew = 0 as ::core::ffi::c_uint as uint32_t;
        run_group(p, s0, 0 as uint64_t, (*p).n_items);
        if (*s0).busy_bytes >= PAR_MIN_BYTES as uint64_t {
            ewma(
                &raw mut (*p).c_ps,
                (*s0)
                    .busy_ns
                    .wrapping_mul(1000 as uint64_t)
                    .wrapping_div((*s0).busy_bytes),
            );
            ewma(
                &raw mut (*p).c_ser,
                (*s0)
                    .busy_ns
                    .wrapping_mul(1000 as uint64_t)
                    .wrapping_div((*s0).busy_bytes),
            );
        }
        (*p).o_wake = (*p)
            .o_wake
            .wrapping_sub(
                if (*p).o_wake > (*p).wake0 {
                    (*p).o_wake.wrapping_sub((*p).wake0).wrapping_div(64 as uint64_t)
                } else {
                    0 as uint64_t
                },
            );
        (*p).o_join = (*p)
            .o_join
            .wrapping_sub(
                if (*p).o_join > (*p).join0 {
                    (*p).o_join.wrapping_sub((*p).join0).wrapping_div(64 as uint64_t)
                } else {
                    0 as uint64_t
                },
            );
        (*p).t_last = now_ns();
        return;
    }
    let mut i: uint64_t = 0 as uint64_t;
    while i < (*p).n_items {
        let mut it: *mut toks_par_item = (*p).items.offset(i as isize)
            as *mut toks_par_item;
        let mut grew: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut r: int64_t = if item_ok(it) != 0 {
            scratch_fit(p, s0, (*it).len, &raw mut grew)
        } else {
            0 as int64_t
        };
        (*it).n = if r < 0 as int64_t {
            r
        } else {
            toks_encode(
                (*p).ctx,
                (*it).text,
                (*it).len,
                (*p).flags,
                (*it).out,
                (*it).cap,
                (*s0).scr as *mut ::core::ffi::c_void,
            )
        };
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn up(mut v: uint64_t) -> uint64_t {
    return v.wrapping_add(PAR_LINE as uint64_t).wrapping_sub(1 as uint64_t)
        & !(PAR_LINE.wrapping_sub(1 as ::core::ffi::c_uint) as uint64_t);
}
#[no_mangle]
pub unsafe extern "C" fn toks_par_create(
    mut out: *mut *mut toks_par,
    mut ctx: *const toks_ctx,
    mut n_threads: uint32_t,
    mut scratch_flags: uint32_t,
) -> int64_t {
    if out.is_null() {
        return TOKS_E_ARG as int64_t;
    }
    *out = ::core::ptr::null_mut::<toks_par>();
    if ctx.is_null() || n_threads > PAR_MAX_N as uint32_t {
        return TOKS_E_ARG as int64_t;
    }
    if scratch_flags
        & !(TOKS_SCRATCH_MEMO_SET as uint32_t
            | 0xfff as ::core::ffi::c_uint as uint32_t & 0xfff as uint32_t
            | (0xff as ::core::ffi::c_uint as uint32_t & 0xff as uint32_t)
                << 12 as ::core::ffi::c_int) != 0 as uint32_t
        || toks_scr_cache_ok(toks_scr_cache_mib(scratch_flags)) == 0
    {
        return TOKS_E_ARG as int64_t;
    }
    let mut tp: topo = topo {
        n_cpu: 0,
        n_fast: 0,
        fast: cpu_set_t { __bits: [0; 16] },
    };
    topo_read(&raw mut tp);
    let mut n: uint32_t = n_threads;
    if n == 0 as uint32_t {
        n = if tp.n_fast < PAR_DEFAULT_N as uint32_t {
            tp.n_fast
        } else {
            PAR_DEFAULT_N as uint32_t
        };
    }
    if n > tp.n_cpu {
        n = tp.n_cpu;
    }
    let mut o_slots: uint64_t = up(::core::mem::size_of::<toks_par>() as uint64_t);
    let mut o_thr: uint64_t = o_slots
        .wrapping_add(
            up((n as uint64_t).wrapping_mul(::core::mem::size_of::<slot>() as uint64_t)),
        );
    let mut bytes: uint64_t = o_thr
        .wrapping_add(
            up((n as uint64_t).wrapping_mul(::core::mem::size_of::<thr_t>() as uint64_t)),
        );
    let mut m: *mut uint8_t = toks_plat_arena(bytes);
    if m.is_null() {
        return TOKS_E_NOMEM as int64_t;
    }
    let mut p: *mut toks_par = m as *mut ::core::ffi::c_void as *mut toks_par;
    (*p).ctx = ctx;
    (*p).n = n;
    (*p).n_fast = tp.n_fast;
    (*p).scr_flags = scratch_flags;
    (*p).mem = m;
    (*p).mem_bytes = bytes;
    (*p).slots = m.offset(o_slots as isize) as *mut ::core::ffi::c_void as *mut slot;
    (*p).thr = m.offset(o_thr as isize) as *mut ::core::ffi::c_void as *mut thr_t;
    (*p).c_ser = PAR_C0 as uint64_t;
    (*p).c_ps = (*p).c_ser;
    (*p).eps = PAR_EPS0 as uint64_t;
    (*p).wake0 = PAR_WAKE0 as uint64_t;
    (*p).lat_wake = (*p).wake0;
    (*p).o_wake = (*p).lat_wake;
    (*p).join0 = PAR_JOIN0 as uint64_t;
    (*p).o_join = (*p).join0;
    crate::atomic::atomic_store_seqcst(
        &raw mut (*p).spin_ns,
        30000 as ::core::ffi::c_uint as uint64_t,
    );
    let mut ev: [::core::ffi::c_char; 8] = [0; 8];
    (*p).eager = (toks_plat_getenv(
        b"TOKS_PAR_EAGER\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut ev as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 8]>() as uint64_t,
    ) == 1 as int64_t
        && ev[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int == '1' as i32)
        as ::core::ffi::c_int as uint32_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < n {
        let ref mut fresh0 = (*(*p).slots.offset(i as isize)).par;
        *fresh0 = p;
        (*(*p).slots.offset(i as isize)).id = i;
        i = i.wrapping_add(1);
    }
    let mut i_0: uint32_t = 1 as uint32_t;
    while i_0 < n {
        if thr_start(
            (*p).thr.offset(i_0.wrapping_sub(1 as uint32_t) as isize) as *mut thr_t,
            (*p).slots.offset(i_0 as isize) as *mut slot,
            &raw mut tp,
            (i_0 < tp.n_fast) as ::core::ffi::c_int,
        ) != 0 as ::core::ffi::c_int
        {
            (*p).n = i_0;
            toks_par_destroy(p as *mut toks_par);
            return TOKS_E_NOMEM as int64_t;
        }
        i_0 = i_0.wrapping_add(1);
    }
    if n > 1 as uint32_t {
        calibrate(p);
    }
    *out = p as *mut toks_par;
    return 0 as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_par_destroy(mut p: *mut toks_par) {
    if p.is_null() {
        return;
    }
    crate::atomic::atomic_store_seqcst(
        &raw mut (*p).quit.v,
        1 as ::core::ffi::c_uint,
    );
    let mut i: uint32_t = 1 as uint32_t;
    while i < (*p).n {
        crate::atomic::atomic_xadd_seqcst(
            &raw mut (*(*p).slots.offset(i as isize)).word,
            1 as ::core::ffi::c_uint,
        );
        addr_wake(&raw mut (*(*p).slots.offset(i as isize)).word);
        i = i.wrapping_add(1);
    }
    let mut i_0: uint32_t = 1 as uint32_t;
    while i_0 < (*p).n {
        thr_join(*(*p).thr.offset(i_0.wrapping_sub(1 as uint32_t) as isize));
        i_0 = i_0.wrapping_add(1);
    }
    let mut i_1: uint32_t = 0 as uint32_t;
    while i_1 < (*p).n {
        toks_plat_arena_free(
            (*(*p).slots.offset(i_1 as isize)).scr,
            (*(*p).slots.offset(i_1 as isize)).scr_bytes,
        );
        i_1 = i_1.wrapping_add(1);
    }
    toks_plat_arena_free(
        (*p).stage as *mut uint8_t,
        (*p).stage_ids.wrapping_mul(4 as uint64_t),
    );
    toks_plat_free(
        (*p).units as *mut ::core::ffi::c_void,
        (*p).units_cap.wrapping_mul(::core::mem::size_of::<unit>() as uint64_t),
    );
    toks_plat_free(
        (*p).parts as *mut ::core::ffi::c_void,
        (*p).parts_cap.wrapping_mul(::core::mem::size_of::<part>() as uint64_t),
    );
    toks_plat_free(
        (*p).cuts as *mut ::core::ffi::c_void,
        (*p).cuts_cap.wrapping_mul(8 as uint64_t),
    );
    toks_plat_arena_free((*p).mem, (*p).mem_bytes);
}
unsafe extern "C" fn flags_ok(mut flags: uint32_t) -> ::core::ffi::c_int {
    return (flags
        & !(TOKS_ADDED_MASK as uint32_t | TOKS_NO_POSTPROCESS as uint32_t
            | TOKS_CONTINUATION as uint32_t | TOKS_NO_TRUNCATE as uint32_t
            | TOKS_NO_PAD as uint32_t) == 0 as uint32_t
        && flags & TOKS_ADDED_MASK as uint32_t != TOKS_ADDED_MASK as uint32_t)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn job_begin(
    mut p: *mut toks_par,
    mut items: *mut toks_par_item,
    mut n_items: uint64_t,
    mut flags: uint32_t,
) {
    (*p).flags = flags;
    (*p).items = items;
    (*p).n_items = n_items;
    toks_pp_ids(
        (*p).ctx,
        flags,
        &raw mut (*p).pre,
        &raw mut (*p).n_pre,
        &raw mut (*p).suf,
        &raw mut (*p).n_suf,
    );
    (*p).n_units = 0 as uint64_t;
    (*p).n_parts = 0 as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_par_encode_batch(
    mut p: *mut toks_par,
    mut items: *mut toks_par_item,
    mut n_items: uint64_t,
    mut flags: uint32_t,
) -> int64_t {
    if p.is_null() || items.is_null() && n_items != 0 as uint64_t || flags_ok(flags) == 0
    {
        return TOKS_E_ARG as int64_t;
    }
    lock(p as *mut toks_par);
    let mut bytes: uint64_t = 0 as uint64_t;
    let mut i: uint64_t = 0 as uint64_t;
    while i < n_items {
        bytes = bytes
            .wrapping_add(item_bytes(items.offset(i as isize) as *mut toks_par_item));
        i = i.wrapping_add(1);
    }
    job_begin(p as *mut toks_par, items, n_items, flags);
    run_items(p as *mut toks_par, bytes);
    unlock(p as *mut toks_par);
    return 0 as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn toks_par_encode(
    mut p: *mut toks_par,
    mut text: *const ::core::ffi::c_void,
    mut len: uint64_t,
    mut flags: uint32_t,
    mut out: *mut uint32_t,
    mut cap: uint64_t,
) -> int64_t {
    if p.is_null() {
        return TOKS_E_ARG as int64_t;
    }
    if flags_ok(flags) == 0 || out.is_null() && cap != 0 as uint64_t {
        return toks_encode((*p).ctx, text, len, flags, out, cap, NULL);
    }
    let mut one: toks_par_item = toks_par_item {
        text: text,
        len: len,
        out: out,
        cap: cap,
        n: 0 as int64_t,
    };
    lock(p as *mut toks_par);
    job_begin(p as *mut toks_par, &raw mut one, 1 as uint64_t, flags);
    run_items(p as *mut toks_par, item_bytes(&raw mut one));
    unlock(p as *mut toks_par);
    return one.n;
}
#[no_mangle]
pub unsafe extern "C" fn toks_par_get_info(
    mut par: *const toks_par,
    mut out: *mut toks_par_info,
) -> int64_t {
    if par.is_null() || out.is_null()
        || ((*out).size as usize) < ::core::mem::size_of::<toks_par_info>() as usize
    {
        return TOKS_E_ARG as int64_t;
    }
    let mut p: *mut toks_par = par as uintptr_t as *mut toks_par;
    let mut in_0: toks_par_info = toks_par_info {
        size: 0,
        threads: 0,
        fast: 0,
        last: 0,
        ns_per_mib: 0,
        wake_ns: 0,
        join_ns: 0,
        min_bytes: 0,
    };
    memset(
        &raw mut in_0 as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<toks_par_info>() as size_t,
    );
    in_0.size = ::core::mem::size_of::<toks_par_info>() as uint32_t;
    lock(p);
    in_0.threads = (*p).n;
    in_0.fast = (*p).n_fast;
    in_0.last = (*p).last_k;
    in_0.ns_per_mib = c_dec(p)
        .wrapping_mul(1048576 as uint64_t)
        .wrapping_div(1000 as uint64_t);
    in_0.wake_ns = (*p).o_wake;
    in_0.join_ns = (*p).o_join;
    if (*p).n > 1 as uint32_t {
        let mut lo: uint64_t = PAR_MIN_BYTES as uint64_t;
        let mut hi: uint64_t = TOKS_MAX_TEXT as uint64_t;
        let mut now: uint64_t = now_ns();
        if choose_k(p, lo, now) > 1 as uint32_t {
            hi = lo;
        } else if choose_k(p, hi, now) == 1 as uint32_t {
            hi = 0 as uint64_t;
            lo = hi;
        }
        while lo < hi {
            let mut mid: uint64_t = lo
                .wrapping_add(hi.wrapping_sub(lo).wrapping_div(2 as uint64_t));
            if choose_k(p, mid, now) > 1 as uint32_t {
                hi = mid;
            } else {
                lo = mid.wrapping_add(1 as uint64_t);
            }
        }
        in_0.min_bytes = hi;
    }
    unlock(p);
    memcpy(
        out as *mut ::core::ffi::c_void,
        &raw mut in_0 as *const ::core::ffi::c_void,
        ::core::mem::size_of::<toks_par_info>() as size_t,
    );
    return 0 as int64_t;
}
pub const __ATOMIC_SEQ_CST: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
    ::core::ffi::c_void,
>();

#[cfg(target_os = "macos")]
extern "C" {
    fn __ulock_wait(op: u32, addr: *mut libc::c_void, val: u64, timeout: u32) -> i32;
    fn __ulock_wake(op: u32, addr: *mut libc::c_void, val: u64) -> i32;
    fn qos_class_self() -> u32;
    fn pthread_attr_set_qos_class_np(attr: *mut libc::pthread_attr_t, class: u32, rel: i32) -> i32;
}
#[cfg(target_os = "macos")]
unsafe extern "C" fn addr_wait(a: *mut u32, v: u32) { __ulock_wait(0x01000001, a.cast(), v as u64, 0); }
#[cfg(target_os = "macos")]
unsafe extern "C" fn addr_wake(a: *mut u32) { __ulock_wake(0x01000001, a.cast(), 0); }
#[cfg(target_os = "macos")]
unsafe extern "C" fn topo_read(t: *mut topo) {
    let n = libc::sysconf(libc::_SC_NPROCESSORS_ONLN).clamp(1, 1024) as u32;
    let mut pc = 0i32;
    let mut len = std::mem::size_of_val(&pc);
    libc::sysctlbyname(c"hw.perflevel0.logicalcpu".as_ptr(), (&mut pc as *mut i32).cast(), &mut len, std::ptr::null_mut(), 0);
    (*t).n_cpu = n;
    (*t).n_fast = if pc > 0 && (pc as u32) < n { pc as u32 } else { n };
}
#[cfg(target_os = "macos")]
unsafe extern "C" fn thr_start(t: *mut thr_t, s: *mut slot, _: *const topo, _: i32) -> i32 {
    let mut attr = std::mem::MaybeUninit::<libc::pthread_attr_t>::uninit();
    if libc::pthread_attr_init(attr.as_mut_ptr()) != 0 { return -1; }
    pthread_attr_set_qos_class_np(attr.as_mut_ptr(), qos_class_self(), 0);
    extern "C" fn run(p: *mut libc::c_void) -> *mut libc::c_void { unsafe { thr_main(p) } }
    let r = libc::pthread_create(t, attr.as_ptr(), run, s.cast());
    libc::pthread_attr_destroy(attr.as_mut_ptr());
    r
}
