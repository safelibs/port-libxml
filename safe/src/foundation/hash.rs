#[repr(C)]
pub struct _xmlDict {
    _private: [u8; 0],
}

extern "C" {
    fn xmlStrdup(cur: *const xmlChar) -> *mut xmlChar;
    fn xmlStrEqual(str1: *const xmlChar, str2: *const xmlChar) -> ::core::ffi::c_int;
    fn xmlStrQEqual(
        pref: *const xmlChar,
        name: *const xmlChar,
        str: *const xmlChar,
    ) -> ::core::ffi::c_int;
    fn __xmlRandom() -> ::core::ffi::c_int;
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
    fn xmlDictReference(dict: xmlDictPtr) -> ::core::ffi::c_int;
    fn xmlDictFree(dict: xmlDictPtr);
    fn xmlDictLookup(
        dict: xmlDictPtr,
        name: *const xmlChar,
        len: ::core::ffi::c_int,
    ) -> *const xmlChar;
    fn xmlDictOwns(dict: xmlDictPtr, str: *const xmlChar) -> ::core::ffi::c_int;
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlFree: xmlFreeFunc;
}
pub type xmlChar = ::core::ffi::c_uchar;
pub type size_t = usize;
pub type xmlHashTablePtr = *mut xmlHashTable;
pub type xmlHashTable = _xmlHashTable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlHashTable {
    pub table: *mut _xmlHashEntry,
    pub size: ::core::ffi::c_int,
    pub nbElems: ::core::ffi::c_int,
    pub dict: xmlDictPtr,
    pub random_seed: ::core::ffi::c_int,
}
pub type xmlDictPtr = *mut xmlDict;
pub type xmlDict = _xmlDict;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlHashEntry {
    pub next: *mut _xmlHashEntry,
    pub name: *mut xmlChar,
    pub name2: *mut xmlChar,
    pub name3: *mut xmlChar,
    pub payload: *mut ::core::ffi::c_void,
    pub valid: ::core::ffi::c_int,
}
pub type xmlHashDeallocator =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> ()>;
pub type xmlHashCopier = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const xmlChar) -> *mut ::core::ffi::c_void,
>;
pub type xmlHashScanner = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void, *const xmlChar) -> (),
>;
pub type xmlHashScannerFull = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
        *const xmlChar,
        *const xmlChar,
        *const xmlChar,
    ) -> (),
