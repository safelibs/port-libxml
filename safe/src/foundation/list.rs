extern "C" {
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlFree: xmlFreeFunc;
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    fn __xmlGenericErrorContext() -> *mut *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type xmlFreeFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type xmlMallocFunc = Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
pub type xmlGenericErrorFunc =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlLink {
    pub next: *mut _xmlLink,
    pub prev: *mut _xmlLink,
    pub data: *mut ::core::ffi::c_void,
}
pub type xmlLink = _xmlLink;
pub type xmlLinkPtr = *mut xmlLink;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlList {
    pub sentinel: xmlLinkPtr,
    pub linkDeallocator: Option<unsafe extern "C" fn(xmlLinkPtr) -> ()>,
    pub linkCompare: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
}
pub type xmlList = _xmlList;
pub type xmlListPtr = *mut xmlList;
pub type xmlListDeallocator = Option<unsafe extern "C" fn(xmlLinkPtr) -> ()>;
pub type xmlListDataCompare = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type xmlListWalker = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
unsafe extern "C" fn xmlLinkDeallocator(mut l: xmlListPtr, mut lk: xmlLinkPtr) {
    unsafe {
        (*(*lk).prev).next = (*lk).next;
        (*(*lk).next).prev = (*lk).prev;
    }
    if unsafe { (*l).linkDeallocator.is_some() } {
        unsafe { (*l).linkDeallocator.expect("non-null function pointer")(lk) };
    }
    unsafe { xmlFree.expect("non-null function pointer")(lk as *mut ::core::ffi::c_void) };
}
unsafe extern "C" fn xmlLinkCompare(
    mut data0: *const ::core::ffi::c_void,
    mut data1: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if data0 < data1 {
        return -(1 as ::core::ffi::c_int);
    } else if data0 == data1 {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlListLowerSearch(
    mut l: xmlListPtr,
    mut data: *mut ::core::ffi::c_void,
) -> xmlLinkPtr {
    if l.is_null() {
        return ::core::ptr::null_mut::<xmlLink>();
    }
    let mut lk = unsafe { (*(*l).sentinel).next as xmlLinkPtr };
    while lk != unsafe { (*l).sentinel }
        && unsafe { (*l).linkCompare.expect("non-null function pointer")((*lk).data, data) }
            < 0 as ::core::ffi::c_int
    {
        lk = unsafe { (*lk).next as xmlLinkPtr };
    }
    return lk;
}
unsafe extern "C" fn xmlListHigherSearch(
    mut l: xmlListPtr,
    mut data: *mut ::core::ffi::c_void,
) -> xmlLinkPtr {
    if l.is_null() {
        return ::core::ptr::null_mut::<xmlLink>();
    }
    let mut lk = unsafe { (*(*l).sentinel).prev as xmlLinkPtr };
    while lk != unsafe { (*l).sentinel }
        && unsafe { (*l).linkCompare.expect("non-null function pointer")((*lk).data, data) }
            > 0 as ::core::ffi::c_int
    {
        lk = unsafe { (*lk).prev as xmlLinkPtr };
    }
    return lk;
}
unsafe extern "C" fn xmlListLinkSearch(
    mut l: xmlListPtr,
    mut data: *mut ::core::ffi::c_void,
) -> xmlLinkPtr {
    if l.is_null() {
        return ::core::ptr::null_mut::<xmlLink>();
    }
    let lk = unsafe { xmlListLowerSearch(l, data) };
    if lk == unsafe { (*l).sentinel } {
        return ::core::ptr::null_mut::<xmlLink>();
    } else {
        if unsafe { (*l).linkCompare.expect("non-null function pointer")((*lk).data, data) }
            == 0 as ::core::ffi::c_int
        {
            return lk;
        }
        return ::core::ptr::null_mut::<xmlLink>();
    };
}
unsafe extern "C" fn xmlListLinkReverseSearch(
    mut l: xmlListPtr,
    mut data: *mut ::core::ffi::c_void,
) -> xmlLinkPtr {
    if l.is_null() {
        return ::core::ptr::null_mut::<xmlLink>();
    }
    let lk = unsafe { xmlListHigherSearch(l, data) };
    if lk == unsafe { (*l).sentinel } {
        return ::core::ptr::null_mut::<xmlLink>();
    } else {
        if unsafe { (*l).linkCompare.expect("non-null function pointer")((*lk).data, data) }
            == 0 as ::core::ffi::c_int
        {
            return lk;
        }
        return ::core::ptr::null_mut::<xmlLink>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlListCreate(
    mut deallocator: xmlListDeallocator,
    mut compare: xmlListDataCompare,
) -> xmlListPtr {
    let mut l: xmlListPtr = ::core::ptr::null_mut::<xmlList>();
    l = unsafe {
        xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlList>() as size_t)
            as xmlListPtr
    };
    if l.is_null() {
        unsafe {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Cannot initialize memory for list\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        return ::core::ptr::null_mut::<xmlList>();
    }
    unsafe {
        memset(
            l as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<xmlList>() as size_t,
        );
        (*l).sentinel = xmlMalloc.expect("non-null function pointer")(
            ::core::mem::size_of::<xmlLink>() as size_t,
        ) as xmlLinkPtr;
    }
    if unsafe { (*l).sentinel.is_null() } {
        unsafe {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Cannot initialize memory for sentinel\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            xmlFree.expect("non-null function pointer")(l as *mut ::core::ffi::c_void);
        }
        return ::core::ptr::null_mut::<xmlList>();
    }
    unsafe {
        (*(*l).sentinel).next = (*l).sentinel as *mut _xmlLink;
        (*(*l).sentinel).prev = (*l).sentinel as *mut _xmlLink;
        (*(*l).sentinel).data = NULL;
    }
    if deallocator.is_some() {
        unsafe {
            (*l).linkDeallocator = deallocator as Option<unsafe extern "C" fn(xmlLinkPtr) -> ()>;
        }
    }
    if compare.is_some() {
        unsafe {
            (*l).linkCompare = compare
                as Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
                >;
        }
    } else {
        unsafe {
            (*l).linkCompare = Some(
                xmlLinkCompare
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
                >;
        }
    }
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListSearch(
    mut l: xmlListPtr,
    mut data: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    if l.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    let lk = unsafe { xmlListLinkSearch(l, data) };
    if !lk.is_null() {
        return unsafe { (*lk).data };
    }
    return NULL;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListReverseSearch(
    mut l: xmlListPtr,
    mut data: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    if l.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    let lk = unsafe { xmlListLinkReverseSearch(l, data) };
    if !lk.is_null() {
        return unsafe { (*lk).data };
    }
    return NULL;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListInsert(
    mut l: xmlListPtr,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if l.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    let lkPlace = unsafe { xmlListLowerSearch(l, data) };
    let lkNew = unsafe {
        xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlLink>() as size_t)
            as xmlLinkPtr
    };
    if lkNew.is_null() {
        unsafe {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Cannot initialize memory for new link\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        return 1 as ::core::ffi::c_int;
    }
    unsafe {
        let lk_place_prev = (*lkPlace).prev as xmlLinkPtr;
        (*lkNew).data = data;
        (*lkNew).next = (*lk_place_prev).next;
        (*(*lk_place_prev).next).prev = lkNew as *mut _xmlLink;
        (*lk_place_prev).next = lkNew as *mut _xmlLink;
        (*lkNew).prev = lk_place_prev as *mut _xmlLink;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListAppend(
    mut l: xmlListPtr,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if l.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    let lkPlace = unsafe { xmlListHigherSearch(l, data) };
    let lkNew = unsafe {
        xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlLink>() as size_t)
            as xmlLinkPtr
    };
    if lkNew.is_null() {
        unsafe {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Cannot initialize memory for new link\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        return 1 as ::core::ffi::c_int;
    }
    unsafe {
        (*lkNew).data = data;
        (*lkNew).next = (*lkPlace).next;
        (*(*lkPlace).next).prev = lkNew as *mut _xmlLink;
        (*lkPlace).next = lkNew as *mut _xmlLink;
        (*lkNew).prev = lkPlace as *mut _xmlLink;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListDelete(mut l: xmlListPtr) {
    if l.is_null() {
        return;
    }
    unsafe { xmlListClear(l) };
    let sentinel = unsafe { (*l).sentinel };
    unsafe {
        xmlFree.expect("non-null function pointer")(sentinel as *mut ::core::ffi::c_void);
        xmlFree.expect("non-null function pointer")(l as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlListRemoveFirst(
    mut l: xmlListPtr,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if l.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let lk = unsafe { xmlListLinkSearch(l, data) };
    if !lk.is_null() {
        unsafe { xmlLinkDeallocator(l, lk) };
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListRemoveLast(
    mut l: xmlListPtr,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if l.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let lk = unsafe { xmlListLinkReverseSearch(l, data) };
    if !lk.is_null() {
        unsafe { xmlLinkDeallocator(l, lk) };
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListRemoveAll(
    mut l: xmlListPtr,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if l.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    while unsafe { xmlListRemoveFirst(l, data) } != 0 {
        count += 1;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListClear(mut l: xmlListPtr) {
    let mut lk: xmlLinkPtr = ::core::ptr::null_mut::<xmlLink>();
    if l.is_null() {
        return;
    }
    let sentinel = unsafe { (*l).sentinel };
    lk = unsafe { (*sentinel).next as xmlLinkPtr };
    while lk != sentinel {
        let mut next: xmlLinkPtr = unsafe { (*lk).next as xmlLinkPtr };
        unsafe { xmlLinkDeallocator(l, lk) };
        lk = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlListEmpty(mut l: xmlListPtr) -> ::core::ffi::c_int {
    if l.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let sentinel = unsafe { (*l).sentinel };
    return (unsafe { (*sentinel).next } == sentinel) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListFront(mut l: xmlListPtr) -> xmlLinkPtr {
    if l.is_null() {
        return ::core::ptr::null_mut::<xmlLink>();
    }
    let sentinel = unsafe { (*l).sentinel };
    return unsafe { (*sentinel).next as xmlLinkPtr };
}
#[no_mangle]
pub unsafe extern "C" fn xmlListEnd(mut l: xmlListPtr) -> xmlLinkPtr {
    if l.is_null() {
        return ::core::ptr::null_mut::<xmlLink>();
    }
    let sentinel = unsafe { (*l).sentinel };
    return unsafe { (*sentinel).prev as xmlLinkPtr };
}
#[no_mangle]
pub unsafe extern "C" fn xmlListSize(mut l: xmlListPtr) -> ::core::ffi::c_int {
    let mut lk: xmlLinkPtr = ::core::ptr::null_mut::<xmlLink>();
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if l.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let sentinel = unsafe { (*l).sentinel };
    lk = unsafe { (*sentinel).next as xmlLinkPtr };
    while lk != sentinel {
        lk = unsafe { (*lk).next as xmlLinkPtr };
        count += 1;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListPopFront(mut l: xmlListPtr) {
    if unsafe { xmlListEmpty(l) } == 0 {
        let sentinel = unsafe { (*l).sentinel };
        let next = unsafe { (*sentinel).next as xmlLinkPtr };
        unsafe { xmlLinkDeallocator(l, next) };
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlListPopBack(mut l: xmlListPtr) {
    if unsafe { xmlListEmpty(l) } == 0 {
        let sentinel = unsafe { (*l).sentinel };
        let prev = unsafe { (*sentinel).prev as xmlLinkPtr };
        unsafe { xmlLinkDeallocator(l, prev) };
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlListPushFront(
    mut l: xmlListPtr,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut lkPlace: xmlLinkPtr = ::core::ptr::null_mut::<xmlLink>();
    let mut lkNew: xmlLinkPtr = ::core::ptr::null_mut::<xmlLink>();
    if l.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    lkPlace = unsafe { (*l).sentinel };
    lkNew = unsafe {
        xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlLink>() as size_t)
            as xmlLinkPtr
    };
    if lkNew.is_null() {
        unsafe {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Cannot initialize memory for new link\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        return 0 as ::core::ffi::c_int;
    }
    unsafe {
        (*lkNew).data = data;
        (*lkNew).next = (*lkPlace).next;
        (*(*lkPlace).next).prev = lkNew as *mut _xmlLink;
        (*lkPlace).next = lkNew as *mut _xmlLink;
        (*lkNew).prev = lkPlace as *mut _xmlLink;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListPushBack(
    mut l: xmlListPtr,
    mut data: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut lkPlace: xmlLinkPtr = ::core::ptr::null_mut::<xmlLink>();
    let mut lkNew: xmlLinkPtr = ::core::ptr::null_mut::<xmlLink>();
    if l.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let sentinel = unsafe { (*l).sentinel };
    lkPlace = unsafe { (*sentinel).prev as xmlLinkPtr };
    lkNew = unsafe {
        xmlMalloc.expect("non-null function pointer")(::core::mem::size_of::<xmlLink>() as size_t)
            as xmlLinkPtr
    };
    if lkNew.is_null() {
        unsafe {
            (*__xmlGenericError()).expect("non-null function pointer")(
                *__xmlGenericErrorContext(),
                b"Cannot initialize memory for new link\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        return 0 as ::core::ffi::c_int;
    }
    unsafe {
        (*lkNew).data = data;
        (*lkNew).next = (*lkPlace).next;
        (*(*lkPlace).next).prev = lkNew as *mut _xmlLink;
        (*lkPlace).next = lkNew as *mut _xmlLink;
        (*lkNew).prev = lkPlace as *mut _xmlLink;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlLinkGetData(mut lk: xmlLinkPtr) -> *mut ::core::ffi::c_void {
    if lk.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return unsafe { (*lk).data };
}
#[no_mangle]
pub unsafe extern "C" fn xmlListReverse(mut l: xmlListPtr) {
    let mut lk: xmlLinkPtr = ::core::ptr::null_mut::<xmlLink>();
    let mut lkPrev: xmlLinkPtr = ::core::ptr::null_mut::<xmlLink>();
    if l.is_null() {
        return;
    }
    let sentinel = unsafe { (*l).sentinel };
    lkPrev = sentinel;
    lk = unsafe { (*sentinel).next as xmlLinkPtr };
    while lk != sentinel {
        unsafe {
            (*lkPrev).next = (*lkPrev).prev;
            (*lkPrev).prev = lk as *mut _xmlLink;
        }
        lkPrev = lk;
        lk = unsafe { (*lk).next as xmlLinkPtr };
    }
    unsafe {
        (*lkPrev).next = (*lkPrev).prev;
        (*lkPrev).prev = lk as *mut _xmlLink;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlListSort(mut l: xmlListPtr) {
    let mut lTemp: xmlListPtr = ::core::ptr::null_mut::<xmlList>();
    if l.is_null() {
        return;
    }
    if unsafe { xmlListEmpty(l) } != 0 {
        return;
    }
    lTemp = unsafe { xmlListDup(l) };
    if lTemp.is_null() {
        return;
    }
    unsafe { xmlListClear(l) };
    xmlListMerge(l, lTemp);
    unsafe { xmlListDelete(lTemp) };
}
#[no_mangle]
pub unsafe extern "C" fn xmlListWalk(
    mut l: xmlListPtr,
    mut walker: xmlListWalker,
    mut user: *mut ::core::ffi::c_void,
) {
    let mut lk: xmlLinkPtr = ::core::ptr::null_mut::<xmlLink>();
    if l.is_null() || walker.is_none() {
        return;
    }
    let sentinel = unsafe { (*l).sentinel };
    lk = unsafe { (*sentinel).next as xmlLinkPtr };
    while lk != sentinel {
        if unsafe { walker.expect("non-null function pointer")((*lk).data, user) }
            == 0 as ::core::ffi::c_int
        {
            break;
        }
        lk = unsafe { (*lk).next as xmlLinkPtr };
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlListReverseWalk(
    mut l: xmlListPtr,
    mut walker: xmlListWalker,
    mut user: *mut ::core::ffi::c_void,
) {
    let mut lk: xmlLinkPtr = ::core::ptr::null_mut::<xmlLink>();
    if l.is_null() || walker.is_none() {
        return;
    }
    let sentinel = unsafe { (*l).sentinel };
    lk = unsafe { (*sentinel).prev as xmlLinkPtr };
    while lk != sentinel {
        if unsafe { walker.expect("non-null function pointer")((*lk).data, user) }
            == 0 as ::core::ffi::c_int
        {
            break;
        }
        lk = unsafe { (*lk).prev as xmlLinkPtr };
    }
}
#[no_mangle]
pub extern "C" fn xmlListMerge(mut l1: xmlListPtr, mut l2: xmlListPtr) {
    unsafe {
        xmlListCopy(l1, l2);
        xmlListClear(l2);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlListDup(old: xmlListPtr) -> xmlListPtr {
    if old.is_null() {
        return ::core::ptr::null_mut::<xmlList>();
    }
    let cur = unsafe { xmlListCreate(None, (*old).linkCompare as xmlListDataCompare) };
    if cur.is_null() {
        return ::core::ptr::null_mut::<xmlList>();
    }
    if unsafe { xmlListCopy(cur, old) } != 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<xmlList>();
    }
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListCopy(mut cur: xmlListPtr, old: xmlListPtr) -> ::core::ffi::c_int {
    if old.is_null() || cur.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    let mut lk = unsafe { (*(*old).sentinel).next as xmlLinkPtr };
    while lk != unsafe { (*old).sentinel } {
        if unsafe { xmlListInsert(cur, (*lk).data) } != 0 as ::core::ffi::c_int {
            unsafe { xmlListDelete(cur) };
            return 1 as ::core::ffi::c_int;
        }
        lk = unsafe { (*lk).next as xmlLinkPtr };
    }
    return 0 as ::core::ffi::c_int;
}
