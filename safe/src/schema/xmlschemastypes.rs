use super::*;
use crate::internal_ffi;

unsafe extern "C" {
    #[link_name = "safe_schema_internal_xmlSchemaCheckFacet"]
    fn xmlSchemaCheckFacet_internal(
        facet: xmlSchemaFacetPtr,
        typeDecl: xmlSchemaTypePtr,
        ctxt: xmlSchemaParserCtxtPtr,
        name: *const xmlChar,
    ) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaCleanupTypes"]
    fn xmlSchemaCleanupTypes_internal();
    #[link_name = "safe_schema_internal_xmlSchemaCollapseString"]
    fn xmlSchemaCollapseString_internal(value: *const xmlChar) -> *mut xmlChar;
    #[link_name = "safe_schema_internal_xmlSchemaCompareValues"]
    fn xmlSchemaCompareValues_internal(x: xmlSchemaValPtr, y: xmlSchemaValPtr) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaCompareValuesWhtsp"]
    fn xmlSchemaCompareValuesWhtsp_internal(
        x: xmlSchemaValPtr,
        xws: xmlSchemaWhitespaceValueType,
        y: xmlSchemaValPtr,
        yws: xmlSchemaWhitespaceValueType,
    ) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaCopyValue"]
    fn xmlSchemaCopyValue_internal(val: xmlSchemaValPtr) -> xmlSchemaValPtr;
    #[link_name = "safe_schema_internal_xmlSchemaFreeFacet"]
    fn xmlSchemaFreeFacet_internal(facet: xmlSchemaFacetPtr);
    #[link_name = "safe_schema_internal_xmlSchemaFreeType"]
    fn xmlSchemaFreeType_internal(type_0: xmlSchemaTypePtr);
    #[link_name = "safe_schema_internal_xmlSchemaFreeValue"]
    fn xmlSchemaFreeValue_internal(val: xmlSchemaValPtr);
    #[link_name = "safe_schema_internal_xmlSchemaFreeWildcard"]
    fn xmlSchemaFreeWildcard_internal(wildcard: xmlSchemaWildcardPtr);
    #[link_name = "safe_schema_internal_xmlSchemaGetBuiltInListSimpleTypeItemType"]
    fn xmlSchemaGetBuiltInListSimpleTypeItemType_internal(
        type_0: xmlSchemaTypePtr,
    ) -> xmlSchemaTypePtr;
    #[link_name = "safe_schema_internal_xmlSchemaGetBuiltInType"]
    fn xmlSchemaGetBuiltInType_internal(type_0: xmlSchemaValType) -> xmlSchemaTypePtr;
    #[link_name = "safe_schema_internal_xmlSchemaGetCanonValue"]
    fn xmlSchemaGetCanonValue_internal(
        val: xmlSchemaValPtr,
        retValue: *mut *const xmlChar,
    ) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaGetCanonValueWhtsp"]
    fn xmlSchemaGetCanonValueWhtsp_internal(
        val: xmlSchemaValPtr,
        retValue: *mut *const xmlChar,
        ws: xmlSchemaWhitespaceValueType,
    ) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaGetFacetValueAsULong"]
    fn xmlSchemaGetFacetValueAsULong_internal(facet: xmlSchemaFacetPtr) -> c_ulong;
    #[link_name = "safe_schema_internal_xmlSchemaGetPredefinedType"]
    fn xmlSchemaGetPredefinedType_internal(
        name: *const xmlChar,
        ns: *const xmlChar,
    ) -> xmlSchemaTypePtr;
    #[link_name = "safe_schema_internal_xmlSchemaGetValType"]
    fn xmlSchemaGetValType_internal(val: xmlSchemaValPtr) -> xmlSchemaValType;
    #[link_name = "safe_schema_internal_xmlSchemaInitTypes"]
    fn xmlSchemaInitTypes_internal();
    #[link_name = "safe_schema_internal_xmlSchemaIsBuiltInTypeFacet"]
    fn xmlSchemaIsBuiltInTypeFacet_internal(type_0: xmlSchemaTypePtr, facetType: c_int) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaNewFacet"]
    fn xmlSchemaNewFacet_internal() -> xmlSchemaFacetPtr;
    #[link_name = "safe_schema_internal_xmlSchemaNewNOTATIONValue"]
    fn xmlSchemaNewNOTATIONValue_internal(
        name: *const xmlChar,
        ns: *const xmlChar,
    ) -> xmlSchemaValPtr;
    #[link_name = "safe_schema_internal_xmlSchemaNewQNameValue"]
    fn xmlSchemaNewQNameValue_internal(
        namespaceName: *const xmlChar,
        localName: *const xmlChar,
    ) -> xmlSchemaValPtr;
    #[link_name = "safe_schema_internal_xmlSchemaNewStringValue"]
    fn xmlSchemaNewStringValue_internal(
        type_0: xmlSchemaValType,
        value: *const xmlChar,
    ) -> xmlSchemaValPtr;
    #[link_name = "safe_schema_internal_xmlSchemaValidateFacet"]
    fn xmlSchemaValidateFacet_internal(
        base: xmlSchemaTypePtr,
        facet: xmlSchemaFacetPtr,
        value: *const xmlChar,
        val: xmlSchemaValPtr,
    ) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaValidateFacetWhtsp"]
    fn xmlSchemaValidateFacetWhtsp_internal(
        facet: xmlSchemaFacetPtr,
        fws: xmlSchemaWhitespaceValueType,
        valType: xmlSchemaValType,
        value: *const xmlChar,
        val: xmlSchemaValPtr,
        ws: xmlSchemaWhitespaceValueType,
    ) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaValidateLengthFacet"]
    fn xmlSchemaValidateLengthFacet_internal(
        type_0: xmlSchemaTypePtr,
        facet: xmlSchemaFacetPtr,
        value: *const xmlChar,
        val: xmlSchemaValPtr,
        length: *mut c_ulong,
    ) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaValidateLengthFacetWhtsp"]
    fn xmlSchemaValidateLengthFacetWhtsp_internal(
        facet: xmlSchemaFacetPtr,
        valType: xmlSchemaValType,
        value: *const xmlChar,
        val: xmlSchemaValPtr,
        length: *mut c_ulong,
        ws: xmlSchemaWhitespaceValueType,
    ) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaValidateListSimpleTypeFacet"]
    fn xmlSchemaValidateListSimpleTypeFacet_internal(
        facet: xmlSchemaFacetPtr,
        value: *const xmlChar,
        actualLen: c_ulong,
        expectedLen: *mut c_ulong,
    ) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaValidatePredefinedType"]
    fn xmlSchemaValidatePredefinedType_internal(
        type_0: xmlSchemaTypePtr,
        value: *const xmlChar,
        val: *mut xmlSchemaValPtr,
    ) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaValPredefTypeNode"]
    fn xmlSchemaValPredefTypeNode_internal(
        type_0: xmlSchemaTypePtr,
        value: *const xmlChar,
        val: *mut xmlSchemaValPtr,
        node: xmlNodePtr,
    ) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaValPredefTypeNodeNoNorm"]
    fn xmlSchemaValPredefTypeNodeNoNorm_internal(
        type_0: xmlSchemaTypePtr,
        value: *const xmlChar,
        val: *mut xmlSchemaValPtr,
        node: xmlNodePtr,
    ) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaValueAppend"]
    fn xmlSchemaValueAppend_internal(prev: xmlSchemaValPtr, cur: xmlSchemaValPtr) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaValueGetAsBoolean"]
    fn xmlSchemaValueGetAsBoolean_internal(val: xmlSchemaValPtr) -> c_int;
    #[link_name = "safe_schema_internal_xmlSchemaValueGetAsString"]
    fn xmlSchemaValueGetAsString_internal(val: xmlSchemaValPtr) -> *const xmlChar;
    #[link_name = "safe_schema_internal_xmlSchemaValueGetNext"]
    fn xmlSchemaValueGetNext_internal(cur: xmlSchemaValPtr) -> xmlSchemaValPtr;
    #[link_name = "safe_schema_internal_xmlSchemaWhiteSpaceReplace"]
    fn xmlSchemaWhiteSpaceReplace_internal(value: *const xmlChar) -> *mut xmlChar;
}

