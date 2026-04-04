#[repr(C)]
pub struct _xmlRMutex {
    _private: [u8; 0],
}

extern "C" {
    fn xmlStrQEqual(
        pref: *const xmlChar,
        name: *const xmlChar,
        str: *const xmlChar,
    ) -> ::core::ffi::c_int;
    fn rand_r(__seed: *mut ::core::ffi::c_uint) -> ::core::ffi::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlFree: xmlFreeFunc;
    fn xmlNewRMutex() -> xmlRMutexPtr;
    fn xmlRMutexLock(tok: xmlRMutexPtr);
    fn xmlRMutexUnlock(tok: xmlRMutexPtr);
    fn xmlFreeRMutex(tok: xmlRMutexPtr);
}
pub type xmlChar = ::core::ffi::c_uchar;
pub type size_t = usize;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
pub type xmlRMutexPtr = *mut xmlRMutex;
pub type xmlRMutex = _xmlRMutex;
pub type time_t = __time_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDict {
    pub ref_counter: ::core::ffi::c_int,
    pub dict: *mut _xmlDictEntry,
    pub size: size_t,
    pub nbElems: ::core::ffi::c_uint,
    pub strings: xmlDictStringsPtr,
    pub subdict: *mut _xmlDict,
    pub seed: ::core::ffi::c_int,
    pub limit: size_t,
}
pub type xmlDictStringsPtr = *mut xmlDictStrings;
pub type xmlDictStrings = _xmlDictStrings;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDictStrings {
    pub next: xmlDictStringsPtr,
    pub free: *mut xmlChar,
    pub end: *mut xmlChar,
    pub size: size_t,
    pub nbStrings: size_t,
    pub array: [xmlChar; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDictEntry {
    pub next: *mut _xmlDictEntry,
    pub name: *const xmlChar,
    pub len: ::core::ffi::c_uint,
    pub valid: ::core::ffi::c_int,
    pub okey: ::core::ffi::c_ulong,
}
pub type xmlDictPtr = *mut xmlDict;
pub type xmlDict = _xmlDict;
pub type xmlFreeFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type xmlDictEntry = _xmlDictEntry;
pub type xmlMallocFunc = Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
pub type xmlDictEntryPtr = *mut xmlDictEntry;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAX_HASH_LEN: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MIN_DICT_SIZE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const MAX_DICT_HASH: ::core::ffi::c_int = 8 as ::core::ffi::c_int * 2048 as ::core::ffi::c_int;
static mut xmlDictMutex: xmlRMutexPtr = ::core::ptr::null::<xmlRMutex>() as *mut xmlRMutex;
static mut xmlDictInitialized: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut rand_seed: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C" fn xmlInitializeDict() -> ::core::ffi::c_int {
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlInitializeDict() -> ::core::ffi::c_int {
    if unsafe { xmlDictInitialized } != 0 {
        return 1 as ::core::ffi::c_int;
    }
    let dict_mutex = unsafe { xmlNewRMutex() };
    if dict_mutex.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    unsafe {
        xmlDictMutex = dict_mutex;
        xmlRMutexLock(dict_mutex);
        rand_seed = time(::core::ptr::null_mut::<time_t>()) as ::core::ffi::c_uint;
        rand_r(&raw mut rand_seed);
        xmlDictInitialized = 1 as ::core::ffi::c_int;
        xmlRMutexUnlock(dict_mutex);
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlRandom() -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if unsafe { xmlDictInitialized } == 0 as ::core::ffi::c_int {
        unsafe { __xmlInitializeDict() };
    }
    unsafe {
        xmlRMutexLock(xmlDictMutex);
        ret = rand_r(&raw mut rand_seed);
        xmlRMutexUnlock(xmlDictMutex);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictCleanup() {
    if unsafe { xmlDictInitialized } == 0 {
        return;
    }
    unsafe {
        xmlFreeRMutex(xmlDictMutex);
        xmlDictInitialized = 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn xmlDictAddString(
    mut dict: xmlDictPtr,
    mut name: *const xmlChar,
    mut namelen: ::core::ffi::c_uint,
) -> *const xmlChar {
    let mut current_block: u64;
    let mut pool: xmlDictStringsPtr = ::core::ptr::null_mut::<xmlDictStrings>();
    let mut ret: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut size: size_t = 0 as size_t;
    let mut limit: size_t = 0 as size_t;
    pool = unsafe { (*dict).strings };
    loop {
        if pool.is_null() {
            current_block = 7351195479953500246;
            break;
        }
        let available =
            unsafe { (*pool).end.offset_from((*pool).free) as ::core::ffi::c_long as size_t };
        if available > namelen as size_t {
            current_block = 12564705494504611164;
            break;
        }
        let pool_size = unsafe { (*pool).size };
        if pool_size > size {
            size = pool_size;
        }
        limit = limit.wrapping_add(pool_size);
        pool = unsafe { (*pool).next };
    }
    match current_block {
        7351195479953500246 => {
            if pool.is_null() {
                let dict_limit = unsafe { (*dict).limit };
                if dict_limit > 0 as size_t && limit > dict_limit {
                    return ::core::ptr::null::<xmlChar>();
                }
                if size == 0 as size_t {
                    size = 1000 as size_t;
                } else {
                    size = size.wrapping_mul(4 as size_t);
                }
                if size < (4 as ::core::ffi::c_uint).wrapping_mul(namelen) as size_t {
                    size = (4 as ::core::ffi::c_uint).wrapping_mul(namelen) as size_t;
                }
                pool = unsafe {
                    xmlMalloc.expect("non-null function pointer")(
                        (::core::mem::size_of::<xmlDictStrings>() as size_t).wrapping_add(size),
                    ) as xmlDictStringsPtr
                };
                if pool.is_null() {
                    return ::core::ptr::null::<xmlChar>();
                }
                unsafe {
                    (*pool).size = size;
                    (*pool).nbStrings = 0 as size_t;
                    (*pool).free = (&raw mut (*pool).array as *mut xmlChar)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut xmlChar;
                    (*pool).end = (&raw mut (*pool).array as *mut xmlChar).offset(size as isize)
                        as *mut xmlChar;
                    (*pool).next = (*dict).strings;
                    (*dict).strings = pool;
                }
            }
        }
        _ => {}
    }
    unsafe {
        ret = (*pool).free;
        memcpy(
            (*pool).free as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            namelen as size_t,
        );
        (*pool).free = (*pool).free.offset(namelen as isize);
        let fresh4 = (*pool).free;
        (*pool).free = (*pool).free.offset(1);
        *fresh4 = 0 as xmlChar;
        (*pool).nbStrings = (*pool).nbStrings.wrapping_add(1);
    }
    return ret;
}
unsafe extern "C" fn xmlDictAddQString(
    mut dict: xmlDictPtr,
    mut prefix: *const xmlChar,
    mut plen: ::core::ffi::c_uint,
    mut name: *const xmlChar,
    mut namelen: ::core::ffi::c_uint,
) -> *const xmlChar {
    let mut current_block: u64;
    let mut pool: xmlDictStringsPtr = ::core::ptr::null_mut::<xmlDictStrings>();
    let mut size: size_t = 0 as size_t;
    let mut limit: size_t = 0 as size_t;
    if prefix.is_null() {
        return unsafe { xmlDictAddString(dict, name, namelen) };
    }
    pool = unsafe { (*dict).strings };
    loop {
        if pool.is_null() {
            current_block = 13513818773234778473;
            break;
        }
        let available =
            unsafe { (*pool).end.offset_from((*pool).free) as ::core::ffi::c_long as size_t };
        if available
            > namelen
                .wrapping_add(plen)
                .wrapping_add(1 as ::core::ffi::c_uint) as size_t
        {
            current_block = 9272350092442567110;
            break;
        }
        let pool_size = unsafe { (*pool).size };
        if pool_size > size {
            size = pool_size;
        }
        limit = limit.wrapping_add(pool_size);
        pool = unsafe { (*pool).next };
    }
    match current_block {
        13513818773234778473 => {
            if pool.is_null() {
                let dict_limit = unsafe { (*dict).limit };
                if dict_limit > 0 as size_t && limit > dict_limit {
                    return ::core::ptr::null::<xmlChar>();
                }
                if size == 0 as size_t {
                    size = 1000 as size_t;
                } else {
                    size = size.wrapping_mul(4 as size_t);
                }
                if size
                    < (4 as ::core::ffi::c_uint).wrapping_mul(
                        namelen
                            .wrapping_add(plen)
                            .wrapping_add(1 as ::core::ffi::c_uint),
                    ) as size_t
                {
                    size = (4 as ::core::ffi::c_uint).wrapping_mul(
                        namelen
                            .wrapping_add(plen)
                            .wrapping_add(1 as ::core::ffi::c_uint),
                    ) as size_t;
                }
                pool = unsafe {
                    xmlMalloc.expect("non-null function pointer")(
                        (::core::mem::size_of::<xmlDictStrings>() as size_t).wrapping_add(size),
                    ) as xmlDictStringsPtr
                };
                if pool.is_null() {
                    return ::core::ptr::null::<xmlChar>();
                }
                unsafe {
                    (*pool).size = size;
                    (*pool).nbStrings = 0 as size_t;
                    (*pool).free = (&raw mut (*pool).array as *mut xmlChar)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut xmlChar;
                    (*pool).end = (&raw mut (*pool).array as *mut xmlChar).offset(size as isize)
                        as *mut xmlChar;
                    (*pool).next = (*dict).strings;
                    (*dict).strings = pool;
                }
            }
        }
        _ => {}
    }
    let ret = unsafe { (*pool).free as *const xmlChar };
    unsafe {
        memcpy(
            (*pool).free as *mut ::core::ffi::c_void,
            prefix as *const ::core::ffi::c_void,
            plen as size_t,
        );
        (*pool).free = (*pool).free.offset(plen as isize);
        let fresh5 = (*pool).free;
        (*pool).free = (*pool).free.offset(1);
        *fresh5 = ':' as i32 as xmlChar;
        memcpy(
            (*pool).free as *mut ::core::ffi::c_void,
            name as *const ::core::ffi::c_void,
            namelen as size_t,
        );
        (*pool).free = (*pool).free.offset(namelen as isize);
        let fresh6 = (*pool).free;
        (*pool).free = (*pool).free.offset(1);
        *fresh6 = 0 as xmlChar;
        (*pool).nbStrings = (*pool).nbStrings.wrapping_add(1);
    }
    return ret;
}
unsafe extern "C" fn xmlDictComputeBigKey(
    mut data: *const xmlChar,
    mut namelen: ::core::ffi::c_int,
    mut seed: ::core::ffi::c_int,
) -> uint32_t {
    let mut hash: uint32_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    if namelen <= 0 as ::core::ffi::c_int || data.is_null() {
        return 0 as uint32_t;
    }
    hash = seed as uint32_t;
    i = 0 as ::core::ffi::c_int;
    while i < namelen {
        hash = hash.wrapping_add(unsafe { *data.offset(i as isize) as uint32_t });
        hash = hash.wrapping_add(hash << 10 as ::core::ffi::c_int);
        hash ^= hash >> 6 as ::core::ffi::c_int;
        i += 1;
    }
    hash = hash.wrapping_add(hash << 3 as ::core::ffi::c_int);
    hash ^= hash >> 11 as ::core::ffi::c_int;
    hash = hash.wrapping_add(hash << 15 as ::core::ffi::c_int);
    return hash;
}
#[inline]
fn dict_code_unit(ptr: *const xmlChar, index: ::core::ffi::c_int) -> ::core::ffi::c_ulong {
    unsafe { *ptr.offset(index as isize) as ::core::ffi::c_ulong }
}
#[inline]
fn dict_bucket_entry(dict: &xmlDict, key: ::core::ffi::c_ulong) -> xmlDictEntryPtr {
    unsafe { dict.dict.offset(key as isize) as xmlDictEntryPtr }
}
#[inline]
fn dict_entry_next(entry: xmlDictEntryPtr) -> xmlDictEntryPtr {
    unsafe { (*entry).next as xmlDictEntryPtr }
}
#[inline]
fn dict_entry_valid(entry: xmlDictEntryPtr) -> bool {
    unsafe { (*entry).valid != 0 as ::core::ffi::c_int }
}
#[inline]
fn dict_entry_name(entry: xmlDictEntryPtr) -> *const xmlChar {
    unsafe { (*entry).name }
}
#[inline]
fn dict_entry_matches(
    entry: xmlDictEntryPtr,
    okey: ::core::ffi::c_ulong,
    len: ::core::ffi::c_uint,
    name: *const xmlChar,
) -> bool {
    unsafe {
        (*entry).okey == okey
            && (*entry).len == len
            && memcmp(
                (*entry).name as *const ::core::ffi::c_void,
                name as *const ::core::ffi::c_void,
                len as size_t,
            ) == 0
    }
}
#[inline]
fn dict_qentry_matches(
    entry: xmlDictEntryPtr,
    okey: ::core::ffi::c_ulong,
    len: ::core::ffi::c_uint,
    prefix: *const xmlChar,
    name: *const xmlChar,
) -> bool {
    unsafe {
        (*entry).okey == okey
            && (*entry).len == len
            && xmlStrQEqual(prefix, name, (*entry).name) != 0
    }
}
#[inline]
fn dict_compute_key(
    size: size_t,
    name: *const xmlChar,
    len: ::core::ffi::c_int,
    seed: ::core::ffi::c_int,
) -> ::core::ffi::c_ulong {
    if size == MIN_DICT_SIZE as size_t {
        unsafe { xmlDictComputeFastKey(name, len, seed) }
    } else {
        unsafe { xmlDictComputeBigKey(name, len, seed) as ::core::ffi::c_ulong }
    }
}
#[inline]
fn dict_compute_qkey(
    size: size_t,
    prefix: *const xmlChar,
    plen: ::core::ffi::c_int,
    name: *const xmlChar,
    len: ::core::ffi::c_int,
    seed: ::core::ffi::c_int,
) -> ::core::ffi::c_ulong {
    if prefix.is_null() {
        dict_compute_key(size, name, len, seed)
    } else if size == MIN_DICT_SIZE as size_t {
        unsafe { xmlDictComputeFastQKey(prefix, plen, name, len, seed) }
    } else {
        unsafe { xmlDictComputeBigQKey(prefix, plen, name, len, seed) }
    }
}
unsafe extern "C" fn xmlDictComputeBigQKey(
    mut prefix: *const xmlChar,
    mut plen: ::core::ffi::c_int,
    mut name: *const xmlChar,
    mut len: ::core::ffi::c_int,
    mut seed: ::core::ffi::c_int,
) -> ::core::ffi::c_ulong {
    let mut hash: uint32_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    hash = seed as uint32_t;
    i = 0 as ::core::ffi::c_int;
    while i < plen {
        hash = hash.wrapping_add(dict_code_unit(prefix, i) as uint32_t);
        hash = hash.wrapping_add(hash << 10 as ::core::ffi::c_int);
        hash ^= hash >> 6 as ::core::ffi::c_int;
        i += 1;
    }
    hash = hash.wrapping_add(':' as i32 as uint32_t);
    hash = hash.wrapping_add(hash << 10 as ::core::ffi::c_int);
    hash ^= hash >> 6 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < len {
        hash = hash.wrapping_add(dict_code_unit(name, i) as uint32_t);
        hash = hash.wrapping_add(hash << 10 as ::core::ffi::c_int);
        hash ^= hash >> 6 as ::core::ffi::c_int;
        i += 1;
    }
    hash = hash.wrapping_add(hash << 3 as ::core::ffi::c_int);
    hash ^= hash >> 11 as ::core::ffi::c_int;
    hash = hash.wrapping_add(hash << 15 as ::core::ffi::c_int);
    return hash as ::core::ffi::c_ulong;
}
unsafe extern "C" fn xmlDictComputeFastKey(
    mut name: *const xmlChar,
    mut namelen: ::core::ffi::c_int,
    mut seed: ::core::ffi::c_int,
) -> ::core::ffi::c_ulong {
    let mut value: ::core::ffi::c_ulong = seed as ::core::ffi::c_ulong;
    if name.is_null() || namelen <= 0 as ::core::ffi::c_int {
        return value;
    }
    value = value.wrapping_add(dict_code_unit(name, 0));
    value <<= 5 as ::core::ffi::c_int;
    if namelen > 10 as ::core::ffi::c_int {
        value = value.wrapping_add(dict_code_unit(name, namelen - 1 as ::core::ffi::c_int));
        namelen = 10 as ::core::ffi::c_int;
    }
    let mut current_block_16: u64;
    match namelen {
        10 => {
            value = value.wrapping_add(dict_code_unit(name, 9 as ::core::ffi::c_int));
            current_block_16 = 14140296040720354316;
        }
        9 => {
            current_block_16 = 14140296040720354316;
        }
        8 => {
            current_block_16 = 10477942228951044767;
        }
        7 => {
            current_block_16 = 6178868810057640957;
        }
        6 => {
            current_block_16 = 9879368920036322223;
        }
        5 => {
            current_block_16 = 7599933913323472286;
        }
        4 => {
            current_block_16 = 1634274037810632810;
        }
        3 => {
            current_block_16 = 3992153788896120206;
        }
        2 => {
            current_block_16 = 8753412356531521447;
        }
        _ => {
            current_block_16 = 8831408221741692167;
        }
    }
    match current_block_16 {
        14140296040720354316 => {
            value = value.wrapping_add(dict_code_unit(name, 8 as ::core::ffi::c_int));
            current_block_16 = 10477942228951044767;
        }
        _ => {}
    }
    match current_block_16 {
        10477942228951044767 => {
            value = value.wrapping_add(dict_code_unit(name, 7 as ::core::ffi::c_int));
            current_block_16 = 6178868810057640957;
        }
        _ => {}
    }
    match current_block_16 {
        6178868810057640957 => {
            value = value.wrapping_add(dict_code_unit(name, 6 as ::core::ffi::c_int));
            current_block_16 = 9879368920036322223;
        }
        _ => {}
    }
    match current_block_16 {
        9879368920036322223 => {
            value = value.wrapping_add(dict_code_unit(name, 5 as ::core::ffi::c_int));
            current_block_16 = 7599933913323472286;
        }
        _ => {}
    }
    match current_block_16 {
        7599933913323472286 => {
            value = value.wrapping_add(dict_code_unit(name, 4 as ::core::ffi::c_int));
            current_block_16 = 1634274037810632810;
        }
        _ => {}
    }
    match current_block_16 {
        1634274037810632810 => {
            value = value.wrapping_add(dict_code_unit(name, 3 as ::core::ffi::c_int));
            current_block_16 = 3992153788896120206;
        }
        _ => {}
    }
    match current_block_16 {
        3992153788896120206 => {
            value = value.wrapping_add(dict_code_unit(name, 2 as ::core::ffi::c_int));
            current_block_16 = 8753412356531521447;
        }
        _ => {}
    }
    match current_block_16 {
        8753412356531521447 => {
            value = value.wrapping_add(dict_code_unit(name, 1 as ::core::ffi::c_int));
        }
        _ => {}
    }
    return value;
}
unsafe extern "C" fn xmlDictComputeFastQKey(
    mut prefix: *const xmlChar,
    mut plen: ::core::ffi::c_int,
    mut name: *const xmlChar,
    mut len: ::core::ffi::c_int,
    mut seed: ::core::ffi::c_int,
) -> ::core::ffi::c_ulong {
    let mut value: ::core::ffi::c_ulong = seed as ::core::ffi::c_ulong;
    if plen == 0 as ::core::ffi::c_int {
        value = value.wrapping_add(
            (30 as ::core::ffi::c_ulong).wrapping_mul(':' as i32 as ::core::ffi::c_ulong),
        );
    } else {
        value = value.wrapping_add(
            (30 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                .wrapping_mul(dict_code_unit(prefix, 0)),
        );
    }
    if len > 10 as ::core::ffi::c_int {
        let mut offset: ::core::ffi::c_int =
            len - (plen + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int);
        if offset < 0 as ::core::ffi::c_int {
            offset = len - (10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int);
        }
        value = value.wrapping_add(dict_code_unit(name, offset));
        len = 10 as ::core::ffi::c_int;
        if plen > 10 as ::core::ffi::c_int {
            plen = 10 as ::core::ffi::c_int;
        }
    }
    let mut current_block_20: u64;
    match plen {
        10 => {
            value = value.wrapping_add(dict_code_unit(prefix, 9 as ::core::ffi::c_int));
            current_block_20 = 9537446021666286793;
        }
        9 => {
            current_block_20 = 9537446021666286793;
        }
        8 => {
            current_block_20 = 9570895743713506940;
        }
        7 => {
            current_block_20 = 3809271318960479663;
        }
        6 => {
            current_block_20 = 1760573699125010265;
        }
        5 => {
            current_block_20 = 8203539728489389117;
        }
        4 => {
            current_block_20 = 3174106958175389174;
        }
        3 => {
            current_block_20 = 71433336188168654;
        }
        2 => {
            current_block_20 = 6539070127515362870;
        }
        1 => {
            current_block_20 = 10006674590530960285;
        }
        _ => {
            current_block_20 = 11584701595673473500;
        }
    }
    match current_block_20 {
        9537446021666286793 => {
            value = value.wrapping_add(dict_code_unit(prefix, 8 as ::core::ffi::c_int));
            current_block_20 = 9570895743713506940;
        }
        _ => {}
    }
    match current_block_20 {
        9570895743713506940 => {
            value = value.wrapping_add(dict_code_unit(prefix, 7 as ::core::ffi::c_int));
            current_block_20 = 3809271318960479663;
        }
        _ => {}
    }
    match current_block_20 {
        3809271318960479663 => {
            value = value.wrapping_add(dict_code_unit(prefix, 6 as ::core::ffi::c_int));
            current_block_20 = 1760573699125010265;
        }
        _ => {}
    }
    match current_block_20 {
        1760573699125010265 => {
            value = value.wrapping_add(dict_code_unit(prefix, 5 as ::core::ffi::c_int));
            current_block_20 = 8203539728489389117;
        }
        _ => {}
    }
    match current_block_20 {
        8203539728489389117 => {
            value = value.wrapping_add(dict_code_unit(prefix, 4 as ::core::ffi::c_int));
            current_block_20 = 3174106958175389174;
        }
        _ => {}
    }
    match current_block_20 {
        3174106958175389174 => {
            value = value.wrapping_add(dict_code_unit(prefix, 3 as ::core::ffi::c_int));
            current_block_20 = 71433336188168654;
        }
        _ => {}
    }
    match current_block_20 {
        71433336188168654 => {
            value = value.wrapping_add(dict_code_unit(prefix, 2 as ::core::ffi::c_int));
            current_block_20 = 6539070127515362870;
        }
        _ => {}
    }
    match current_block_20 {
        6539070127515362870 => {
            value = value.wrapping_add(dict_code_unit(prefix, 1 as ::core::ffi::c_int));
            current_block_20 = 10006674590530960285;
        }
        _ => {}
    }
    match current_block_20 {
        10006674590530960285 => {
            value = value.wrapping_add(dict_code_unit(prefix, 0 as ::core::ffi::c_int));
        }
        _ => {}
    }
    len -= plen;
    if len > 0 as ::core::ffi::c_int {
        value = value.wrapping_add(':' as i32 as ::core::ffi::c_ulong);
        len -= 1;
    }
    let mut current_block_36: u64;
    match len {
        10 => {
            value = value.wrapping_add(dict_code_unit(name, 9 as ::core::ffi::c_int));
            current_block_36 = 9108573116658949053;
        }
        9 => {
            current_block_36 = 9108573116658949053;
        }
        8 => {
            current_block_36 = 16454796981030128573;
        }
        7 => {
            current_block_36 = 10379619488888424139;
        }
        6 => {
            current_block_36 = 18024754184016332365;
        }
        5 => {
            current_block_36 = 8167404252455708155;
        }
        4 => {
            current_block_36 = 16790513941173568777;
        }
        3 => {
            current_block_36 = 2447873016272255407;
        }
        2 => {
            current_block_36 = 3840978892466691305;
        }
        1 => {
            current_block_36 = 11321417492605204613;
        }
        _ => {
            current_block_36 = 2604890879466389055;
        }
    }
    match current_block_36 {
        9108573116658949053 => {
            value = value.wrapping_add(dict_code_unit(name, 8 as ::core::ffi::c_int));
            current_block_36 = 16454796981030128573;
        }
        _ => {}
    }
    match current_block_36 {
        16454796981030128573 => {
            value = value.wrapping_add(dict_code_unit(name, 7 as ::core::ffi::c_int));
            current_block_36 = 10379619488888424139;
        }
        _ => {}
    }
    match current_block_36 {
        10379619488888424139 => {
            value = value.wrapping_add(dict_code_unit(name, 6 as ::core::ffi::c_int));
            current_block_36 = 18024754184016332365;
        }
        _ => {}
    }
    match current_block_36 {
        18024754184016332365 => {
            value = value.wrapping_add(dict_code_unit(name, 5 as ::core::ffi::c_int));
            current_block_36 = 8167404252455708155;
        }
        _ => {}
    }
    match current_block_36 {
        8167404252455708155 => {
            value = value.wrapping_add(dict_code_unit(name, 4 as ::core::ffi::c_int));
            current_block_36 = 16790513941173568777;
        }
        _ => {}
    }
    match current_block_36 {
        16790513941173568777 => {
            value = value.wrapping_add(dict_code_unit(name, 3 as ::core::ffi::c_int));
            current_block_36 = 2447873016272255407;
        }
        _ => {}
    }
    match current_block_36 {
        2447873016272255407 => {
            value = value.wrapping_add(dict_code_unit(name, 2 as ::core::ffi::c_int));
            current_block_36 = 3840978892466691305;
        }
        _ => {}
    }
    match current_block_36 {
        3840978892466691305 => {
            value = value.wrapping_add(dict_code_unit(name, 1 as ::core::ffi::c_int));
            current_block_36 = 11321417492605204613;
        }
        _ => {}
    }
    match current_block_36 {
        11321417492605204613 => {
            value = value.wrapping_add(dict_code_unit(name, 0 as ::core::ffi::c_int));
        }
        _ => {}
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictCreate() -> xmlDictPtr {
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    if unsafe { xmlDictInitialized } == 0 {
        if unsafe { __xmlInitializeDict() } == 0 {
            return ::core::ptr::null_mut::<xmlDict>();
        }
    }
    dict = unsafe {
        xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlDict>() as size_t)
            as xmlDictPtr
    };
    if !dict.is_null() {
        let dict_ref = unsafe { &mut *dict };
        dict_ref.ref_counter = 1 as ::core::ffi::c_int;
        dict_ref.limit = 0 as size_t;
        dict_ref.size = MIN_DICT_SIZE as size_t;
        dict_ref.nbElems = 0 as ::core::ffi::c_uint;
        dict_ref.dict = unsafe {
            xmlMalloc.expect("non-null function pointer")(
                (MIN_DICT_SIZE as size_t)
                    .wrapping_mul(::core::mem::size_of::<xmlDictEntry>() as size_t),
            ) as *mut _xmlDictEntry
        };
        dict_ref.strings = ::core::ptr::null_mut::<xmlDictStrings>();
        dict_ref.subdict = ::core::ptr::null_mut::<_xmlDict>();
        if !dict_ref.dict.is_null() {
            unsafe {
                memset(
                    dict_ref.dict as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (MIN_DICT_SIZE as size_t)
                        .wrapping_mul(::core::mem::size_of::<xmlDictEntry>() as size_t),
                )
            };
            dict_ref.seed = unsafe { __xmlRandom() };
            return dict;
        }
        unsafe { xmlFree.expect("non-null function pointer")(dict as *mut ::core::ffi::c_void) };
    }
    return ::core::ptr::null_mut::<xmlDict>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictCreateSub(mut sub: xmlDictPtr) -> xmlDictPtr {
    let mut dict: xmlDictPtr = unsafe { xmlDictCreate() };
    if !dict.is_null() && !sub.is_null() {
        unsafe {
            (*dict).seed = (*sub).seed;
            (*dict).subdict = sub as *mut _xmlDict;
            xmlDictReference((*dict).subdict as xmlDictPtr);
        }
    }
    return dict;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictReference(mut dict: xmlDictPtr) -> ::core::ffi::c_int {
    if unsafe { xmlDictInitialized } == 0 {
        if unsafe { __xmlInitializeDict() } == 0 {
            return -(1 as ::core::ffi::c_int);
        }
    }
    if dict.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    unsafe { xmlRMutexLock(xmlDictMutex) };
    unsafe { (*dict).ref_counter += 1 };
    unsafe { xmlRMutexUnlock(xmlDictMutex) };
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlDictGrow(mut dict: xmlDictPtr, mut size: size_t) -> ::core::ffi::c_int {
    let Some(dict) = (unsafe { dict.as_mut() }) else {
        return -(1 as ::core::ffi::c_int);
    };
    if size < 8 as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    if size > (8 as ::core::ffi::c_int * 2048 as ::core::ffi::c_int) as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    let oldsize = dict.size;
    let olddict = dict.dict;
    if olddict.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let keep_keys = (oldsize != MIN_DICT_SIZE as size_t) as ::core::ffi::c_int;
    let newdict = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            size.wrapping_mul(::core::mem::size_of::<xmlDictEntry>() as size_t),
        ) as *mut _xmlDictEntry
    };
    if newdict.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    unsafe {
        memset(
            newdict as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            size.wrapping_mul(::core::mem::size_of::<xmlDictEntry>() as size_t),
        );
    }
    dict.dict = newdict;
    dict.size = size;
    let old_entries = unsafe { ::core::slice::from_raw_parts_mut(olddict, oldsize) };
    let new_entries = unsafe { ::core::slice::from_raw_parts_mut(newdict, size) };
    let mut ret = 0 as ::core::ffi::c_int;

    for old_entry in old_entries.iter_mut() {
        if old_entry.valid == 0 as ::core::ffi::c_int {
            continue;
        }
        let okey = if keep_keys != 0 {
            old_entry.okey
        } else if dict.size == MIN_DICT_SIZE as size_t {
            unsafe {
                xmlDictComputeFastKey(
                    old_entry.name,
                    old_entry.len as ::core::ffi::c_int,
                    dict.seed,
                )
            }
        } else {
            unsafe {
                xmlDictComputeBigKey(
                    old_entry.name,
                    old_entry.len as ::core::ffi::c_int,
                    dict.seed,
                ) as ::core::ffi::c_ulong
            }
        };
        let key = (okey as size_t).wrapping_rem(dict.size) as usize;
        if new_entries[key].valid == 0 as ::core::ffi::c_int {
            new_entries[key] = *old_entry;
            new_entries[key].next = ::core::ptr::null_mut::<_xmlDictEntry>();
            new_entries[key].okey = okey;
        } else {
            let entry = unsafe {
                xmlMalloc.expect("non-null function pointer")(
                    ::core::mem::size_of::<xmlDictEntry>() as size_t,
                ) as xmlDictEntryPtr
            };
            if entry.is_null() {
                ret = -(1 as ::core::ffi::c_int);
                continue;
            }
            let entry_ref = unsafe { &mut *entry };
            entry_ref.name = old_entry.name;
            entry_ref.len = old_entry.len;
            entry_ref.okey = okey;
            entry_ref.next = new_entries[key].next;
            entry_ref.valid = 1 as ::core::ffi::c_int;
            new_entries[key].next = entry as *mut _xmlDictEntry;
        }
    }

    for old_entry in old_entries.iter_mut() {
        let mut iter = old_entry.next as xmlDictEntryPtr;
        while !iter.is_null() {
            let next = unsafe { (*iter).next as xmlDictEntryPtr };
            let okey = if keep_keys != 0 {
                unsafe { (*iter).okey }
            } else if dict.size == MIN_DICT_SIZE as size_t {
                unsafe {
                    xmlDictComputeFastKey(
                        (*iter).name,
                        (*iter).len as ::core::ffi::c_int,
                        dict.seed,
                    )
                }
            } else {
                unsafe {
                    xmlDictComputeBigKey((*iter).name, (*iter).len as ::core::ffi::c_int, dict.seed)
                        as ::core::ffi::c_ulong
                }
            };
            let key = (okey as size_t).wrapping_rem(dict.size) as usize;
            if new_entries[key].valid == 0 as ::core::ffi::c_int {
                new_entries[key] = unsafe { *iter };
                new_entries[key].next = ::core::ptr::null_mut::<_xmlDictEntry>();
                new_entries[key].valid = 1 as ::core::ffi::c_int;
                new_entries[key].okey = okey;
                unsafe {
                    xmlFree.expect("non-null function pointer")(iter as *mut ::core::ffi::c_void)
                };
            } else {
                unsafe { (*iter).next = new_entries[key].next };
                unsafe { (*iter).okey = okey };
                new_entries[key].next = iter as *mut _xmlDictEntry;
            }
            iter = next;
        }
    }
    unsafe { xmlFree.expect("non-null function pointer")(olddict as *mut ::core::ffi::c_void) };
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictFree(mut dict: xmlDictPtr) {
    let Some(dict_ref) = (unsafe { dict.as_mut() }) else {
        return;
    };
    if unsafe { xmlDictInitialized } == 0 {
        if unsafe { __xmlInitializeDict() } == 0 {
            return;
        }
    }
    unsafe {
        xmlRMutexLock(xmlDictMutex);
    }
    dict_ref.ref_counter -= 1;
    if dict_ref.ref_counter > 0 as ::core::ffi::c_int {
        unsafe {
            xmlRMutexUnlock(xmlDictMutex);
        }
        return;
    }
    unsafe {
        xmlRMutexUnlock(xmlDictMutex);
    }
    let subdict = dict_ref.subdict as xmlDictPtr;
    if !subdict.is_null() {
        unsafe {
            xmlDictFree(subdict);
        }
    }
    let dict_table = dict_ref.dict;
    let size = dict_ref.size;
    if !dict_table.is_null() {
        let mut i = 0 as size_t;
        while i < size && dict_ref.nbElems > 0 as ::core::ffi::c_uint {
            let mut iter = unsafe { dict_table.offset(i as isize) as xmlDictEntryPtr };
            if dict_entry_valid(iter) {
                let mut inside_dict = true;
                while !iter.is_null() {
                    let next = dict_entry_next(iter);
                    if !inside_dict {
                        unsafe {
                            xmlFree.expect("non-null function pointer")(
                                iter as *mut ::core::ffi::c_void,
                            );
                        }
                    }
                    dict_ref.nbElems = dict_ref.nbElems.wrapping_sub(1);
                    inside_dict = false;
                    iter = next;
                }
            }
            i = i.wrapping_add(1);
        }
        unsafe {
            xmlFree.expect("non-null function pointer")(dict_table as *mut ::core::ffi::c_void);
        }
    }
    let mut pool = dict_ref.strings;
    while !pool.is_null() {
        let nextp = unsafe { (*pool).next };
        unsafe {
            xmlFree.expect("non-null function pointer")(pool as *mut ::core::ffi::c_void);
        }
        pool = nextp;
    }
    unsafe {
        xmlFree.expect("non-null function pointer")(dict as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictLookup(
    mut dict: xmlDictPtr,
    mut name: *const xmlChar,
    mut len: ::core::ffi::c_int,
) -> *const xmlChar {
    let Some(dict_ref) = (unsafe { dict.as_mut() }) else {
        return ::core::ptr::null::<xmlChar>();
    };
    if name.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    let l = if len < 0 as ::core::ffi::c_int {
        unsafe { strlen(name as *const ::core::ffi::c_char) as ::core::ffi::c_uint }
    } else {
        len as ::core::ffi::c_uint
    };
    if (dict_ref.limit > 0 as size_t && l as size_t >= dict_ref.limit)
        || l > (INT_MAX / 2 as ::core::ffi::c_int) as ::core::ffi::c_uint
    {
        return ::core::ptr::null::<xmlChar>();
    }
    let okey = dict_compute_key(dict_ref.size, name, l as ::core::ffi::c_int, dict_ref.seed);
    let mut key = (okey as size_t).wrapping_rem(dict_ref.size) as ::core::ffi::c_ulong;
    let bucket = dict_bucket_entry(dict_ref, key);
    let mut nbi = 0 as ::core::ffi::c_ulong;
    let mut insert = if dict_entry_valid(bucket) {
        bucket
    } else {
        ::core::ptr::null_mut::<xmlDictEntry>()
    };
    while !insert.is_null() {
        if dict_entry_matches(insert, okey, l, name) {
            return dict_entry_name(insert);
        }
        let next = dict_entry_next(insert);
        if next.is_null() {
            break;
        }
        nbi = nbi.wrapping_add(1);
        insert = next;
    }
    if let Some(subdict) = unsafe { dict_ref.subdict.as_ref() } {
        let skey = if (dict_ref.size == MIN_DICT_SIZE as size_t
            && subdict.size != MIN_DICT_SIZE as size_t)
            || (dict_ref.size != MIN_DICT_SIZE as size_t && subdict.size == MIN_DICT_SIZE as size_t)
        {
            dict_compute_key(subdict.size, name, l as ::core::ffi::c_int, subdict.seed)
        } else {
            okey
        };
        key = (skey as size_t).wrapping_rem(subdict.size) as ::core::ffi::c_ulong;
        let mut tmp = dict_bucket_entry(subdict, key);
        if dict_entry_valid(tmp) {
            while !tmp.is_null() {
                if dict_entry_matches(tmp, skey, l, name) {
                    return dict_entry_name(tmp);
                }
                let next = dict_entry_next(tmp);
                if next.is_null() {
                    break;
                }
                nbi = nbi.wrapping_add(1);
                tmp = next;
            }
        }
        key = (okey as size_t).wrapping_rem(dict_ref.size) as ::core::ffi::c_ulong;
    }
    let ret = unsafe { xmlDictAddString(dict, name, l) };
    if ret.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    let entry = if insert.is_null() {
        bucket
    } else {
        let entry = unsafe {
            xmlMalloc.expect("non-null function pointer")(
                ::core::mem::size_of::<xmlDictEntry>() as size_t
            ) as xmlDictEntryPtr
        };
        if entry.is_null() {
            return ::core::ptr::null::<xmlChar>();
        }
        entry
    };
    unsafe {
        (*entry).name = ret;
        (*entry).len = l;
        (*entry).next = ::core::ptr::null_mut::<_xmlDictEntry>();
        (*entry).valid = 1 as ::core::ffi::c_int;
        (*entry).okey = okey;
    }
    if !insert.is_null() {
        unsafe {
            (*insert).next = entry as *mut _xmlDictEntry;
        }
    }
    dict_ref.nbElems = dict_ref.nbElems.wrapping_add(1);
    if nbi > MAX_HASH_LEN as ::core::ffi::c_ulong
        && dict_ref.size <= (MAX_DICT_HASH / 2 as ::core::ffi::c_int / MAX_HASH_LEN) as size_t
        && unsafe {
            xmlDictGrow(
                dict,
                ((MAX_HASH_LEN * 2 as ::core::ffi::c_int) as size_t).wrapping_mul(dict_ref.size),
            )
        } != 0 as ::core::ffi::c_int
    {
        return ::core::ptr::null::<xmlChar>();
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictExists(
    mut dict: xmlDictPtr,
    mut name: *const xmlChar,
    mut len: ::core::ffi::c_int,
) -> *const xmlChar {
    let Some(dict_ref) = (unsafe { dict.as_ref() }) else {
        return ::core::ptr::null::<xmlChar>();
    };
    if name.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    let l = if len < 0 as ::core::ffi::c_int {
        unsafe { strlen(name as *const ::core::ffi::c_char) as ::core::ffi::c_uint }
    } else {
        len as ::core::ffi::c_uint
    };
    if (dict_ref.limit > 0 as size_t && l as size_t >= dict_ref.limit)
        || l > (INT_MAX / 2 as ::core::ffi::c_int) as ::core::ffi::c_uint
    {
        return ::core::ptr::null::<xmlChar>();
    }
    let okey = dict_compute_key(dict_ref.size, name, l as ::core::ffi::c_int, dict_ref.seed);
    let key = (okey as size_t).wrapping_rem(dict_ref.size) as ::core::ffi::c_ulong;
    let mut insert = dict_bucket_entry(dict_ref, key);
    if dict_entry_valid(insert) {
        while !insert.is_null() {
            if dict_entry_matches(insert, okey, l, name) {
                return dict_entry_name(insert);
            }
            let next = dict_entry_next(insert);
            if next.is_null() {
                break;
            }
            insert = next;
        }
    }
    if let Some(subdict) = unsafe { dict_ref.subdict.as_ref() } {
        let skey = if (dict_ref.size == MIN_DICT_SIZE as size_t
            && subdict.size != MIN_DICT_SIZE as size_t)
            || (dict_ref.size != MIN_DICT_SIZE as size_t && subdict.size == MIN_DICT_SIZE as size_t)
        {
            dict_compute_key(subdict.size, name, l as ::core::ffi::c_int, subdict.seed)
        } else {
            okey
        };
        let key = (skey as size_t).wrapping_rem(subdict.size) as ::core::ffi::c_ulong;
        let mut tmp = dict_bucket_entry(subdict, key);
        if dict_entry_valid(tmp) {
            while !tmp.is_null() {
                if dict_entry_matches(tmp, skey, l, name) {
                    return dict_entry_name(tmp);
                }
                let next = dict_entry_next(tmp);
                if next.is_null() {
                    break;
                }
                tmp = next;
            }
        }
    }
    return ::core::ptr::null::<xmlChar>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictQLookup(
    mut dict: xmlDictPtr,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
) -> *const xmlChar {
    let Some(dict_ref) = (unsafe { dict.as_mut() }) else {
        return ::core::ptr::null::<xmlChar>();
    };
    if name.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if prefix.is_null() {
        return unsafe { xmlDictLookup(dict, name, -(1 as ::core::ffi::c_int)) };
    }
    let l = unsafe { strlen(name as *const ::core::ffi::c_char) as ::core::ffi::c_uint };
    let plen = unsafe { strlen(prefix as *const ::core::ffi::c_char) as ::core::ffi::c_uint };
    let len = l.wrapping_add((1 as ::core::ffi::c_uint).wrapping_add(plen));
    let okey = dict_compute_qkey(
        dict_ref.size,
        prefix,
        plen as ::core::ffi::c_int,
        name,
        l as ::core::ffi::c_int,
        dict_ref.seed,
    );
    let mut key = (okey as size_t).wrapping_rem(dict_ref.size) as ::core::ffi::c_ulong;
    let bucket = dict_bucket_entry(dict_ref, key);
    let mut nbi = 0 as ::core::ffi::c_ulong;
    let mut insert = if dict_entry_valid(bucket) {
        bucket
    } else {
        ::core::ptr::null_mut::<xmlDictEntry>()
    };
    while !insert.is_null() {
        if dict_qentry_matches(insert, okey, len, prefix, name) {
            return dict_entry_name(insert);
        }
        let next = dict_entry_next(insert);
        if next.is_null() {
            break;
        }
        nbi = nbi.wrapping_add(1);
        insert = next;
    }
    if let Some(subdict) = unsafe { dict_ref.subdict.as_ref() } {
        let skey = if (dict_ref.size == MIN_DICT_SIZE as size_t
            && subdict.size != MIN_DICT_SIZE as size_t)
            || (dict_ref.size != MIN_DICT_SIZE as size_t && subdict.size == MIN_DICT_SIZE as size_t)
        {
            dict_compute_qkey(
                subdict.size,
                prefix,
                plen as ::core::ffi::c_int,
                name,
                l as ::core::ffi::c_int,
                subdict.seed,
            )
        } else {
            okey
        };
        key = (skey as size_t).wrapping_rem(subdict.size) as ::core::ffi::c_ulong;
        let mut tmp = dict_bucket_entry(subdict, key);
        if dict_entry_valid(tmp) {
            while !tmp.is_null() {
                if dict_qentry_matches(tmp, skey, len, prefix, name) {
                    return dict_entry_name(tmp);
                }
                let next = dict_entry_next(tmp);
                if next.is_null() {
                    break;
                }
                nbi = nbi.wrapping_add(1);
                tmp = next;
            }
        }
        key = (okey as size_t).wrapping_rem(dict_ref.size) as ::core::ffi::c_ulong;
    }
    let ret = unsafe { xmlDictAddQString(dict, prefix, plen, name, l) };
    if ret.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    let entry = if insert.is_null() {
        bucket
    } else {
        let entry = unsafe {
            xmlMalloc.expect("non-null function pointer")(
                ::core::mem::size_of::<xmlDictEntry>() as size_t
            ) as xmlDictEntryPtr
        };
        if entry.is_null() {
            return ::core::ptr::null::<xmlChar>();
        }
        entry
    };
    unsafe {
        (*entry).name = ret;
        (*entry).len = len;
        (*entry).next = ::core::ptr::null_mut::<_xmlDictEntry>();
        (*entry).valid = 1 as ::core::ffi::c_int;
        (*entry).okey = okey;
    }
    if !insert.is_null() {
        unsafe {
            (*insert).next = entry as *mut _xmlDictEntry;
        }
    }
    dict_ref.nbElems = dict_ref.nbElems.wrapping_add(1);
    if nbi > MAX_HASH_LEN as ::core::ffi::c_ulong
        && dict_ref.size <= (MAX_DICT_HASH / 2 as ::core::ffi::c_int / MAX_HASH_LEN) as size_t
    {
        unsafe {
            xmlDictGrow(
                dict,
                ((MAX_HASH_LEN * 2 as ::core::ffi::c_int) as size_t).wrapping_mul(dict_ref.size),
            );
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictOwns(
    mut dict: xmlDictPtr,
    mut str: *const xmlChar,
) -> ::core::ffi::c_int {
    let mut pool: xmlDictStringsPtr = ::core::ptr::null_mut::<xmlDictStrings>();
    let Some(dict) = (unsafe { dict.as_ref() }) else {
        return -(1 as ::core::ffi::c_int);
    };
    if str.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    pool = dict.strings;
    while !pool.is_null() {
        let start =
            unsafe { ::core::ptr::addr_of!((*pool).array).cast::<xmlChar>() } as *const xmlChar;
        let end = unsafe { (*pool).free } as *const xmlChar;
        if str >= start && str <= end {
            return 1 as ::core::ffi::c_int;
        }
        pool = unsafe { (*pool).next };
    }
    if !dict.subdict.is_null() {
        return unsafe { xmlDictOwns(dict.subdict as xmlDictPtr, str) };
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictSize(mut dict: xmlDictPtr) -> ::core::ffi::c_int {
    let Some(dict) = (unsafe { dict.as_ref() }) else {
        return -(1 as ::core::ffi::c_int);
    };
    if !dict.subdict.is_null() {
        return dict
            .nbElems
            .wrapping_add(unsafe { (*dict.subdict).nbElems }) as ::core::ffi::c_int;
    }
    return dict.nbElems as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictSetLimit(mut dict: xmlDictPtr, mut limit: size_t) -> size_t {
    let Some(dict) = (unsafe { dict.as_mut() }) else {
        return 0 as size_t;
    };
    let ret = dict.limit;
    dict.limit = limit;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictGetUsage(mut dict: xmlDictPtr) -> size_t {
    let mut pool: xmlDictStringsPtr = ::core::ptr::null_mut::<xmlDictStrings>();
    let mut limit: size_t = 0 as size_t;
    let Some(dict) = (unsafe { dict.as_ref() }) else {
        return 0 as size_t;
    };
    pool = dict.strings;
    while !pool.is_null() {
        limit = limit.wrapping_add(unsafe { (*pool).size });
        pool = unsafe { (*pool).next };
    }
    return limit;
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
