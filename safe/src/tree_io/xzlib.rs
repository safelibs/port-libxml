use crate::tree_io::common::{
    xz_close_budget, xz_open_budget, xz_record_loop, xz_record_output, xz_record_read,
    xz_record_terminal_error,
};

#[repr(C)]
pub struct internal_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lzma_internal_s {
    _private: [u8; 0],
}

extern "C" {
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn lseek(
        __fd: ::core::ffi::c_int,
        __offset: __off64_t,
        __whence: ::core::ffi::c_int,
    ) -> __off64_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn inflate(strm: z_streamp, flush: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn inflateEnd(strm: z_streamp) -> ::core::ffi::c_int;
    fn inflateReset(strm: z_streamp) -> ::core::ffi::c_int;
    fn crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
    fn inflateInit2_(
        strm: z_streamp,
        windowBits: ::core::ffi::c_int,
        version: *const ::core::ffi::c_char,
        stream_size: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn lzma_code(strm: *mut lzma_stream, action: lzma_action) -> lzma_ret;
    fn lzma_end(strm: *mut lzma_stream);
    fn lzma_properties_decode(
        filter: *mut lzma_filter,
        allocator: *const lzma_allocator,
        props: *const uint8_t,
        props_size: size_t,
    ) -> lzma_ret;
    fn lzma_auto_decoder(strm: *mut lzma_stream, memlimit: uint64_t, flags: uint32_t) -> lzma_ret;
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlFree: xmlFreeFunc;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off64_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type ssize_t = __ssize_t;
pub type Byte = ::core::ffi::c_uchar;
pub type uInt = ::core::ffi::c_uint;
pub type uLong = ::core::ffi::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut ::core::ffi::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut ::core::ffi::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: ::core::ffi::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type lzma_reserved_enum = ::core::ffi::c_uint;
pub const LZMA_RESERVED_ENUM: lzma_reserved_enum = 0;
pub type lzma_ret = ::core::ffi::c_uint;
pub const LZMA_RET_INTERNAL8: lzma_ret = 108;
pub const LZMA_RET_INTERNAL7: lzma_ret = 107;
pub const LZMA_RET_INTERNAL6: lzma_ret = 106;
pub const LZMA_RET_INTERNAL5: lzma_ret = 105;
pub const LZMA_RET_INTERNAL4: lzma_ret = 104;
pub const LZMA_RET_INTERNAL3: lzma_ret = 103;
pub const LZMA_RET_INTERNAL2: lzma_ret = 102;
pub const LZMA_RET_INTERNAL1: lzma_ret = 101;
pub const LZMA_SEEK_NEEDED: lzma_ret = 12;
pub const LZMA_PROG_ERROR: lzma_ret = 11;
pub const LZMA_BUF_ERROR: lzma_ret = 10;
pub const LZMA_DATA_ERROR: lzma_ret = 9;
pub const LZMA_OPTIONS_ERROR: lzma_ret = 8;
pub const LZMA_FORMAT_ERROR: lzma_ret = 7;
pub const LZMA_MEMLIMIT_ERROR: lzma_ret = 6;
pub const LZMA_MEM_ERROR: lzma_ret = 5;
pub const LZMA_GET_CHECK: lzma_ret = 4;
pub const LZMA_UNSUPPORTED_CHECK: lzma_ret = 3;
pub const LZMA_NO_CHECK: lzma_ret = 2;
pub const LZMA_STREAM_END: lzma_ret = 1;
pub const LZMA_OK: lzma_ret = 0;
pub type lzma_action = ::core::ffi::c_uint;
pub const LZMA_FINISH: lzma_action = 3;
pub const LZMA_FULL_BARRIER: lzma_action = 4;
pub const LZMA_FULL_FLUSH: lzma_action = 2;
pub const LZMA_SYNC_FLUSH: lzma_action = 1;
pub const LZMA_RUN: lzma_action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_allocator {
    pub alloc: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t, size_t) -> *mut ::core::ffi::c_void,
    >,
    pub free:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>,
    pub opaque: *mut ::core::ffi::c_void,
}
pub type lzma_internal = lzma_internal_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_stream {
    pub next_in: *const uint8_t,
    pub avail_in: size_t,
    pub total_in: uint64_t,
    pub next_out: *mut uint8_t,
    pub avail_out: size_t,
    pub total_out: uint64_t,
    pub allocator: *const lzma_allocator,
    pub internal: *mut lzma_internal,
    pub reserved_ptr1: *mut ::core::ffi::c_void,
    pub reserved_ptr2: *mut ::core::ffi::c_void,
    pub reserved_ptr3: *mut ::core::ffi::c_void,
    pub reserved_ptr4: *mut ::core::ffi::c_void,
    pub seek_pos: uint64_t,
    pub reserved_int2: uint64_t,
    pub reserved_int3: size_t,
    pub reserved_int4: size_t,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
}
pub type lzma_vli = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter {
    pub id: lzma_vli,
    pub options: *mut ::core::ffi::c_void,
}
pub type lzma_match_finder = ::core::ffi::c_uint;
pub const LZMA_MF_BT4: lzma_match_finder = 20;
pub const LZMA_MF_BT3: lzma_match_finder = 19;
pub const LZMA_MF_BT2: lzma_match_finder = 18;
pub const LZMA_MF_HC4: lzma_match_finder = 4;
pub const LZMA_MF_HC3: lzma_match_finder = 3;
pub type lzma_mode = ::core::ffi::c_uint;
pub const LZMA_MODE_NORMAL: lzma_mode = 2;
pub const LZMA_MODE_FAST: lzma_mode = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_options_lzma {
    pub dict_size: uint32_t,
    pub preset_dict: *const uint8_t,
    pub preset_dict_size: uint32_t,
    pub lc: uint32_t,
    pub lp: uint32_t,
    pub pb: uint32_t,
    pub mode: lzma_mode,
    pub nice_len: uint32_t,
    pub mf: lzma_match_finder,
    pub depth: uint32_t,
    pub ext_flags: uint32_t,
    pub ext_size_low: uint32_t,
    pub ext_size_high: uint32_t,
    pub reserved_int4: uint32_t,
    pub reserved_int5: uint32_t,
    pub reserved_int6: uint32_t,
    pub reserved_int7: uint32_t,
    pub reserved_int8: uint32_t,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
    pub reserved_enum3: lzma_reserved_enum,
    pub reserved_enum4: lzma_reserved_enum,
    pub reserved_ptr1: *mut ::core::ffi::c_void,
    pub reserved_ptr2: *mut ::core::ffi::c_void,
}
pub type xzFile = *mut ::core::ffi::c_void;
pub type xz_statep = *mut xz_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xz_state {
    pub mode: ::core::ffi::c_int,
    pub fd: ::core::ffi::c_int,
    pub path: *mut ::core::ffi::c_char,
    pub pos: uint64_t,
    pub size: ::core::ffi::c_uint,
    pub want: ::core::ffi::c_uint,
    pub in_0: *mut ::core::ffi::c_uchar,
    pub out: *mut ::core::ffi::c_uchar,
    pub next: *mut ::core::ffi::c_uchar,
    pub have: ::core::ffi::c_uint,
    pub eof: ::core::ffi::c_int,
    pub start: uint64_t,
    pub raw: uint64_t,
    pub how: ::core::ffi::c_int,
    pub direct: ::core::ffi::c_int,
    pub skip: uint64_t,
    pub seek: ::core::ffi::c_int,
    pub err: ::core::ffi::c_int,
    pub msg: *mut ::core::ffi::c_char,
    pub init: ::core::ffi::c_int,
    pub strm: lzma_stream,
    pub padding1: [::core::ffi::c_char; 32],
    pub zstrm: z_stream,
    pub padding2: [::core::ffi::c_char; 32],
}
pub type xmlMallocFunc = Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
pub type xmlFreeFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub const BUFSIZ: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ZLIB_VERSION: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"1.3\0") };
pub const Z_NO_FLUSH: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Z_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const Z_STREAM_END: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const Z_NEED_DICT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const Z_STREAM_ERROR: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
pub const Z_DATA_ERROR: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
pub const Z_MEM_ERROR: ::core::ffi::c_int = -(4 as ::core::ffi::c_int);
pub const Z_NULL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const LOOK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const COPY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const GZIP: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LZMA: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
unsafe extern "C" fn xz_error(
    mut state: xz_statep,
    mut err: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    if !(*state).msg.is_null() {
        if (*state).err != LZMA_MEM_ERROR as ::core::ffi::c_int {
            xmlFree.expect("non-null function pointer")((*state).msg as *mut ::core::ffi::c_void);
        }
        (*state).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    (*state).err = err;
    if msg.is_null() {
        return;
    }
    if err == LZMA_MEM_ERROR as ::core::ffi::c_int {
        (*state).msg = msg as *mut ::core::ffi::c_char;
        return;
    }
    (*state).msg = xmlMalloc.expect("non-null function pointer")(
        strlen((*state).path)
            .wrapping_add(strlen(msg))
            .wrapping_add(3 as size_t),
    ) as *mut ::core::ffi::c_char;
    if (*state).msg.is_null() {
        (*state).err = LZMA_MEM_ERROR as ::core::ffi::c_int;
        (*state).msg = b"out of memory\0" as *const u8 as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char;
        return;
    }
    strcpy((*state).msg, (*state).path);
    strcat(
        (*state).msg,
        b": \0" as *const u8 as *const ::core::ffi::c_char,
    );
    strcat((*state).msg, msg);
}
unsafe extern "C" fn xz_reset(mut state: xz_statep) {
    (*state).have = 0 as ::core::ffi::c_uint;
    (*state).eof = 0 as ::core::ffi::c_int;
    (*state).how = LOOK;
    (*state).direct = 1 as ::core::ffi::c_int;
    (*state).seek = 0 as ::core::ffi::c_int;
    xz_error(
        state,
        LZMA_OK as ::core::ffi::c_int,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    (*state).pos = 0 as uint64_t;
    (*state).strm.avail_in = 0 as size_t;
    (*state).zstrm.avail_in = 0 as uInt;
}
unsafe extern "C" fn xz_open(
    mut path: *const ::core::ffi::c_char,
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> xzFile {
    let mut state: xz_statep = ::core::ptr::null_mut::<xz_state>();
    state =
        xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xz_state>() as size_t)
            as xz_statep;
    if state.is_null() {
        return NULL;
    }
    (*state).size = 0 as ::core::ffi::c_uint;
    (*state).want = BUFSIZ as ::core::ffi::c_uint;
    (*state).msg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*state).init = 0 as ::core::ffi::c_int;
    (*state).path =
        xmlMalloc.expect("non-null function pointer")(strlen(path).wrapping_add(1 as size_t))
            as *mut ::core::ffi::c_char;
    if (*state).path.is_null() {
        xmlFree.expect("non-null function pointer")(state as *mut ::core::ffi::c_void);
        return NULL;
    }
    strcpy((*state).path, path);
    (*state).fd = if fd != -(1 as ::core::ffi::c_int) {
        fd
    } else {
        open(path, O_RDONLY, 0o666 as ::core::ffi::c_int)
    };
    if (*state).fd == -(1 as ::core::ffi::c_int) {
        xmlFree.expect("non-null function pointer")((*state).path as *mut ::core::ffi::c_void);
        xmlFree.expect("non-null function pointer")(state as *mut ::core::ffi::c_void);
        return NULL;
    }
    (*state).start = lseek((*state).fd, 0 as __off64_t, SEEK_CUR) as uint64_t;
    if (*state).start == -(1 as ::core::ffi::c_int) as uint64_t {
        (*state).start = 0 as uint64_t;
    }
    xz_reset(state);
    return state as xzFile;
}
unsafe extern "C" fn xz_compressed(mut f: xzFile) -> ::core::ffi::c_int {
    let mut state: xz_statep = ::core::ptr::null_mut::<xz_state>();
    if f.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    state = f as xz_statep;
    if (*state).init <= 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    match (*state).how {
        COPY => return 0 as ::core::ffi::c_int,
        GZIP | LZMA => return 1 as ::core::ffi::c_int,
        _ => {}
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn __libxml2_xzopen(
    mut path: *const ::core::ffi::c_char,
    mut mode: *const ::core::ffi::c_char,
) -> xzFile {
    let file = xz_open(path, -(1 as ::core::ffi::c_int), mode);
    xz_open_budget(file);
    return file;
}
#[no_mangle]
pub unsafe extern "C" fn __libxml2_xzcompressed(mut f: xzFile) -> ::core::ffi::c_int {
    return xz_compressed(f);
}
#[no_mangle]
pub unsafe extern "C" fn __libxml2_xzdopen(
    mut fd: ::core::ffi::c_int,
    mut mode: *const ::core::ffi::c_char,
) -> xzFile {
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut xz: xzFile = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if fd == -(1 as ::core::ffi::c_int) || {
        path = xmlMalloc.expect("non-null function pointer")((7 as size_t).wrapping_add(
            (3 as size_t).wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
        )) as *mut ::core::ffi::c_char;
        path.is_null()
    } {
        return NULL;
    }
    sprintf(
        path,
        b"<fd:%d>\0" as *const u8 as *const ::core::ffi::c_char,
        fd,
    );
    xz = xz_open(path, fd, mode);
    xz_open_budget(xz);
    xmlFree.expect("non-null function pointer")(path as *mut ::core::ffi::c_void);
    return xz;
}
unsafe extern "C" fn xz_load(
    mut state: xz_statep,
    mut buf: *mut ::core::ffi::c_uchar,
    mut len: ::core::ffi::c_uint,
    mut have: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    *have = 0 as ::core::ffi::c_uint;
    loop {
        ret = read(
            (*state).fd,
            buf.offset(*have as isize) as *mut ::core::ffi::c_void,
            len.wrapping_sub(*have) as size_t,
        ) as ::core::ffi::c_int;
        if ret <= 0 as ::core::ffi::c_int {
            break;
        }
        *have = (*have).wrapping_add(ret as ::core::ffi::c_uint);
        if !(*have < len) {
            break;
        }
    }
    if ret < 0 as ::core::ffi::c_int {
        xz_error(
            state,
            -(1 as ::core::ffi::c_int),
            strerror(*__errno_location()),
        );
        return -(1 as ::core::ffi::c_int);
    }
    if ret == 0 as ::core::ffi::c_int {
        (*state).eof = 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xz_avail(mut state: xz_statep) -> ::core::ffi::c_int {
    let mut strm: *mut lzma_stream = &raw mut (*state).strm;
    if (*state).err != LZMA_OK as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if (*state).eof == 0 as ::core::ffi::c_int {
        let mut tmp: ::core::ffi::c_uint = (*strm).avail_in as ::core::ffi::c_uint;
        if xz_load(state, (*state).in_0, (*state).size, &raw mut tmp) == -(1 as ::core::ffi::c_int)
        {
            (*strm).avail_in = tmp as size_t;
            return -(1 as ::core::ffi::c_int);
        }
        (*strm).avail_in = tmp as size_t;
        (*strm).next_in = (*state).in_0;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xz_avail_zstrm(mut state: xz_statep) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    (*state).strm.avail_in = (*state).zstrm.avail_in as size_t;
    (*state).strm.next_in = (*state).zstrm.next_in;
    ret = xz_avail(state);
    (*state).zstrm.avail_in = (*state).strm.avail_in as uInt;
    (*state).zstrm.next_in = (*state).strm.next_in as *mut Bytef;
    return ret;
}
unsafe extern "C" fn is_format_xz(mut state: xz_statep) -> ::core::ffi::c_int {
    let mut strm: *mut lzma_stream = &raw mut (*state).strm;
    return ((*strm).avail_in >= 6 as size_t
        && memcmp(
            (*state).in_0 as *const ::core::ffi::c_void,
            b"\xFD7zXZ\0" as *const u8 as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            6 as size_t,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn is_format_lzma(mut state: xz_statep) -> ::core::ffi::c_int {
    let mut strm: *mut lzma_stream = &raw mut (*state).strm;
    let mut filter: lzma_filter = lzma_filter {
        id: 0,
        options: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    let mut opt: *mut lzma_options_lzma = ::core::ptr::null_mut::<lzma_options_lzma>();
    let mut dict_size: uint32_t = 0;
    let mut uncompressed_size: uint64_t = 0;
    let mut i: size_t = 0;
    if (*strm).avail_in < 13 as size_t {
        return 0 as ::core::ffi::c_int;
    }
    filter.id = 0x4000000000000001 as ::core::ffi::c_ulong as lzma_vli;
    if lzma_properties_decode(
        &raw mut filter,
        ::core::ptr::null::<lzma_allocator>(),
        (*state).in_0,
        5 as size_t,
    ) as ::core::ffi::c_uint
        != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    opt = filter.options as *mut lzma_options_lzma;
    dict_size = (*opt).dict_size;
    free(opt as *mut ::core::ffi::c_void);
    if dict_size != UINT32_MAX as uint32_t {
        let mut d: uint32_t = dict_size.wrapping_sub(1 as uint32_t);
        d |= d >> 2 as ::core::ffi::c_int;
        d |= d >> 3 as ::core::ffi::c_int;
        d |= d >> 4 as ::core::ffi::c_int;
        d |= d >> 8 as ::core::ffi::c_int;
        d |= d >> 16 as ::core::ffi::c_int;
        d = d.wrapping_add(1);
        if d != dict_size || dict_size == 0 as uint32_t {
            return 0 as ::core::ffi::c_int;
        }
    }
    uncompressed_size = 0 as uint64_t;
    i = 0 as size_t;
    while i < 8 as size_t {
        uncompressed_size |= (*(*state).in_0.offset((5 as size_t).wrapping_add(i) as isize)
            as uint64_t)
            << i.wrapping_mul(8 as size_t);
        i = i.wrapping_add(1);
    }
    if uncompressed_size != UINT64_MAX as uint64_t
        && uncompressed_size > (1 as uint64_t) << 38 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn gz_next4(
    mut state: xz_statep,
    mut ret: *mut ::core::ffi::c_ulong,
) -> ::core::ffi::c_int {
    let mut ch: ::core::ffi::c_int = 0;
    let mut val: ::core::ffi::c_ulong = 0;
    let mut strm: z_streamp = &raw mut (*state).zstrm;
    val = (if (*strm).avail_in == 0 as uInt && xz_avail_zstrm(state) == -(1 as ::core::ffi::c_int) {
        -(1 as ::core::ffi::c_int)
    } else if (*strm).avail_in == 0 as uInt {
        -(1 as ::core::ffi::c_int)
    } else {
        (*strm).avail_in = (*strm).avail_in.wrapping_sub(1);
        let fresh0 = (*strm).next_in;
        (*strm).next_in = (*strm).next_in.offset(1);
        *fresh0 as ::core::ffi::c_int
    }) as ::core::ffi::c_ulong;
    val = val.wrapping_add(
        (((if (*strm).avail_in == 0 as uInt && xz_avail_zstrm(state) == -(1 as ::core::ffi::c_int) {
            -(1 as ::core::ffi::c_int)
        } else {
            (if (*strm).avail_in == 0 as uInt {
                -(1 as ::core::ffi::c_int)
            } else {
                (*strm).avail_in = (*strm).avail_in.wrapping_sub(1);
                let fresh1 = (*strm).next_in;
                (*strm).next_in = (*strm).next_in.offset(1);
                *fresh1 as ::core::ffi::c_int
            })
        }) as ::core::ffi::c_uint)
            << 8 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
    );
    val = val.wrapping_add(
        ((if (*strm).avail_in == 0 as uInt && xz_avail_zstrm(state) == -(1 as ::core::ffi::c_int) {
            -(1 as ::core::ffi::c_int)
        } else {
            (if (*strm).avail_in == 0 as uInt {
                -(1 as ::core::ffi::c_int)
            } else {
                (*strm).avail_in = (*strm).avail_in.wrapping_sub(1);
                let fresh2 = (*strm).next_in;
                (*strm).next_in = (*strm).next_in.offset(1);
                *fresh2 as ::core::ffi::c_int
            })
        }) as ::core::ffi::c_ulong)
            << 16 as ::core::ffi::c_int,
    );
    ch = if (*strm).avail_in == 0 as uInt && xz_avail_zstrm(state) == -(1 as ::core::ffi::c_int) {
        -(1 as ::core::ffi::c_int)
    } else if (*strm).avail_in == 0 as uInt {
        -(1 as ::core::ffi::c_int)
    } else {
        (*strm).avail_in = (*strm).avail_in.wrapping_sub(1);
        let fresh3 = (*strm).next_in;
        (*strm).next_in = (*strm).next_in.offset(1);
        *fresh3 as ::core::ffi::c_int
    };
    if ch == -(1 as ::core::ffi::c_int) {
        return -(1 as ::core::ffi::c_int);
    }
    val = val.wrapping_add((ch as ::core::ffi::c_ulong) << 24 as ::core::ffi::c_int);
    *ret = val;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xz_head(mut state: xz_statep) -> ::core::ffi::c_int {
    let mut strm: *mut lzma_stream = &raw mut (*state).strm;
    let mut init: lzma_stream = lzma_stream {
        next_in: ::core::ptr::null::<uint8_t>(),
        avail_in: 0 as size_t,
        total_in: 0 as uint64_t,
        next_out: ::core::ptr::null_mut::<uint8_t>(),
        avail_out: 0 as size_t,
        total_out: 0 as uint64_t,
        allocator: ::core::ptr::null::<lzma_allocator>(),
        internal: ::core::ptr::null_mut::<lzma_internal>(),
        reserved_ptr1: NULL,
        reserved_ptr2: NULL,
        reserved_ptr3: NULL,
        reserved_ptr4: NULL,
        seek_pos: 0 as uint64_t,
        reserved_int2: 0 as uint64_t,
        reserved_int3: 0 as size_t,
        reserved_int4: 0 as size_t,
        reserved_enum1: LZMA_RESERVED_ENUM,
        reserved_enum2: LZMA_RESERVED_ENUM,
    };
    let mut flags: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_uint = 0;
    if (*state).size == 0 as ::core::ffi::c_uint {
        (*state).in_0 = xmlMalloc.expect("non-null function pointer")((*state).want as size_t)
            as *mut ::core::ffi::c_uchar;
        (*state).out = xmlMalloc.expect("non-null function pointer")(
            ((*state).want << 1 as ::core::ffi::c_int) as size_t,
        ) as *mut ::core::ffi::c_uchar;
        if (*state).in_0.is_null() || (*state).out.is_null() {
            if !(*state).out.is_null() {
                xmlFree.expect("non-null function pointer")(
                    (*state).out as *mut ::core::ffi::c_void,
                );
            }
            if !(*state).in_0.is_null() {
                xmlFree.expect("non-null function pointer")(
                    (*state).in_0 as *mut ::core::ffi::c_void,
                );
            }
            xz_error(
                state,
                LZMA_MEM_ERROR as ::core::ffi::c_int,
                b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        (*state).size = (*state).want;
        (*state).strm = init;
        (*state).strm.avail_in = 0 as size_t;
        (*state).strm.next_in = ::core::ptr::null::<uint8_t>();
        if lzma_auto_decoder(&raw mut (*state).strm, 100000000 as uint64_t, 0 as uint32_t)
            as ::core::ffi::c_uint
            != LZMA_OK as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            xmlFree.expect("non-null function pointer")((*state).out as *mut ::core::ffi::c_void);
            xmlFree.expect("non-null function pointer")((*state).in_0 as *mut ::core::ffi::c_void);
            (*state).size = 0 as ::core::ffi::c_uint;
            xz_error(
                state,
                LZMA_MEM_ERROR as ::core::ffi::c_int,
                b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        (*state).zstrm.zalloc = None;
        (*state).zstrm.zfree = None;
        (*state).zstrm.opaque = ::core::ptr::null_mut::<::core::ffi::c_void>();
        (*state).zstrm.avail_in = 0 as uInt;
        (*state).zstrm.next_in = ::core::ptr::null_mut::<Bytef>();
        if (*state).init == 0 as ::core::ffi::c_int {
            if inflateInit2_(
                &raw mut (*state).zstrm,
                -(15 as ::core::ffi::c_int),
                ZLIB_VERSION.as_ptr(),
                ::core::mem::size_of::<z_stream>() as ::core::ffi::c_int,
            ) != Z_OK
            {
                xmlFree.expect("non-null function pointer")(
                    (*state).out as *mut ::core::ffi::c_void,
                );
                xmlFree.expect("non-null function pointer")(
                    (*state).in_0 as *mut ::core::ffi::c_void,
                );
                (*state).size = 0 as ::core::ffi::c_uint;
                xz_error(
                    state,
                    LZMA_MEM_ERROR as ::core::ffi::c_int,
                    b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            (*state).init = 1 as ::core::ffi::c_int;
        }
    }
    if (*strm).avail_in == 0 as size_t {
        if xz_avail(state) == -(1 as ::core::ffi::c_int) {
            return -(1 as ::core::ffi::c_int);
        }
        if (*strm).avail_in == 0 as size_t {
            return 0 as ::core::ffi::c_int;
        }
    }
    if is_format_xz(state) != 0 || is_format_lzma(state) != 0 {
        (*state).how = LZMA;
        (*state).direct = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    if *(*strm).next_in.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == 31 as ::core::ffi::c_int
    {
        (*strm).avail_in = (*strm).avail_in.wrapping_sub(1);
        (*strm).next_in = (*strm).next_in.offset(1);
        if (*strm).avail_in == 0 as size_t && xz_avail(state) == -(1 as ::core::ffi::c_int) {
            return -(1 as ::core::ffi::c_int);
        }
        if (*strm).avail_in != 0
            && *(*strm).next_in.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 139 as ::core::ffi::c_int
        {
            (*strm).avail_in = (*strm).avail_in.wrapping_sub(1);
            (*strm).next_in = (*strm).next_in.offset(1);
            if (if (*strm).avail_in == 0 as size_t && xz_avail(state) == -(1 as ::core::ffi::c_int)
            {
                -(1 as ::core::ffi::c_int)
            } else {
                (if (*strm).avail_in == 0 as size_t {
                    -(1 as ::core::ffi::c_int)
                } else {
                    (*strm).avail_in = (*strm).avail_in.wrapping_sub(1);
                    let fresh4 = (*strm).next_in;
                    (*strm).next_in = (*strm).next_in.offset(1);
                    *fresh4 as ::core::ffi::c_int
                })
            }) != 8 as ::core::ffi::c_int
            {
                xz_error(
                    state,
                    LZMA_DATA_ERROR as ::core::ffi::c_int,
                    b"unknown compression method\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            flags = if (*strm).avail_in == 0 as size_t
                && xz_avail(state) == -(1 as ::core::ffi::c_int)
            {
                -(1 as ::core::ffi::c_int)
            } else if (*strm).avail_in == 0 as size_t {
                -(1 as ::core::ffi::c_int)
            } else {
                (*strm).avail_in = (*strm).avail_in.wrapping_sub(1);
                let fresh5 = (*strm).next_in;
                (*strm).next_in = (*strm).next_in.offset(1);
                *fresh5 as ::core::ffi::c_int
            };
            if flags & 0xe0 as ::core::ffi::c_int != 0 {
                xz_error(
                    state,
                    LZMA_DATA_ERROR as ::core::ffi::c_int,
                    b"unknown header flags set\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            if (*strm).avail_in == 0 as size_t && xz_avail(state) == -(1 as ::core::ffi::c_int) {
                -(1 as ::core::ffi::c_int);
            } else {
                if (*strm).avail_in == 0 as size_t {
                    -(1 as ::core::ffi::c_int);
                } else {
                    (*strm).avail_in = (*strm).avail_in.wrapping_sub(1);
                    let fresh6 = (*strm).next_in;
                    (*strm).next_in = (*strm).next_in.offset(1);
                    *fresh6;
                };
            };
            if (*strm).avail_in == 0 as size_t && xz_avail(state) == -(1 as ::core::ffi::c_int) {
                -(1 as ::core::ffi::c_int);
            } else {
                if (*strm).avail_in == 0 as size_t {
                    -(1 as ::core::ffi::c_int);
                } else {
                    (*strm).avail_in = (*strm).avail_in.wrapping_sub(1);
                    let fresh7 = (*strm).next_in;
                    (*strm).next_in = (*strm).next_in.offset(1);
                    *fresh7;
                };
            };
            if (*strm).avail_in == 0 as size_t && xz_avail(state) == -(1 as ::core::ffi::c_int) {
                -(1 as ::core::ffi::c_int);
            } else {
                if (*strm).avail_in == 0 as size_t {
                    -(1 as ::core::ffi::c_int);
                } else {
                    (*strm).avail_in = (*strm).avail_in.wrapping_sub(1);
                    let fresh8 = (*strm).next_in;
                    (*strm).next_in = (*strm).next_in.offset(1);
                    *fresh8;
                };
            };
            if (*strm).avail_in == 0 as size_t && xz_avail(state) == -(1 as ::core::ffi::c_int) {
                -(1 as ::core::ffi::c_int);
            } else {
                if (*strm).avail_in == 0 as size_t {
                    -(1 as ::core::ffi::c_int);
                } else {
                    (*strm).avail_in = (*strm).avail_in.wrapping_sub(1);
                    let fresh9 = (*strm).next_in;
                    (*strm).next_in = (*strm).next_in.offset(1);
                    *fresh9;
                };
            };
            if (*strm).avail_in == 0 as size_t && xz_avail(state) == -(1 as ::core::ffi::c_int) {
                -(1 as ::core::ffi::c_int);
            } else {
                if (*strm).avail_in == 0 as size_t {
                    -(1 as ::core::ffi::c_int);
                } else {
                    (*strm).avail_in = (*strm).avail_in.wrapping_sub(1);
                    let fresh10 = (*strm).next_in;
                    (*strm).next_in = (*strm).next_in.offset(1);
                    *fresh10;
                };
            };
            if (*strm).avail_in == 0 as size_t && xz_avail(state) == -(1 as ::core::ffi::c_int) {
                -(1 as ::core::ffi::c_int);
            } else {
                if (*strm).avail_in == 0 as size_t {
                    -(1 as ::core::ffi::c_int);
                } else {
                    (*strm).avail_in = (*strm).avail_in.wrapping_sub(1);
                    let fresh11 = (*strm).next_in;
                    (*strm).next_in = (*strm).next_in.offset(1);
                    *fresh11;
                };
            };
            if flags & 4 as ::core::ffi::c_int != 0 {
                len = (if (*strm).avail_in == 0 as size_t
                    && xz_avail(state) == -(1 as ::core::ffi::c_int)
                {
                    -(1 as ::core::ffi::c_int)
                } else if (*strm).avail_in == 0 as size_t {
                    -(1 as ::core::ffi::c_int)
                } else {
                    (*strm).avail_in = (*strm).avail_in.wrapping_sub(1);
                    let fresh12 = (*strm).next_in;
                    (*strm).next_in = (*strm).next_in.offset(1);
                    *fresh12 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint;
                len = len.wrapping_add(
                    ((if (*strm).avail_in == 0 as size_t
                        && xz_avail(state) == -(1 as ::core::ffi::c_int)
                    {
                        -(1 as ::core::ffi::c_int)
                    } else {
                        (if (*strm).avail_in == 0 as size_t {
                            -(1 as ::core::ffi::c_int)
                        } else {
                            (*strm).avail_in = (*strm).avail_in.wrapping_sub(1);
                            let fresh13 = (*strm).next_in;
                            (*strm).next_in = (*strm).next_in.offset(1);
                            *fresh13 as ::core::ffi::c_int
                        })
                    }) as ::core::ffi::c_uint)
                        << 8 as ::core::ffi::c_int,
                );
                loop {
                    let fresh14 = len;
                    len = len.wrapping_sub(1);
                    if !(fresh14 != 0) {
                        break;
                    }
                    if (if (*strm).avail_in == 0 as size_t
                        && xz_avail(state) == -(1 as ::core::ffi::c_int)
                    {
                        -(1 as ::core::ffi::c_int)
                    } else {
                        (if (*strm).avail_in == 0 as size_t {
                            -(1 as ::core::ffi::c_int)
                        } else {
                            (*strm).avail_in = (*strm).avail_in.wrapping_sub(1);
                            let fresh15 = (*strm).next_in;
                            (*strm).next_in = (*strm).next_in.offset(1);
                            *fresh15 as ::core::ffi::c_int
                        })
                    }) < 0 as ::core::ffi::c_int
                    {
                        break;
                    }
                }
            }
            if flags & 8 as ::core::ffi::c_int != 0 {
                while (if (*strm).avail_in == 0 as size_t
                    && xz_avail(state) == -(1 as ::core::ffi::c_int)
                {
                    -(1 as ::core::ffi::c_int)
                } else {
                    (if (*strm).avail_in == 0 as size_t {
                        -(1 as ::core::ffi::c_int)
                    } else {
                        (*strm).avail_in = (*strm).avail_in.wrapping_sub(1);
                        let fresh16 = (*strm).next_in;
                        (*strm).next_in = (*strm).next_in.offset(1);
                        *fresh16 as ::core::ffi::c_int
                    })
                }) > 0 as ::core::ffi::c_int
                {}
            }
            if flags & 16 as ::core::ffi::c_int != 0 {
                while (if (*strm).avail_in == 0 as size_t
                    && xz_avail(state) == -(1 as ::core::ffi::c_int)
                {
                    -(1 as ::core::ffi::c_int)
                } else {
                    (if (*strm).avail_in == 0 as size_t {
                        -(1 as ::core::ffi::c_int)
                    } else {
                        (*strm).avail_in = (*strm).avail_in.wrapping_sub(1);
                        let fresh17 = (*strm).next_in;
                        (*strm).next_in = (*strm).next_in.offset(1);
                        *fresh17 as ::core::ffi::c_int
                    })
                }) > 0 as ::core::ffi::c_int
                {}
            }
            if flags & 2 as ::core::ffi::c_int != 0 {
                if (*strm).avail_in == 0 as size_t && xz_avail(state) == -(1 as ::core::ffi::c_int)
                {
                    -(1 as ::core::ffi::c_int);
                } else {
                    if (*strm).avail_in == 0 as size_t {
                        -(1 as ::core::ffi::c_int);
                    } else {
                        (*strm).avail_in = (*strm).avail_in.wrapping_sub(1);
                        let fresh18 = (*strm).next_in;
                        (*strm).next_in = (*strm).next_in.offset(1);
                        *fresh18;
                    };
                };
                if (*strm).avail_in == 0 as size_t && xz_avail(state) == -(1 as ::core::ffi::c_int)
                {
                    -(1 as ::core::ffi::c_int);
                } else {
                    if (*strm).avail_in == 0 as size_t {
                        -(1 as ::core::ffi::c_int);
                    } else {
                        (*strm).avail_in = (*strm).avail_in.wrapping_sub(1);
                        let fresh19 = (*strm).next_in;
                        (*strm).next_in = (*strm).next_in.offset(1);
                        *fresh19;
                    };
                };
            }
            inflateReset(&raw mut (*state).zstrm);
            (*state).zstrm.adler = crc32(0 as uLong, ::core::ptr::null::<Bytef>(), 0 as uInt);
            (*state).how = GZIP;
            (*state).direct = 0 as ::core::ffi::c_int;
            return 0 as ::core::ffi::c_int;
        } else {
            *(*state).out.offset(0 as ::core::ffi::c_int as isize) = 31 as ::core::ffi::c_uchar;
            (*state).have = 1 as ::core::ffi::c_uint;
        }
    }
    (*state).raw = (*state).pos;
    (*state).next = (*state).out;
    if (*strm).avail_in != 0 {
        memcpy(
            (*state).next.offset((*state).have as isize) as *mut ::core::ffi::c_void,
            (*strm).next_in as *const ::core::ffi::c_void,
            (*strm).avail_in,
        );
        (*state).have = ((*state).have as size_t).wrapping_add((*strm).avail_in)
            as ::core::ffi::c_uint as ::core::ffi::c_uint;
        (*strm).avail_in = 0 as size_t;
    }
    (*state).how = COPY;
    (*state).direct = 1 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xz_decomp(mut state: xz_statep) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut had: ::core::ffi::c_uint = 0;
    let mut crc: ::core::ffi::c_ulong = 0;
    let mut len: ::core::ffi::c_ulong = 0;
    let mut strm: *mut lzma_stream = &raw mut (*state).strm;
    let mut action: lzma_action = LZMA_RUN;
    had = (*strm).avail_out as ::core::ffi::c_uint;
    loop {
        if (*strm).avail_in == 0 as size_t && xz_avail(state) == -(1 as ::core::ffi::c_int) {
            return -(1 as ::core::ffi::c_int);
        }
        if (*strm).avail_in == 0 as size_t {
            xz_error(
                state,
                LZMA_DATA_ERROR as ::core::ffi::c_int,
                b"unexpected end of file\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if (*state).eof != 0 {
            action = LZMA_FINISH;
        }
        if (*state).how == GZIP {
            (*state).zstrm.avail_in = (*state).strm.avail_in as uInt;
            (*state).zstrm.next_in = (*state).strm.next_in as *mut Bytef;
            (*state).zstrm.avail_out = (*state).strm.avail_out as uInt;
            (*state).zstrm.next_out = (*state).strm.next_out as *mut Bytef;
            ret = inflate(&raw mut (*state).zstrm, Z_NO_FLUSH);
            if ret == Z_STREAM_ERROR || ret == Z_NEED_DICT {
                xz_error(
                    state,
                    Z_STREAM_ERROR,
                    b"internal error: inflate stream corrupt\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            if ret == Z_MEM_ERROR {
                ret = LZMA_MEM_ERROR as ::core::ffi::c_int;
            }
            if ret == Z_DATA_ERROR {
                ret = LZMA_DATA_ERROR as ::core::ffi::c_int;
            }
            if ret == Z_STREAM_END {
                ret = LZMA_STREAM_END as ::core::ffi::c_int;
            }
            (*state).strm.avail_in = (*state).zstrm.avail_in as size_t;
            (*state).strm.next_in = (*state).zstrm.next_in;
            (*state).strm.avail_out = (*state).zstrm.avail_out as size_t;
            (*state).strm.next_out = (*state).zstrm.next_out as *mut uint8_t;
        } else {
            ret = lzma_code(strm, action) as ::core::ffi::c_int;
        }
        if ret == LZMA_MEM_ERROR as ::core::ffi::c_int {
            xz_error(
                state,
                LZMA_MEM_ERROR as ::core::ffi::c_int,
                b"out of memory\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if ret == LZMA_DATA_ERROR as ::core::ffi::c_int {
            xz_error(
                state,
                LZMA_DATA_ERROR as ::core::ffi::c_int,
                b"compressed data error\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if ret == LZMA_PROG_ERROR as ::core::ffi::c_int {
            xz_error(
                state,
                LZMA_PROG_ERROR as ::core::ffi::c_int,
                b"compression error\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if (*state).how != GZIP
            && ret != LZMA_OK as ::core::ffi::c_int
            && ret != LZMA_STREAM_END as ::core::ffi::c_int
        {
            xz_error(
                state,
                ret,
                b"lzma error\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if !((*strm).avail_out != 0 && ret != LZMA_STREAM_END as ::core::ffi::c_int) {
            break;
        }
    }
    (*state).have = (had as size_t).wrapping_sub((*strm).avail_out) as ::core::ffi::c_uint;
    (*state).next = (*strm).next_out.offset(-((*state).have as isize)) as *mut ::core::ffi::c_uchar;
    (*state).zstrm.adler = crc32((*state).zstrm.adler, (*state).next, (*state).have as uInt);
    if ret == LZMA_STREAM_END as ::core::ffi::c_int {
        if (*state).how == GZIP {
            if gz_next4(state, &raw mut crc) == -(1 as ::core::ffi::c_int)
                || gz_next4(state, &raw mut len) == -(1 as ::core::ffi::c_int)
            {
                xz_error(
                    state,
                    LZMA_DATA_ERROR as ::core::ffi::c_int,
                    b"unexpected end of file\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            if crc != (*state).zstrm.adler {
                xz_error(
                    state,
                    LZMA_DATA_ERROR as ::core::ffi::c_int,
                    b"incorrect data check\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            if len != (*state).zstrm.total_out & 0xffffffff as uLong {
                xz_error(
                    state,
                    LZMA_DATA_ERROR as ::core::ffi::c_int,
                    b"incorrect length check\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            (*state).strm.avail_in = 0 as size_t;
            (*state).strm.next_in = ::core::ptr::null::<uint8_t>();
            (*state).strm.avail_out = 0 as size_t;
            (*state).strm.next_out = ::core::ptr::null_mut::<uint8_t>();
        } else if (*strm).avail_in != 0 as size_t || (*state).eof == 0 {
            xz_error(
                state,
                LZMA_DATA_ERROR as ::core::ffi::c_int,
                b"trailing garbage\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        (*state).how = LOOK;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xz_make(mut state: xz_statep) -> ::core::ffi::c_int {
    let mut strm: *mut lzma_stream = &raw mut (*state).strm;
    if (*state).how == LOOK {
        if xz_head(state) == -(1 as ::core::ffi::c_int) {
            return -(1 as ::core::ffi::c_int);
        }
        if (*state).have != 0 {
            return 0 as ::core::ffi::c_int;
        }
    }
    if (*state).how == COPY {
        if xz_load(
            state,
            (*state).out,
            (*state).size << 1 as ::core::ffi::c_int,
            &raw mut (*state).have,
        ) == -(1 as ::core::ffi::c_int)
        {
            return -(1 as ::core::ffi::c_int);
        }
        (*state).next = (*state).out;
    } else if (*state).how == LZMA || (*state).how == GZIP {
        (*strm).avail_out = ((*state).size << 1 as ::core::ffi::c_int) as size_t;
        (*strm).next_out = (*state).out as *mut uint8_t;
        if xz_decomp(state) == -(1 as ::core::ffi::c_int) {
            return -(1 as ::core::ffi::c_int);
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xz_skip(mut state: xz_statep, mut len: uint64_t) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_uint = 0;
    while len != 0 {
        if (*state).have != 0 {
            n = if (*state).have as uint64_t > len {
                len as ::core::ffi::c_uint
            } else {
                (*state).have
            };
            (*state).have = (*state).have.wrapping_sub(n);
            (*state).next = (*state).next.offset(n as isize);
            (*state).pos = (*state).pos.wrapping_add(n as uint64_t);
            len = len.wrapping_sub(n as uint64_t);
        } else {
            if (*state).eof != 0 && (*state).strm.avail_in == 0 as size_t {
                break;
            }
            if xz_make(state) == -(1 as ::core::ffi::c_int) {
                return -(1 as ::core::ffi::c_int);
            }
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn __libxml2_xzread(
    mut file: xzFile,
    mut buf: *mut ::core::ffi::c_void,
    mut len: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut got: ::core::ffi::c_uint = 0;
    let mut n: ::core::ffi::c_uint = 0;
    let mut state: xz_statep = ::core::ptr::null_mut::<xz_state>();
    let mut strm: *mut lzma_stream = ::core::ptr::null_mut::<lzma_stream>();
    if file.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    state = file as xz_statep;
    strm = &raw mut (*state).strm;
    if !xz_record_read(file) {
        xz_error(
            state,
            -(1 as ::core::ffi::c_int),
            b"xz read budget exceeded\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*state).err != LZMA_OK as ::core::ffi::c_int {
        xz_record_terminal_error(file);
        return -(1 as ::core::ffi::c_int);
    }
    if (len as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        xz_error(
            state,
            LZMA_BUF_ERROR as ::core::ffi::c_int,
            b"requested length does not fit in int\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if len == 0 as ::core::ffi::c_uint {
        return 0 as ::core::ffi::c_int;
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as ::core::ffi::c_int;
        if xz_skip(state, (*state).skip) == -(1 as ::core::ffi::c_int) {
            return -(1 as ::core::ffi::c_int);
        }
    }
    got = 0 as ::core::ffi::c_uint;
    let mut current_block_39: u64;
    loop {
        if !xz_record_loop(file) {
            xz_error(
                state,
                -(1 as ::core::ffi::c_int),
                b"xz loop budget exceeded\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if (*state).have != 0 {
            n = if (*state).have > len {
                len
            } else {
                (*state).have
            };
            if !xz_record_output(file, n as usize) {
                xz_error(
                    state,
                    -(1 as ::core::ffi::c_int),
                    b"xz output budget exceeded\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
            memcpy(
                buf,
                (*state).next as *const ::core::ffi::c_void,
                n as size_t,
            );
            (*state).next = (*state).next.offset(n as isize);
            (*state).have = (*state).have.wrapping_sub(n);
            current_block_39 = 8693738493027456495;
        } else {
            if (*state).eof != 0 && (*strm).avail_in == 0 as size_t {
                break;
            }
            if (*state).how == LOOK || len < (*state).size << 1 as ::core::ffi::c_int {
                if xz_make(state) == -(1 as ::core::ffi::c_int) {
                    xz_record_terminal_error(file);
                    return -(1 as ::core::ffi::c_int);
                }
                current_block_39 = 4166486009154926805;
            } else {
                if (*state).how == COPY {
                    if xz_load(state, buf as *mut ::core::ffi::c_uchar, len, &raw mut n)
                        == -(1 as ::core::ffi::c_int)
                    {
                        xz_record_terminal_error(file);
                        return -(1 as ::core::ffi::c_int);
                    }
                } else {
                    (*strm).avail_out = len as size_t;
                    (*strm).next_out = buf as *mut uint8_t;
                    if xz_decomp(state) == -(1 as ::core::ffi::c_int) {
                        xz_record_terminal_error(file);
                        return -(1 as ::core::ffi::c_int);
                    }
                    n = (*state).have;
                    (*state).have = 0 as ::core::ffi::c_uint;
                }
                if !xz_record_output(file, n as usize) {
                    xz_error(
                        state,
                        -(1 as ::core::ffi::c_int),
                        b"xz output budget exceeded\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
                current_block_39 = 8693738493027456495;
            }
        }
        match current_block_39 {
            8693738493027456495 => {
                len = len.wrapping_sub(n);
                buf = (buf as *mut ::core::ffi::c_char).offset(n as isize)
                    as *mut ::core::ffi::c_void;
                got = got.wrapping_add(n);
                (*state).pos = (*state).pos.wrapping_add(n as uint64_t);
            }
            _ => {}
        }
        if !(len != 0) {
            break;
        }
    }
    return got as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn __libxml2_xzclose(mut file: xzFile) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut state: xz_statep = ::core::ptr::null_mut::<xz_state>();
    if file.is_null() {
        return LZMA_DATA_ERROR as ::core::ffi::c_int;
    }
    state = file as xz_statep;
    if (*state).size != 0 {
        lzma_end(&raw mut (*state).strm);
        if (*state).init == 1 as ::core::ffi::c_int {
            inflateEnd(&raw mut (*state).zstrm);
        }
        (*state).init = 0 as ::core::ffi::c_int;
        xmlFree.expect("non-null function pointer")((*state).out as *mut ::core::ffi::c_void);
        xmlFree.expect("non-null function pointer")((*state).in_0 as *mut ::core::ffi::c_void);
    }
    xmlFree.expect("non-null function pointer")((*state).path as *mut ::core::ffi::c_void);
    if !(*state).msg.is_null() && (*state).err != LZMA_MEM_ERROR as ::core::ffi::c_int {
        xmlFree.expect("non-null function pointer")((*state).msg as *mut ::core::ffi::c_void);
    }
    ret = close((*state).fd);
    xmlFree.expect("non-null function pointer")(state as *mut ::core::ffi::c_void);
    xz_close_budget(file);
    return if ret != 0 {
        ret
    } else {
        LZMA_OK as ::core::ffi::c_int
    };
}