macro_rules! wrap_fn {
    ($public:ident($($arg:ident: $arg_ty:ty),* $(,)?) -> $ret:ty, $internal:ident, $default:expr) => {
        #[no_mangle]
        pub unsafe extern "C" fn $public($($arg: $arg_ty),*) -> $ret {
            internal_ffi::ffi_boundary($default, || unsafe { $internal($($arg),*) })
        }
    };
}

macro_rules! wrap_void {
    ($public:ident($($arg:ident: $arg_ty:ty),* $(,)?), $internal:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $public($($arg: $arg_ty),*) {
            let _ = internal_ffi::ffi_boundary_unit(|| unsafe {
                $internal($($arg),*);
            });
        }
    };
}

wrap_void!(xmlSchemaInitTypes(), xmlSchemaInitTypes_internal);
wrap_void!(xmlSchemaCleanupTypes(), xmlSchemaCleanupTypes_internal);
wrap_fn!(
    xmlSchemaGetPredefinedType(name: *const xmlChar, ns: *const xmlChar) -> xmlSchemaTypePtr,
    xmlSchemaGetPredefinedType_internal,
    std::ptr::null_mut()
);
wrap_fn!(
    xmlSchemaValidatePredefinedType(
        type_0: xmlSchemaTypePtr,
        value: *const xmlChar,
        val: *mut xmlSchemaValPtr,
    ) -> c_int,
    xmlSchemaValidatePredefinedType_internal,
    -1
);
wrap_fn!(
    xmlSchemaValPredefTypeNode(
        type_0: xmlSchemaTypePtr,
        value: *const xmlChar,
        val: *mut xmlSchemaValPtr,
        node: xmlNodePtr,
    ) -> c_int,
    xmlSchemaValPredefTypeNode_internal,
    -1
);
wrap_fn!(
    xmlSchemaValidateFacet(
        base: xmlSchemaTypePtr,
        facet: xmlSchemaFacetPtr,
        value: *const xmlChar,
        val: xmlSchemaValPtr,
    ) -> c_int,
    xmlSchemaValidateFacet_internal,
    -1
);
wrap_fn!(
    xmlSchemaValidateFacetWhtsp(
        facet: xmlSchemaFacetPtr,
        fws: xmlSchemaWhitespaceValueType,
        valType: xmlSchemaValType,
        value: *const xmlChar,
        val: xmlSchemaValPtr,
        ws: xmlSchemaWhitespaceValueType,
    ) -> c_int,
    xmlSchemaValidateFacetWhtsp_internal,
    -1
);
wrap_void!(xmlSchemaFreeValue(val: xmlSchemaValPtr), xmlSchemaFreeValue_internal);
wrap_fn!(
    xmlSchemaNewFacet() -> xmlSchemaFacetPtr,
    xmlSchemaNewFacet_internal,
    std::ptr::null_mut()
);
wrap_fn!(
    xmlSchemaCheckFacet(
        facet: xmlSchemaFacetPtr,
        typeDecl: xmlSchemaTypePtr,
        ctxt: xmlSchemaParserCtxtPtr,
        name: *const xmlChar,
    ) -> c_int,
    xmlSchemaCheckFacet_internal,
    -1
);
wrap_void!(xmlSchemaFreeFacet(facet: xmlSchemaFacetPtr), xmlSchemaFreeFacet_internal);
wrap_fn!(
    xmlSchemaCompareValues(x: xmlSchemaValPtr, y: xmlSchemaValPtr) -> c_int,
    xmlSchemaCompareValues_internal,
    -1
);
wrap_fn!(
    xmlSchemaGetBuiltInListSimpleTypeItemType(type_0: xmlSchemaTypePtr) -> xmlSchemaTypePtr,
    xmlSchemaGetBuiltInListSimpleTypeItemType_internal,
    std::ptr::null_mut()
);
wrap_fn!(
    xmlSchemaValidateListSimpleTypeFacet(
        facet: xmlSchemaFacetPtr,
        value: *const xmlChar,
        actualLen: c_ulong,
        expectedLen: *mut c_ulong,
    ) -> c_int,
    xmlSchemaValidateListSimpleTypeFacet_internal,
    -1
);
wrap_fn!(
    xmlSchemaGetBuiltInType(type_0: xmlSchemaValType) -> xmlSchemaTypePtr,
    xmlSchemaGetBuiltInType_internal,
    std::ptr::null_mut()
);
wrap_fn!(
    xmlSchemaIsBuiltInTypeFacet(type_0: xmlSchemaTypePtr, facetType: c_int) -> c_int,
    xmlSchemaIsBuiltInTypeFacet_internal,
    0
);
wrap_fn!(
    xmlSchemaCollapseString(value: *const xmlChar) -> *mut xmlChar,
    xmlSchemaCollapseString_internal,
    std::ptr::null_mut()
);
wrap_fn!(
    xmlSchemaWhiteSpaceReplace(value: *const xmlChar) -> *mut xmlChar,
    xmlSchemaWhiteSpaceReplace_internal,
    std::ptr::null_mut()
);
wrap_fn!(
    xmlSchemaGetFacetValueAsULong(facet: xmlSchemaFacetPtr) -> c_ulong,
    xmlSchemaGetFacetValueAsULong_internal,
    0
);
wrap_fn!(
    xmlSchemaValidateLengthFacet(
        type_0: xmlSchemaTypePtr,
        facet: xmlSchemaFacetPtr,
        value: *const xmlChar,
        val: xmlSchemaValPtr,
        length: *mut c_ulong,
    ) -> c_int,
    xmlSchemaValidateLengthFacet_internal,
    -1
);
wrap_fn!(
    xmlSchemaValidateLengthFacetWhtsp(
        facet: xmlSchemaFacetPtr,
        valType: xmlSchemaValType,
        value: *const xmlChar,
        val: xmlSchemaValPtr,
        length: *mut c_ulong,
        ws: xmlSchemaWhitespaceValueType,
    ) -> c_int,
    xmlSchemaValidateLengthFacetWhtsp_internal,
    -1
);
wrap_fn!(
    xmlSchemaValPredefTypeNodeNoNorm(
        type_0: xmlSchemaTypePtr,
        value: *const xmlChar,
        val: *mut xmlSchemaValPtr,
        node: xmlNodePtr,
    ) -> c_int,
    xmlSchemaValPredefTypeNodeNoNorm_internal,
    -1
);
wrap_fn!(
    xmlSchemaGetCanonValue(val: xmlSchemaValPtr, retValue: *mut *const xmlChar) -> c_int,
    xmlSchemaGetCanonValue_internal,
    -1
);
wrap_fn!(
    xmlSchemaGetCanonValueWhtsp(
        val: xmlSchemaValPtr,
        retValue: *mut *const xmlChar,
        ws: xmlSchemaWhitespaceValueType,
    ) -> c_int,
    xmlSchemaGetCanonValueWhtsp_internal,
    -1
);
wrap_fn!(
    xmlSchemaValueAppend(prev: xmlSchemaValPtr, cur: xmlSchemaValPtr) -> c_int,
    xmlSchemaValueAppend_internal,
    -1
);
wrap_fn!(
    xmlSchemaValueGetNext(cur: xmlSchemaValPtr) -> xmlSchemaValPtr,
    xmlSchemaValueGetNext_internal,
    std::ptr::null_mut()
);
wrap_fn!(
    xmlSchemaValueGetAsString(val: xmlSchemaValPtr) -> *const xmlChar,
    xmlSchemaValueGetAsString_internal,
    std::ptr::null()
);
wrap_fn!(
    xmlSchemaValueGetAsBoolean(val: xmlSchemaValPtr) -> c_int,
    xmlSchemaValueGetAsBoolean_internal,
    -1
);
wrap_fn!(
    xmlSchemaNewStringValue(type_0: xmlSchemaValType, value: *const xmlChar) -> xmlSchemaValPtr,
    xmlSchemaNewStringValue_internal,
    std::ptr::null_mut()
);
wrap_fn!(
    xmlSchemaNewNOTATIONValue(name: *const xmlChar, ns: *const xmlChar) -> xmlSchemaValPtr,
    xmlSchemaNewNOTATIONValue_internal,
    std::ptr::null_mut()
);
wrap_fn!(
    xmlSchemaNewQNameValue(
        namespaceName: *const xmlChar,
        localName: *const xmlChar,
    ) -> xmlSchemaValPtr,
    xmlSchemaNewQNameValue_internal,
    std::ptr::null_mut()
);
wrap_fn!(
    xmlSchemaCompareValuesWhtsp(
        x: xmlSchemaValPtr,
        xws: xmlSchemaWhitespaceValueType,
        y: xmlSchemaValPtr,
        yws: xmlSchemaWhitespaceValueType,
    ) -> c_int,
    xmlSchemaCompareValuesWhtsp_internal,
    -1
);
wrap_fn!(
    xmlSchemaCopyValue(val: xmlSchemaValPtr) -> xmlSchemaValPtr,
    xmlSchemaCopyValue_internal,
    std::ptr::null_mut()
);
wrap_fn!(
    xmlSchemaGetValType(val: xmlSchemaValPtr) -> xmlSchemaValType,
    xmlSchemaGetValType_internal,
    XML_SCHEMAS_UNKNOWN
);
wrap_void!(xmlSchemaFreeType(type_0: xmlSchemaTypePtr), xmlSchemaFreeType_internal);
wrap_void!(xmlSchemaFreeWildcard(wildcard: xmlSchemaWildcardPtr), xmlSchemaFreeWildcard_internal);
