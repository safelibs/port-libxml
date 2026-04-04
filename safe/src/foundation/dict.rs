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
        let available = unsafe { (*pool).end.offset_from((*pool).free) as ::core::ffi::c_long as size_t };
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
) -> *const xmlChar { unsafe {
    let mut current_block: u64;
    let mut pool: xmlDictStringsPtr = ::core::ptr::null_mut::<xmlDictStrings>();
    let mut ret: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut size: size_t = 0 as size_t;
    let mut limit: size_t = 0 as size_t;
    if prefix.is_null() {
        return xmlDictAddString(dict, name, namelen);
    }
    pool = (*dict).strings;
    loop {
        if pool.is_null() {
            current_block = 13513818773234778473;
            break;
        }
        if (*pool).end.offset_from((*pool).free) as ::core::ffi::c_long as size_t
            > namelen
                .wrapping_add(plen)
                .wrapping_add(1 as ::core::ffi::c_uint) as size_t
        {
            current_block = 9272350092442567110;
            break;
        }
        if (*pool).size > size {
            size = (*pool).size;
        }
        limit = limit.wrapping_add((*pool).size);
        pool = (*pool).next;
    }
    match current_block {
        13513818773234778473 => {
            if pool.is_null() {
                if (*dict).limit > 0 as size_t && limit > (*dict).limit {
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
                pool = xmlMalloc.expect("non-null function pointer")(
                    (::core::mem::size_of::<xmlDictStrings>() as size_t).wrapping_add(size),
                ) as xmlDictStringsPtr;
                if pool.is_null() {
                    return ::core::ptr::null::<xmlChar>();
                }
                (*pool).size = size;
                (*pool).nbStrings = 0 as size_t;
                (*pool).free = (&raw mut (*pool).array as *mut xmlChar)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut xmlChar;
                (*pool).end =
                    (&raw mut (*pool).array as *mut xmlChar).offset(size as isize) as *mut xmlChar;
                (*pool).next = (*dict).strings;
                (*dict).strings = pool;
            }
        }
        _ => {}
    }
    ret = (*pool).free;
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
    return ret;
}}
unsafe extern "C" fn xmlDictComputeBigKey(
    mut data: *const xmlChar,
    mut namelen: ::core::ffi::c_int,
    mut seed: ::core::ffi::c_int,
) -> uint32_t { unsafe {
    let mut hash: uint32_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    if namelen <= 0 as ::core::ffi::c_int || data.is_null() {
        return 0 as uint32_t;
    }
    hash = seed as uint32_t;
    i = 0 as ::core::ffi::c_int;
    while i < namelen {
        hash = hash.wrapping_add(*data.offset(i as isize) as uint32_t);
        hash = hash.wrapping_add(hash << 10 as ::core::ffi::c_int);
        hash ^= hash >> 6 as ::core::ffi::c_int;
        i += 1;
    }
    hash = hash.wrapping_add(hash << 3 as ::core::ffi::c_int);
    hash ^= hash >> 11 as ::core::ffi::c_int;
    hash = hash.wrapping_add(hash << 15 as ::core::ffi::c_int);
    return hash;
}}
unsafe extern "C" fn xmlDictComputeBigQKey(
    mut prefix: *const xmlChar,
    mut plen: ::core::ffi::c_int,
    mut name: *const xmlChar,
    mut len: ::core::ffi::c_int,
    mut seed: ::core::ffi::c_int,
) -> ::core::ffi::c_ulong { unsafe {
    let mut hash: uint32_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    hash = seed as uint32_t;
    i = 0 as ::core::ffi::c_int;
    while i < plen {
        hash = hash.wrapping_add(*prefix.offset(i as isize) as uint32_t);
        hash = hash.wrapping_add(hash << 10 as ::core::ffi::c_int);
        hash ^= hash >> 6 as ::core::ffi::c_int;
        i += 1;
    }
    hash = hash.wrapping_add(':' as i32 as uint32_t);
    hash = hash.wrapping_add(hash << 10 as ::core::ffi::c_int);
    hash ^= hash >> 6 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < len {
        hash = hash.wrapping_add(*name.offset(i as isize) as uint32_t);
        hash = hash.wrapping_add(hash << 10 as ::core::ffi::c_int);
        hash ^= hash >> 6 as ::core::ffi::c_int;
        i += 1;
    }
    hash = hash.wrapping_add(hash << 3 as ::core::ffi::c_int);
    hash ^= hash >> 11 as ::core::ffi::c_int;
    hash = hash.wrapping_add(hash << 15 as ::core::ffi::c_int);
    return hash as ::core::ffi::c_ulong;
}}
unsafe extern "C" fn xmlDictComputeFastKey(
    mut name: *const xmlChar,
    mut namelen: ::core::ffi::c_int,
    mut seed: ::core::ffi::c_int,
) -> ::core::ffi::c_ulong { unsafe {
    let mut value: ::core::ffi::c_ulong = seed as ::core::ffi::c_ulong;
    if name.is_null() || namelen <= 0 as ::core::ffi::c_int {
        return value;
    }
    value = value.wrapping_add(*name as ::core::ffi::c_ulong);
    value <<= 5 as ::core::ffi::c_int;
    if namelen > 10 as ::core::ffi::c_int {
        value =
            value
                .wrapping_add(*name.offset((namelen - 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_ulong);
        namelen = 10 as ::core::ffi::c_int;
    }
    let mut current_block_16: u64;
    match namelen {
        10 => {
            value = value.wrapping_add(
                *name.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
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
            value = value.wrapping_add(
                *name.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_16 = 10477942228951044767;
        }
        _ => {}
    }
    match current_block_16 {
        10477942228951044767 => {
            value = value.wrapping_add(
                *name.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_16 = 6178868810057640957;
        }
        _ => {}
    }
    match current_block_16 {
        6178868810057640957 => {
            value = value.wrapping_add(
                *name.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_16 = 9879368920036322223;
        }
        _ => {}
    }
    match current_block_16 {
        9879368920036322223 => {
            value = value.wrapping_add(
                *name.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_16 = 7599933913323472286;
        }
        _ => {}
    }
    match current_block_16 {
        7599933913323472286 => {
            value = value.wrapping_add(
                *name.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_16 = 1634274037810632810;
        }
        _ => {}
    }
    match current_block_16 {
        1634274037810632810 => {
            value = value.wrapping_add(
                *name.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_16 = 3992153788896120206;
        }
        _ => {}
    }
    match current_block_16 {
        3992153788896120206 => {
            value = value.wrapping_add(
                *name.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_16 = 8753412356531521447;
        }
        _ => {}
    }
    match current_block_16 {
        8753412356531521447 => {
            value = value.wrapping_add(
                *name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
        }
        _ => {}
    }
    return value;
}}
unsafe extern "C" fn xmlDictComputeFastQKey(
    mut prefix: *const xmlChar,
    mut plen: ::core::ffi::c_int,
    mut name: *const xmlChar,
    mut len: ::core::ffi::c_int,
    mut seed: ::core::ffi::c_int,
) -> ::core::ffi::c_ulong { unsafe {
    let mut value: ::core::ffi::c_ulong = seed as ::core::ffi::c_ulong;
    if plen == 0 as ::core::ffi::c_int {
        value = value.wrapping_add(
            (30 as ::core::ffi::c_ulong).wrapping_mul(':' as i32 as ::core::ffi::c_ulong),
        );
    } else {
        value = value.wrapping_add(
            (30 as ::core::ffi::c_int * *prefix as ::core::ffi::c_int) as ::core::ffi::c_ulong,
        );
    }
    if len > 10 as ::core::ffi::c_int {
        let mut offset: ::core::ffi::c_int =
            len - (plen + 1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int);
        if offset < 0 as ::core::ffi::c_int {
            offset = len - (10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int);
        }
        value = value.wrapping_add(*name.offset(offset as isize) as ::core::ffi::c_ulong);
        len = 10 as ::core::ffi::c_int;
        if plen > 10 as ::core::ffi::c_int {
            plen = 10 as ::core::ffi::c_int;
        }
    }
    let mut current_block_20: u64;
    match plen {
        10 => {
            value = value.wrapping_add(
                *prefix.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
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
            value = value.wrapping_add(
                *prefix.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_20 = 9570895743713506940;
        }
        _ => {}
    }
    match current_block_20 {
        9570895743713506940 => {
            value = value.wrapping_add(
                *prefix.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_20 = 3809271318960479663;
        }
        _ => {}
    }
    match current_block_20 {
        3809271318960479663 => {
            value = value.wrapping_add(
                *prefix.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_20 = 1760573699125010265;
        }
        _ => {}
    }
    match current_block_20 {
        1760573699125010265 => {
            value = value.wrapping_add(
                *prefix.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_20 = 8203539728489389117;
        }
        _ => {}
    }
    match current_block_20 {
        8203539728489389117 => {
            value = value.wrapping_add(
                *prefix.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_20 = 3174106958175389174;
        }
        _ => {}
    }
    match current_block_20 {
        3174106958175389174 => {
            value = value.wrapping_add(
                *prefix.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_20 = 71433336188168654;
        }
        _ => {}
    }
    match current_block_20 {
        71433336188168654 => {
            value = value.wrapping_add(
                *prefix.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_20 = 6539070127515362870;
        }
        _ => {}
    }
    match current_block_20 {
        6539070127515362870 => {
            value = value.wrapping_add(
                *prefix.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_20 = 10006674590530960285;
        }
        _ => {}
    }
    match current_block_20 {
        10006674590530960285 => {
            value = value.wrapping_add(
                *prefix.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
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
            value = value.wrapping_add(
                *name.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
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
            value = value.wrapping_add(
                *name.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_36 = 16454796981030128573;
        }
        _ => {}
    }
    match current_block_36 {
        16454796981030128573 => {
            value = value.wrapping_add(
                *name.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_36 = 10379619488888424139;
        }
        _ => {}
    }
    match current_block_36 {
        10379619488888424139 => {
            value = value.wrapping_add(
                *name.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_36 = 18024754184016332365;
        }
        _ => {}
    }
    match current_block_36 {
        18024754184016332365 => {
            value = value.wrapping_add(
                *name.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_36 = 8167404252455708155;
        }
        _ => {}
    }
    match current_block_36 {
        8167404252455708155 => {
            value = value.wrapping_add(
                *name.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_36 = 16790513941173568777;
        }
        _ => {}
    }
    match current_block_36 {
        16790513941173568777 => {
            value = value.wrapping_add(
                *name.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_36 = 2447873016272255407;
        }
        _ => {}
    }
    match current_block_36 {
        2447873016272255407 => {
            value = value.wrapping_add(
                *name.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_36 = 3840978892466691305;
        }
        _ => {}
    }
    match current_block_36 {
        3840978892466691305 => {
            value = value.wrapping_add(
                *name.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
            current_block_36 = 11321417492605204613;
        }
        _ => {}
    }
    match current_block_36 {
        11321417492605204613 => {
            value = value.wrapping_add(
                *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulong
            );
        }
        _ => {}
    }
    return value;
}}
#[no_mangle]
pub unsafe extern "C" fn xmlDictCreate() -> xmlDictPtr { unsafe {
    let mut dict: xmlDictPtr = ::core::ptr::null_mut::<xmlDict>();
    if xmlDictInitialized == 0 {
        if __xmlInitializeDict() == 0 {
            return ::core::ptr::null_mut::<xmlDict>();
        }
    }
    dict = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlDict>() as size_t)
        as xmlDictPtr;
    if !dict.is_null() {
        (*dict).ref_counter = 1 as ::core::ffi::c_int;
        (*dict).limit = 0 as size_t;
        (*dict).size = MIN_DICT_SIZE as size_t;
        (*dict).nbElems = 0 as ::core::ffi::c_uint;
        (*dict).dict = xmlMalloc.expect("non-null function pointer")(
            (MIN_DICT_SIZE as size_t)
                .wrapping_mul(::core::mem::size_of::<xmlDictEntry>() as size_t),
        ) as *mut _xmlDictEntry;
        (*dict).strings = ::core::ptr::null_mut::<xmlDictStrings>();
        (*dict).subdict = ::core::ptr::null_mut::<_xmlDict>();
        if !(*dict).dict.is_null() {
            memset(
                (*dict).dict as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (MIN_DICT_SIZE as size_t)
                    .wrapping_mul(::core::mem::size_of::<xmlDictEntry>() as size_t),
            );
            (*dict).seed = __xmlRandom();
            return dict;
        }
        xmlFree.expect("non-null function pointer")(dict as *mut ::core::ffi::c_void);
    }
    return ::core::ptr::null_mut::<xmlDict>();
}}
#[no_mangle]
pub unsafe extern "C" fn xmlDictCreateSub(mut sub: xmlDictPtr) -> xmlDictPtr { unsafe {
    let mut dict: xmlDictPtr = xmlDictCreate();
    if !dict.is_null() && !sub.is_null() {
        (*dict).seed = (*sub).seed;
        (*dict).subdict = sub as *mut _xmlDict;
        xmlDictReference((*dict).subdict as xmlDictPtr);
    }
    return dict;
}}
#[no_mangle]
pub unsafe extern "C" fn xmlDictReference(mut dict: xmlDictPtr) -> ::core::ffi::c_int { unsafe {
    if xmlDictInitialized == 0 {
        if __xmlInitializeDict() == 0 {
            return -(1 as ::core::ffi::c_int);
        }
    }
    if dict.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    xmlRMutexLock(xmlDictMutex);
    (*dict).ref_counter += 1;
    xmlRMutexUnlock(xmlDictMutex);
    return 0 as ::core::ffi::c_int;
}}
unsafe extern "C" fn xmlDictGrow(mut dict: xmlDictPtr, mut size: size_t) -> ::core::ffi::c_int { unsafe {
    let mut key: ::core::ffi::c_ulong = 0;
    let mut okey: ::core::ffi::c_ulong = 0;
    let mut oldsize: size_t = 0;
    let mut i: size_t = 0;
    let mut iter: xmlDictEntryPtr = ::core::ptr::null_mut::<xmlDictEntry>();
    let mut next: xmlDictEntryPtr = ::core::ptr::null_mut::<xmlDictEntry>();
    let mut olddict: *mut _xmlDictEntry = ::core::ptr::null_mut::<_xmlDictEntry>();
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut keep_keys: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if dict.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if size < 8 as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    if size > (8 as ::core::ffi::c_int * 2048 as ::core::ffi::c_int) as size_t {
        return -(1 as ::core::ffi::c_int);
    }
    oldsize = (*dict).size;
    olddict = (*dict).dict;
    if olddict.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if oldsize == MIN_DICT_SIZE as size_t {
        keep_keys = 0 as ::core::ffi::c_int;
    }
    (*dict).dict = xmlMalloc.expect("non-null function pointer")(
        size.wrapping_mul(::core::mem::size_of::<xmlDictEntry>() as size_t),
    ) as *mut _xmlDictEntry;
    if (*dict).dict.is_null() {
        (*dict).dict = olddict;
        return -(1 as ::core::ffi::c_int);
    }
    memset(
        (*dict).dict as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        size.wrapping_mul(::core::mem::size_of::<xmlDictEntry>() as size_t),
    );
    (*dict).size = size;
    i = 0 as size_t;
    while i < oldsize {
        if !((*olddict.offset(i as isize)).valid == 0 as ::core::ffi::c_int) {
            if keep_keys != 0 {
                okey = (*olddict.offset(i as isize)).okey;
            } else {
                okey = if (*dict).size == MIN_DICT_SIZE as size_t {
                    xmlDictComputeFastKey(
                        (*olddict.offset(i as isize)).name,
                        (*olddict.offset(i as isize)).len as ::core::ffi::c_int,
                        (*dict).seed,
                    )
                } else {
                    xmlDictComputeBigKey(
                        (*olddict.offset(i as isize)).name,
                        (*olddict.offset(i as isize)).len as ::core::ffi::c_int,
                        (*dict).seed,
                    ) as ::core::ffi::c_ulong
                };
            }
            key = (okey as size_t).wrapping_rem((*dict).size) as ::core::ffi::c_ulong;
            if (*(*dict).dict.offset(key as isize)).valid == 0 as ::core::ffi::c_int {
                memcpy(
                    (*dict).dict.offset(key as isize) as *mut _xmlDictEntry
                        as *mut ::core::ffi::c_void,
                    olddict.offset(i as isize) as *mut _xmlDictEntry as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<xmlDictEntry>() as size_t,
                );
                let ref mut fresh0 = (*(*dict).dict.offset(key as isize)).next;
                *fresh0 = ::core::ptr::null_mut::<_xmlDictEntry>();
                (*(*dict).dict.offset(key as isize)).okey = okey;
            } else {
                let mut entry: xmlDictEntryPtr = ::core::ptr::null_mut::<xmlDictEntry>();
                entry = xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<
                    xmlDictEntry,
                >() as size_t) as xmlDictEntryPtr;
                if !entry.is_null() {
                    (*entry).name = (*olddict.offset(i as isize)).name;
                    (*entry).len = (*olddict.offset(i as isize)).len;
                    (*entry).okey = okey;
                    (*entry).next = (*(*dict).dict.offset(key as isize)).next;
                    (*entry).valid = 1 as ::core::ffi::c_int;
                    let ref mut fresh1 = (*(*dict).dict.offset(key as isize)).next;
                    *fresh1 = entry as *mut _xmlDictEntry;
                } else {
                    ret = -(1 as ::core::ffi::c_int);
                }
            }
        }
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < oldsize {
        iter = (*olddict.offset(i as isize)).next as xmlDictEntryPtr;
        while !iter.is_null() {
            next = (*iter).next as xmlDictEntryPtr;
            if keep_keys != 0 {
                okey = (*iter).okey;
            } else {
                okey = if (*dict).size == MIN_DICT_SIZE as size_t {
                    xmlDictComputeFastKey(
                        (*iter).name,
                        (*iter).len as ::core::ffi::c_int,
                        (*dict).seed,
                    )
                } else {
                    xmlDictComputeBigKey(
                        (*iter).name,
                        (*iter).len as ::core::ffi::c_int,
                        (*dict).seed,
                    ) as ::core::ffi::c_ulong
                };
            }
            key = (okey as size_t).wrapping_rem((*dict).size) as ::core::ffi::c_ulong;
            if (*(*dict).dict.offset(key as isize)).valid == 0 as ::core::ffi::c_int {
                memcpy(
                    (*dict).dict.offset(key as isize) as *mut _xmlDictEntry
                        as *mut ::core::ffi::c_void,
                    iter as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<xmlDictEntry>() as size_t,
                );
                let ref mut fresh2 = (*(*dict).dict.offset(key as isize)).next;
                *fresh2 = ::core::ptr::null_mut::<_xmlDictEntry>();
                (*(*dict).dict.offset(key as isize)).valid = 1 as ::core::ffi::c_int;
                (*(*dict).dict.offset(key as isize)).okey = okey;
                xmlFree.expect("non-null function pointer")(iter as *mut ::core::ffi::c_void);
            } else {
                (*iter).next = (*(*dict).dict.offset(key as isize)).next;
                (*iter).okey = okey;
                let ref mut fresh3 = (*(*dict).dict.offset(key as isize)).next;
                *fresh3 = iter as *mut _xmlDictEntry;
            }
            iter = next;
        }
        i = i.wrapping_add(1);
    }
    xmlFree.expect("non-null function pointer")(olddict as *mut ::core::ffi::c_void);
    return ret;
}}
#[no_mangle]
pub unsafe extern "C" fn xmlDictFree(mut dict: xmlDictPtr) { unsafe {
    let mut i: size_t = 0;
    let mut iter: xmlDictEntryPtr = ::core::ptr::null_mut::<xmlDictEntry>();
    let mut next: xmlDictEntryPtr = ::core::ptr::null_mut::<xmlDictEntry>();
    let mut inside_dict: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pool: xmlDictStringsPtr = ::core::ptr::null_mut::<xmlDictStrings>();
    let mut nextp: xmlDictStringsPtr = ::core::ptr::null_mut::<xmlDictStrings>();
    if dict.is_null() {
        return;
    }
    if xmlDictInitialized == 0 {
        if __xmlInitializeDict() == 0 {
            return;
        }
    }
    xmlRMutexLock(xmlDictMutex);
    (*dict).ref_counter -= 1;
    if (*dict).ref_counter > 0 as ::core::ffi::c_int {
        xmlRMutexUnlock(xmlDictMutex);
        return;
    }
    xmlRMutexUnlock(xmlDictMutex);
    if !(*dict).subdict.is_null() {
        xmlDictFree((*dict).subdict as xmlDictPtr);
    }
    if !(*dict).dict.is_null() {
        i = 0 as size_t;
        while i < (*dict).size && (*dict).nbElems > 0 as ::core::ffi::c_uint {
            iter = (*dict).dict.offset(i as isize) as *mut _xmlDictEntry as xmlDictEntryPtr;
            if !((*iter).valid == 0 as ::core::ffi::c_int) {
                inside_dict = 1 as ::core::ffi::c_int;
                while !iter.is_null() {
                    next = (*iter).next as xmlDictEntryPtr;
                    if inside_dict == 0 {
                        xmlFree.expect("non-null function pointer")(
                            iter as *mut ::core::ffi::c_void,
                        );
                    }
                    (*dict).nbElems = (*dict).nbElems.wrapping_sub(1);
                    inside_dict = 0 as ::core::ffi::c_int;
                    iter = next;
                }
            }
            i = i.wrapping_add(1);
        }
        xmlFree.expect("non-null function pointer")((*dict).dict as *mut ::core::ffi::c_void);
    }
    pool = (*dict).strings;
    while !pool.is_null() {
        nextp = (*pool).next;
        xmlFree.expect("non-null function pointer")(pool as *mut ::core::ffi::c_void);
        pool = nextp;
    }
    xmlFree.expect("non-null function pointer")(dict as *mut ::core::ffi::c_void);
}}
#[no_mangle]
pub unsafe extern "C" fn xmlDictLookup(
    mut dict: xmlDictPtr,
    mut name: *const xmlChar,
    mut len: ::core::ffi::c_int,
) -> *const xmlChar { unsafe {
    let mut key: ::core::ffi::c_ulong = 0;
    let mut okey: ::core::ffi::c_ulong = 0;
    let mut nbi: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut entry: xmlDictEntryPtr = ::core::ptr::null_mut::<xmlDictEntry>();
    let mut insert: xmlDictEntryPtr = ::core::ptr::null_mut::<xmlDictEntry>();
    let mut ret: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut l: ::core::ffi::c_uint = 0;
    if dict.is_null() || name.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if len < 0 as ::core::ffi::c_int {
        l = strlen(name as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
    } else {
        l = len as ::core::ffi::c_uint;
    }
    if (*dict).limit > 0 as size_t && l as size_t >= (*dict).limit
        || l > (INT_MAX / 2 as ::core::ffi::c_int) as ::core::ffi::c_uint
    {
        return ::core::ptr::null::<xmlChar>();
    }
    okey = if (*dict).size == MIN_DICT_SIZE as size_t {
        xmlDictComputeFastKey(name, l as ::core::ffi::c_int, (*dict).seed)
    } else {
        xmlDictComputeBigKey(name, l as ::core::ffi::c_int, (*dict).seed) as ::core::ffi::c_ulong
    };
    key = (okey as size_t).wrapping_rem((*dict).size) as ::core::ffi::c_ulong;
    if (*(*dict).dict.offset(key as isize)).valid == 0 as ::core::ffi::c_int {
        insert = ::core::ptr::null_mut::<xmlDictEntry>();
    } else {
        insert = (*dict).dict.offset(key as isize) as *mut _xmlDictEntry as xmlDictEntryPtr;
        while !(*insert).next.is_null() {
            if (*insert).okey == okey && (*insert).len == l {
                if memcmp(
                    (*insert).name as *const ::core::ffi::c_void,
                    name as *const ::core::ffi::c_void,
                    l as size_t,
                ) == 0
                {
                    return (*insert).name;
                }
            }
            nbi = nbi.wrapping_add(1);
            insert = (*insert).next as xmlDictEntryPtr;
        }
        if (*insert).okey == okey && (*insert).len == l {
            if memcmp(
                (*insert).name as *const ::core::ffi::c_void,
                name as *const ::core::ffi::c_void,
                l as size_t,
            ) == 0
            {
                return (*insert).name;
            }
        }
    }
    if !(*dict).subdict.is_null() {
        let mut skey: ::core::ffi::c_ulong = 0;
        if (*dict).size == MIN_DICT_SIZE as size_t
            && (*(*dict).subdict).size != MIN_DICT_SIZE as size_t
            || (*dict).size != MIN_DICT_SIZE as size_t
                && (*(*dict).subdict).size == MIN_DICT_SIZE as size_t
        {
            skey = if (*(*dict).subdict).size == MIN_DICT_SIZE as size_t {
                xmlDictComputeFastKey(name, l as ::core::ffi::c_int, (*(*dict).subdict).seed)
            } else {
                xmlDictComputeBigKey(name, l as ::core::ffi::c_int, (*(*dict).subdict).seed)
                    as ::core::ffi::c_ulong
            };
        } else {
            skey = okey;
        }
        key = (skey as size_t).wrapping_rem((*(*dict).subdict).size) as ::core::ffi::c_ulong;
        if (*(*(*dict).subdict).dict.offset(key as isize)).valid != 0 as ::core::ffi::c_int {
            let mut tmp: xmlDictEntryPtr = ::core::ptr::null_mut::<xmlDictEntry>();
            tmp = (*(*dict).subdict).dict.offset(key as isize) as *mut _xmlDictEntry
                as xmlDictEntryPtr;
            while !(*tmp).next.is_null() {
                if (*tmp).okey == skey && (*tmp).len == l {
                    if memcmp(
                        (*tmp).name as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        l as size_t,
                    ) == 0
                    {
                        return (*tmp).name;
                    }
                }
                nbi = nbi.wrapping_add(1);
                tmp = (*tmp).next as xmlDictEntryPtr;
            }
            if (*tmp).okey == skey && (*tmp).len == l {
                if memcmp(
                    (*tmp).name as *const ::core::ffi::c_void,
                    name as *const ::core::ffi::c_void,
                    l as size_t,
                ) == 0
                {
                    return (*tmp).name;
                }
            }
        }
        key = (okey as size_t).wrapping_rem((*dict).size) as ::core::ffi::c_ulong;
    }
    ret = xmlDictAddString(dict, name, l);
    if ret.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if insert.is_null() {
        entry = (*dict).dict.offset(key as isize) as *mut _xmlDictEntry as xmlDictEntryPtr;
    } else {
        entry = xmlMalloc.expect("non-null function pointer")(
            ::core::mem::size_of::<xmlDictEntry>() as size_t,
        ) as xmlDictEntryPtr;
        if entry.is_null() {
            return ::core::ptr::null::<xmlChar>();
        }
    }
    (*entry).name = ret;
    (*entry).len = l;
    (*entry).next = ::core::ptr::null_mut::<_xmlDictEntry>();
    (*entry).valid = 1 as ::core::ffi::c_int;
    (*entry).okey = okey;
    if !insert.is_null() {
        (*insert).next = entry as *mut _xmlDictEntry;
    }
    (*dict).nbElems = (*dict).nbElems.wrapping_add(1);
    if nbi > MAX_HASH_LEN as ::core::ffi::c_ulong
        && (*dict).size <= (MAX_DICT_HASH / 2 as ::core::ffi::c_int / MAX_HASH_LEN) as size_t
    {
        if xmlDictGrow(
            dict,
            ((MAX_HASH_LEN * 2 as ::core::ffi::c_int) as size_t).wrapping_mul((*dict).size),
        ) != 0 as ::core::ffi::c_int
        {
            return ::core::ptr::null::<xmlChar>();
        }
    }
    return ret;
}}
#[no_mangle]
pub unsafe extern "C" fn xmlDictExists(
    mut dict: xmlDictPtr,
    mut name: *const xmlChar,
    mut len: ::core::ffi::c_int,
) -> *const xmlChar { unsafe {
    let mut key: ::core::ffi::c_ulong = 0;
    let mut okey: ::core::ffi::c_ulong = 0;
    let mut nbi: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut insert: xmlDictEntryPtr = ::core::ptr::null_mut::<xmlDictEntry>();
    let mut l: ::core::ffi::c_uint = 0;
    if dict.is_null() || name.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if len < 0 as ::core::ffi::c_int {
        l = strlen(name as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
    } else {
        l = len as ::core::ffi::c_uint;
    }
    if (*dict).limit > 0 as size_t && l as size_t >= (*dict).limit
        || l > (INT_MAX / 2 as ::core::ffi::c_int) as ::core::ffi::c_uint
    {
        return ::core::ptr::null::<xmlChar>();
    }
    okey = if (*dict).size == MIN_DICT_SIZE as size_t {
        xmlDictComputeFastKey(name, l as ::core::ffi::c_int, (*dict).seed)
    } else {
        xmlDictComputeBigKey(name, l as ::core::ffi::c_int, (*dict).seed) as ::core::ffi::c_ulong
    };
    key = (okey as size_t).wrapping_rem((*dict).size) as ::core::ffi::c_ulong;
    if (*(*dict).dict.offset(key as isize)).valid == 0 as ::core::ffi::c_int {
        insert = ::core::ptr::null_mut::<xmlDictEntry>();
    } else {
        insert = (*dict).dict.offset(key as isize) as *mut _xmlDictEntry as xmlDictEntryPtr;
        while !(*insert).next.is_null() {
            if (*insert).okey == okey && (*insert).len == l {
                if memcmp(
                    (*insert).name as *const ::core::ffi::c_void,
                    name as *const ::core::ffi::c_void,
                    l as size_t,
                ) == 0
                {
                    return (*insert).name;
                }
            }
            nbi = nbi.wrapping_add(1);
            insert = (*insert).next as xmlDictEntryPtr;
        }
        if (*insert).okey == okey && (*insert).len == l {
            if memcmp(
                (*insert).name as *const ::core::ffi::c_void,
                name as *const ::core::ffi::c_void,
                l as size_t,
            ) == 0
            {
                return (*insert).name;
            }
        }
    }
    if !(*dict).subdict.is_null() {
        let mut skey: ::core::ffi::c_ulong = 0;
        if (*dict).size == MIN_DICT_SIZE as size_t
            && (*(*dict).subdict).size != MIN_DICT_SIZE as size_t
            || (*dict).size != MIN_DICT_SIZE as size_t
                && (*(*dict).subdict).size == MIN_DICT_SIZE as size_t
        {
            skey = if (*(*dict).subdict).size == MIN_DICT_SIZE as size_t {
                xmlDictComputeFastKey(name, l as ::core::ffi::c_int, (*(*dict).subdict).seed)
            } else {
                xmlDictComputeBigKey(name, l as ::core::ffi::c_int, (*(*dict).subdict).seed)
                    as ::core::ffi::c_ulong
            };
        } else {
            skey = okey;
        }
        key = (skey as size_t).wrapping_rem((*(*dict).subdict).size) as ::core::ffi::c_ulong;
        if (*(*(*dict).subdict).dict.offset(key as isize)).valid != 0 as ::core::ffi::c_int {
            let mut tmp: xmlDictEntryPtr = ::core::ptr::null_mut::<xmlDictEntry>();
            tmp = (*(*dict).subdict).dict.offset(key as isize) as *mut _xmlDictEntry
                as xmlDictEntryPtr;
            while !(*tmp).next.is_null() {
                if (*tmp).okey == skey && (*tmp).len == l {
                    if memcmp(
                        (*tmp).name as *const ::core::ffi::c_void,
                        name as *const ::core::ffi::c_void,
                        l as size_t,
                    ) == 0
                    {
                        return (*tmp).name;
                    }
                }
                nbi = nbi.wrapping_add(1);
                tmp = (*tmp).next as xmlDictEntryPtr;
            }
            if (*tmp).okey == skey && (*tmp).len == l {
                if memcmp(
                    (*tmp).name as *const ::core::ffi::c_void,
                    name as *const ::core::ffi::c_void,
                    l as size_t,
                ) == 0
                {
                    return (*tmp).name;
                }
            }
        }
    }
    return ::core::ptr::null::<xmlChar>();
}}
#[no_mangle]
pub unsafe extern "C" fn xmlDictQLookup(
    mut dict: xmlDictPtr,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
) -> *const xmlChar { unsafe {
    let mut okey: ::core::ffi::c_ulong = 0;
    let mut key: ::core::ffi::c_ulong = 0;
    let mut nbi: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut entry: xmlDictEntryPtr = ::core::ptr::null_mut::<xmlDictEntry>();
    let mut insert: xmlDictEntryPtr = ::core::ptr::null_mut::<xmlDictEntry>();
    let mut ret: *const xmlChar = ::core::ptr::null::<xmlChar>();
    let mut len: ::core::ffi::c_uint = 0;
    let mut plen: ::core::ffi::c_uint = 0;
    let mut l: ::core::ffi::c_uint = 0;
    if dict.is_null() || name.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if prefix.is_null() {
        return xmlDictLookup(dict, name, -(1 as ::core::ffi::c_int));
    }
    len = strlen(name as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
    l = len;
    plen = strlen(prefix as *const ::core::ffi::c_char) as ::core::ffi::c_uint;
    len = len.wrapping_add((1 as ::core::ffi::c_uint).wrapping_add(plen));
    okey = if prefix.is_null() {
        if (*dict).size == MIN_DICT_SIZE as size_t {
            xmlDictComputeFastKey(name, l as ::core::ffi::c_int, (*dict).seed)
        } else {
            xmlDictComputeBigKey(name, l as ::core::ffi::c_int, (*dict).seed)
                as ::core::ffi::c_ulong
        }
    } else if (*dict).size == MIN_DICT_SIZE as size_t {
        xmlDictComputeFastQKey(
            prefix,
            plen as ::core::ffi::c_int,
            name,
            l as ::core::ffi::c_int,
            (*dict).seed,
        )
    } else {
        xmlDictComputeBigQKey(
            prefix,
            plen as ::core::ffi::c_int,
            name,
            l as ::core::ffi::c_int,
            (*dict).seed,
        )
    };
    key = (okey as size_t).wrapping_rem((*dict).size) as ::core::ffi::c_ulong;
    if (*(*dict).dict.offset(key as isize)).valid == 0 as ::core::ffi::c_int {
        insert = ::core::ptr::null_mut::<xmlDictEntry>();
    } else {
        insert = (*dict).dict.offset(key as isize) as *mut _xmlDictEntry as xmlDictEntryPtr;
        while !(*insert).next.is_null() {
            if (*insert).okey == okey
                && (*insert).len == len
                && xmlStrQEqual(prefix, name, (*insert).name) != 0
            {
                return (*insert).name;
            }
            nbi = nbi.wrapping_add(1);
            insert = (*insert).next as xmlDictEntryPtr;
        }
        if (*insert).okey == okey
            && (*insert).len == len
            && xmlStrQEqual(prefix, name, (*insert).name) != 0
        {
            return (*insert).name;
        }
    }
    if !(*dict).subdict.is_null() {
        let mut skey: ::core::ffi::c_ulong = 0;
        if (*dict).size == MIN_DICT_SIZE as size_t
            && (*(*dict).subdict).size != MIN_DICT_SIZE as size_t
            || (*dict).size != MIN_DICT_SIZE as size_t
                && (*(*dict).subdict).size == MIN_DICT_SIZE as size_t
        {
            skey = if prefix.is_null() {
                if (*(*dict).subdict).size == MIN_DICT_SIZE as size_t {
                    xmlDictComputeFastKey(name, l as ::core::ffi::c_int, (*(*dict).subdict).seed)
                } else {
                    xmlDictComputeBigKey(name, l as ::core::ffi::c_int, (*(*dict).subdict).seed)
                        as ::core::ffi::c_ulong
                }
            } else if (*(*dict).subdict).size == MIN_DICT_SIZE as size_t {
                xmlDictComputeFastQKey(
                    prefix,
                    plen as ::core::ffi::c_int,
                    name,
                    l as ::core::ffi::c_int,
                    (*(*dict).subdict).seed,
                )
            } else {
                xmlDictComputeBigQKey(
                    prefix,
                    plen as ::core::ffi::c_int,
                    name,
                    l as ::core::ffi::c_int,
                    (*(*dict).subdict).seed,
                )
            };
        } else {
            skey = okey;
        }
        key = (skey as size_t).wrapping_rem((*(*dict).subdict).size) as ::core::ffi::c_ulong;
        if (*(*(*dict).subdict).dict.offset(key as isize)).valid != 0 as ::core::ffi::c_int {
            let mut tmp: xmlDictEntryPtr = ::core::ptr::null_mut::<xmlDictEntry>();
            tmp = (*(*dict).subdict).dict.offset(key as isize) as *mut _xmlDictEntry
                as xmlDictEntryPtr;
            while !(*tmp).next.is_null() {
                if (*tmp).okey == skey
                    && (*tmp).len == len
                    && xmlStrQEqual(prefix, name, (*tmp).name) != 0
                {
                    return (*tmp).name;
                }
                nbi = nbi.wrapping_add(1);
                tmp = (*tmp).next as xmlDictEntryPtr;
            }
            if (*tmp).okey == skey
                && (*tmp).len == len
                && xmlStrQEqual(prefix, name, (*tmp).name) != 0
            {
                return (*tmp).name;
            }
        }
        key = (okey as size_t).wrapping_rem((*dict).size) as ::core::ffi::c_ulong;
    }
    ret = xmlDictAddQString(dict, prefix, plen, name, l);
    if ret.is_null() {
        return ::core::ptr::null::<xmlChar>();
    }
    if insert.is_null() {
        entry = (*dict).dict.offset(key as isize) as *mut _xmlDictEntry as xmlDictEntryPtr;
    } else {
        entry = xmlMalloc.expect("non-null function pointer")(
            ::core::mem::size_of::<xmlDictEntry>() as size_t,
        ) as xmlDictEntryPtr;
        if entry.is_null() {
            return ::core::ptr::null::<xmlChar>();
        }
    }
    (*entry).name = ret;
    (*entry).len = len;
    (*entry).next = ::core::ptr::null_mut::<_xmlDictEntry>();
    (*entry).valid = 1 as ::core::ffi::c_int;
    (*entry).okey = okey;
    if !insert.is_null() {
        (*insert).next = entry as *mut _xmlDictEntry;
    }
    (*dict).nbElems = (*dict).nbElems.wrapping_add(1);
    if nbi > MAX_HASH_LEN as ::core::ffi::c_ulong
        && (*dict).size <= (MAX_DICT_HASH / 2 as ::core::ffi::c_int / MAX_HASH_LEN) as size_t
    {
        xmlDictGrow(
            dict,
            ((MAX_HASH_LEN * 2 as ::core::ffi::c_int) as size_t).wrapping_mul((*dict).size),
        );
    }
    return ret;
}}
#[no_mangle]
pub unsafe extern "C" fn xmlDictOwns(
    mut dict: xmlDictPtr,
    mut str: *const xmlChar,
) -> ::core::ffi::c_int { unsafe {
    let mut pool: xmlDictStringsPtr = ::core::ptr::null_mut::<xmlDictStrings>();
    if dict.is_null() || str.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    pool = (*dict).strings;
    while !pool.is_null() {
        if str
            >= (&raw mut (*pool).array as *mut xmlChar).offset(0 as ::core::ffi::c_int as isize)
                as *mut xmlChar as *const xmlChar
            && str <= (*pool).free as *const xmlChar
        {
            return 1 as ::core::ffi::c_int;
        }
        pool = (*pool).next;
    }
    if !(*dict).subdict.is_null() {
        return xmlDictOwns((*dict).subdict as xmlDictPtr, str);
    }
    return 0 as ::core::ffi::c_int;
}}
#[no_mangle]
pub unsafe extern "C" fn xmlDictSize(mut dict: xmlDictPtr) -> ::core::ffi::c_int { unsafe {
    if dict.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if !(*dict).subdict.is_null() {
        return (*dict).nbElems.wrapping_add((*(*dict).subdict).nbElems) as ::core::ffi::c_int;
    }
    return (*dict).nbElems as ::core::ffi::c_int;
}}
#[no_mangle]
pub unsafe extern "C" fn xmlDictSetLimit(mut dict: xmlDictPtr, mut limit: size_t) -> size_t { unsafe {
    let mut ret: size_t = 0;
    if dict.is_null() {
        return 0 as size_t;
    }
    ret = (*dict).limit;
    (*dict).limit = limit;
    return ret;
}}
#[no_mangle]
pub unsafe extern "C" fn xmlDictGetUsage(mut dict: xmlDictPtr) -> size_t { unsafe {
    let mut pool: xmlDictStringsPtr = ::core::ptr::null_mut::<xmlDictStrings>();
    let mut limit: size_t = 0 as size_t;
    if dict.is_null() {
        return 0 as size_t;
    }
    pool = (*dict).strings;
    while !pool.is_null() {
        limit = limit.wrapping_add((*pool).size);
        pool = (*pool).next;
    }
    return limit;
}}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