>;
pub type xmlFreeFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type xmlHashEntry = _xmlHashEntry;
pub type xmlMallocFunc = Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
pub type xmlHashEntryPtr = *mut xmlHashEntry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stubData {
    pub hashscanner: xmlHashScanner,
    pub data: *mut ::core::ffi::c_void,
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAX_HASH_LEN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
unsafe extern "C" fn xmlHashComputeKey(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut name2: *const xmlChar,
    mut name3: *const xmlChar,
) -> ::core::ffi::c_ulong {
    let mut value: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut ch: ::core::ffi::c_ulong = 0;
    value = unsafe { (*table).random_seed as ::core::ffi::c_ulong };
    if !name.is_null() {
        value = value.wrapping_add(
            (30 as ::core::ffi::c_int * unsafe { *name } as ::core::ffi::c_int)
                as ::core::ffi::c_ulong,
        );
        loop {
            ch = unsafe {
                let fresh3 = name;
                name = name.offset(1);
                *fresh3 as ::core::ffi::c_ulong
            };
            if !(ch != 0 as ::core::ffi::c_ulong) {
                break;
            }
            value = value
                ^ (value << 5 as ::core::ffi::c_int)
                    .wrapping_add(value >> 3 as ::core::ffi::c_int)
                    .wrapping_add(ch);
        }
    }
    value =
        value ^ (value << 5 as ::core::ffi::c_int).wrapping_add(value >> 3 as ::core::ffi::c_int);
    if !name2.is_null() {
        loop {
            ch = unsafe {
                let fresh4 = name2;
                name2 = name2.offset(1);
                *fresh4 as ::core::ffi::c_ulong
            };
            if !(ch != 0 as ::core::ffi::c_ulong) {
                break;
            }
            value = value
                ^ (value << 5 as ::core::ffi::c_int)
                    .wrapping_add(value >> 3 as ::core::ffi::c_int)
                    .wrapping_add(ch);
        }
    }
    value =
        value ^ (value << 5 as ::core::ffi::c_int).wrapping_add(value >> 3 as ::core::ffi::c_int);
    if !name3.is_null() {
        loop {
            ch = unsafe {
                let fresh5 = name3;
                name3 = name3.offset(1);
                *fresh5 as ::core::ffi::c_ulong
            };
            if !(ch != 0 as ::core::ffi::c_ulong) {
                break;
            }
            value = value
                ^ (value << 5 as ::core::ffi::c_int)
                    .wrapping_add(value >> 3 as ::core::ffi::c_int)
                    .wrapping_add(ch);
        }
    }
    return value.wrapping_rem(unsafe { (*table).size as ::core::ffi::c_ulong });
}
unsafe extern "C" fn xmlHashComputeQKey(
    mut table: xmlHashTablePtr,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
    mut prefix2: *const xmlChar,
    mut name2: *const xmlChar,
    mut prefix3: *const xmlChar,
    mut name3: *const xmlChar,
) -> ::core::ffi::c_ulong { unsafe {
    let mut value: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut ch: ::core::ffi::c_ulong = 0;
    value = (*table).random_seed as ::core::ffi::c_ulong;
    if !prefix.is_null() {
        value = value.wrapping_add(
            (30 as ::core::ffi::c_int * *prefix as ::core::ffi::c_int) as ::core::ffi::c_ulong,
        );
    } else {
        value = value.wrapping_add(
            (30 as ::core::ffi::c_int * *name as ::core::ffi::c_int) as ::core::ffi::c_ulong,
        );
    }
    if !prefix.is_null() {
        loop {
            let fresh6 = prefix;
            prefix = prefix.offset(1);
            ch = *fresh6 as ::core::ffi::c_ulong;
            if !(ch != 0 as ::core::ffi::c_ulong) {
                break;
            }
            value = value
                ^ (value << 5 as ::core::ffi::c_int)
                    .wrapping_add(value >> 3 as ::core::ffi::c_int)
                    .wrapping_add(ch);
        }
        value = value
            ^ (value << 5 as ::core::ffi::c_int)
                .wrapping_add(value >> 3 as ::core::ffi::c_int)
                .wrapping_add(':' as i32 as ::core::ffi::c_ulong);
    }
    if !name.is_null() {
        loop {
            let fresh7 = name;
            name = name.offset(1);
            ch = *fresh7 as ::core::ffi::c_ulong;
            if !(ch != 0 as ::core::ffi::c_ulong) {
                break;
            }
            value = value
                ^ (value << 5 as ::core::ffi::c_int)
                    .wrapping_add(value >> 3 as ::core::ffi::c_int)
                    .wrapping_add(ch);
        }
    }
    value =
        value ^ (value << 5 as ::core::ffi::c_int).wrapping_add(value >> 3 as ::core::ffi::c_int);
    if !prefix2.is_null() {
        loop {
            let fresh8 = prefix2;
            prefix2 = prefix2.offset(1);
            ch = *fresh8 as ::core::ffi::c_ulong;
            if !(ch != 0 as ::core::ffi::c_ulong) {
                break;
            }
            value = value
                ^ (value << 5 as ::core::ffi::c_int)
                    .wrapping_add(value >> 3 as ::core::ffi::c_int)
                    .wrapping_add(ch);
        }
        value = value
            ^ (value << 5 as ::core::ffi::c_int)
                .wrapping_add(value >> 3 as ::core::ffi::c_int)
                .wrapping_add(':' as i32 as ::core::ffi::c_ulong);
    }
    if !name2.is_null() {
        loop {
            let fresh9 = name2;
            name2 = name2.offset(1);
            ch = *fresh9 as ::core::ffi::c_ulong;
            if !(ch != 0 as ::core::ffi::c_ulong) {
                break;
            }
            value = value
                ^ (value << 5 as ::core::ffi::c_int)
                    .wrapping_add(value >> 3 as ::core::ffi::c_int)
                    .wrapping_add(ch);
        }
    }
    value =
        value ^ (value << 5 as ::core::ffi::c_int).wrapping_add(value >> 3 as ::core::ffi::c_int);
    if !prefix3.is_null() {
        loop {
            let fresh10 = prefix3;
            prefix3 = prefix3.offset(1);
            ch = *fresh10 as ::core::ffi::c_ulong;
            if !(ch != 0 as ::core::ffi::c_ulong) {
                break;
            }
            value = value
                ^ (value << 5 as ::core::ffi::c_int)
                    .wrapping_add(value >> 3 as ::core::ffi::c_int)
                    .wrapping_add(ch);
        }
        value = value
            ^ (value << 5 as ::core::ffi::c_int)
                .wrapping_add(value >> 3 as ::core::ffi::c_int)
                .wrapping_add(':' as i32 as ::core::ffi::c_ulong);
    }
    if !name3.is_null() {
        loop {
            let fresh11 = name3;
            name3 = name3.offset(1);
            ch = *fresh11 as ::core::ffi::c_ulong;
            if !(ch != 0 as ::core::ffi::c_ulong) {
                break;
            }
            value = value
                ^ (value << 5 as ::core::ffi::c_int)
                    .wrapping_add(value >> 3 as ::core::ffi::c_int)
                    .wrapping_add(ch);
        }
    }
    return value.wrapping_rem((*table).size as ::core::ffi::c_ulong);
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashCreate(mut size: ::core::ffi::c_int) -> xmlHashTablePtr { unsafe {
    let mut table: xmlHashTablePtr = ::core::ptr::null_mut::<xmlHashTable>();
    if size <= 0 as ::core::ffi::c_int {
        size = 256 as ::core::ffi::c_int;
    }
    table = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlHashTable>() as size_t
    ) as xmlHashTablePtr;
    if !table.is_null() {
        (*table).dict = ::core::ptr::null_mut::<xmlDict>();
        (*table).size = size;
        (*table).nbElems = 0 as ::core::ffi::c_int;
        (*table).table = xmlMalloc.expect("non-null function pointer")(
            (size as size_t).wrapping_mul(::core::mem::size_of::<xmlHashEntry>() as size_t),
        ) as *mut _xmlHashEntry;
        if !(*table).table.is_null() {
            memset(
                (*table).table as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (size as size_t).wrapping_mul(::core::mem::size_of::<xmlHashEntry>() as size_t),
            );
            (*table).random_seed = __xmlRandom();
            return table;
        }
        xmlFree.expect("non-null function pointer")(table as *mut ::core::ffi::c_void);
    }
    return ::core::ptr::null_mut::<xmlHashTable>();
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashCreateDict(
    mut size: ::core::ffi::c_int,
    mut dict: xmlDictPtr,
) -> xmlHashTablePtr { unsafe {
    let mut table: xmlHashTablePtr = ::core::ptr::null_mut::<xmlHashTable>();
    table = xmlHashCreate(size);
    if !table.is_null() {
        (*table).dict = dict;
        xmlDictReference(dict);
    }
    return table;
}}
unsafe extern "C" fn xmlHashGrow(
    mut table: xmlHashTablePtr,
    mut size: ::core::ffi::c_int,
) -> ::core::ffi::c_int { unsafe {
    let mut key: ::core::ffi::c_ulong = 0;
    let mut oldsize: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut iter: xmlHashEntryPtr = ::core::ptr::null_mut::<xmlHashEntry>();
    let mut next: xmlHashEntryPtr = ::core::ptr::null_mut::<xmlHashEntry>();
    let mut oldtable: *mut _xmlHashEntry = ::core::ptr::null_mut::<_xmlHashEntry>();
    if table.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if size < 8 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if size > 8 as ::core::ffi::c_int * 2048 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    oldsize = (*table).size;
    oldtable = (*table).table;
    if oldtable.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*table).table = xmlMalloc.expect("non-null function pointer")(
        (size as size_t).wrapping_mul(::core::mem::size_of::<xmlHashEntry>() as size_t),
    ) as *mut _xmlHashEntry;
    if (*table).table.is_null() {
        (*table).table = oldtable;
        return -(1 as ::core::ffi::c_int);
    }
    memset(
        (*table).table as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (size as size_t).wrapping_mul(::core::mem::size_of::<xmlHashEntry>() as size_t),
    );
    (*table).size = size;
    i = 0 as ::core::ffi::c_int;
    while i < oldsize {
        if !((*oldtable.offset(i as isize)).valid == 0 as ::core::ffi::c_int) {
            key = xmlHashComputeKey(
                table,
                (*oldtable.offset(i as isize)).name,
                (*oldtable.offset(i as isize)).name2,
                (*oldtable.offset(i as isize)).name3,
            );
            memcpy(
                (*table).table.offset(key as isize) as *mut _xmlHashEntry
                    as *mut ::core::ffi::c_void,
                oldtable.offset(i as isize) as *mut _xmlHashEntry as *const ::core::ffi::c_void,
                ::core::mem::size_of::<xmlHashEntry>() as size_t,
            );
            let ref mut fresh0 = (*(*table).table.offset(key as isize)).next;
            *fresh0 = ::core::ptr::null_mut::<_xmlHashEntry>();
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < oldsize {
        iter = (*oldtable.offset(i as isize)).next as xmlHashEntryPtr;
        while !iter.is_null() {
            next = (*iter).next as xmlHashEntryPtr;
            key = xmlHashComputeKey(table, (*iter).name, (*iter).name2, (*iter).name3);
            if (*(*table).table.offset(key as isize)).valid == 0 as ::core::ffi::c_int {
                memcpy(
                    (*table).table.offset(key as isize) as *mut _xmlHashEntry
                        as *mut ::core::ffi::c_void,
                    iter as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<xmlHashEntry>() as size_t,
                );
                let ref mut fresh1 = (*(*table).table.offset(key as isize)).next;
                *fresh1 = ::core::ptr::null_mut::<_xmlHashEntry>();
                xmlFree.expect("non-null function pointer")(iter as *mut ::core::ffi::c_void);
            } else {
                (*iter).next = (*(*table).table.offset(key as isize)).next;
                let ref mut fresh2 = (*(*table).table.offset(key as isize)).next;
                *fresh2 = iter as *mut _xmlHashEntry;
            }
            iter = next;
        }
        i += 1;
    }
    xmlFree.expect("non-null function pointer")(oldtable as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashFree(mut table: xmlHashTablePtr, mut f: xmlHashDeallocator) { unsafe {
    let mut i: ::core::ffi::c_int = 0;
    let mut iter: xmlHashEntryPtr = ::core::ptr::null_mut::<xmlHashEntry>();
    let mut next: xmlHashEntryPtr = ::core::ptr::null_mut::<xmlHashEntry>();
    let mut inside_table: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nbElems: ::core::ffi::c_int = 0;
    if table.is_null() {
        return;
    }
    if !(*table).table.is_null() {
        nbElems = (*table).nbElems;
        i = 0 as ::core::ffi::c_int;
        while i < (*table).size && nbElems > 0 as ::core::ffi::c_int {
            iter = (*table).table.offset(i as isize) as *mut _xmlHashEntry as xmlHashEntryPtr;
            if !((*iter).valid == 0 as ::core::ffi::c_int) {
                inside_table = 1 as ::core::ffi::c_int;
                while !iter.is_null() {
                    next = (*iter).next as xmlHashEntryPtr;
                    if f.is_some() && !(*iter).payload.is_null() {
                        f.expect("non-null function pointer")((*iter).payload, (*iter).name);
                    }
                    if (*table).dict.is_null() {
                        if !(*iter).name.is_null() {
                            xmlFree.expect("non-null function pointer")(
                                (*iter).name as *mut ::core::ffi::c_void,
                            );
                        }
                        if !(*iter).name2.is_null() {
                            xmlFree.expect("non-null function pointer")(
                                (*iter).name2 as *mut ::core::ffi::c_void,
                            );
                        }
                        if !(*iter).name3.is_null() {
                            xmlFree.expect("non-null function pointer")(
                                (*iter).name3 as *mut ::core::ffi::c_void,
                            );
                        }
                    }
                    (*iter).payload = NULL;
                    if inside_table == 0 {
                        xmlFree.expect("non-null function pointer")(
                            iter as *mut ::core::ffi::c_void,
                        );
                    }
                    nbElems -= 1;
                    inside_table = 0 as ::core::ffi::c_int;
                    iter = next;
                }
            }
            i += 1;
        }
        xmlFree.expect("non-null function pointer")((*table).table as *mut ::core::ffi::c_void);
    }
    if !(*table).dict.is_null() {
        xmlDictFree((*table).dict);
    }
    xmlFree.expect("non-null function pointer")(table as *mut ::core::ffi::c_void);
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashDefaultDeallocator(
    mut entry: *mut ::core::ffi::c_void,
    mut _name: *const xmlChar,
) { unsafe {
    xmlFree.expect("non-null function pointer")(entry);
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashAddEntry(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut userdata: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int { unsafe {
    return xmlHashAddEntry3(
        table,
        name,
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        userdata,
    );
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashAddEntry2(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut name2: *const xmlChar,
    mut userdata: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int { unsafe {
    return xmlHashAddEntry3(table, name, name2, ::core::ptr::null::<xmlChar>(), userdata);
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashUpdateEntry(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut userdata: *mut ::core::ffi::c_void,
    mut f: xmlHashDeallocator,
) -> ::core::ffi::c_int { unsafe {
    return xmlHashUpdateEntry3(
        table,
        name,
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        userdata,
        f,
    );
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashUpdateEntry2(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut name2: *const xmlChar,
    mut userdata: *mut ::core::ffi::c_void,
    mut f: xmlHashDeallocator,
) -> ::core::ffi::c_int { unsafe {
    return xmlHashUpdateEntry3(
        table,
        name,
        name2,
        ::core::ptr::null::<xmlChar>(),
        userdata,
        f,
    );
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashLookup(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
) -> *mut ::core::ffi::c_void { unsafe {
    return xmlHashLookup3(
        table,
        name,
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
    );
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashLookup2(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut name2: *const xmlChar,
) -> *mut ::core::ffi::c_void { unsafe {
    return xmlHashLookup3(table, name, name2, ::core::ptr::null::<xmlChar>());
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashQLookup(
    mut table: xmlHashTablePtr,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
) -> *mut ::core::ffi::c_void { unsafe {
    return xmlHashQLookup3(
        table,
        prefix,
        name,
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
    );
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashQLookup2(
    mut table: xmlHashTablePtr,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
    mut prefix2: *const xmlChar,
    mut name2: *const xmlChar,
) -> *mut ::core::ffi::c_void { unsafe {
    return xmlHashQLookup3(
        table,
        prefix,
        name,
        prefix2,
        name2,
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
    );
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashAddEntry3(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut name2: *const xmlChar,
    mut name3: *const xmlChar,
    mut userdata: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut key: ::core::ffi::c_ulong = 0;
    let mut len: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut entry: xmlHashEntryPtr = ::core::ptr::null_mut::<xmlHashEntry>();
    let mut insert: xmlHashEntryPtr = ::core::ptr::null_mut::<xmlHashEntry>();
    if table.is_null() || name.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let dict = unsafe { (*table).dict };
    if !dict.is_null() {
        if unsafe { xmlDictOwns(dict, name) } == 0 {
            name = unsafe { xmlDictLookup(dict, name, -(1 as ::core::ffi::c_int)) };
            if name.is_null() {
                return -(1 as ::core::ffi::c_int);
            }
        }
        if !name2.is_null() && unsafe { xmlDictOwns(dict, name2) } == 0 {
            name2 = unsafe { xmlDictLookup(dict, name2, -(1 as ::core::ffi::c_int)) };
            if name2.is_null() {
                return -(1 as ::core::ffi::c_int);
            }
        }
        if !name3.is_null() && unsafe { xmlDictOwns(dict, name3) } == 0 {
            name3 = unsafe { xmlDictLookup(dict, name3, -(1 as ::core::ffi::c_int)) };
            if name3.is_null() {
                return -(1 as ::core::ffi::c_int);
            }
        }
    }
    key = unsafe { xmlHashComputeKey(table, name, name2, name3) };
    let bucket = unsafe { (*table).table.offset(key as isize) as xmlHashEntryPtr };
    if unsafe { (*bucket).valid } == 0 as ::core::ffi::c_int {
        insert = ::core::ptr::null_mut::<xmlHashEntry>();
    } else if !dict.is_null() {
        insert = bucket;
        while unsafe { !(*insert).next.is_null() } {
            if unsafe {
                (*insert).name == name as *mut xmlChar
                    && (*insert).name2 == name2 as *mut xmlChar
                    && (*insert).name3 == name3 as *mut xmlChar
            }
            {
                return -(1 as ::core::ffi::c_int);
            }
            len = len.wrapping_add(1);
            insert = unsafe { (*insert).next as xmlHashEntryPtr };
        }
        if unsafe {
            (*insert).name == name as *mut xmlChar
                && (*insert).name2 == name2 as *mut xmlChar
                && (*insert).name3 == name3 as *mut xmlChar
        }
        {
            return -(1 as ::core::ffi::c_int);
        }
    } else {
        insert = bucket;
        while unsafe { !(*insert).next.is_null() } {
            if unsafe { xmlStrEqual((*insert).name, name) } != 0
                && unsafe { xmlStrEqual((*insert).name2, name2) } != 0
                && unsafe { xmlStrEqual((*insert).name3, name3) } != 0
            {
                return -(1 as ::core::ffi::c_int);
            }
            len = len.wrapping_add(1);
            insert = unsafe { (*insert).next as xmlHashEntryPtr };
        }
        if unsafe { xmlStrEqual((*insert).name, name) } != 0
            && unsafe { xmlStrEqual((*insert).name2, name2) } != 0
            && unsafe { xmlStrEqual((*insert).name3, name3) } != 0
        {
            return -(1 as ::core::ffi::c_int);
        }
    }
    if insert.is_null() {
        entry = bucket;
    } else {
        entry = unsafe {
            xmlMalloc.expect("non-null function pointer")(
                ::core::mem::size_of::<xmlHashEntry>() as size_t,
            ) as xmlHashEntryPtr
        };
        if entry.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
    }
    if !dict.is_null() {
        unsafe {
            (*entry).name = name as *mut xmlChar;
            (*entry).name2 = name2 as *mut xmlChar;
            (*entry).name3 = name3 as *mut xmlChar;
        }
    } else {
        unsafe {
            (*entry).name = xmlStrdup(name);
            (*entry).name2 = xmlStrdup(name2);
            (*entry).name3 = xmlStrdup(name3);
        }
    }
    unsafe {
        (*entry).payload = userdata;
        (*entry).next = ::core::ptr::null_mut::<_xmlHashEntry>();
        (*entry).valid = 1 as ::core::ffi::c_int;
    }
    if !insert.is_null() {
        unsafe {
            (*insert).next = entry as *mut _xmlHashEntry;
        }
    }
    unsafe {
        (*table).nbElems += 1;
    }
    if len > MAX_HASH_LEN as ::core::ffi::c_ulong {
        unsafe {
            xmlHashGrow(table, MAX_HASH_LEN * (*table).size);
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashUpdateEntry3(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut name2: *const xmlChar,
    mut name3: *const xmlChar,
    mut userdata: *mut ::core::ffi::c_void,
    mut f: xmlHashDeallocator,
) -> ::core::ffi::c_int {
    let mut insert: xmlHashEntryPtr = ::core::ptr::null_mut::<xmlHashEntry>();
    if table.is_null() || name.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let dict = unsafe { (*table).dict };
    if !dict.is_null() {
        if unsafe { xmlDictOwns(dict, name) } == 0 {
            name = unsafe { xmlDictLookup(dict, name, -(1 as ::core::ffi::c_int)) };
            if name.is_null() {
                return -(1 as ::core::ffi::c_int);
            }
        }
        if !name2.is_null() && unsafe { xmlDictOwns(dict, name2) } == 0 {
            name2 = unsafe { xmlDictLookup(dict, name2, -(1 as ::core::ffi::c_int)) };
            if name2.is_null() {
                return -(1 as ::core::ffi::c_int);
            }
        }
        if !name3.is_null() && unsafe { xmlDictOwns(dict, name3) } == 0 {
            name3 = unsafe { xmlDictLookup(dict, name3, -(1 as ::core::ffi::c_int)) };
            if name3.is_null() {
                return -(1 as ::core::ffi::c_int);
            }
        }
    }
    let key = unsafe { xmlHashComputeKey(table, name, name2, name3) };
    let bucket = unsafe { (*table).table.offset(key as isize) as xmlHashEntryPtr };
    if unsafe { (*bucket).valid } != 0 {
        insert = bucket;
        if !dict.is_null() {
            while unsafe { !(*insert).next.is_null() } {
                if unsafe {
                    (*insert).name == name as *mut xmlChar
                        && (*insert).name2 == name2 as *mut xmlChar
                        && (*insert).name3 == name3 as *mut xmlChar
                }
                {
                    if let Some(dealloc) = f {
                        unsafe { dealloc((*insert).payload, (*insert).name) };
                    }
                    unsafe {
                        (*insert).payload = userdata;
                    }
                    return 0 as ::core::ffi::c_int;
                }
                insert = unsafe { (*insert).next as xmlHashEntryPtr };
            }
            if unsafe {
                (*insert).name == name as *mut xmlChar
                    && (*insert).name2 == name2 as *mut xmlChar
                    && (*insert).name3 == name3 as *mut xmlChar
            } {
                if let Some(dealloc) = f {
                    unsafe { dealloc((*insert).payload, (*insert).name) };
                }
                unsafe {
                    (*insert).payload = userdata;
                }
                return 0 as ::core::ffi::c_int;
            }
        } else {
            while unsafe { !(*insert).next.is_null() } {
                if unsafe {
                    xmlStrEqual((*insert).name, name) != 0
                        && xmlStrEqual((*insert).name2, name2) != 0
                        && xmlStrEqual((*insert).name3, name3) != 0
                }
            {
                    if let Some(dealloc) = f {
                        unsafe { dealloc((*insert).payload, (*insert).name) };
                    }
                    unsafe {
                        (*insert).payload = userdata;
                    }
                    return 0 as ::core::ffi::c_int;
                }
                insert = unsafe { (*insert).next as xmlHashEntryPtr };
            }
            if unsafe {
                xmlStrEqual((*insert).name, name) != 0
                    && xmlStrEqual((*insert).name2, name2) != 0
                    && xmlStrEqual((*insert).name3, name3) != 0
            } {
                if let Some(dealloc) = f {
                    unsafe { dealloc((*insert).payload, (*insert).name) };
                }
                unsafe {
                    (*insert).payload = userdata;
                }
                return 0 as ::core::ffi::c_int;
            }
        }
    }
    let entry = if insert.is_null() {
        bucket
    } else {
        let entry = unsafe {
            xmlMalloc.expect("non-null function pointer")(
                ::core::mem::size_of::<xmlHashEntry>() as size_t,
            ) as xmlHashEntryPtr
        };
        if entry.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        entry
    };
    if !dict.is_null() {
        unsafe {
            (*entry).name = name as *mut xmlChar;
            (*entry).name2 = name2 as *mut xmlChar;
            (*entry).name3 = name3 as *mut xmlChar;
        }
    } else {
        unsafe {
            (*entry).name = xmlStrdup(name);
            (*entry).name2 = xmlStrdup(name2);
            (*entry).name3 = xmlStrdup(name3);
        }
    }
    unsafe {
        (*entry).payload = userdata;
        (*entry).next = ::core::ptr::null_mut::<_xmlHashEntry>();
        (*entry).valid = 1 as ::core::ffi::c_int;
        (*table).nbElems += 1;
    }
    if !insert.is_null() {
        unsafe {
            (*insert).next = entry as *mut _xmlHashEntry;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashLookup3(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut name2: *const xmlChar,
    mut name3: *const xmlChar,
) -> *mut ::core::ffi::c_void {
    if table.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if name.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    let key = unsafe { xmlHashComputeKey(table, name, name2, name3) };
    let dict = unsafe { (*table).dict };
    let mut entry = unsafe { (*table).table.offset(key as isize) as xmlHashEntryPtr };
    if unsafe { (*entry).valid } == 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if !dict.is_null() {
        while !entry.is_null() {
            if unsafe {
                (*entry).name == name as *mut xmlChar
                    && (*entry).name2 == name2 as *mut xmlChar
                    && (*entry).name3 == name3 as *mut xmlChar
            }
            {
                return unsafe { (*entry).payload };
            }
            entry = unsafe { (*entry).next as xmlHashEntryPtr };
        }
    }
    entry = unsafe { (*table).table.offset(key as isize) as xmlHashEntryPtr };
    while !entry.is_null() {
        if unsafe {
            xmlStrEqual((*entry).name, name) != 0
                && xmlStrEqual((*entry).name2, name2) != 0
                && xmlStrEqual((*entry).name3, name3) != 0
        }
        {
            return unsafe { (*entry).payload };
        }
        entry = unsafe { (*entry).next as xmlHashEntryPtr };
    }
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashQLookup3(
    mut table: xmlHashTablePtr,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
    mut prefix2: *const xmlChar,
    mut name2: *const xmlChar,
    mut prefix3: *const xmlChar,
    mut name3: *const xmlChar,
) -> *mut ::core::ffi::c_void { unsafe {
    let mut key: ::core::ffi::c_ulong = 0;
    let mut entry: xmlHashEntryPtr = ::core::ptr::null_mut::<xmlHashEntry>();
    if table.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if name.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    key = xmlHashComputeQKey(table, prefix, name, prefix2, name2, prefix3, name3);
    if (*(*table).table.offset(key as isize)).valid == 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    entry = (*table).table.offset(key as isize) as *mut _xmlHashEntry as xmlHashEntryPtr;
    while !entry.is_null() {
        if xmlStrQEqual(prefix, name, (*entry).name) != 0
            && xmlStrQEqual(prefix2, name2, (*entry).name2) != 0
            && xmlStrQEqual(prefix3, name3, (*entry).name3) != 0
        {
            return (*entry).payload;
        }
        entry = (*entry).next as xmlHashEntryPtr;
    }
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}}
unsafe extern "C" fn stubHashScannerFull(
    mut payload: *mut ::core::ffi::c_void,
    mut data: *mut ::core::ffi::c_void,
    mut name: *const xmlChar,
    mut _name2: *const xmlChar,
    mut _name3: *const xmlChar,
) { unsafe {
    let mut stubdata: *mut stubData = data as *mut stubData;
    if let Some(scanner) = (*stubdata).hashscanner {
        scanner(payload, (*stubdata).data, name as *mut xmlChar);
    }
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashScan(
    mut table: xmlHashTablePtr,
    mut f: xmlHashScanner,
    mut data: *mut ::core::ffi::c_void,
) { unsafe {
    let mut stubdata: stubData = stubData {
        hashscanner: None,
        data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    if f.is_none() {
        return;
    }
    stubdata.data = data;
    stubdata.hashscanner = f;
    xmlHashScanFull(
        table,
        Some(
            stubHashScannerFull
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        &raw mut stubdata as *mut ::core::ffi::c_void,
    );
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashScanFull(
    mut table: xmlHashTablePtr,
    mut f: xmlHashScannerFull,
    mut data: *mut ::core::ffi::c_void,
) { unsafe {
    let mut i: ::core::ffi::c_int = 0;
    let mut nb: ::core::ffi::c_int = 0;
    let mut iter: xmlHashEntryPtr = ::core::ptr::null_mut::<xmlHashEntry>();
    let mut next: xmlHashEntryPtr = ::core::ptr::null_mut::<xmlHashEntry>();
    if table.is_null() {
        return;
    }
    if f.is_none() {
        return;
    }
    if !(*table).table.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < (*table).size {
            if !((*(*table).table.offset(i as isize)).valid == 0 as ::core::ffi::c_int) {
                iter = (*table).table.offset(i as isize) as *mut _xmlHashEntry as xmlHashEntryPtr;
                while !iter.is_null() {
                    next = (*iter).next as xmlHashEntryPtr;
                    nb = (*table).nbElems;
                    if f.is_some() && !(*iter).payload.is_null() {
                        f.expect("non-null function pointer")(
                            (*iter).payload,
                            data,
                            (*iter).name,
                            (*iter).name2,
                            (*iter).name3,
                        );
                    }
                    if nb != (*table).nbElems {
                        if iter == (*table).table.offset(i as isize) as *mut _xmlHashEntry {
                            if (*(*table).table.offset(i as isize)).valid == 0 as ::core::ffi::c_int
                            {
                                iter = ::core::ptr::null_mut::<xmlHashEntry>();
                            }
                            if (*(*table).table.offset(i as isize)).next != next {
                                iter = (*table).table.offset(i as isize) as *mut _xmlHashEntry
                                    as xmlHashEntryPtr;
                            }
                        } else {
                            iter = next;
                        }
                    } else {
                        iter = next;
                    }
                }
            }
            i += 1;
        }
    }
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashScan3(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut name2: *const xmlChar,
    mut name3: *const xmlChar,
    mut f: xmlHashScanner,
    mut data: *mut ::core::ffi::c_void,
) { unsafe {
    let mut stubdata: stubData = stubData {
        hashscanner: None,
        data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    if f.is_none() {
        return;
    }
    stubdata.data = data;
    stubdata.hashscanner = f;
    xmlHashScanFull3(
        table,
        name,
        name2,
        name3,
        Some(
            stubHashScannerFull
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                    *const xmlChar,
                    *const xmlChar,
                    *const xmlChar,
                ) -> (),
        ),
        &raw mut stubdata as *mut ::core::ffi::c_void,
    );
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashScanFull3(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut name2: *const xmlChar,
    mut name3: *const xmlChar,
    mut f: xmlHashScannerFull,
    mut data: *mut ::core::ffi::c_void,
) { unsafe {
    let mut i: ::core::ffi::c_int = 0;
    let mut iter: xmlHashEntryPtr = ::core::ptr::null_mut::<xmlHashEntry>();
    let mut next: xmlHashEntryPtr = ::core::ptr::null_mut::<xmlHashEntry>();
    if table.is_null() {
        return;
    }
    if f.is_none() {
        return;
    }
    if !(*table).table.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < (*table).size {
            if !((*(*table).table.offset(i as isize)).valid == 0 as ::core::ffi::c_int) {
                iter = (*table).table.offset(i as isize) as *mut _xmlHashEntry as xmlHashEntryPtr;
                while !iter.is_null() {
                    next = (*iter).next as xmlHashEntryPtr;
                    if (name.is_null() || xmlStrEqual(name, (*iter).name) != 0)
                        && (name2.is_null() || xmlStrEqual(name2, (*iter).name2) != 0)
                        && (name3.is_null() || xmlStrEqual(name3, (*iter).name3) != 0)
                        && !(*iter).payload.is_null()
                    {
                        f.expect("non-null function pointer")(
                            (*iter).payload,
                            data,
                            (*iter).name,
                            (*iter).name2,
                            (*iter).name3,
                        );
                    }
                    iter = next;
                }
            }
            i += 1;
        }
    }
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashCopy(
    mut table: xmlHashTablePtr,
    mut f: xmlHashCopier,
) -> xmlHashTablePtr { unsafe {
    let mut i: ::core::ffi::c_int = 0;
    let mut iter: xmlHashEntryPtr = ::core::ptr::null_mut::<xmlHashEntry>();
    let mut next: xmlHashEntryPtr = ::core::ptr::null_mut::<xmlHashEntry>();
    let mut ret: xmlHashTablePtr = ::core::ptr::null_mut::<xmlHashTable>();
    if table.is_null() {
        return ::core::ptr::null_mut::<xmlHashTable>();
    }
    if f.is_none() {
        return ::core::ptr::null_mut::<xmlHashTable>();
    }
    ret = xmlHashCreate((*table).size);
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlHashTable>();
    }
    if !(*table).table.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < (*table).size {
            if !((*(*table).table.offset(i as isize)).valid == 0 as ::core::ffi::c_int) {
                iter = (*table).table.offset(i as isize) as *mut _xmlHashEntry as xmlHashEntryPtr;
                while !iter.is_null() {
                    next = (*iter).next as xmlHashEntryPtr;
                    xmlHashAddEntry3(
                        ret,
                        (*iter).name,
                        (*iter).name2,
                        (*iter).name3,
                        f.expect("non-null function pointer")((*iter).payload, (*iter).name),
                    );
                    iter = next;
                }
            }
            i += 1;
        }
    }
    (*ret).nbElems = (*table).nbElems;
    return ret;
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashSize(mut table: xmlHashTablePtr) -> ::core::ffi::c_int { unsafe {
    if table.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    return (*table).nbElems;
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashRemoveEntry(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut f: xmlHashDeallocator,
) -> ::core::ffi::c_int { unsafe {
    return xmlHashRemoveEntry3(
        table,
        name,
        ::core::ptr::null::<xmlChar>(),
        ::core::ptr::null::<xmlChar>(),
        f,
    );
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashRemoveEntry2(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut name2: *const xmlChar,
    mut f: xmlHashDeallocator,
) -> ::core::ffi::c_int { unsafe {
    return xmlHashRemoveEntry3(table, name, name2, ::core::ptr::null::<xmlChar>(), f);
}}
#[no_mangle]
pub unsafe extern "C" fn xmlHashRemoveEntry3(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut name2: *const xmlChar,
    mut name3: *const xmlChar,
    mut f: xmlHashDeallocator,
) -> ::core::ffi::c_int { unsafe {
    let mut key: ::core::ffi::c_ulong = 0;
    let mut entry: xmlHashEntryPtr = ::core::ptr::null_mut::<xmlHashEntry>();
    let mut prev: xmlHashEntryPtr = ::core::ptr::null_mut::<xmlHashEntry>();
    if table.is_null() || name.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    key = xmlHashComputeKey(table, name, name2, name3);
    if (*(*table).table.offset(key as isize)).valid == 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    } else {
        entry = (*table).table.offset(key as isize) as *mut _xmlHashEntry as xmlHashEntryPtr;
        while !entry.is_null() {
            if xmlStrEqual((*entry).name, name) != 0
                && xmlStrEqual((*entry).name2, name2) != 0
                && xmlStrEqual((*entry).name3, name3) != 0
            {
                if f.is_some() && !(*entry).payload.is_null() {
                    f.expect("non-null function pointer")((*entry).payload, (*entry).name);
                }
                (*entry).payload = NULL;
                if (*table).dict.is_null() {
                    if !(*entry).name.is_null() {
                        xmlFree.expect("non-null function pointer")(
                            (*entry).name as *mut ::core::ffi::c_void,
                        );
                    }
                    if !(*entry).name2.is_null() {
                        xmlFree.expect("non-null function pointer")(
                            (*entry).name2 as *mut ::core::ffi::c_void,
                        );
                    }
                    if !(*entry).name3.is_null() {
                        xmlFree.expect("non-null function pointer")(
                            (*entry).name3 as *mut ::core::ffi::c_void,
                        );
                    }
                }
                if !prev.is_null() {
                    (*prev).next = (*entry).next;
                    xmlFree.expect("non-null function pointer")(entry as *mut ::core::ffi::c_void);
                } else if (*entry).next.is_null() {
                    (*entry).valid = 0 as ::core::ffi::c_int;
                } else {
                    entry = (*entry).next as xmlHashEntryPtr;
                    memcpy(
                        (*table).table.offset(key as isize) as *mut _xmlHashEntry
                            as *mut ::core::ffi::c_void,
                        entry as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<xmlHashEntry>() as size_t,
                    );
                    xmlFree.expect("non-null function pointer")(entry as *mut ::core::ffi::c_void);
                }
                (*table).nbElems -= 1;
                return 0 as ::core::ffi::c_int;
            }
            prev = entry;
            entry = (*entry).next as xmlHashEntryPtr;
        }
        return -(1 as ::core::ffi::c_int);
    };
}}
