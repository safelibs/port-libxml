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
#[inline]
fn hash_mix_step(value: ::core::ffi::c_ulong, ch: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    value
        ^ (value << 5 as ::core::ffi::c_int)
            .wrapping_add(value >> 3 as ::core::ffi::c_int)
            .wrapping_add(ch)
}
#[inline]
fn hash_mix_finalize(value: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    value ^ (value << 5 as ::core::ffi::c_int).wrapping_add(value >> 3 as ::core::ffi::c_int)
}
#[inline]
fn hash_mix_cstr(mut value: ::core::ffi::c_ulong, mut ptr: *const xmlChar) -> ::core::ffi::c_ulong {
    while !ptr.is_null() {
        let ch = unsafe { *ptr as ::core::ffi::c_ulong };
        if ch == 0 as ::core::ffi::c_ulong {
            break;
        }
        value = hash_mix_step(value, ch);
        ptr = unsafe { ptr.offset(1) };
    }
    value
}
#[inline]
fn hash_bucket_entry(table: &xmlHashTable, key: ::core::ffi::c_ulong) -> xmlHashEntryPtr {
    unsafe { table.table.offset(key as isize) as xmlHashEntryPtr }
}
#[inline]
fn hash_entry_next(entry: xmlHashEntryPtr) -> xmlHashEntryPtr {
    unsafe { (*entry).next as xmlHashEntryPtr }
}
#[inline]
fn hash_entry_valid(entry: xmlHashEntryPtr) -> bool {
    unsafe { (*entry).valid != 0 as ::core::ffi::c_int }
}
#[inline]
fn hash_entry_payload(entry: xmlHashEntryPtr) -> *mut ::core::ffi::c_void {
    unsafe { (*entry).payload }
}
#[inline]
fn hash_entry_name(entry: xmlHashEntryPtr) -> *const xmlChar {
    unsafe { (*entry).name }
}
#[inline]
fn hash_entry_name2(entry: xmlHashEntryPtr) -> *const xmlChar {
    unsafe { (*entry).name2 }
}
#[inline]
fn hash_entry_name3(entry: xmlHashEntryPtr) -> *const xmlChar {
    unsafe { (*entry).name3 }
}
#[inline]
fn hash_entry_matches_qname(
    entry: xmlHashEntryPtr,
    prefix: *const xmlChar,
    name: *const xmlChar,
    prefix2: *const xmlChar,
    name2: *const xmlChar,
    prefix3: *const xmlChar,
    name3: *const xmlChar,
) -> bool {
    unsafe {
        xmlStrQEqual(prefix, name, (*entry).name) != 0
            && xmlStrQEqual(prefix2, name2, (*entry).name2) != 0
            && xmlStrQEqual(prefix3, name3, (*entry).name3) != 0
    }
}
#[inline]
fn hash_entry_matches_str(
    entry: xmlHashEntryPtr,
    name: *const xmlChar,
    name2: *const xmlChar,
    name3: *const xmlChar,
) -> bool {
    unsafe {
        xmlStrEqual((*entry).name, name) != 0
            && xmlStrEqual((*entry).name2, name2) != 0
            && xmlStrEqual((*entry).name3, name3) != 0
    }
}
unsafe extern "C" fn xmlHashComputeQKey(
    mut table: xmlHashTablePtr,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
    mut prefix2: *const xmlChar,
    mut name2: *const xmlChar,
    mut prefix3: *const xmlChar,
    mut name3: *const xmlChar,
) -> ::core::ffi::c_ulong {
    let table_ref = unsafe { &*table };
    let mut value = table_ref.random_seed as ::core::ffi::c_ulong;
    if !prefix.is_null() {
        value = value.wrapping_add(
            (30 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                .wrapping_mul(unsafe { *prefix as ::core::ffi::c_ulong }),
        );
    } else {
        value = value.wrapping_add(
            (30 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                .wrapping_mul(unsafe { *name as ::core::ffi::c_ulong }),
        );
    }
    if !prefix.is_null() {
        value = hash_mix_cstr(value, prefix);
        value = hash_mix_step(value, ':' as i32 as ::core::ffi::c_ulong);
    }
    if !name.is_null() {
        value = hash_mix_cstr(value, name);
    }
    value = hash_mix_finalize(value);
    if !prefix2.is_null() {
        value = hash_mix_cstr(value, prefix2);
        value = hash_mix_step(value, ':' as i32 as ::core::ffi::c_ulong);
    }
    if !name2.is_null() {
        value = hash_mix_cstr(value, name2);
    }
    value = hash_mix_finalize(value);
    if !prefix3.is_null() {
        value = hash_mix_cstr(value, prefix3);
        value = hash_mix_step(value, ':' as i32 as ::core::ffi::c_ulong);
    }
    if !name3.is_null() {
        value = hash_mix_cstr(value, name3);
    }
    return value.wrapping_rem(table_ref.size as ::core::ffi::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashCreate(mut size: ::core::ffi::c_int) -> xmlHashTablePtr {
    let mut table: xmlHashTablePtr = ::core::ptr::null_mut::<xmlHashTable>();
    if size <= 0 as ::core::ffi::c_int {
        size = 256 as ::core::ffi::c_int;
    }
    table = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            ::core::mem::size_of::<xmlHashTable>() as size_t
        ) as xmlHashTablePtr
    };
    if !table.is_null() {
        let table_ref = unsafe { &mut *table };
        table_ref.dict = ::core::ptr::null_mut::<xmlDict>();
        table_ref.size = size;
        table_ref.nbElems = 0 as ::core::ffi::c_int;
        table_ref.table = unsafe {
            xmlMalloc.expect("non-null function pointer")(
                (size as size_t).wrapping_mul(::core::mem::size_of::<xmlHashEntry>() as size_t),
            ) as *mut _xmlHashEntry
        };
        if !table_ref.table.is_null() {
            unsafe {
                memset(
                    table_ref.table as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (size as size_t).wrapping_mul(::core::mem::size_of::<xmlHashEntry>() as size_t),
                )
            };
            table_ref.random_seed = unsafe { __xmlRandom() };
            return table;
        }
        unsafe { xmlFree.expect("non-null function pointer")(table as *mut ::core::ffi::c_void) };
    }
    return ::core::ptr::null_mut::<xmlHashTable>();
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashCreateDict(
    mut size: ::core::ffi::c_int,
    mut dict: xmlDictPtr,
) -> xmlHashTablePtr {
    let mut table: xmlHashTablePtr = unsafe { xmlHashCreate(size) };
    if !table.is_null() {
        unsafe { (*table).dict = dict };
        unsafe { xmlDictReference(dict) };
    }
    return table;
}
unsafe extern "C" fn xmlHashGrow(
    mut table: xmlHashTablePtr,
    mut size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let Some(table) = (unsafe { table.as_mut() }) else {
        return -(1 as ::core::ffi::c_int);
    };
    if size < 8 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if size > 8 as ::core::ffi::c_int * 2048 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    let oldsize = table.size;
    let oldtable = table.table;
    if oldtable.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let newtable = unsafe {
        xmlMalloc.expect("non-null function pointer")(
            (size as size_t).wrapping_mul(::core::mem::size_of::<xmlHashEntry>() as size_t),
        ) as *mut _xmlHashEntry
    };
    if newtable.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    unsafe {
        memset(
            newtable as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (size as size_t).wrapping_mul(::core::mem::size_of::<xmlHashEntry>() as size_t),
        )
    };
    table.table = newtable;
    table.size = size;
    let old_entries = unsafe { ::core::slice::from_raw_parts_mut(oldtable, oldsize as usize) };
    let new_entries = unsafe { ::core::slice::from_raw_parts_mut(newtable, size as usize) };

    for old_entry in old_entries.iter_mut() {
        if old_entry.valid == 0 as ::core::ffi::c_int {
            continue;
        }
        let key =
            unsafe { xmlHashComputeKey(table, old_entry.name, old_entry.name2, old_entry.name3) }
                as usize;
        new_entries[key] = *old_entry;
        new_entries[key].next = ::core::ptr::null_mut::<_xmlHashEntry>();
    }
    for old_entry in old_entries.iter_mut() {
        let mut iter = old_entry.next as xmlHashEntryPtr;
        while !iter.is_null() {
            let next = unsafe { (*iter).next as xmlHashEntryPtr };
            let key =
                unsafe { xmlHashComputeKey(table, (*iter).name, (*iter).name2, (*iter).name3) }
                    as usize;
            if new_entries[key].valid == 0 as ::core::ffi::c_int {
                new_entries[key] = unsafe { *iter };
                new_entries[key].next = ::core::ptr::null_mut::<_xmlHashEntry>();
                unsafe {
                    xmlFree.expect("non-null function pointer")(iter as *mut ::core::ffi::c_void)
                };
            } else {
                unsafe { (*iter).next = new_entries[key].next };
                new_entries[key].next = iter as *mut _xmlHashEntry;
            }
            iter = next;
        }
    }
    unsafe { xmlFree.expect("non-null function pointer")(oldtable as *mut ::core::ffi::c_void) };
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashFree(mut table: xmlHashTablePtr, mut f: xmlHashDeallocator) {
    let Some(table_ref) = (unsafe { table.as_mut() }) else {
        return;
    };
    let table_entries = table_ref.table;
    if !table_entries.is_null() {
        let mut nb_elems = table_ref.nbElems;
        let mut i = 0 as ::core::ffi::c_int;
        while i < table_ref.size && nb_elems > 0 as ::core::ffi::c_int {
            let mut iter = unsafe { table_entries.offset(i as isize) as xmlHashEntryPtr };
            if hash_entry_valid(iter) {
                let mut inside_table = true;
                while !iter.is_null() {
                    let next = hash_entry_next(iter);
                    let payload = hash_entry_payload(iter);
                    if let Some(dealloc) = f {
                        if !payload.is_null() {
                            unsafe {
                                dealloc(payload, hash_entry_name(iter));
                            }
                        }
                    }
                    if table_ref.dict.is_null() {
                        let name = hash_entry_name(iter);
                        if !name.is_null() {
                            unsafe {
                                xmlFree.expect("non-null function pointer")(
                                    name as *mut ::core::ffi::c_void,
                                );
                            }
                        }
                        let name2 = hash_entry_name2(iter);
                        if !name2.is_null() {
                            unsafe {
                                xmlFree.expect("non-null function pointer")(
                                    name2 as *mut ::core::ffi::c_void,
                                );
                            }
                        }
                        let name3 = hash_entry_name3(iter);
                        if !name3.is_null() {
                            unsafe {
                                xmlFree.expect("non-null function pointer")(
                                    name3 as *mut ::core::ffi::c_void,
                                );
                            }
                        }
                    }
                    unsafe {
                        (*iter).payload = NULL;
                    }
                    if !inside_table {
                        unsafe {
                            xmlFree.expect("non-null function pointer")(
                                iter as *mut ::core::ffi::c_void,
                            );
                        }
                    }
                    nb_elems -= 1;
                    inside_table = false;
                    iter = next;
                }
            }
            i += 1;
        }
        unsafe {
            xmlFree.expect("non-null function pointer")(table_entries as *mut ::core::ffi::c_void);
        }
    }
    if !table_ref.dict.is_null() {
        unsafe {
            xmlDictFree(table_ref.dict);
        }
    }
    unsafe {
        xmlFree.expect("non-null function pointer")(table as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashDefaultDeallocator(
    mut entry: *mut ::core::ffi::c_void,
    mut _name: *const xmlChar,
) {
    let free = unsafe { xmlFree.expect("non-null function pointer") };
    unsafe { free(entry) };
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashAddEntry(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut userdata: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return unsafe {
        xmlHashAddEntry3(
            table,
            name,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
            userdata,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashAddEntry2(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut name2: *const xmlChar,
    mut userdata: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return unsafe {
        xmlHashAddEntry3(table, name, name2, ::core::ptr::null::<xmlChar>(), userdata)
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashUpdateEntry(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut userdata: *mut ::core::ffi::c_void,
    mut f: xmlHashDeallocator,
) -> ::core::ffi::c_int {
    return unsafe {
        xmlHashUpdateEntry3(
            table,
            name,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
            userdata,
            f,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashUpdateEntry2(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut name2: *const xmlChar,
    mut userdata: *mut ::core::ffi::c_void,
    mut f: xmlHashDeallocator,
) -> ::core::ffi::c_int {
    return unsafe {
        xmlHashUpdateEntry3(
            table,
            name,
            name2,
            ::core::ptr::null::<xmlChar>(),
            userdata,
            f,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashLookup(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
) -> *mut ::core::ffi::c_void {
    return unsafe {
        xmlHashLookup3(
            table,
            name,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashLookup2(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut name2: *const xmlChar,
) -> *mut ::core::ffi::c_void {
    return unsafe { xmlHashLookup3(table, name, name2, ::core::ptr::null::<xmlChar>()) };
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashQLookup(
    mut table: xmlHashTablePtr,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
) -> *mut ::core::ffi::c_void {
    return unsafe {
        xmlHashQLookup3(
            table,
            prefix,
            name,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashQLookup2(
    mut table: xmlHashTablePtr,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
    mut prefix2: *const xmlChar,
    mut name2: *const xmlChar,
) -> *mut ::core::ffi::c_void {
    return unsafe {
        xmlHashQLookup3(
            table,
            prefix,
            name,
            prefix2,
            name2,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
        )
    };
}
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
            } {
                return -(1 as ::core::ffi::c_int);
            }
            len = len.wrapping_add(1);
            insert = unsafe { (*insert).next as xmlHashEntryPtr };
        }
        if unsafe {
            (*insert).name == name as *mut xmlChar
                && (*insert).name2 == name2 as *mut xmlChar
                && (*insert).name3 == name3 as *mut xmlChar
        } {
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
                ::core::mem::size_of::<xmlHashEntry>() as size_t
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
                } {
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
                } {
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
                ::core::mem::size_of::<xmlHashEntry>() as size_t
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
            } {
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
        } {
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
) -> *mut ::core::ffi::c_void {
    let Some(table_ref) = (unsafe { table.as_ref() }) else {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    };
    if name.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    let key = unsafe { xmlHashComputeQKey(table, prefix, name, prefix2, name2, prefix3, name3) };
    let mut entry = hash_bucket_entry(table_ref, key);
    if !hash_entry_valid(entry) {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    while !entry.is_null() {
        if hash_entry_matches_qname(entry, prefix, name, prefix2, name2, prefix3, name3) {
            return hash_entry_payload(entry);
        }
        entry = hash_entry_next(entry);
    }
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
unsafe extern "C" fn stubHashScannerFull(
    mut payload: *mut ::core::ffi::c_void,
    mut data: *mut ::core::ffi::c_void,
    mut name: *const xmlChar,
    mut _name2: *const xmlChar,
    mut _name3: *const xmlChar,
) {
    let stubdata = data as *mut stubData;
    let Some(scanner) = (unsafe { (*stubdata).hashscanner }) else {
        return;
    };
    let scanner_data = unsafe { (*stubdata).data };
    unsafe {
        scanner(payload, scanner_data, name as *mut xmlChar);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashScan(
    mut table: xmlHashTablePtr,
    mut f: xmlHashScanner,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut stubdata: stubData = stubData {
        hashscanner: None,
        data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    if f.is_none() {
        return;
    }
    stubdata.data = data;
    stubdata.hashscanner = f;
    unsafe {
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
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashScanFull(
    mut table: xmlHashTablePtr,
    mut f: xmlHashScannerFull,
    mut data: *mut ::core::ffi::c_void,
) {
    let Some(table_ref) = (unsafe { table.as_ref() }) else {
        return;
    };
    let Some(scanner) = f else {
        return;
    };
    let table_entries = table_ref.table;
    if table_entries.is_null() {
        return;
    }
    let mut i = 0 as ::core::ffi::c_int;
    while i < table_ref.size {
        let bucket = unsafe { table_entries.offset(i as isize) as xmlHashEntryPtr };
        if hash_entry_valid(bucket) {
            let mut iter = bucket;
            while !iter.is_null() {
                let next = hash_entry_next(iter);
                let nb = table_ref.nbElems;
                let payload = hash_entry_payload(iter);
                if !payload.is_null() {
                    unsafe {
                        scanner(
                            payload,
                            data,
                            hash_entry_name(iter),
                            hash_entry_name2(iter),
                            hash_entry_name3(iter),
                        );
                    }
                }
                if nb != table_ref.nbElems {
                    if iter == bucket {
                        if !hash_entry_valid(bucket) {
                            iter = ::core::ptr::null_mut::<xmlHashEntry>();
                        } else if hash_entry_next(bucket) != next {
                            iter = bucket;
                        } else {
                            iter = next;
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
#[no_mangle]
pub unsafe extern "C" fn xmlHashScan3(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut name2: *const xmlChar,
    mut name3: *const xmlChar,
    mut f: xmlHashScanner,
    mut data: *mut ::core::ffi::c_void,
) {
    let mut stubdata: stubData = stubData {
        hashscanner: None,
        data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    };
    if f.is_none() {
        return;
    }
    stubdata.data = data;
    stubdata.hashscanner = f;
    unsafe {
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
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashScanFull3(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut name2: *const xmlChar,
    mut name3: *const xmlChar,
    mut f: xmlHashScannerFull,
    mut data: *mut ::core::ffi::c_void,
) {
    let Some(table_ref) = (unsafe { table.as_ref() }) else {
        return;
    };
    let Some(scanner) = f else {
        return;
    };
    let table_entries = table_ref.table;
    if table_entries.is_null() {
        return;
    }
    let mut i = 0 as ::core::ffi::c_int;
    while i < table_ref.size {
        let bucket = unsafe { table_entries.offset(i as isize) as xmlHashEntryPtr };
        if hash_entry_valid(bucket) {
            let mut iter = bucket;
            while !iter.is_null() {
                let next = hash_entry_next(iter);
                let payload = hash_entry_payload(iter);
                if (name.is_null() || unsafe { xmlStrEqual(name, hash_entry_name(iter)) } != 0)
                    && (name2.is_null()
                        || unsafe { xmlStrEqual(name2, hash_entry_name2(iter)) } != 0)
                    && (name3.is_null()
                        || unsafe { xmlStrEqual(name3, hash_entry_name3(iter)) } != 0)
                    && !payload.is_null()
                {
                    unsafe {
                        scanner(
                            payload,
                            data,
                            hash_entry_name(iter),
                            hash_entry_name2(iter),
                            hash_entry_name3(iter),
                        );
                    }
                }
                iter = next;
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashCopy(
    mut table: xmlHashTablePtr,
    mut f: xmlHashCopier,
) -> xmlHashTablePtr {
    let Some(table_ref) = (unsafe { table.as_ref() }) else {
        return ::core::ptr::null_mut::<xmlHashTable>();
    };
    let Some(copier) = f else {
        return ::core::ptr::null_mut::<xmlHashTable>();
    };
    let ret = unsafe { xmlHashCreate(table_ref.size) };
    if ret.is_null() {
        return ::core::ptr::null_mut::<xmlHashTable>();
    }
    let table_entries = table_ref.table;
    if !table_entries.is_null() {
        let mut i = 0 as ::core::ffi::c_int;
        while i < table_ref.size {
            let bucket = unsafe { table_entries.offset(i as isize) as xmlHashEntryPtr };
            if hash_entry_valid(bucket) {
                let mut iter = bucket;
                while !iter.is_null() {
                    let next = hash_entry_next(iter);
                    unsafe {
                        xmlHashAddEntry3(
                            ret,
                            hash_entry_name(iter),
                            hash_entry_name2(iter),
                            hash_entry_name3(iter),
                            copier(hash_entry_payload(iter), hash_entry_name(iter)),
                        );
                    }
                    iter = next;
                }
            }
            i += 1;
        }
    }
    unsafe {
        (*ret).nbElems = table_ref.nbElems;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashSize(mut table: xmlHashTablePtr) -> ::core::ffi::c_int {
    let Some(table) = (unsafe { table.as_ref() }) else {
        return -(1 as ::core::ffi::c_int);
    };
    return table.nbElems;
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashRemoveEntry(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut f: xmlHashDeallocator,
) -> ::core::ffi::c_int {
    return unsafe {
        xmlHashRemoveEntry3(
            table,
            name,
            ::core::ptr::null::<xmlChar>(),
            ::core::ptr::null::<xmlChar>(),
            f,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashRemoveEntry2(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut name2: *const xmlChar,
    mut f: xmlHashDeallocator,
) -> ::core::ffi::c_int {
    return unsafe { xmlHashRemoveEntry3(table, name, name2, ::core::ptr::null::<xmlChar>(), f) };
}
#[no_mangle]
pub unsafe extern "C" fn xmlHashRemoveEntry3(
    mut table: xmlHashTablePtr,
    mut name: *const xmlChar,
    mut name2: *const xmlChar,
    mut name3: *const xmlChar,
    mut f: xmlHashDeallocator,
) -> ::core::ffi::c_int {
    let Some(table_ref) = (unsafe { table.as_mut() }) else {
        return -(1 as ::core::ffi::c_int);
    };
    if name.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let key = unsafe { xmlHashComputeKey(table, name, name2, name3) };
    let bucket = hash_bucket_entry(table_ref, key);
    if !hash_entry_valid(bucket) {
        return -(1 as ::core::ffi::c_int);
    }
    let mut entry = bucket;
    let mut prev = ::core::ptr::null_mut::<xmlHashEntry>();
    while !entry.is_null() {
        if hash_entry_matches_str(entry, name, name2, name3) {
            let payload = hash_entry_payload(entry);
            if let Some(dealloc) = f {
                if !payload.is_null() {
                    unsafe {
                        dealloc(payload, hash_entry_name(entry));
                    }
                }
            }
            unsafe {
                (*entry).payload = NULL;
            }
            if table_ref.dict.is_null() {
                let entry_name = hash_entry_name(entry);
                if !entry_name.is_null() {
                    unsafe {
                        xmlFree.expect("non-null function pointer")(
                            entry_name as *mut ::core::ffi::c_void,
                        );
                    }
                }
                let entry_name2 = hash_entry_name2(entry);
                if !entry_name2.is_null() {
                    unsafe {
                        xmlFree.expect("non-null function pointer")(
                            entry_name2 as *mut ::core::ffi::c_void,
                        );
                    }
                }
                let entry_name3 = hash_entry_name3(entry);
                if !entry_name3.is_null() {
                    unsafe {
                        xmlFree.expect("non-null function pointer")(
                            entry_name3 as *mut ::core::ffi::c_void,
                        );
                    }
                }
            }
            if !prev.is_null() {
                unsafe {
                    (*prev).next = (*entry).next;
                    xmlFree.expect("non-null function pointer")(entry as *mut ::core::ffi::c_void);
                }
            } else {
                let next = hash_entry_next(entry);
                if next.is_null() {
                    unsafe {
                        (*entry).valid = 0 as ::core::ffi::c_int;
                    }
                } else {
                    unsafe {
                        memcpy(
                            bucket as *mut _xmlHashEntry as *mut ::core::ffi::c_void,
                            next as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<xmlHashEntry>() as size_t,
                        );
                        xmlFree.expect("non-null function pointer")(
                            next as *mut ::core::ffi::c_void,
                        );
                    }
                }
            }
            table_ref.nbElems -= 1;
            return 0 as ::core::ffi::c_int;
        }
        prev = entry;
        entry = hash_entry_next(entry);
    }
    return -(1 as ::core::ffi::c_int);
}
