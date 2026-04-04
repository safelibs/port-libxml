use super::*;
use crate::internal_ffi;

macro_rules! forward_fn {
    ($name:ident($($arg:ident: $arg_ty:ty),* $(,)?) -> $ret:ty, $symbol:literal, $default:expr) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name($($arg: $arg_ty),*) -> $ret {
            internal_ffi::ffi_boundary($default, || {
                type FnTy = unsafe extern "C" fn($($arg_ty),*) -> $ret;
                match load_helper_symbol!($symbol, FnTy) {
                    Some(f) => unsafe { f($($arg),*) },
                    None => $default,
                }
            })
        }
    };
}

macro_rules! forward_void {
    ($name:ident($($arg:ident: $arg_ty:ty),* $(,)?), $symbol:literal) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name($($arg: $arg_ty),*) {
            let _ = internal_ffi::ffi_boundary_unit(|| {
                type FnTy = unsafe extern "C" fn($($arg_ty),*);
                if let Some(f) = load_helper_symbol!($symbol, FnTy) {
                    unsafe { f($($arg),*) };
                }
            });
        }
    };
}

forward_void!(xmlSchemaInitTypes(), "xmlSchemaInitTypes");
forward_void!(xmlSchemaCleanupTypes(), "xmlSchemaCleanupTypes");
forward_fn!(
    xmlSchemaGetPredefinedType(name: *const xmlChar, ns: *const xmlChar) -> xmlSchemaTypePtr,
    "xmlSchemaGetPredefinedType",
    std::ptr::null_mut()
);
forward_fn!(
    xmlSchemaValidatePredefinedType(
        type_0: xmlSchemaTypePtr,
        value: *const xmlChar,
        val: *mut xmlSchemaValPtr,
    ) -> c_int,
    "xmlSchemaValidatePredefinedType",
    -1
);
forward_fn!(
    xmlSchemaValPredefTypeNode(
        type_0: xmlSchemaTypePtr,
        value: *const xmlChar,
        val: *mut xmlSchemaValPtr,
        node: xmlNodePtr,
    ) -> c_int,
    "xmlSchemaValPredefTypeNode",
    -1
);
forward_fn!(
    xmlSchemaValidateFacet(
        base: xmlSchemaTypePtr,
        facet: xmlSchemaFacetPtr,
        value: *const xmlChar,
        val: xmlSchemaValPtr,
    ) -> c_int,
    "xmlSchemaValidateFacet",
    -1
);
forward_fn!(
    xmlSchemaValidateFacetWhtsp(
        facet: xmlSchemaFacetPtr,
        fws: xmlSchemaWhitespaceValueType,
        valType: xmlSchemaValType,
        value: *const xmlChar,
        val: xmlSchemaValPtr,
        ws: xmlSchemaWhitespaceValueType,
    ) -> c_int,
    "xmlSchemaValidateFacetWhtsp",
    -1
);
forward_void!(xmlSchemaFreeValue(val: xmlSchemaValPtr), "xmlSchemaFreeValue");
forward_fn!(
    xmlSchemaNewFacet() -> xmlSchemaFacetPtr,
    "xmlSchemaNewFacet",
    std::ptr::null_mut()
);
forward_fn!(
    xmlSchemaCheckFacet(
        facet: xmlSchemaFacetPtr,
        typeDecl: xmlSchemaTypePtr,
        ctxt: xmlSchemaParserCtxtPtr,
        name: *const xmlChar,
    ) -> c_int,
    "xmlSchemaCheckFacet",
    -1
);
forward_void!(xmlSchemaFreeFacet(facet: xmlSchemaFacetPtr), "xmlSchemaFreeFacet");
forward_fn!(
    xmlSchemaCompareValues(x: xmlSchemaValPtr, y: xmlSchemaValPtr) -> c_int,
    "xmlSchemaCompareValues",
    -1
);
forward_fn!(
    xmlSchemaGetBuiltInListSimpleTypeItemType(type_0: xmlSchemaTypePtr) -> xmlSchemaTypePtr,
    "xmlSchemaGetBuiltInListSimpleTypeItemType",
    std::ptr::null_mut()
);
forward_fn!(
    xmlSchemaValidateListSimpleTypeFacet(
        facet: xmlSchemaFacetPtr,
        value: *const xmlChar,
        actualLen: c_ulong,
        expectedLen: *mut c_ulong,
    ) -> c_int,
    "xmlSchemaValidateListSimpleTypeFacet",
    -1
);
forward_fn!(
    xmlSchemaGetBuiltInType(type_0: xmlSchemaValType) -> xmlSchemaTypePtr,
    "xmlSchemaGetBuiltInType",
    std::ptr::null_mut()
);
forward_fn!(
    xmlSchemaIsBuiltInTypeFacet(type_0: xmlSchemaTypePtr, facetType: c_int) -> c_int,
    "xmlSchemaIsBuiltInTypeFacet",
    0
);
forward_fn!(
    xmlSchemaCollapseString(value: *const xmlChar) -> *mut xmlChar,
    "xmlSchemaCollapseString",
    std::ptr::null_mut()
);
forward_fn!(
    xmlSchemaWhiteSpaceReplace(value: *const xmlChar) -> *mut xmlChar,
    "xmlSchemaWhiteSpaceReplace",
    std::ptr::null_mut()
);
forward_fn!(
    xmlSchemaGetFacetValueAsULong(facet: xmlSchemaFacetPtr) -> c_ulong,
    "xmlSchemaGetFacetValueAsULong",
    0
);
forward_fn!(
    xmlSchemaValidateLengthFacet(
        type_0: xmlSchemaTypePtr,
        facet: xmlSchemaFacetPtr,
        value: *const xmlChar,
        val: xmlSchemaValPtr,
        length: *mut c_ulong,
    ) -> c_int,
    "xmlSchemaValidateLengthFacet",
    -1
);
forward_fn!(
    xmlSchemaValidateLengthFacetWhtsp(
        facet: xmlSchemaFacetPtr,
        valType: xmlSchemaValType,
        value: *const xmlChar,
        val: xmlSchemaValPtr,
        length: *mut c_ulong,
        ws: xmlSchemaWhitespaceValueType,
    ) -> c_int,
    "xmlSchemaValidateLengthFacetWhtsp",
    -1
);
forward_fn!(
    xmlSchemaValPredefTypeNodeNoNorm(
        type_0: xmlSchemaTypePtr,
        value: *const xmlChar,
        val: *mut xmlSchemaValPtr,
        node: xmlNodePtr,
    ) -> c_int,
    "xmlSchemaValPredefTypeNodeNoNorm",
    -1
);
forward_fn!(
    xmlSchemaGetCanonValue(val: xmlSchemaValPtr, retValue: *mut *const xmlChar) -> c_int,
    "xmlSchemaGetCanonValue",
    -1
);
forward_fn!(
    xmlSchemaGetCanonValueWhtsp(
        val: xmlSchemaValPtr,
        retValue: *mut *const xmlChar,
        ws: xmlSchemaWhitespaceValueType,
    ) -> c_int,
    "xmlSchemaGetCanonValueWhtsp",
    -1
);
forward_fn!(
    xmlSchemaValueAppend(prev: xmlSchemaValPtr, cur: xmlSchemaValPtr) -> c_int,
    "xmlSchemaValueAppend",
    -1
);
forward_fn!(
    xmlSchemaValueGetNext(cur: xmlSchemaValPtr) -> xmlSchemaValPtr,
    "xmlSchemaValueGetNext",
    std::ptr::null_mut()
);
forward_fn!(
    xmlSchemaValueGetAsString(val: xmlSchemaValPtr) -> *const xmlChar,
    "xmlSchemaValueGetAsString",
    std::ptr::null()
);
forward_fn!(
    xmlSchemaValueGetAsBoolean(val: xmlSchemaValPtr) -> c_int,
    "xmlSchemaValueGetAsBoolean",
    -1
);
forward_fn!(
    xmlSchemaNewStringValue(type_0: xmlSchemaValType, value: *const xmlChar) -> xmlSchemaValPtr,
    "xmlSchemaNewStringValue",
    std::ptr::null_mut()
);
forward_fn!(
    xmlSchemaNewNOTATIONValue(name: *const xmlChar, ns: *const xmlChar) -> xmlSchemaValPtr,
    "xmlSchemaNewNOTATIONValue",
    std::ptr::null_mut()
);
forward_fn!(
    xmlSchemaNewQNameValue(namespaceName: *const xmlChar, localName: *const xmlChar) -> xmlSchemaValPtr,
    "xmlSchemaNewQNameValue",
    std::ptr::null_mut()
);
forward_fn!(
    xmlSchemaCompareValuesWhtsp(
        x: xmlSchemaValPtr,
        xws: xmlSchemaWhitespaceValueType,
        y: xmlSchemaValPtr,
        yws: xmlSchemaWhitespaceValueType,
    ) -> c_int,
    "xmlSchemaCompareValuesWhtsp",
    -1
);
forward_fn!(
    xmlSchemaCopyValue(val: xmlSchemaValPtr) -> xmlSchemaValPtr,
    "xmlSchemaCopyValue",
    std::ptr::null_mut()
);
forward_fn!(
    xmlSchemaGetValType(val: xmlSchemaValPtr) -> xmlSchemaValType,
    "xmlSchemaGetValType",
    XML_SCHEMAS_UNKNOWN
);
forward_void!(xmlSchemaFreeType(type_0: xmlSchemaTypePtr), "xmlSchemaFreeType");
forward_void!(xmlSchemaFreeWildcard(wildcard: xmlSchemaWildcardPtr), "xmlSchemaFreeWildcard");
