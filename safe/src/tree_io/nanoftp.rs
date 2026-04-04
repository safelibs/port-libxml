use crate::tree_io::common::{deny_network_host, deny_network_uri};

#[repr(C)]
pub struct _xmlDict {
    _private: [u8; 0],
}

extern "C" {
    fn xmlStrndup(cur: *const xmlChar, len: ::core::ffi::c_int) -> *mut xmlChar;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn __xmlIOErr(
        domain: ::core::ffi::c_int,
        code: ::core::ffi::c_int,
        extra: *const ::core::ffi::c_char,
    );
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn select(
        __nfds: ::core::ffi::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::core::ffi::c_int;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn socket(
        __domain: ::core::ffi::c_int,
        __type: ::core::ffi::c_int,
        __protocol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn bind(
        __fd: ::core::ffi::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> ::core::ffi::c_int;
    fn getsockname(
        __fd: ::core::ffi::c_int,
        __addr: *mut sockaddr,
        __len: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    fn connect(
        __fd: ::core::ffi::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> ::core::ffi::c_int;
    fn send(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
    fn recv(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
    fn listen(__fd: ::core::ffi::c_int, __n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn htons(__hostshort: uint16_t) -> uint16_t;
    fn inet_ntop(
        __af: ::core::ffi::c_int,
        __cp: *const ::core::ffi::c_void,
        __buf: *mut ::core::ffi::c_char,
        __len: socklen_t,
    ) -> *const ::core::ffi::c_char;
    fn gethostbyname(__name: *const ::core::ffi::c_char) -> *mut hostent;
    fn getaddrinfo(
        __name: *const ::core::ffi::c_char,
        __service: *const ::core::ffi::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> ::core::ffi::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn __xmlSimpleError(
        domain: ::core::ffi::c_int,
        code: ::core::ffi::c_int,
        node: xmlNodePtr,
        msg: *const ::core::ffi::c_char,
        extra: *const ::core::ffi::c_char,
    );
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlFree: xmlFreeFunc;
    static mut xmlMemStrdup: xmlStrdupFunc;
    fn xmlParseURIRaw(str: *const ::core::ffi::c_char, raw: ::core::ffi::c_int) -> xmlURIPtr;
    fn xmlURIUnescapeString(
        str: *const ::core::ffi::c_char,
        len: ::core::ffi::c_int,
        target: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn xmlFreeURI(uri: xmlURIPtr);
}
pub type xmlChar = ::core::ffi::c_uchar;
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
pub type socklen_t = __socklen_t;
pub type __socket_type = ::core::ffi::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [::core::ffi::c_char; 118],
    pub __ss_align: ::core::ffi::c_ulong,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed = 263;
pub const IPPROTO_MPTCP: C2RustUnnamed = 262;
pub const IPPROTO_RAW: C2RustUnnamed = 255;
pub const IPPROTO_ETHERNET: C2RustUnnamed = 143;
pub const IPPROTO_MPLS: C2RustUnnamed = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed = 136;
pub const IPPROTO_SCTP: C2RustUnnamed = 132;
pub const IPPROTO_L2TP: C2RustUnnamed = 115;
pub const IPPROTO_COMP: C2RustUnnamed = 108;
pub const IPPROTO_PIM: C2RustUnnamed = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed = 94;
pub const IPPROTO_MTP: C2RustUnnamed = 92;
pub const IPPROTO_AH: C2RustUnnamed = 51;
pub const IPPROTO_ESP: C2RustUnnamed = 50;
pub const IPPROTO_GRE: C2RustUnnamed = 47;
pub const IPPROTO_RSVP: C2RustUnnamed = 46;
pub const IPPROTO_IPV6: C2RustUnnamed = 41;
pub const IPPROTO_DCCP: C2RustUnnamed = 33;
pub const IPPROTO_TP: C2RustUnnamed = 29;
pub const IPPROTO_IDP: C2RustUnnamed = 22;
pub const IPPROTO_UDP: C2RustUnnamed = 17;
pub const IPPROTO_PUP: C2RustUnnamed = 12;
pub const IPPROTO_EGP: C2RustUnnamed = 8;
pub const IPPROTO_TCP: C2RustUnnamed = 6;
pub const IPPROTO_IPIP: C2RustUnnamed = 4;
pub const IPPROTO_IGMP: C2RustUnnamed = 2;
pub const IPPROTO_ICMP: C2RustUnnamed = 1;
pub const IPPROTO_IP: C2RustUnnamed = 0;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [::core::ffi::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
    pub h_name: *mut ::core::ffi::c_char,
    pub h_aliases: *mut *mut ::core::ffi::c_char,
    pub h_addrtype: ::core::ffi::c_int,
    pub h_length: ::core::ffi::c_int,
    pub h_addr_list: *mut *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: ::core::ffi::c_int,
    pub ai_family: ::core::ffi::c_int,
    pub ai_socktype: ::core::ffi::c_int,
    pub ai_protocol: ::core::ffi::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut ::core::ffi::c_char,
    pub ai_next: *mut addrinfo,
}
pub type xmlFreeFunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type xmlMallocFunc = Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>;
pub type xmlStrdupFunc =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_char>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNode {
    pub _private: *mut ::core::ffi::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub content: *mut xmlChar,
    pub properties: *mut _xmlAttr,
    pub nsDef: *mut xmlNs,
    pub psvi: *mut ::core::ffi::c_void,
    pub line: ::core::ffi::c_ushort,
    pub extra: ::core::ffi::c_ushort,
}
pub type xmlNs = _xmlNs;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNs {
    pub next: *mut _xmlNs,
    pub type_0: xmlNsType,
    pub href: *const xmlChar,
    pub prefix: *const xmlChar,
    pub _private: *mut ::core::ffi::c_void,
    pub context: *mut _xmlDoc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDoc {
    pub _private: *mut ::core::ffi::c_void,
    pub type_0: xmlElementType,
    pub name: *mut ::core::ffi::c_char,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub compression: ::core::ffi::c_int,
    pub standalone: ::core::ffi::c_int,
    pub intSubset: *mut _xmlDtd,
    pub extSubset: *mut _xmlDtd,
    pub oldNs: *mut _xmlNs,
    pub version: *const xmlChar,
    pub encoding: *const xmlChar,
    pub ids: *mut ::core::ffi::c_void,
    pub refs: *mut ::core::ffi::c_void,
    pub URL: *const xmlChar,
    pub charset: ::core::ffi::c_int,
    pub dict: *mut _xmlDict,
    pub psvi: *mut ::core::ffi::c_void,
    pub parseFlags: ::core::ffi::c_int,
    pub properties: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDtd {
    pub _private: *mut ::core::ffi::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlDoc,
    pub next: *mut _xmlNode,
    pub prev: *mut _xmlNode,
    pub doc: *mut _xmlDoc,
    pub notations: *mut ::core::ffi::c_void,
    pub elements: *mut ::core::ffi::c_void,
    pub attributes: *mut ::core::ffi::c_void,
    pub entities: *mut ::core::ffi::c_void,
    pub ExternalID: *const xmlChar,
    pub SystemID: *const xmlChar,
    pub pentities: *mut ::core::ffi::c_void,
}
pub type xmlElementType = ::core::ffi::c_uint;
pub const XML_DOCB_DOCUMENT_NODE: xmlElementType = 21;
pub const XML_XINCLUDE_END: xmlElementType = 20;
pub const XML_XINCLUDE_START: xmlElementType = 19;
pub const XML_NAMESPACE_DECL: xmlElementType = 18;
pub const XML_ENTITY_DECL: xmlElementType = 17;
pub const XML_ATTRIBUTE_DECL: xmlElementType = 16;
pub const XML_ELEMENT_DECL: xmlElementType = 15;
pub const XML_DTD_NODE: xmlElementType = 14;
pub const XML_HTML_DOCUMENT_NODE: xmlElementType = 13;
pub const XML_NOTATION_NODE: xmlElementType = 12;
pub const XML_DOCUMENT_FRAG_NODE: xmlElementType = 11;
pub const XML_DOCUMENT_TYPE_NODE: xmlElementType = 10;
pub const XML_DOCUMENT_NODE: xmlElementType = 9;
pub const XML_COMMENT_NODE: xmlElementType = 8;
pub const XML_PI_NODE: xmlElementType = 7;
pub const XML_ENTITY_NODE: xmlElementType = 6;
pub const XML_ENTITY_REF_NODE: xmlElementType = 5;
pub const XML_CDATA_SECTION_NODE: xmlElementType = 4;
pub const XML_TEXT_NODE: xmlElementType = 3;
pub const XML_ATTRIBUTE_NODE: xmlElementType = 2;
pub const XML_ELEMENT_NODE: xmlElementType = 1;
pub type xmlNsType = xmlElementType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlAttr {
    pub _private: *mut ::core::ffi::c_void,
    pub type_0: xmlElementType,
    pub name: *const xmlChar,
    pub children: *mut _xmlNode,
    pub last: *mut _xmlNode,
    pub parent: *mut _xmlNode,
    pub next: *mut _xmlAttr,
    pub prev: *mut _xmlAttr,
    pub doc: *mut _xmlDoc,
    pub ns: *mut xmlNs,
    pub atype: xmlAttributeType,
    pub psvi: *mut ::core::ffi::c_void,
}
pub type xmlAttributeType = ::core::ffi::c_uint;
pub const XML_ATTRIBUTE_NOTATION: xmlAttributeType = 10;
pub const XML_ATTRIBUTE_ENUMERATION: xmlAttributeType = 9;
pub const XML_ATTRIBUTE_NMTOKENS: xmlAttributeType = 8;
pub const XML_ATTRIBUTE_NMTOKEN: xmlAttributeType = 7;
pub const XML_ATTRIBUTE_ENTITIES: xmlAttributeType = 6;
pub const XML_ATTRIBUTE_ENTITY: xmlAttributeType = 5;
pub const XML_ATTRIBUTE_IDREFS: xmlAttributeType = 4;
pub const XML_ATTRIBUTE_IDREF: xmlAttributeType = 3;
pub const XML_ATTRIBUTE_ID: xmlAttributeType = 2;
pub const XML_ATTRIBUTE_CDATA: xmlAttributeType = 1;
pub type xmlNodePtr = *mut xmlNode;
pub type xmlNode = _xmlNode;
pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
pub const XML_FROM_URI: C2RustUnnamed_1 = 30;
pub const XML_FROM_BUFFER: C2RustUnnamed_1 = 29;
pub const XML_FROM_SCHEMATRONV: C2RustUnnamed_1 = 28;
pub const XML_FROM_I18N: C2RustUnnamed_1 = 27;
pub const XML_FROM_MODULE: C2RustUnnamed_1 = 26;
pub const XML_FROM_WRITER: C2RustUnnamed_1 = 25;
pub const XML_FROM_CHECK: C2RustUnnamed_1 = 24;
pub const XML_FROM_VALID: C2RustUnnamed_1 = 23;
pub const XML_FROM_XSLT: C2RustUnnamed_1 = 22;
pub const XML_FROM_C14N: C2RustUnnamed_1 = 21;
pub const XML_FROM_CATALOG: C2RustUnnamed_1 = 20;
pub const XML_FROM_RELAXNGV: C2RustUnnamed_1 = 19;
pub const XML_FROM_RELAXNGP: C2RustUnnamed_1 = 18;
pub const XML_FROM_SCHEMASV: C2RustUnnamed_1 = 17;
pub const XML_FROM_SCHEMASP: C2RustUnnamed_1 = 16;
pub const XML_FROM_DATATYPE: C2RustUnnamed_1 = 15;
pub const XML_FROM_REGEXP: C2RustUnnamed_1 = 14;
pub const XML_FROM_XPOINTER: C2RustUnnamed_1 = 13;
pub const XML_FROM_XPATH: C2RustUnnamed_1 = 12;
pub const XML_FROM_XINCLUDE: C2RustUnnamed_1 = 11;
pub const XML_FROM_HTTP: C2RustUnnamed_1 = 10;
pub const XML_FROM_FTP: C2RustUnnamed_1 = 9;
pub const XML_FROM_IO: C2RustUnnamed_1 = 8;
pub const XML_FROM_OUTPUT: C2RustUnnamed_1 = 7;
pub const XML_FROM_MEMORY: C2RustUnnamed_1 = 6;
pub const XML_FROM_HTML: C2RustUnnamed_1 = 5;
pub const XML_FROM_DTD: C2RustUnnamed_1 = 4;
pub const XML_FROM_NAMESPACE: C2RustUnnamed_1 = 3;
pub const XML_FROM_TREE: C2RustUnnamed_1 = 2;
pub const XML_FROM_PARSER: C2RustUnnamed_1 = 1;
pub const XML_FROM_NONE: C2RustUnnamed_1 = 0;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const XML_BUF_OVERFLOW: C2RustUnnamed_2 = 7000;
pub const XML_I18N_NO_OUTPUT: C2RustUnnamed_2 = 6004;
pub const XML_I18N_CONV_FAILED: C2RustUnnamed_2 = 6003;
pub const XML_I18N_EXCESS_HANDLER: C2RustUnnamed_2 = 6002;
pub const XML_I18N_NO_HANDLER: C2RustUnnamed_2 = 6001;
pub const XML_I18N_NO_NAME: C2RustUnnamed_2 = 6000;
pub const XML_CHECK_NAME_NOT_NULL: C2RustUnnamed_2 = 5037;
pub const XML_CHECK_WRONG_NAME: C2RustUnnamed_2 = 5036;
pub const XML_CHECK_OUTSIDE_DICT: C2RustUnnamed_2 = 5035;
pub const XML_CHECK_NOT_NCNAME: C2RustUnnamed_2 = 5034;
pub const XML_CHECK_NO_DICT: C2RustUnnamed_2 = 5033;
pub const XML_CHECK_NOT_UTF8: C2RustUnnamed_2 = 5032;
pub const XML_CHECK_NS_ANCESTOR: C2RustUnnamed_2 = 5031;
pub const XML_CHECK_NS_SCOPE: C2RustUnnamed_2 = 5030;
pub const XML_CHECK_WRONG_PARENT: C2RustUnnamed_2 = 5029;
pub const XML_CHECK_NO_HREF: C2RustUnnamed_2 = 5028;
pub const XML_CHECK_NOT_NS_DECL: C2RustUnnamed_2 = 5027;
pub const XML_CHECK_NOT_ENTITY_DECL: C2RustUnnamed_2 = 5026;
pub const XML_CHECK_NOT_ELEM_DECL: C2RustUnnamed_2 = 5025;
pub const XML_CHECK_NOT_ATTR_DECL: C2RustUnnamed_2 = 5024;
pub const XML_CHECK_NOT_ATTR: C2RustUnnamed_2 = 5023;
pub const XML_CHECK_NOT_DTD: C2RustUnnamed_2 = 5022;
pub const XML_CHECK_WRONG_NEXT: C2RustUnnamed_2 = 5021;
pub const XML_CHECK_NO_NEXT: C2RustUnnamed_2 = 5020;
pub const XML_CHECK_WRONG_PREV: C2RustUnnamed_2 = 5019;
pub const XML_CHECK_NO_PREV: C2RustUnnamed_2 = 5018;
pub const XML_CHECK_WRONG_DOC: C2RustUnnamed_2 = 5017;
pub const XML_CHECK_NO_ELEM: C2RustUnnamed_2 = 5016;
pub const XML_CHECK_NO_NAME: C2RustUnnamed_2 = 5015;
pub const XML_CHECK_NO_DOC: C2RustUnnamed_2 = 5014;
pub const XML_CHECK_NO_PARENT: C2RustUnnamed_2 = 5013;
pub const XML_CHECK_ENTITY_TYPE: C2RustUnnamed_2 = 5012;
pub const XML_CHECK_UNKNOWN_NODE: C2RustUnnamed_2 = 5011;
pub const XML_CHECK_FOUND_NOTATION: C2RustUnnamed_2 = 5010;
pub const XML_CHECK_FOUND_FRAGMENT: C2RustUnnamed_2 = 5009;
pub const XML_CHECK_FOUND_DOCTYPE: C2RustUnnamed_2 = 5008;
pub const XML_CHECK_FOUND_COMMENT: C2RustUnnamed_2 = 5007;
pub const XML_CHECK_FOUND_PI: C2RustUnnamed_2 = 5006;
pub const XML_CHECK_FOUND_ENTITY: C2RustUnnamed_2 = 5005;
pub const XML_CHECK_FOUND_ENTITYREF: C2RustUnnamed_2 = 5004;
pub const XML_CHECK_FOUND_CDATA: C2RustUnnamed_2 = 5003;
pub const XML_CHECK_FOUND_TEXT: C2RustUnnamed_2 = 5002;
pub const XML_CHECK_FOUND_ATTRIBUTE: C2RustUnnamed_2 = 5001;
pub const XML_CHECK_FOUND_ELEMENT: C2RustUnnamed_2 = 5000;
pub const XML_MODULE_CLOSE: C2RustUnnamed_2 = 4901;
pub const XML_MODULE_OPEN: C2RustUnnamed_2 = 4900;
pub const XML_SCHEMATRONV_REPORT: C2RustUnnamed_2 = 4001;
pub const XML_SCHEMATRONV_ASSERT: C2RustUnnamed_2 = 4000;
pub const XML_SCHEMAP_COS_ALL_LIMITED: C2RustUnnamed_2 = 3091;
pub const XML_SCHEMAP_A_PROPS_CORRECT_3: C2RustUnnamed_2 = 3090;
pub const XML_SCHEMAP_AU_PROPS_CORRECT: C2RustUnnamed_2 = 3089;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_2: C2RustUnnamed_2 = 3088;
pub const XML_SCHEMAP_AG_PROPS_CORRECT: C2RustUnnamed_2 = 3087;
pub const XML_SCHEMAP_WARN_ATTR_POINTLESS_PROH: C2RustUnnamed_2 = 3086;
pub const XML_SCHEMAP_WARN_ATTR_REDECL_PROH: C2RustUnnamed_2 = 3085;
pub const XML_SCHEMAP_WARN_UNLOCATED_SCHEMA: C2RustUnnamed_2 = 3084;
pub const XML_SCHEMAP_WARN_SKIP_SCHEMA: C2RustUnnamed_2 = 3083;
pub const XML_SCHEMAP_SRC_IMPORT: C2RustUnnamed_2 = 3082;
pub const XML_SCHEMAP_SRC_REDEFINE: C2RustUnnamed_2 = 3081;
pub const XML_SCHEMAP_C_PROPS_CORRECT: C2RustUnnamed_2 = 3080;
pub const XML_SCHEMAP_A_PROPS_CORRECT_2: C2RustUnnamed_2 = 3079;
pub const XML_SCHEMAP_AU_PROPS_CORRECT_2: C2RustUnnamed_2 = 3078;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_3: C2RustUnnamed_2 = 3077;
pub const XML_SCHEMAP_SRC_CT_1: C2RustUnnamed_2 = 3076;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_2: C2RustUnnamed_2 = 3075;
pub const XML_SCHEMAP_MG_PROPS_CORRECT_1: C2RustUnnamed_2 = 3074;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_3: C2RustUnnamed_2 = 3073;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_2: C2RustUnnamed_2 = 3072;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_GROUP_1: C2RustUnnamed_2 = 3071;
pub const XML_SCHEMAP_NOT_DETERMINISTIC: C2RustUnnamed_2 = 3070;
pub const XML_SCHEMAP_INTERNAL: C2RustUnnamed_2 = 3069;
pub const XML_SCHEMAP_SRC_IMPORT_2_2: C2RustUnnamed_2 = 3068;
pub const XML_SCHEMAP_SRC_IMPORT_2_1: C2RustUnnamed_2 = 3067;
pub const XML_SCHEMAP_SRC_IMPORT_2: C2RustUnnamed_2 = 3066;
pub const XML_SCHEMAP_SRC_IMPORT_1_2: C2RustUnnamed_2 = 3065;
pub const XML_SCHEMAP_SRC_IMPORT_1_1: C2RustUnnamed_2 = 3064;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_1: C2RustUnnamed_2 = 3063;
pub const XML_SCHEMAP_CVC_SIMPLE_TYPE: C2RustUnnamed_2 = 3062;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_2: C2RustUnnamed_2 = 3061;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_2_1: C2RustUnnamed_2 = 3060;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_2_1: C2RustUnnamed_2 = 3059;
pub const XML_SCHEMAP_COS_VALID_DEFAULT_1: C2RustUnnamed_2 = 3058;
pub const XML_SCHEMAP_NO_XSI: C2RustUnnamed_2 = 3057;
pub const XML_SCHEMAP_NO_XMLNS: C2RustUnnamed_2 = 3056;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_4: C2RustUnnamed_2 = 3055;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_2: C2RustUnnamed_2 = 3054;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_3_1: C2RustUnnamed_2 = 3053;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_2: C2RustUnnamed_2 = 3052;
pub const XML_SCHEMAP_SRC_ATTRIBUTE_1: C2RustUnnamed_2 = 3051;
pub const XML_SCHEMAP_SRC_INCLUDE: C2RustUnnamed_2 = 3050;
pub const XML_SCHEMAP_E_PROPS_CORRECT_6: C2RustUnnamed_2 = 3049;
pub const XML_SCHEMAP_E_PROPS_CORRECT_5: C2RustUnnamed_2 = 3048;
pub const XML_SCHEMAP_E_PROPS_CORRECT_4: C2RustUnnamed_2 = 3047;
pub const XML_SCHEMAP_E_PROPS_CORRECT_3: C2RustUnnamed_2 = 3046;
pub const XML_SCHEMAP_E_PROPS_CORRECT_2: C2RustUnnamed_2 = 3045;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_2: C2RustUnnamed_2 = 3044;
pub const XML_SCHEMAP_P_PROPS_CORRECT_2_1: C2RustUnnamed_2 = 3043;
pub const XML_SCHEMAP_P_PROPS_CORRECT_1: C2RustUnnamed_2 = 3042;
pub const XML_SCHEMAP_SRC_ELEMENT_3: C2RustUnnamed_2 = 3041;
pub const XML_SCHEMAP_SRC_ELEMENT_2_2: C2RustUnnamed_2 = 3040;
pub const XML_SCHEMAP_SRC_ELEMENT_2_1: C2RustUnnamed_2 = 3039;
pub const XML_SCHEMAP_SRC_ELEMENT_1: C2RustUnnamed_2 = 3038;
pub const XML_SCHEMAP_S4S_ATTR_INVALID_VALUE: C2RustUnnamed_2 = 3037;
pub const XML_SCHEMAP_S4S_ATTR_MISSING: C2RustUnnamed_2 = 3036;
pub const XML_SCHEMAP_S4S_ATTR_NOT_ALLOWED: C2RustUnnamed_2 = 3035;
pub const XML_SCHEMAP_S4S_ELEM_MISSING: C2RustUnnamed_2 = 3034;
pub const XML_SCHEMAP_S4S_ELEM_NOT_ALLOWED: C2RustUnnamed_2 = 3033;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_2: C2RustUnnamed_2 = 3032;
pub const XML_SCHEMAP_COS_ST_DERIVED_OK_2_1: C2RustUnnamed_2 = 3031;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_5: C2RustUnnamed_2 = 3030;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_4: C2RustUnnamed_2 = 3029;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_3: C2RustUnnamed_2 = 3028;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_1: C2RustUnnamed_2 = 3027;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_2_2: C2RustUnnamed_2 = 3026;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1_2: C2RustUnnamed_2 = 3025;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_3_1: C2RustUnnamed_2 = 3024;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_3_1: C2RustUnnamed_2 = 3023;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_5: C2RustUnnamed_2 = 3022;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_4: C2RustUnnamed_2 = 3021;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_3: C2RustUnnamed_2 = 3020;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_2: C2RustUnnamed_2 = 3019;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_2_1: C2RustUnnamed_2 = 3018;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_2: C2RustUnnamed_2 = 3017;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_3_1_1: C2RustUnnamed_2 = 3016;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_2_1: C2RustUnnamed_2 = 3015;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_2: C2RustUnnamed_2 = 3014;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_3_1: C2RustUnnamed_2 = 3013;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_2: C2RustUnnamed_2 = 3012;
pub const XML_SCHEMAP_COS_ST_RESTRICTS_1_1: C2RustUnnamed_2 = 3011;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_3: C2RustUnnamed_2 = 3010;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_2: C2RustUnnamed_2 = 3009;
pub const XML_SCHEMAP_ST_PROPS_CORRECT_1: C2RustUnnamed_2 = 3008;
pub const XML_SCHEMAP_SRC_UNION_MEMBERTYPES_OR_SIMPLETYPES: C2RustUnnamed_2 = 3007;
pub const XML_SCHEMAP_SRC_LIST_ITEMTYPE_OR_SIMPLETYPE: C2RustUnnamed_2 = 3006;
pub const XML_SCHEMAP_SRC_RESTRICTION_BASE_OR_SIMPLETYPE: C2RustUnnamed_2 = 3005;
pub const XML_SCHEMAP_SRC_RESOLVE: C2RustUnnamed_2 = 3004;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_4: C2RustUnnamed_2 = 3003;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_3: C2RustUnnamed_2 = 3002;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_2: C2RustUnnamed_2 = 3001;
pub const XML_SCHEMAP_SRC_SIMPLE_TYPE_1: C2RustUnnamed_2 = 3000;
pub const XML_HTTP_UNKNOWN_HOST: C2RustUnnamed_2 = 2022;
pub const XML_HTTP_USE_IP: C2RustUnnamed_2 = 2021;
pub const XML_HTTP_URL_SYNTAX: C2RustUnnamed_2 = 2020;
pub const XML_FTP_URL_SYNTAX: C2RustUnnamed_2 = 2003;
pub const XML_FTP_ACCNT: C2RustUnnamed_2 = 2002;
pub const XML_FTP_EPSV_ANSWER: C2RustUnnamed_2 = 2001;
pub const XML_FTP_PASV_ANSWER: C2RustUnnamed_2 = 2000;
pub const XML_C14N_RELATIVE_NAMESPACE: C2RustUnnamed_2 = 1955;
pub const XML_C14N_UNKNOW_NODE: C2RustUnnamed_2 = 1954;
pub const XML_C14N_INVALID_NODE: C2RustUnnamed_2 = 1953;
pub const XML_C14N_CREATE_STACK: C2RustUnnamed_2 = 1952;
pub const XML_C14N_REQUIRES_UTF8: C2RustUnnamed_2 = 1951;
pub const XML_C14N_CREATE_CTXT: C2RustUnnamed_2 = 1950;
pub const XML_XPTR_EXTRA_OBJECTS: C2RustUnnamed_2 = 1903;
pub const XML_XPTR_EVAL_FAILED: C2RustUnnamed_2 = 1902;
pub const XML_XPTR_CHILDSEQ_START: C2RustUnnamed_2 = 1901;
pub const XML_XPTR_UNKNOWN_SCHEME: C2RustUnnamed_2 = 1900;
pub const XML_SCHEMAV_MISC: C2RustUnnamed_2 = 1879;
pub const XML_SCHEMAV_CVC_WILDCARD: C2RustUnnamed_2 = 1878;
pub const XML_SCHEMAV_CVC_IDC: C2RustUnnamed_2 = 1877;
pub const XML_SCHEMAV_CVC_TYPE_2: C2RustUnnamed_2 = 1876;
pub const XML_SCHEMAV_CVC_TYPE_1: C2RustUnnamed_2 = 1875;
pub const XML_SCHEMAV_CVC_AU: C2RustUnnamed_2 = 1874;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_1: C2RustUnnamed_2 = 1873;
pub const XML_SCHEMAV_DOCUMENT_ELEMENT_MISSING: C2RustUnnamed_2 = 1872;
pub const XML_SCHEMAV_ELEMENT_CONTENT: C2RustUnnamed_2 = 1871;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_2: C2RustUnnamed_2 = 1870;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_5_1: C2RustUnnamed_2 = 1869;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_4: C2RustUnnamed_2 = 1868;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_2: C2RustUnnamed_2 = 1867;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_2_1: C2RustUnnamed_2 = 1866;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_3_1: C2RustUnnamed_2 = 1865;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_4: C2RustUnnamed_2 = 1864;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_3: C2RustUnnamed_2 = 1863;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_2: C2RustUnnamed_2 = 1862;
pub const XML_SCHEMAV_CVC_ATTRIBUTE_1: C2RustUnnamed_2 = 1861;
pub const XML_SCHEMAV_CVC_ELT_7: C2RustUnnamed_2 = 1860;
pub const XML_SCHEMAV_CVC_ELT_6: C2RustUnnamed_2 = 1859;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_2: C2RustUnnamed_2 = 1858;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_2_1: C2RustUnnamed_2 = 1857;
pub const XML_SCHEMAV_CVC_ELT_5_2_2_1: C2RustUnnamed_2 = 1856;
pub const XML_SCHEMAV_CVC_ELT_5_2_1: C2RustUnnamed_2 = 1855;
pub const XML_SCHEMAV_CVC_ELT_5_1_2: C2RustUnnamed_2 = 1854;
pub const XML_SCHEMAV_CVC_ELT_5_1_1: C2RustUnnamed_2 = 1853;
pub const XML_SCHEMAV_CVC_ELT_4_3: C2RustUnnamed_2 = 1852;
pub const XML_SCHEMAV_CVC_ELT_4_2: C2RustUnnamed_2 = 1851;
pub const XML_SCHEMAV_CVC_ELT_4_1: C2RustUnnamed_2 = 1850;
pub const XML_SCHEMAV_CVC_ELT_3_2_2: C2RustUnnamed_2 = 1849;
pub const XML_SCHEMAV_CVC_ELT_3_2_1: C2RustUnnamed_2 = 1848;
pub const XML_SCHEMAV_CVC_ELT_3_1: C2RustUnnamed_2 = 1847;
pub const XML_SCHEMAV_CVC_ELT_2: C2RustUnnamed_2 = 1846;
pub const XML_SCHEMAV_CVC_ELT_1: C2RustUnnamed_2 = 1845;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_4: C2RustUnnamed_2 = 1844;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_3: C2RustUnnamed_2 = 1843;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_2: C2RustUnnamed_2 = 1842;
pub const XML_SCHEMAV_CVC_COMPLEX_TYPE_2_1: C2RustUnnamed_2 = 1841;
pub const XML_SCHEMAV_CVC_ENUMERATION_VALID: C2RustUnnamed_2 = 1840;
pub const XML_SCHEMAV_CVC_PATTERN_VALID: C2RustUnnamed_2 = 1839;
pub const XML_SCHEMAV_CVC_FRACTIONDIGITS_VALID: C2RustUnnamed_2 = 1838;
pub const XML_SCHEMAV_CVC_TOTALDIGITS_VALID: C2RustUnnamed_2 = 1837;
pub const XML_SCHEMAV_CVC_MAXEXCLUSIVE_VALID: C2RustUnnamed_2 = 1836;
pub const XML_SCHEMAV_CVC_MINEXCLUSIVE_VALID: C2RustUnnamed_2 = 1835;
pub const XML_SCHEMAV_CVC_MAXINCLUSIVE_VALID: C2RustUnnamed_2 = 1834;
pub const XML_SCHEMAV_CVC_MININCLUSIVE_VALID: C2RustUnnamed_2 = 1833;
pub const XML_SCHEMAV_CVC_MAXLENGTH_VALID: C2RustUnnamed_2 = 1832;
pub const XML_SCHEMAV_CVC_MINLENGTH_VALID: C2RustUnnamed_2 = 1831;
pub const XML_SCHEMAV_CVC_LENGTH_VALID: C2RustUnnamed_2 = 1830;
pub const XML_SCHEMAV_CVC_FACET_VALID: C2RustUnnamed_2 = 1829;
pub const XML_SCHEMAV_CVC_TYPE_3_1_2: C2RustUnnamed_2 = 1828;
pub const XML_SCHEMAV_CVC_TYPE_3_1_1: C2RustUnnamed_2 = 1827;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_3: C2RustUnnamed_2 = 1826;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_2: C2RustUnnamed_2 = 1825;
pub const XML_SCHEMAV_CVC_DATATYPE_VALID_1_2_1: C2RustUnnamed_2 = 1824;
pub const XML_SCHEMAV_FACET: C2RustUnnamed_2 = 1823;
pub const XML_SCHEMAV_VALUE: C2RustUnnamed_2 = 1822;
pub const XML_SCHEMAV_ATTRINVALID: C2RustUnnamed_2 = 1821;
pub const XML_SCHEMAV_ATTRUNKNOWN: C2RustUnnamed_2 = 1820;
pub const XML_SCHEMAV_NOTSIMPLE: C2RustUnnamed_2 = 1819;
pub const XML_SCHEMAV_INTERNAL: C2RustUnnamed_2 = 1818;
pub const XML_SCHEMAV_CONSTRUCT: C2RustUnnamed_2 = 1817;
pub const XML_SCHEMAV_NOTDETERMINIST: C2RustUnnamed_2 = 1816;
pub const XML_SCHEMAV_INVALIDELEM: C2RustUnnamed_2 = 1815;
pub const XML_SCHEMAV_INVALIDATTR: C2RustUnnamed_2 = 1814;
pub const XML_SCHEMAV_EXTRACONTENT: C2RustUnnamed_2 = 1813;
pub const XML_SCHEMAV_NOTNILLABLE: C2RustUnnamed_2 = 1812;
pub const XML_SCHEMAV_HAVEDEFAULT: C2RustUnnamed_2 = 1811;
pub const XML_SCHEMAV_ELEMCONT: C2RustUnnamed_2 = 1810;
pub const XML_SCHEMAV_NOTEMPTY: C2RustUnnamed_2 = 1809;
pub const XML_SCHEMAV_ISABSTRACT: C2RustUnnamed_2 = 1808;
pub const XML_SCHEMAV_NOROLLBACK: C2RustUnnamed_2 = 1807;
pub const XML_SCHEMAV_NOTYPE: C2RustUnnamed_2 = 1806;
pub const XML_SCHEMAV_WRONGELEM: C2RustUnnamed_2 = 1805;
pub const XML_SCHEMAV_MISSING: C2RustUnnamed_2 = 1804;
pub const XML_SCHEMAV_NOTTOPLEVEL: C2RustUnnamed_2 = 1803;
pub const XML_SCHEMAV_UNDECLAREDELEM: C2RustUnnamed_2 = 1802;
pub const XML_SCHEMAV_NOROOT: C2RustUnnamed_2 = 1801;
pub const XML_SCHEMAP_COS_CT_EXTENDS_1_3: C2RustUnnamed_2 = 1800;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_3: C2RustUnnamed_2 = 1799;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_2: C2RustUnnamed_2 = 1798;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_4_1: C2RustUnnamed_2 = 1797;
pub const XML_SCHEMAP_SRC_IMPORT_3_2: C2RustUnnamed_2 = 1796;
pub const XML_SCHEMAP_SRC_IMPORT_3_1: C2RustUnnamed_2 = 1795;
pub const XML_SCHEMAP_UNION_NOT_EXPRESSIBLE: C2RustUnnamed_2 = 1794;
pub const XML_SCHEMAP_INTERSECTION_NOT_EXPRESSIBLE: C2RustUnnamed_2 = 1793;
pub const XML_SCHEMAP_WILDCARD_INVALID_NS_MEMBER: C2RustUnnamed_2 = 1792;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_3: C2RustUnnamed_2 = 1791;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_2: C2RustUnnamed_2 = 1790;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_2: C2RustUnnamed_2 = 1789;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_2_1_1: C2RustUnnamed_2 = 1788;
pub const XML_SCHEMAP_DERIVATION_OK_RESTRICTION_1: C2RustUnnamed_2 = 1787;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_5: C2RustUnnamed_2 = 1786;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_4: C2RustUnnamed_2 = 1785;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_3: C2RustUnnamed_2 = 1784;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_2: C2RustUnnamed_2 = 1783;
pub const XML_SCHEMAP_CT_PROPS_CORRECT_1: C2RustUnnamed_2 = 1782;
pub const XML_SCHEMAP_REF_AND_CONTENT: C2RustUnnamed_2 = 1781;
pub const XML_SCHEMAP_INVALID_ATTR_NAME: C2RustUnnamed_2 = 1780;
pub const XML_SCHEMAP_MISSING_SIMPLETYPE_CHILD: C2RustUnnamed_2 = 1779;
pub const XML_SCHEMAP_INVALID_ATTR_INLINE_COMBINATION: C2RustUnnamed_2 = 1778;
pub const XML_SCHEMAP_INVALID_ATTR_COMBINATION: C2RustUnnamed_2 = 1777;
pub const XML_SCHEMAP_SUPERNUMEROUS_LIST_ITEM_TYPE: C2RustUnnamed_2 = 1776;
pub const XML_SCHEMAP_RECURSIVE: C2RustUnnamed_2 = 1775;
pub const XML_SCHEMAP_INVALID_ATTR_USE: C2RustUnnamed_2 = 1774;
pub const XML_SCHEMAP_UNKNOWN_MEMBER_TYPE: C2RustUnnamed_2 = 1773;
pub const XML_SCHEMAP_NOT_SCHEMA: C2RustUnnamed_2 = 1772;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NO_URI: C2RustUnnamed_2 = 1771;
pub const XML_SCHEMAP_INCLUDE_SCHEMA_NOT_URI: C2RustUnnamed_2 = 1770;
pub const XML_SCHEMAP_UNKNOWN_INCLUDE_CHILD: C2RustUnnamed_2 = 1769;
pub const XML_SCHEMAP_DEF_AND_PREFIX: C2RustUnnamed_2 = 1768;
pub const XML_SCHEMAP_UNKNOWN_PREFIX: C2RustUnnamed_2 = 1767;
pub const XML_SCHEMAP_FAILED_PARSE: C2RustUnnamed_2 = 1766;
pub const XML_SCHEMAP_REDEFINED_NOTATION: C2RustUnnamed_2 = 1765;
pub const XML_SCHEMAP_REDEFINED_ATTR: C2RustUnnamed_2 = 1764;
pub const XML_SCHEMAP_REDEFINED_ATTRGROUP: C2RustUnnamed_2 = 1763;
pub const XML_SCHEMAP_REDEFINED_ELEMENT: C2RustUnnamed_2 = 1762;
pub const XML_SCHEMAP_REDEFINED_TYPE: C2RustUnnamed_2 = 1761;
pub const XML_SCHEMAP_REDEFINED_GROUP: C2RustUnnamed_2 = 1760;
pub const XML_SCHEMAP_NOROOT: C2RustUnnamed_2 = 1759;
pub const XML_SCHEMAP_NOTHING_TO_PARSE: C2RustUnnamed_2 = 1758;
pub const XML_SCHEMAP_FAILED_LOAD: C2RustUnnamed_2 = 1757;
pub const XML_SCHEMAP_REGEXP_INVALID: C2RustUnnamed_2 = 1756;
pub const XML_SCHEMAP_ELEM_DEFAULT_FIXED: C2RustUnnamed_2 = 1755;
pub const XML_SCHEMAP_UNKNOWN_UNION_CHILD: C2RustUnnamed_2 = 1754;
pub const XML_SCHEMAP_UNKNOWN_TYPE: C2RustUnnamed_2 = 1753;
pub const XML_SCHEMAP_UNKNOWN_SIMPLETYPE_CHILD: C2RustUnnamed_2 = 1752;
pub const XML_SCHEMAP_UNKNOWN_SIMPLECONTENT_CHILD: C2RustUnnamed_2 = 1751;
pub const XML_SCHEMAP_UNKNOWN_SEQUENCE_CHILD: C2RustUnnamed_2 = 1750;
pub const XML_SCHEMAP_UNKNOWN_SCHEMAS_CHILD: C2RustUnnamed_2 = 1749;
pub const XML_SCHEMAP_UNKNOWN_RESTRICTION_CHILD: C2RustUnnamed_2 = 1748;
pub const XML_SCHEMAP_UNKNOWN_REF: C2RustUnnamed_2 = 1747;
pub const XML_SCHEMAP_UNKNOWN_PROCESSCONTENT_CHILD: C2RustUnnamed_2 = 1746;
pub const XML_SCHEMAP_UNKNOWN_NOTATION_CHILD: C2RustUnnamed_2 = 1745;
pub const XML_SCHEMAP_UNKNOWN_LIST_CHILD: C2RustUnnamed_2 = 1744;
pub const XML_SCHEMAP_UNKNOWN_IMPORT_CHILD: C2RustUnnamed_2 = 1743;
pub const XML_SCHEMAP_UNKNOWN_GROUP_CHILD: C2RustUnnamed_2 = 1742;
pub const XML_SCHEMAP_UNKNOWN_FACET_TYPE: C2RustUnnamed_2 = 1741;
pub const XML_SCHEMAP_UNKNOWN_FACET_CHILD: C2RustUnnamed_2 = 1740;
pub const XML_SCHEMAP_UNKNOWN_EXTENSION_CHILD: C2RustUnnamed_2 = 1739;
pub const XML_SCHEMAP_UNKNOWN_ELEM_CHILD: C2RustUnnamed_2 = 1738;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXTYPE_CHILD: C2RustUnnamed_2 = 1737;
pub const XML_SCHEMAP_UNKNOWN_COMPLEXCONTENT_CHILD: C2RustUnnamed_2 = 1736;
pub const XML_SCHEMAP_UNKNOWN_CHOICE_CHILD: C2RustUnnamed_2 = 1735;
pub const XML_SCHEMAP_UNKNOWN_BASE_TYPE: C2RustUnnamed_2 = 1734;
pub const XML_SCHEMAP_UNKNOWN_ATTRIBUTE_GROUP: C2RustUnnamed_2 = 1733;
pub const XML_SCHEMAP_UNKNOWN_ATTRGRP_CHILD: C2RustUnnamed_2 = 1732;
pub const XML_SCHEMAP_UNKNOWN_ATTR_CHILD: C2RustUnnamed_2 = 1731;
pub const XML_SCHEMAP_UNKNOWN_ANYATTRIBUTE_CHILD: C2RustUnnamed_2 = 1730;
pub const XML_SCHEMAP_UNKNOWN_ALL_CHILD: C2RustUnnamed_2 = 1729;
pub const XML_SCHEMAP_TYPE_AND_SUBTYPE: C2RustUnnamed_2 = 1728;
pub const XML_SCHEMAP_SIMPLETYPE_NONAME: C2RustUnnamed_2 = 1727;
pub const XML_SCHEMAP_RESTRICTION_NONAME_NOREF: C2RustUnnamed_2 = 1726;
pub const XML_SCHEMAP_REF_AND_SUBTYPE: C2RustUnnamed_2 = 1725;
pub const XML_SCHEMAP_NOTYPE_NOREF: C2RustUnnamed_2 = 1724;
pub const XML_SCHEMAP_NOTATION_NO_NAME: C2RustUnnamed_2 = 1723;
pub const XML_SCHEMAP_NOATTR_NOREF: C2RustUnnamed_2 = 1722;
pub const XML_SCHEMAP_INVALID_WHITE_SPACE: C2RustUnnamed_2 = 1721;
pub const XML_SCHEMAP_INVALID_REF_AND_SUBTYPE: C2RustUnnamed_2 = 1720;
pub const XML_SCHEMAP_INVALID_MINOCCURS: C2RustUnnamed_2 = 1719;
pub const XML_SCHEMAP_INVALID_MAXOCCURS: C2RustUnnamed_2 = 1718;
pub const XML_SCHEMAP_INVALID_FACET_VALUE: C2RustUnnamed_2 = 1717;
pub const XML_SCHEMAP_INVALID_FACET: C2RustUnnamed_2 = 1716;
pub const XML_SCHEMAP_INVALID_ENUM: C2RustUnnamed_2 = 1715;
pub const XML_SCHEMAP_INVALID_BOOLEAN: C2RustUnnamed_2 = 1714;
pub const XML_SCHEMAP_IMPORT_SCHEMA_NOT_URI: C2RustUnnamed_2 = 1713;
pub const XML_SCHEMAP_IMPORT_REDEFINE_NSNAME: C2RustUnnamed_2 = 1712;
pub const XML_SCHEMAP_IMPORT_NAMESPACE_NOT_URI: C2RustUnnamed_2 = 1711;
pub const XML_SCHEMAP_GROUP_NONAME_NOREF: C2RustUnnamed_2 = 1710;
pub const XML_SCHEMAP_FAILED_BUILD_IMPORT: C2RustUnnamed_2 = 1709;
pub const XML_SCHEMAP_FACET_NO_VALUE: C2RustUnnamed_2 = 1708;
pub const XML_SCHEMAP_EXTENSION_NO_BASE: C2RustUnnamed_2 = 1707;
pub const XML_SCHEMAP_ELEM_NONAME_NOREF: C2RustUnnamed_2 = 1706;
pub const XML_SCHEMAP_ELEMFORMDEFAULT_VALUE: C2RustUnnamed_2 = 1705;
pub const XML_SCHEMAP_COMPLEXTYPE_NONAME_NOREF: C2RustUnnamed_2 = 1704;
pub const XML_SCHEMAP_ATTR_NONAME_NOREF: C2RustUnnamed_2 = 1703;
pub const XML_SCHEMAP_ATTRGRP_NONAME_NOREF: C2RustUnnamed_2 = 1702;
pub const XML_SCHEMAP_ATTRFORMDEFAULT_VALUE: C2RustUnnamed_2 = 1701;
pub const XML_SCHEMAP_PREFIX_UNDEFINED: C2RustUnnamed_2 = 1700;
pub const XML_CATALOG_RECURSION: C2RustUnnamed_2 = 1654;
pub const XML_CATALOG_NOT_CATALOG: C2RustUnnamed_2 = 1653;
pub const XML_CATALOG_PREFER_VALUE: C2RustUnnamed_2 = 1652;
pub const XML_CATALOG_ENTRY_BROKEN: C2RustUnnamed_2 = 1651;
pub const XML_CATALOG_MISSING_ATTR: C2RustUnnamed_2 = 1650;
pub const XML_XINCLUDE_FRAGMENT_ID: C2RustUnnamed_2 = 1618;
pub const XML_XINCLUDE_DEPRECATED_NS: C2RustUnnamed_2 = 1617;
pub const XML_XINCLUDE_FALLBACK_NOT_IN_INCLUDE: C2RustUnnamed_2 = 1616;
pub const XML_XINCLUDE_FALLBACKS_IN_INCLUDE: C2RustUnnamed_2 = 1615;
pub const XML_XINCLUDE_INCLUDE_IN_INCLUDE: C2RustUnnamed_2 = 1614;
pub const XML_XINCLUDE_XPTR_RESULT: C2RustUnnamed_2 = 1613;
pub const XML_XINCLUDE_XPTR_FAILED: C2RustUnnamed_2 = 1612;
pub const XML_XINCLUDE_MULTIPLE_ROOT: C2RustUnnamed_2 = 1611;
pub const XML_XINCLUDE_UNKNOWN_ENCODING: C2RustUnnamed_2 = 1610;
pub const XML_XINCLUDE_BUILD_FAILED: C2RustUnnamed_2 = 1609;
pub const XML_XINCLUDE_INVALID_CHAR: C2RustUnnamed_2 = 1608;
pub const XML_XINCLUDE_TEXT_DOCUMENT: C2RustUnnamed_2 = 1607;
pub const XML_XINCLUDE_TEXT_FRAGMENT: C2RustUnnamed_2 = 1606;
pub const XML_XINCLUDE_HREF_URI: C2RustUnnamed_2 = 1605;
pub const XML_XINCLUDE_NO_FALLBACK: C2RustUnnamed_2 = 1604;
pub const XML_XINCLUDE_NO_HREF: C2RustUnnamed_2 = 1603;
pub const XML_XINCLUDE_ENTITY_DEF_MISMATCH: C2RustUnnamed_2 = 1602;
pub const XML_XINCLUDE_PARSE_VALUE: C2RustUnnamed_2 = 1601;
pub const XML_XINCLUDE_RECURSION: C2RustUnnamed_2 = 1600;
pub const XML_IO_EAFNOSUPPORT: C2RustUnnamed_2 = 1556;
pub const XML_IO_EALREADY: C2RustUnnamed_2 = 1555;
pub const XML_IO_EADDRINUSE: C2RustUnnamed_2 = 1554;
pub const XML_IO_ENETUNREACH: C2RustUnnamed_2 = 1553;
pub const XML_IO_ECONNREFUSED: C2RustUnnamed_2 = 1552;
pub const XML_IO_EISCONN: C2RustUnnamed_2 = 1551;
pub const XML_IO_ENOTSOCK: C2RustUnnamed_2 = 1550;
pub const XML_IO_LOAD_ERROR: C2RustUnnamed_2 = 1549;
pub const XML_IO_BUFFER_FULL: C2RustUnnamed_2 = 1548;
pub const XML_IO_NO_INPUT: C2RustUnnamed_2 = 1547;
pub const XML_IO_WRITE: C2RustUnnamed_2 = 1546;
pub const XML_IO_FLUSH: C2RustUnnamed_2 = 1545;
pub const XML_IO_ENCODER: C2RustUnnamed_2 = 1544;
pub const XML_IO_NETWORK_ATTEMPT: C2RustUnnamed_2 = 1543;
pub const XML_IO_EXDEV: C2RustUnnamed_2 = 1542;
pub const XML_IO_ETIMEDOUT: C2RustUnnamed_2 = 1541;
pub const XML_IO_ESRCH: C2RustUnnamed_2 = 1540;
pub const XML_IO_ESPIPE: C2RustUnnamed_2 = 1539;
pub const XML_IO_EROFS: C2RustUnnamed_2 = 1538;
pub const XML_IO_ERANGE: C2RustUnnamed_2 = 1537;
pub const XML_IO_EPIPE: C2RustUnnamed_2 = 1536;
pub const XML_IO_EPERM: C2RustUnnamed_2 = 1535;
pub const XML_IO_ENXIO: C2RustUnnamed_2 = 1534;
pub const XML_IO_ENOTTY: C2RustUnnamed_2 = 1533;
pub const XML_IO_ENOTSUP: C2RustUnnamed_2 = 1532;
pub const XML_IO_ENOTEMPTY: C2RustUnnamed_2 = 1531;
pub const XML_IO_ENOTDIR: C2RustUnnamed_2 = 1530;
pub const XML_IO_ENOSYS: C2RustUnnamed_2 = 1529;
pub const XML_IO_ENOSPC: C2RustUnnamed_2 = 1528;
pub const XML_IO_ENOMEM: C2RustUnnamed_2 = 1527;
pub const XML_IO_ENOLCK: C2RustUnnamed_2 = 1526;
pub const XML_IO_ENOEXEC: C2RustUnnamed_2 = 1525;
pub const XML_IO_ENOENT: C2RustUnnamed_2 = 1524;
pub const XML_IO_ENODEV: C2RustUnnamed_2 = 1523;
pub const XML_IO_ENFILE: C2RustUnnamed_2 = 1522;
pub const XML_IO_ENAMETOOLONG: C2RustUnnamed_2 = 1521;
pub const XML_IO_EMSGSIZE: C2RustUnnamed_2 = 1520;
pub const XML_IO_EMLINK: C2RustUnnamed_2 = 1519;
pub const XML_IO_EMFILE: C2RustUnnamed_2 = 1518;
pub const XML_IO_EISDIR: C2RustUnnamed_2 = 1517;
pub const XML_IO_EIO: C2RustUnnamed_2 = 1516;
pub const XML_IO_EINVAL: C2RustUnnamed_2 = 1515;
pub const XML_IO_EINTR: C2RustUnnamed_2 = 1514;
pub const XML_IO_EINPROGRESS: C2RustUnnamed_2 = 1513;
pub const XML_IO_EFBIG: C2RustUnnamed_2 = 1512;
pub const XML_IO_EFAULT: C2RustUnnamed_2 = 1511;
pub const XML_IO_EEXIST: C2RustUnnamed_2 = 1510;
pub const XML_IO_EDOM: C2RustUnnamed_2 = 1509;
pub const XML_IO_EDEADLK: C2RustUnnamed_2 = 1508;
pub const XML_IO_ECHILD: C2RustUnnamed_2 = 1507;
pub const XML_IO_ECANCELED: C2RustUnnamed_2 = 1506;
pub const XML_IO_EBUSY: C2RustUnnamed_2 = 1505;
pub const XML_IO_EBADMSG: C2RustUnnamed_2 = 1504;
pub const XML_IO_EBADF: C2RustUnnamed_2 = 1503;
pub const XML_IO_EAGAIN: C2RustUnnamed_2 = 1502;
pub const XML_IO_EACCES: C2RustUnnamed_2 = 1501;
pub const XML_IO_UNKNOWN: C2RustUnnamed_2 = 1500;
pub const XML_REGEXP_COMPILE_ERROR: C2RustUnnamed_2 = 1450;
pub const XML_SAVE_UNKNOWN_ENCODING: C2RustUnnamed_2 = 1403;
pub const XML_SAVE_NO_DOCTYPE: C2RustUnnamed_2 = 1402;
pub const XML_SAVE_CHAR_INVALID: C2RustUnnamed_2 = 1401;
pub const XML_SAVE_NOT_UTF8: C2RustUnnamed_2 = 1400;
pub const XML_TREE_NOT_UTF8: C2RustUnnamed_2 = 1303;
pub const XML_TREE_UNTERMINATED_ENTITY: C2RustUnnamed_2 = 1302;
pub const XML_TREE_INVALID_DEC: C2RustUnnamed_2 = 1301;
pub const XML_TREE_INVALID_HEX: C2RustUnnamed_2 = 1300;
pub const XML_XPATH_INVALID_CHAR_ERROR: C2RustUnnamed_2 = 1221;
pub const XML_XPATH_ENCODING_ERROR: C2RustUnnamed_2 = 1220;
pub const XML_XPATH_UNDEF_PREFIX_ERROR: C2RustUnnamed_2 = 1219;
pub const XML_XPTR_SUB_RESOURCE_ERROR: C2RustUnnamed_2 = 1218;
pub const XML_XPTR_RESOURCE_ERROR: C2RustUnnamed_2 = 1217;
pub const XML_XPTR_SYNTAX_ERROR: C2RustUnnamed_2 = 1216;
pub const XML_XPATH_MEMORY_ERROR: C2RustUnnamed_2 = 1215;
pub const XML_XPATH_INVALID_CTXT_POSITION: C2RustUnnamed_2 = 1214;
pub const XML_XPATH_INVALID_CTXT_SIZE: C2RustUnnamed_2 = 1213;
pub const XML_XPATH_INVALID_ARITY: C2RustUnnamed_2 = 1212;
pub const XML_XPATH_INVALID_TYPE: C2RustUnnamed_2 = 1211;
pub const XML_XPATH_INVALID_OPERAND: C2RustUnnamed_2 = 1210;
pub const XML_XPATH_UNKNOWN_FUNC_ERROR: C2RustUnnamed_2 = 1209;
pub const XML_XPATH_UNCLOSED_ERROR: C2RustUnnamed_2 = 1208;
pub const XML_XPATH_EXPR_ERROR: C2RustUnnamed_2 = 1207;
pub const XML_XPATH_INVALID_PREDICATE_ERROR: C2RustUnnamed_2 = 1206;
pub const XML_XPATH_UNDEF_VARIABLE_ERROR: C2RustUnnamed_2 = 1205;
pub const XML_XPATH_VARIABLE_REF_ERROR: C2RustUnnamed_2 = 1204;
pub const XML_XPATH_START_LITERAL_ERROR: C2RustUnnamed_2 = 1203;
pub const XML_XPATH_UNFINISHED_LITERAL_ERROR: C2RustUnnamed_2 = 1202;
pub const XML_XPATH_NUMBER_ERROR: C2RustUnnamed_2 = 1201;
pub const XML_XPATH_EXPRESSION_OK: C2RustUnnamed_2 = 1200;
pub const XML_RNGP_XML_NS: C2RustUnnamed_2 = 1122;
pub const XML_RNGP_XMLNS_NAME: C2RustUnnamed_2 = 1121;
pub const XML_RNGP_VALUE_NO_CONTENT: C2RustUnnamed_2 = 1120;
pub const XML_RNGP_VALUE_EMPTY: C2RustUnnamed_2 = 1119;
pub const XML_RNGP_URI_NOT_ABSOLUTE: C2RustUnnamed_2 = 1118;
pub const XML_RNGP_URI_FRAGMENT: C2RustUnnamed_2 = 1117;
pub const XML_RNGP_UNKNOWN_TYPE_LIB: C2RustUnnamed_2 = 1116;
pub const XML_RNGP_UNKNOWN_CONSTRUCT: C2RustUnnamed_2 = 1115;
pub const XML_RNGP_UNKNOWN_COMBINE: C2RustUnnamed_2 = 1114;
pub const XML_RNGP_UNKNOWN_ATTRIBUTE: C2RustUnnamed_2 = 1113;
pub const XML_RNGP_TYPE_VALUE: C2RustUnnamed_2 = 1112;
pub const XML_RNGP_TYPE_NOT_FOUND: C2RustUnnamed_2 = 1111;
pub const XML_RNGP_TYPE_MISSING: C2RustUnnamed_2 = 1110;
pub const XML_RNGP_TEXT_HAS_CHILD: C2RustUnnamed_2 = 1109;
pub const XML_RNGP_TEXT_EXPECTED: C2RustUnnamed_2 = 1108;
pub const XML_RNGP_START_MISSING: C2RustUnnamed_2 = 1107;
pub const XML_RNGP_START_EMPTY: C2RustUnnamed_2 = 1106;
pub const XML_RNGP_START_CONTENT: C2RustUnnamed_2 = 1105;
pub const XML_RNGP_START_CHOICE_AND_INTERLEAVE: C2RustUnnamed_2 = 1104;
pub const XML_RNGP_REF_NOT_EMPTY: C2RustUnnamed_2 = 1103;
pub const XML_RNGP_REF_NO_NAME: C2RustUnnamed_2 = 1102;
pub const XML_RNGP_REF_NO_DEF: C2RustUnnamed_2 = 1101;
pub const XML_RNGP_REF_NAME_INVALID: C2RustUnnamed_2 = 1100;
pub const XML_RNGP_REF_CYCLE: C2RustUnnamed_2 = 1099;
pub const XML_RNGP_REF_CREATE_FAILED: C2RustUnnamed_2 = 1098;
pub const XML_RNGP_PREFIX_UNDEFINED: C2RustUnnamed_2 = 1097;
pub const XML_RNGP_PAT_START_VALUE: C2RustUnnamed_2 = 1096;
pub const XML_RNGP_PAT_START_TEXT: C2RustUnnamed_2 = 1095;
pub const XML_RNGP_PAT_START_ONEMORE: C2RustUnnamed_2 = 1094;
pub const XML_RNGP_PAT_START_LIST: C2RustUnnamed_2 = 1093;
pub const XML_RNGP_PAT_START_INTERLEAVE: C2RustUnnamed_2 = 1092;
pub const XML_RNGP_PAT_START_GROUP: C2RustUnnamed_2 = 1091;
pub const XML_RNGP_PAT_START_EMPTY: C2RustUnnamed_2 = 1090;
pub const XML_RNGP_PAT_START_DATA: C2RustUnnamed_2 = 1089;
pub const XML_RNGP_PAT_START_ATTR: C2RustUnnamed_2 = 1088;
pub const XML_RNGP_PAT_ONEMORE_INTERLEAVE_ATTR: C2RustUnnamed_2 = 1087;
pub const XML_RNGP_PAT_ONEMORE_GROUP_ATTR: C2RustUnnamed_2 = 1086;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_NSNAME: C2RustUnnamed_2 = 1085;
pub const XML_RNGP_PAT_NSNAME_EXCEPT_ANYNAME: C2RustUnnamed_2 = 1084;
pub const XML_RNGP_PAT_LIST_TEXT: C2RustUnnamed_2 = 1083;
pub const XML_RNGP_PAT_LIST_REF: C2RustUnnamed_2 = 1082;
pub const XML_RNGP_PAT_LIST_LIST: C2RustUnnamed_2 = 1081;
pub const XML_RNGP_PAT_LIST_INTERLEAVE: C2RustUnnamed_2 = 1080;
pub const XML_RNGP_PAT_LIST_ELEM: C2RustUnnamed_2 = 1079;
pub const XML_RNGP_PAT_LIST_ATTR: C2RustUnnamed_2 = 1078;
pub const XML_RNGP_PAT_DATA_EXCEPT_TEXT: C2RustUnnamed_2 = 1077;
pub const XML_RNGP_PAT_DATA_EXCEPT_REF: C2RustUnnamed_2 = 1076;
pub const XML_RNGP_PAT_DATA_EXCEPT_ONEMORE: C2RustUnnamed_2 = 1075;
pub const XML_RNGP_PAT_DATA_EXCEPT_LIST: C2RustUnnamed_2 = 1074;
pub const XML_RNGP_PAT_DATA_EXCEPT_INTERLEAVE: C2RustUnnamed_2 = 1073;
pub const XML_RNGP_PAT_DATA_EXCEPT_GROUP: C2RustUnnamed_2 = 1072;
pub const XML_RNGP_PAT_DATA_EXCEPT_EMPTY: C2RustUnnamed_2 = 1071;
pub const XML_RNGP_PAT_DATA_EXCEPT_ELEM: C2RustUnnamed_2 = 1070;
pub const XML_RNGP_PAT_DATA_EXCEPT_ATTR: C2RustUnnamed_2 = 1069;
pub const XML_RNGP_PAT_ATTR_ELEM: C2RustUnnamed_2 = 1068;
pub const XML_RNGP_PAT_ATTR_ATTR: C2RustUnnamed_2 = 1067;
pub const XML_RNGP_PAT_ANYNAME_EXCEPT_ANYNAME: C2RustUnnamed_2 = 1066;
pub const XML_RNGP_PARSE_ERROR: C2RustUnnamed_2 = 1065;
pub const XML_RNGP_PARENTREF_NOT_EMPTY: C2RustUnnamed_2 = 1064;
pub const XML_RNGP_PARENTREF_NO_PARENT: C2RustUnnamed_2 = 1063;
pub const XML_RNGP_PARENTREF_NO_NAME: C2RustUnnamed_2 = 1062;
pub const XML_RNGP_PARENTREF_NAME_INVALID: C2RustUnnamed_2 = 1061;
pub const XML_RNGP_PARENTREF_CREATE_FAILED: C2RustUnnamed_2 = 1060;
pub const XML_RNGP_PARAM_NAME_MISSING: C2RustUnnamed_2 = 1059;
pub const XML_RNGP_PARAM_FORBIDDEN: C2RustUnnamed_2 = 1058;
pub const XML_RNGP_NSNAME_NO_NS: C2RustUnnamed_2 = 1057;
pub const XML_RNGP_NSNAME_ATTR_ANCESTOR: C2RustUnnamed_2 = 1056;
pub const XML_RNGP_NOTALLOWED_NOT_EMPTY: C2RustUnnamed_2 = 1055;
pub const XML_RNGP_NEED_COMBINE: C2RustUnnamed_2 = 1054;
pub const XML_RNGP_NAME_MISSING: C2RustUnnamed_2 = 1053;
pub const XML_RNGP_MISSING_HREF: C2RustUnnamed_2 = 1052;
pub const XML_RNGP_INVALID_VALUE: C2RustUnnamed_2 = 1051;
pub const XML_RNGP_INVALID_URI: C2RustUnnamed_2 = 1050;
pub const XML_RNGP_INVALID_DEFINE_NAME: C2RustUnnamed_2 = 1049;
pub const XML_RNGP_INTERLEAVE_NO_CONTENT: C2RustUnnamed_2 = 1048;
pub const XML_RNGP_INTERLEAVE_EMPTY: C2RustUnnamed_2 = 1047;
pub const XML_RNGP_INTERLEAVE_CREATE_FAILED: C2RustUnnamed_2 = 1046;
pub const XML_RNGP_INTERLEAVE_ADD: C2RustUnnamed_2 = 1045;
pub const XML_RNGP_INCLUDE_RECURSE: C2RustUnnamed_2 = 1044;
pub const XML_RNGP_INCLUDE_FAILURE: C2RustUnnamed_2 = 1043;
pub const XML_RNGP_INCLUDE_EMPTY: C2RustUnnamed_2 = 1042;
pub const XML_RNGP_HREF_ERROR: C2RustUnnamed_2 = 1041;
pub const XML_RNGP_GROUP_ATTR_CONFLICT: C2RustUnnamed_2 = 1040;
pub const XML_RNGP_GRAMMAR_NO_START: C2RustUnnamed_2 = 1039;
pub const XML_RNGP_GRAMMAR_MISSING: C2RustUnnamed_2 = 1038;
pub const XML_RNGP_GRAMMAR_EMPTY: C2RustUnnamed_2 = 1037;
pub const XML_RNGP_GRAMMAR_CONTENT: C2RustUnnamed_2 = 1036;
pub const XML_RNGP_FOREIGN_ELEMENT: C2RustUnnamed_2 = 1035;
pub const XML_RNGP_FORBIDDEN_ATTRIBUTE: C2RustUnnamed_2 = 1034;
pub const XML_RNGP_EXTERNALREF_RECURSE: C2RustUnnamed_2 = 1033;
pub const XML_RNGP_EXTERNAL_REF_FAILURE: C2RustUnnamed_2 = 1032;
pub const XML_RNGP_EXTERNALREF_EMTPY: C2RustUnnamed_2 = 1031;
pub const XML_RNGP_EXCEPT_NO_CONTENT: C2RustUnnamed_2 = 1030;
pub const XML_RNGP_EXCEPT_MULTIPLE: C2RustUnnamed_2 = 1029;
pub const XML_RNGP_EXCEPT_MISSING: C2RustUnnamed_2 = 1028;
pub const XML_RNGP_EXCEPT_EMPTY: C2RustUnnamed_2 = 1027;
pub const XML_RNGP_ERROR_TYPE_LIB: C2RustUnnamed_2 = 1026;
pub const XML_RNGP_EMPTY_NOT_EMPTY: C2RustUnnamed_2 = 1025;
pub const XML_RNGP_EMPTY_CONTENT: C2RustUnnamed_2 = 1024;
pub const XML_RNGP_EMPTY_CONSTRUCT: C2RustUnnamed_2 = 1023;
pub const XML_RNGP_EMPTY: C2RustUnnamed_2 = 1022;
pub const XML_RNGP_ELEM_TEXT_CONFLICT: C2RustUnnamed_2 = 1021;
pub const XML_RNGP_ELEMENT_NO_CONTENT: C2RustUnnamed_2 = 1020;
pub const XML_RNGP_ELEMENT_NAME: C2RustUnnamed_2 = 1019;
pub const XML_RNGP_ELEMENT_CONTENT: C2RustUnnamed_2 = 1018;
pub const XML_RNGP_ELEMENT_EMPTY: C2RustUnnamed_2 = 1017;
pub const XML_RNGP_ELEM_CONTENT_ERROR: C2RustUnnamed_2 = 1016;
pub const XML_RNGP_ELEM_CONTENT_EMPTY: C2RustUnnamed_2 = 1015;
pub const XML_RNGP_DEFINE_NAME_MISSING: C2RustUnnamed_2 = 1014;
pub const XML_RNGP_DEFINE_MISSING: C2RustUnnamed_2 = 1013;
pub const XML_RNGP_DEFINE_EMPTY: C2RustUnnamed_2 = 1012;
pub const XML_RNGP_DEFINE_CREATE_FAILED: C2RustUnnamed_2 = 1011;
pub const XML_RNGP_DEF_CHOICE_AND_INTERLEAVE: C2RustUnnamed_2 = 1010;
pub const XML_RNGP_DATA_CONTENT: C2RustUnnamed_2 = 1009;
pub const XML_RNGP_CREATE_FAILURE: C2RustUnnamed_2 = 1008;
pub const XML_RNGP_CHOICE_EMPTY: C2RustUnnamed_2 = 1007;
pub const XML_RNGP_CHOICE_CONTENT: C2RustUnnamed_2 = 1006;
pub const XML_RNGP_ATTRIBUTE_NOOP: C2RustUnnamed_2 = 1005;
pub const XML_RNGP_ATTRIBUTE_EMPTY: C2RustUnnamed_2 = 1004;
pub const XML_RNGP_ATTRIBUTE_CONTENT: C2RustUnnamed_2 = 1003;
pub const XML_RNGP_ATTRIBUTE_CHILDREN: C2RustUnnamed_2 = 1002;
pub const XML_RNGP_ATTR_CONFLICT: C2RustUnnamed_2 = 1001;
pub const XML_RNGP_ANYNAME_ATTR_ANCESTOR: C2RustUnnamed_2 = 1000;
pub const XML_HTML_UNKNOWN_TAG: C2RustUnnamed_2 = 801;
pub const XML_HTML_STRUCURE_ERROR: C2RustUnnamed_2 = 800;
pub const XML_DTD_DUP_TOKEN: C2RustUnnamed_2 = 541;
pub const XML_DTD_XMLID_TYPE: C2RustUnnamed_2 = 540;
pub const XML_DTD_XMLID_VALUE: C2RustUnnamed_2 = 539;
pub const XML_DTD_STANDALONE_DEFAULTED: C2RustUnnamed_2 = 538;
pub const XML_DTD_UNKNOWN_NOTATION: C2RustUnnamed_2 = 537;
pub const XML_DTD_UNKNOWN_ID: C2RustUnnamed_2 = 536;
pub const XML_DTD_UNKNOWN_ENTITY: C2RustUnnamed_2 = 535;
pub const XML_DTD_UNKNOWN_ELEM: C2RustUnnamed_2 = 534;
pub const XML_DTD_UNKNOWN_ATTRIBUTE: C2RustUnnamed_2 = 533;
pub const XML_DTD_STANDALONE_WHITE_SPACE: C2RustUnnamed_2 = 532;
pub const XML_DTD_ROOT_NAME: C2RustUnnamed_2 = 531;
pub const XML_DTD_NOT_STANDALONE: C2RustUnnamed_2 = 530;
pub const XML_DTD_NOT_PCDATA: C2RustUnnamed_2 = 529;
pub const XML_DTD_NOT_EMPTY: C2RustUnnamed_2 = 528;
pub const XML_DTD_NOTATION_VALUE: C2RustUnnamed_2 = 527;
pub const XML_DTD_NOTATION_REDEFINED: C2RustUnnamed_2 = 526;
pub const XML_DTD_NO_ROOT: C2RustUnnamed_2 = 525;
pub const XML_DTD_NO_PREFIX: C2RustUnnamed_2 = 524;
pub const XML_DTD_NO_ELEM_NAME: C2RustUnnamed_2 = 523;
pub const XML_DTD_NO_DTD: C2RustUnnamed_2 = 522;
pub const XML_DTD_NO_DOC: C2RustUnnamed_2 = 521;
pub const XML_DTD_MULTIPLE_ID: C2RustUnnamed_2 = 520;
pub const XML_DTD_MIXED_CORRUPT: C2RustUnnamed_2 = 519;
pub const XML_DTD_MISSING_ATTRIBUTE: C2RustUnnamed_2 = 518;
pub const XML_DTD_LOAD_ERROR: C2RustUnnamed_2 = 517;
pub const XML_DTD_INVALID_DEFAULT: C2RustUnnamed_2 = 516;
pub const XML_DTD_INVALID_CHILD: C2RustUnnamed_2 = 515;
pub const XML_DTD_ID_SUBSET: C2RustUnnamed_2 = 514;
pub const XML_DTD_ID_REDEFINED: C2RustUnnamed_2 = 513;
pub const XML_DTD_ID_FIXED: C2RustUnnamed_2 = 512;
pub const XML_DTD_ENTITY_TYPE: C2RustUnnamed_2 = 511;
pub const XML_DTD_EMPTY_NOTATION: C2RustUnnamed_2 = 510;
pub const XML_DTD_ELEM_REDEFINED: C2RustUnnamed_2 = 509;
pub const XML_DTD_ELEM_NAMESPACE: C2RustUnnamed_2 = 508;
pub const XML_DTD_ELEM_DEFAULT_NAMESPACE: C2RustUnnamed_2 = 507;
pub const XML_DTD_DIFFERENT_PREFIX: C2RustUnnamed_2 = 506;
pub const XML_DTD_CONTENT_NOT_DETERMINIST: C2RustUnnamed_2 = 505;
pub const XML_DTD_CONTENT_MODEL: C2RustUnnamed_2 = 504;
pub const XML_DTD_CONTENT_ERROR: C2RustUnnamed_2 = 503;
pub const XML_DTD_ATTRIBUTE_VALUE: C2RustUnnamed_2 = 502;
pub const XML_DTD_ATTRIBUTE_REDEFINED: C2RustUnnamed_2 = 501;
pub const XML_DTD_ATTRIBUTE_DEFAULT: C2RustUnnamed_2 = 500;
pub const XML_NS_ERR_COLON: C2RustUnnamed_2 = 205;
pub const XML_NS_ERR_EMPTY: C2RustUnnamed_2 = 204;
pub const XML_NS_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_2 = 203;
pub const XML_NS_ERR_QNAME: C2RustUnnamed_2 = 202;
pub const XML_NS_ERR_UNDEFINED_NAMESPACE: C2RustUnnamed_2 = 201;
pub const XML_NS_ERR_XML_NAMESPACE: C2RustUnnamed_2 = 200;
pub const XML_ERR_USER_STOP: C2RustUnnamed_2 = 111;
pub const XML_ERR_NAME_TOO_LONG: C2RustUnnamed_2 = 110;
pub const XML_ERR_VERSION_MISMATCH: C2RustUnnamed_2 = 109;
pub const XML_ERR_UNKNOWN_VERSION: C2RustUnnamed_2 = 108;
pub const XML_WAR_ENTITY_REDEFINED: C2RustUnnamed_2 = 107;
pub const XML_WAR_NS_COLUMN: C2RustUnnamed_2 = 106;
pub const XML_ERR_NOTATION_PROCESSING: C2RustUnnamed_2 = 105;
pub const XML_ERR_ENTITY_PROCESSING: C2RustUnnamed_2 = 104;
pub const XML_ERR_NOT_STANDALONE: C2RustUnnamed_2 = 103;
pub const XML_WAR_SPACE_VALUE: C2RustUnnamed_2 = 102;
pub const XML_ERR_MISSING_ENCODING: C2RustUnnamed_2 = 101;
pub const XML_WAR_NS_URI_RELATIVE: C2RustUnnamed_2 = 100;
pub const XML_WAR_NS_URI: C2RustUnnamed_2 = 99;
pub const XML_WAR_LANG_VALUE: C2RustUnnamed_2 = 98;
pub const XML_WAR_UNKNOWN_VERSION: C2RustUnnamed_2 = 97;
pub const XML_ERR_VERSION_MISSING: C2RustUnnamed_2 = 96;
pub const XML_ERR_CONDSEC_INVALID_KEYWORD: C2RustUnnamed_2 = 95;
pub const XML_ERR_NO_DTD: C2RustUnnamed_2 = 94;
pub const XML_WAR_CATALOG_PI: C2RustUnnamed_2 = 93;
pub const XML_ERR_URI_FRAGMENT: C2RustUnnamed_2 = 92;
pub const XML_ERR_INVALID_URI: C2RustUnnamed_2 = 91;
pub const XML_ERR_ENTITY_BOUNDARY: C2RustUnnamed_2 = 90;
pub const XML_ERR_ENTITY_LOOP: C2RustUnnamed_2 = 89;
pub const XML_ERR_ENTITY_PE_INTERNAL: C2RustUnnamed_2 = 88;
pub const XML_ERR_ENTITY_CHAR_ERROR: C2RustUnnamed_2 = 87;
pub const XML_ERR_EXTRA_CONTENT: C2RustUnnamed_2 = 86;
pub const XML_ERR_NOT_WELL_BALANCED: C2RustUnnamed_2 = 85;
pub const XML_ERR_VALUE_REQUIRED: C2RustUnnamed_2 = 84;
pub const XML_ERR_CONDSEC_INVALID: C2RustUnnamed_2 = 83;
pub const XML_ERR_EXT_ENTITY_STANDALONE: C2RustUnnamed_2 = 82;
pub const XML_ERR_INVALID_ENCODING: C2RustUnnamed_2 = 81;
pub const XML_ERR_HYPHEN_IN_COMMENT: C2RustUnnamed_2 = 80;
pub const XML_ERR_ENCODING_NAME: C2RustUnnamed_2 = 79;
pub const XML_ERR_STANDALONE_VALUE: C2RustUnnamed_2 = 78;
pub const XML_ERR_TAG_NOT_FINISHED: C2RustUnnamed_2 = 77;
pub const XML_ERR_TAG_NAME_MISMATCH: C2RustUnnamed_2 = 76;
pub const XML_ERR_EQUAL_REQUIRED: C2RustUnnamed_2 = 75;
pub const XML_ERR_LTSLASH_REQUIRED: C2RustUnnamed_2 = 74;
pub const XML_ERR_GT_REQUIRED: C2RustUnnamed_2 = 73;
pub const XML_ERR_LT_REQUIRED: C2RustUnnamed_2 = 72;
pub const XML_ERR_PUBID_REQUIRED: C2RustUnnamed_2 = 71;
pub const XML_ERR_URI_REQUIRED: C2RustUnnamed_2 = 70;
pub const XML_ERR_PCDATA_REQUIRED: C2RustUnnamed_2 = 69;
pub const XML_ERR_NAME_REQUIRED: C2RustUnnamed_2 = 68;
pub const XML_ERR_NMTOKEN_REQUIRED: C2RustUnnamed_2 = 67;
pub const XML_ERR_SEPARATOR_REQUIRED: C2RustUnnamed_2 = 66;
pub const XML_ERR_SPACE_REQUIRED: C2RustUnnamed_2 = 65;
pub const XML_ERR_RESERVED_XML_NAME: C2RustUnnamed_2 = 64;
pub const XML_ERR_CDATA_NOT_FINISHED: C2RustUnnamed_2 = 63;
pub const XML_ERR_MISPLACED_CDATA_END: C2RustUnnamed_2 = 62;
pub const XML_ERR_DOCTYPE_NOT_FINISHED: C2RustUnnamed_2 = 61;
pub const XML_ERR_EXT_SUBSET_NOT_FINISHED: C2RustUnnamed_2 = 60;
pub const XML_ERR_CONDSEC_NOT_FINISHED: C2RustUnnamed_2 = 59;
pub const XML_ERR_CONDSEC_NOT_STARTED: C2RustUnnamed_2 = 58;
pub const XML_ERR_XMLDECL_NOT_FINISHED: C2RustUnnamed_2 = 57;
pub const XML_ERR_XMLDECL_NOT_STARTED: C2RustUnnamed_2 = 56;
pub const XML_ERR_ELEMCONTENT_NOT_FINISHED: C2RustUnnamed_2 = 55;
pub const XML_ERR_ELEMCONTENT_NOT_STARTED: C2RustUnnamed_2 = 54;
pub const XML_ERR_MIXED_NOT_FINISHED: C2RustUnnamed_2 = 53;
pub const XML_ERR_MIXED_NOT_STARTED: C2RustUnnamed_2 = 52;
pub const XML_ERR_ATTLIST_NOT_FINISHED: C2RustUnnamed_2 = 51;
pub const XML_ERR_ATTLIST_NOT_STARTED: C2RustUnnamed_2 = 50;
pub const XML_ERR_NOTATION_NOT_FINISHED: C2RustUnnamed_2 = 49;
pub const XML_ERR_NOTATION_NOT_STARTED: C2RustUnnamed_2 = 48;
pub const XML_ERR_PI_NOT_FINISHED: C2RustUnnamed_2 = 47;
pub const XML_ERR_PI_NOT_STARTED: C2RustUnnamed_2 = 46;
pub const XML_ERR_COMMENT_NOT_FINISHED: C2RustUnnamed_2 = 45;
pub const XML_ERR_LITERAL_NOT_FINISHED: C2RustUnnamed_2 = 44;
pub const XML_ERR_LITERAL_NOT_STARTED: C2RustUnnamed_2 = 43;
pub const XML_ERR_ATTRIBUTE_REDEFINED: C2RustUnnamed_2 = 42;
pub const XML_ERR_ATTRIBUTE_WITHOUT_VALUE: C2RustUnnamed_2 = 41;
pub const XML_ERR_ATTRIBUTE_NOT_FINISHED: C2RustUnnamed_2 = 40;
pub const XML_ERR_ATTRIBUTE_NOT_STARTED: C2RustUnnamed_2 = 39;
pub const XML_ERR_LT_IN_ATTRIBUTE: C2RustUnnamed_2 = 38;
pub const XML_ERR_ENTITY_NOT_FINISHED: C2RustUnnamed_2 = 37;
pub const XML_ERR_ENTITY_NOT_STARTED: C2RustUnnamed_2 = 36;
pub const XML_ERR_NS_DECL_ERROR: C2RustUnnamed_2 = 35;
pub const XML_ERR_STRING_NOT_CLOSED: C2RustUnnamed_2 = 34;
pub const XML_ERR_STRING_NOT_STARTED: C2RustUnnamed_2 = 33;
pub const XML_ERR_UNSUPPORTED_ENCODING: C2RustUnnamed_2 = 32;
pub const XML_ERR_UNKNOWN_ENCODING: C2RustUnnamed_2 = 31;
pub const XML_ERR_ENTITY_IS_PARAMETER: C2RustUnnamed_2 = 30;
pub const XML_ERR_ENTITY_IS_EXTERNAL: C2RustUnnamed_2 = 29;
pub const XML_ERR_UNPARSED_ENTITY: C2RustUnnamed_2 = 28;
pub const XML_WAR_UNDECLARED_ENTITY: C2RustUnnamed_2 = 27;
pub const XML_ERR_UNDECLARED_ENTITY: C2RustUnnamed_2 = 26;
pub const XML_ERR_PEREF_SEMICOL_MISSING: C2RustUnnamed_2 = 25;
pub const XML_ERR_PEREF_NO_NAME: C2RustUnnamed_2 = 24;
pub const XML_ERR_ENTITYREF_SEMICOL_MISSING: C2RustUnnamed_2 = 23;
pub const XML_ERR_ENTITYREF_NO_NAME: C2RustUnnamed_2 = 22;
pub const XML_ERR_PEREF_IN_INT_SUBSET: C2RustUnnamed_2 = 21;
pub const XML_ERR_PEREF_IN_EPILOG: C2RustUnnamed_2 = 20;
pub const XML_ERR_PEREF_IN_PROLOG: C2RustUnnamed_2 = 19;
pub const XML_ERR_PEREF_AT_EOF: C2RustUnnamed_2 = 18;
pub const XML_ERR_ENTITYREF_IN_DTD: C2RustUnnamed_2 = 17;
pub const XML_ERR_ENTITYREF_IN_EPILOG: C2RustUnnamed_2 = 16;
pub const XML_ERR_ENTITYREF_IN_PROLOG: C2RustUnnamed_2 = 15;
pub const XML_ERR_ENTITYREF_AT_EOF: C2RustUnnamed_2 = 14;
pub const XML_ERR_CHARREF_IN_DTD: C2RustUnnamed_2 = 13;
pub const XML_ERR_CHARREF_IN_EPILOG: C2RustUnnamed_2 = 12;
pub const XML_ERR_CHARREF_IN_PROLOG: C2RustUnnamed_2 = 11;
pub const XML_ERR_CHARREF_AT_EOF: C2RustUnnamed_2 = 10;
pub const XML_ERR_INVALID_CHAR: C2RustUnnamed_2 = 9;
pub const XML_ERR_INVALID_CHARREF: C2RustUnnamed_2 = 8;
pub const XML_ERR_INVALID_DEC_CHARREF: C2RustUnnamed_2 = 7;
pub const XML_ERR_INVALID_HEX_CHARREF: C2RustUnnamed_2 = 6;
pub const XML_ERR_DOCUMENT_END: C2RustUnnamed_2 = 5;
pub const XML_ERR_DOCUMENT_EMPTY: C2RustUnnamed_2 = 4;
pub const XML_ERR_DOCUMENT_START: C2RustUnnamed_2 = 3;
pub const XML_ERR_NO_MEMORY: C2RustUnnamed_2 = 2;
pub const XML_ERR_INTERNAL_ERROR: C2RustUnnamed_2 = 1;
pub const XML_ERR_OK: C2RustUnnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlURI {
    pub scheme: *mut ::core::ffi::c_char,
    pub opaque: *mut ::core::ffi::c_char,
    pub authority: *mut ::core::ffi::c_char,
    pub server: *mut ::core::ffi::c_char,
    pub user: *mut ::core::ffi::c_char,
    pub port: ::core::ffi::c_int,
    pub path: *mut ::core::ffi::c_char,
    pub query: *mut ::core::ffi::c_char,
    pub fragment: *mut ::core::ffi::c_char,
    pub cleanup: ::core::ffi::c_int,
    pub query_raw: *mut ::core::ffi::c_char,
}
pub type xmlURI = _xmlURI;
pub type xmlURIPtr = *mut xmlURI;
pub type ftpListCallback = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        *const ::core::ffi::c_char,
        ::core::ffi::c_ulong,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        *const ::core::ffi::c_char,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type ftpDataCallback = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_char,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type xmlNanoFTPCtxtPtr = *mut xmlNanoFTPCtxt;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xmlNanoFTPCtxt {
    pub protocol: *mut ::core::ffi::c_char,
    pub hostname: *mut ::core::ffi::c_char,
    pub port: ::core::ffi::c_int,
    pub path: *mut ::core::ffi::c_char,
    pub user: *mut ::core::ffi::c_char,
    pub passwd: *mut ::core::ffi::c_char,
    pub ftpAddr: sockaddr_storage,
    pub passive: ::core::ffi::c_int,
    pub controlFd: ::core::ffi::c_int,
    pub dataFd: ::core::ffi::c_int,
    pub state: ::core::ffi::c_int,
    pub returnValue: ::core::ffi::c_int,
    pub controlBuf: [::core::ffi::c_char; 1025],
    pub controlBufIndex: ::core::ffi::c_int,
    pub controlBufUsed: ::core::ffi::c_int,
    pub controlBufAnswer: ::core::ffi::c_int,
}
pub const __NFDBITS: ::core::ffi::c_int =
    8 as ::core::ffi::c_int * ::core::mem::size_of::<__fd_mask>() as ::core::ffi::c_int;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const INET6_ADDRSTRLEN: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INVALID_SOCKET: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const FTP_BUF_SIZE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
static mut initialized: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut proxy: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut proxyPort: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut proxyUser: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut proxyPasswd: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
static mut proxyType: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn have_ipv6() -> ::core::ffi::c_int {
    let mut s: ::core::ffi::c_int = 0;
    s = socket(
        AF_INET6,
        SOCK_STREAM as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if s != -(1 as ::core::ffi::c_int) {
        close(s);
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlFTPErrMemory(mut extra: *const ::core::ffi::c_char) {
    __xmlSimpleError(
        XML_FROM_FTP as ::core::ffi::c_int,
        XML_ERR_NO_MEMORY as ::core::ffi::c_int,
        ::core::ptr::null_mut::<xmlNode>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
        extra,
    );
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPInit() {
    let mut env: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if initialized != 0 {
        return;
    }
    proxyPort = 21 as ::core::ffi::c_int;
    env = getenv(b"no_proxy\0" as *const u8 as *const ::core::ffi::c_char);
    if !env.is_null()
        && (*env.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '*' as i32
            && *env.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int)
    {
        return;
    }
    env = getenv(b"ftp_proxy\0" as *const u8 as *const ::core::ffi::c_char);
    if !env.is_null() {
        xmlNanoFTPScanProxy(env);
    } else {
        env = getenv(b"FTP_PROXY\0" as *const u8 as *const ::core::ffi::c_char);
        if !env.is_null() {
            xmlNanoFTPScanProxy(env);
        }
    }
    env = getenv(b"ftp_proxy_user\0" as *const u8 as *const ::core::ffi::c_char);
    if !env.is_null() {
        proxyUser = xmlMemStrdup.expect("non-null function pointer")(env);
    }
    env = getenv(b"ftp_proxy_password\0" as *const u8 as *const ::core::ffi::c_char);
    if !env.is_null() {
        proxyPasswd = xmlMemStrdup.expect("non-null function pointer")(env);
    }
    initialized = 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPCleanup() {
    if !proxy.is_null() {
        xmlFree.expect("non-null function pointer")(proxy as *mut ::core::ffi::c_void);
        proxy = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !proxyUser.is_null() {
        xmlFree.expect("non-null function pointer")(proxyUser as *mut ::core::ffi::c_void);
        proxyUser = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !proxyPasswd.is_null() {
        xmlFree.expect("non-null function pointer")(proxyPasswd as *mut ::core::ffi::c_void);
        proxyPasswd = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    initialized = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPProxy(
    mut host: *const ::core::ffi::c_char,
    mut port: ::core::ffi::c_int,
    mut user: *const ::core::ffi::c_char,
    mut passwd: *const ::core::ffi::c_char,
    mut type_0: ::core::ffi::c_int,
) {
    if !proxy.is_null() {
        xmlFree.expect("non-null function pointer")(proxy as *mut ::core::ffi::c_void);
        proxy = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !proxyUser.is_null() {
        xmlFree.expect("non-null function pointer")(proxyUser as *mut ::core::ffi::c_void);
        proxyUser = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !proxyPasswd.is_null() {
        xmlFree.expect("non-null function pointer")(proxyPasswd as *mut ::core::ffi::c_void);
        proxyPasswd = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !host.is_null() {
        proxy = xmlMemStrdup.expect("non-null function pointer")(host);
    }
    if !user.is_null() {
        proxyUser = xmlMemStrdup.expect("non-null function pointer")(user);
    }
    if !passwd.is_null() {
        proxyPasswd = xmlMemStrdup.expect("non-null function pointer")(passwd);
    }
    proxyPort = port;
    proxyType = type_0;
}
unsafe extern "C" fn xmlNanoFTPScanURL(
    mut ctx: *mut ::core::ffi::c_void,
    mut URL: *const ::core::ffi::c_char,
) {
    let mut ctxt: xmlNanoFTPCtxtPtr = ctx as xmlNanoFTPCtxtPtr;
    let mut uri: xmlURIPtr = ::core::ptr::null_mut::<xmlURI>();
    if !(*ctxt).protocol.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).protocol as *mut ::core::ffi::c_void);
        (*ctxt).protocol = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !(*ctxt).hostname.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).hostname as *mut ::core::ffi::c_void);
        (*ctxt).hostname = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !(*ctxt).path.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).path as *mut ::core::ffi::c_void);
        (*ctxt).path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if URL.is_null() {
        return;
    }
    uri = xmlParseURIRaw(URL, 1 as ::core::ffi::c_int);
    if uri.is_null() {
        return;
    }
    if (*uri).scheme.is_null() || (*uri).server.is_null() {
        xmlFreeURI(uri);
        return;
    }
    (*ctxt).protocol = xmlMemStrdup.expect("non-null function pointer")((*uri).scheme);
    (*ctxt).hostname = xmlMemStrdup.expect("non-null function pointer")((*uri).server);
    if !(*uri).path.is_null() {
        (*ctxt).path = xmlMemStrdup.expect("non-null function pointer")((*uri).path);
    } else {
        (*ctxt).path = xmlMemStrdup.expect("non-null function pointer")(
            b"/\0" as *const u8 as *const ::core::ffi::c_char,
        );
    }
    if (*uri).port != 0 as ::core::ffi::c_int {
        (*ctxt).port = (*uri).port;
    }
    if !(*uri).user.is_null() {
        let mut cptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        cptr = strchr((*uri).user, ':' as i32);
        if cptr.is_null() {
            (*ctxt).user = xmlMemStrdup.expect("non-null function pointer")((*uri).user);
        } else {
            (*ctxt).user = xmlStrndup(
                (*uri).user as *mut xmlChar,
                cptr.offset_from((*uri).user) as ::core::ffi::c_long as ::core::ffi::c_int,
            ) as *mut ::core::ffi::c_char;
            (*ctxt).passwd = xmlMemStrdup.expect("non-null function pointer")(
                cptr.offset(1 as ::core::ffi::c_int as isize),
            );
        }
    }
    xmlFreeURI(uri);
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPUpdateURL(
    mut ctx: *mut ::core::ffi::c_void,
    mut URL: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ctxt: xmlNanoFTPCtxtPtr = ctx as xmlNanoFTPCtxtPtr;
    let mut uri: xmlURIPtr = ::core::ptr::null_mut::<xmlURI>();
    if URL.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*ctxt).protocol.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*ctxt).hostname.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    uri = xmlParseURIRaw(URL, 1 as ::core::ffi::c_int);
    if uri.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*uri).scheme.is_null() || (*uri).server.is_null() {
        xmlFreeURI(uri);
        return -(1 as ::core::ffi::c_int);
    }
    if strcmp((*ctxt).protocol, (*uri).scheme) != 0
        || strcmp((*ctxt).hostname, (*uri).server) != 0
        || (*uri).port != 0 as ::core::ffi::c_int && (*ctxt).port != (*uri).port
    {
        xmlFreeURI(uri);
        return -(1 as ::core::ffi::c_int);
    }
    if (*uri).port != 0 as ::core::ffi::c_int {
        (*ctxt).port = (*uri).port;
    }
    if !(*ctxt).path.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).path as *mut ::core::ffi::c_void);
        (*ctxt).path = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if (*uri).path.is_null() {
        (*ctxt).path = xmlMemStrdup.expect("non-null function pointer")(
            b"/\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        (*ctxt).path = xmlMemStrdup.expect("non-null function pointer")((*uri).path);
    }
    xmlFreeURI(uri);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPScanProxy(mut URL: *const ::core::ffi::c_char) {
    let mut uri: xmlURIPtr = ::core::ptr::null_mut::<xmlURI>();
    if !proxy.is_null() {
        xmlFree.expect("non-null function pointer")(proxy as *mut ::core::ffi::c_void);
        proxy = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    proxyPort = 0 as ::core::ffi::c_int;
    if URL.is_null() {
        return;
    }
    uri = xmlParseURIRaw(URL, 1 as ::core::ffi::c_int);
    if uri.is_null()
        || (*uri).scheme.is_null()
        || strcmp(
            (*uri).scheme,
            b"ftp\0" as *const u8 as *const ::core::ffi::c_char,
        ) != 0
        || (*uri).server.is_null()
    {
        __xmlIOErr(
            XML_FROM_FTP as ::core::ffi::c_int,
            XML_FTP_URL_SYNTAX as ::core::ffi::c_int,
            b"Syntax Error\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !uri.is_null() {
            xmlFreeURI(uri);
        }
        return;
    }
    proxy = xmlMemStrdup.expect("non-null function pointer")((*uri).server);
    if (*uri).port != 0 as ::core::ffi::c_int {
        proxyPort = (*uri).port;
    }
    xmlFreeURI(uri);
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPNewCtxt(
    mut URL: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    let mut ret: xmlNanoFTPCtxtPtr = ::core::ptr::null_mut::<xmlNanoFTPCtxt>();
    let mut unescaped: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    ret = xmlMalloc.expect("non-null function pointer")(
        ::core::mem::size_of::<xmlNanoFTPCtxt>() as size_t
    ) as xmlNanoFTPCtxtPtr;
    if ret.is_null() {
        xmlFTPErrMemory(b"allocating FTP context\0" as *const u8 as *const ::core::ffi::c_char);
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    memset(
        ret as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<xmlNanoFTPCtxt>() as size_t,
    );
    (*ret).port = 21 as ::core::ffi::c_int;
    (*ret).passive = 1 as ::core::ffi::c_int;
    (*ret).returnValue = 0 as ::core::ffi::c_int;
    (*ret).controlBufIndex = 0 as ::core::ffi::c_int;
    (*ret).controlBufUsed = 0 as ::core::ffi::c_int;
    (*ret).controlFd = INVALID_SOCKET;
    unescaped = xmlURIUnescapeString(
        URL,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    );
    if !unescaped.is_null() {
        xmlNanoFTPScanURL(ret as *mut ::core::ffi::c_void, unescaped);
        xmlFree.expect("non-null function pointer")(unescaped as *mut ::core::ffi::c_void);
    } else if !URL.is_null() {
        xmlNanoFTPScanURL(ret as *mut ::core::ffi::c_void, URL);
    }
    return ret as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPFreeCtxt(mut ctx: *mut ::core::ffi::c_void) {
    let mut ctxt: xmlNanoFTPCtxtPtr = ctx as xmlNanoFTPCtxtPtr;
    if ctxt.is_null() {
        return;
    }
    if !(*ctxt).hostname.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).hostname as *mut ::core::ffi::c_void);
    }
    if !(*ctxt).protocol.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).protocol as *mut ::core::ffi::c_void);
    }
    if !(*ctxt).path.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).path as *mut ::core::ffi::c_void);
    }
    if !(*ctxt).user.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).user as *mut ::core::ffi::c_void);
    }
    if !(*ctxt).passwd.is_null() {
        xmlFree.expect("non-null function pointer")((*ctxt).passwd as *mut ::core::ffi::c_void);
    }
    (*ctxt).passive = 1 as ::core::ffi::c_int;
    if (*ctxt).controlFd != INVALID_SOCKET {
        close((*ctxt).controlFd);
    }
    (*ctxt).controlFd = INVALID_SOCKET;
    (*ctxt).controlBufIndex = -(1 as ::core::ffi::c_int);
    (*ctxt).controlBufUsed = -(1 as ::core::ffi::c_int);
    xmlFree.expect("non-null function pointer")(ctxt as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn xmlNanoFTPParseResponse(
    mut buf: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut val: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if len < 3 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if *buf as ::core::ffi::c_int >= '0' as i32 && *buf as ::core::ffi::c_int <= '9' as i32 {
        val = val * 10 as ::core::ffi::c_int + (*buf as ::core::ffi::c_int - '0' as i32);
    } else {
        return 0 as ::core::ffi::c_int;
    }
    buf = buf.offset(1);
    if *buf as ::core::ffi::c_int >= '0' as i32 && *buf as ::core::ffi::c_int <= '9' as i32 {
        val = val * 10 as ::core::ffi::c_int + (*buf as ::core::ffi::c_int - '0' as i32);
    } else {
        return 0 as ::core::ffi::c_int;
    }
    buf = buf.offset(1);
    if *buf as ::core::ffi::c_int >= '0' as i32 && *buf as ::core::ffi::c_int <= '9' as i32 {
        val = val * 10 as ::core::ffi::c_int + (*buf as ::core::ffi::c_int - '0' as i32);
    } else {
        return 0 as ::core::ffi::c_int;
    }
    buf = buf.offset(1);
    if *buf as ::core::ffi::c_int == '-' as i32 {
        return -val;
    }
    return val;
}
unsafe extern "C" fn xmlNanoFTPGetMore(mut ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut ctxt: xmlNanoFTPCtxtPtr = ctx as xmlNanoFTPCtxtPtr;
    let mut len: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = 0;
    if ctxt.is_null() || (*ctxt).controlFd == INVALID_SOCKET {
        return -(1 as ::core::ffi::c_int);
    }
    if (*ctxt).controlBufIndex < 0 as ::core::ffi::c_int || (*ctxt).controlBufIndex > FTP_BUF_SIZE {
        return -(1 as ::core::ffi::c_int);
    }
    if (*ctxt).controlBufUsed < 0 as ::core::ffi::c_int || (*ctxt).controlBufUsed > FTP_BUF_SIZE {
        return -(1 as ::core::ffi::c_int);
    }
    if (*ctxt).controlBufIndex > (*ctxt).controlBufUsed {
        return -(1 as ::core::ffi::c_int);
    }
    if (*ctxt).controlBufIndex > 0 as ::core::ffi::c_int {
        memmove(
            (&raw mut (*ctxt).controlBuf as *mut ::core::ffi::c_char)
                .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void,
            (&raw mut (*ctxt).controlBuf as *mut ::core::ffi::c_char)
                .offset((*ctxt).controlBufIndex as isize) as *mut ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            ((*ctxt).controlBufUsed - (*ctxt).controlBufIndex) as size_t,
        );
        (*ctxt).controlBufUsed -= (*ctxt).controlBufIndex;
        (*ctxt).controlBufIndex = 0 as ::core::ffi::c_int;
    }
    size = FTP_BUF_SIZE - (*ctxt).controlBufUsed;
    if size == 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    len = recv(
        (*ctxt).controlFd,
        (&raw mut (*ctxt).controlBuf as *mut ::core::ffi::c_char)
            .offset((*ctxt).controlBufIndex as isize) as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void,
        size as size_t,
        0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    if len < 0 as ::core::ffi::c_int {
        __xmlIOErr(
            XML_FROM_FTP as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            b"recv failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        close((*ctxt).controlFd);
        (*ctxt).controlFd = INVALID_SOCKET;
        (*ctxt).controlFd = INVALID_SOCKET;
        return -(1 as ::core::ffi::c_int);
    }
    (*ctxt).controlBufUsed += len;
    (*ctxt).controlBuf[(*ctxt).controlBufUsed as usize] = 0 as ::core::ffi::c_char;
    return len;
}
unsafe extern "C" fn xmlNanoFTPReadResponse(
    mut ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctxt: xmlNanoFTPCtxtPtr = ctx as xmlNanoFTPCtxtPtr;
    let mut ptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut cur: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if ctxt.is_null() || (*ctxt).controlFd == INVALID_SOCKET {
        return -(1 as ::core::ffi::c_int);
    }
    '_get_more: loop {
        len = xmlNanoFTPGetMore(ctx);
        if len < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        if (*ctxt).controlBufUsed == 0 as ::core::ffi::c_int && len == 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        ptr = (&raw mut (*ctxt).controlBuf as *mut ::core::ffi::c_char)
            .offset((*ctxt).controlBufIndex as isize) as *mut ::core::ffi::c_char;
        end = (&raw mut (*ctxt).controlBuf as *mut ::core::ffi::c_char)
            .offset((*ctxt).controlBufUsed as isize) as *mut ::core::ffi::c_char;
        while ptr < end {
            cur = xmlNanoFTPParseResponse(
                ptr,
                end.offset_from(ptr) as ::core::ffi::c_long as ::core::ffi::c_int,
            );
            if cur > 0 as ::core::ffi::c_int {
                res = cur;
                ptr = ptr.offset(3 as ::core::ffi::c_int as isize);
                (*ctxt).controlBufAnswer =
                    ptr.offset_from(&raw mut (*ctxt).controlBuf as *mut ::core::ffi::c_char)
                        as ::core::ffi::c_long as ::core::ffi::c_int;
                while ptr < end && *ptr as ::core::ffi::c_int != '\n' as i32 {
                    ptr = ptr.offset(1);
                }
                if *ptr as ::core::ffi::c_int == '\n' as i32 {
                    ptr = ptr.offset(1);
                }
                if *ptr as ::core::ffi::c_int == '\r' as i32 {
                    ptr = ptr.offset(1);
                }
                break;
            } else {
                while ptr < end && *ptr as ::core::ffi::c_int != '\n' as i32 {
                    ptr = ptr.offset(1);
                }
                if ptr >= end {
                    (*ctxt).controlBufIndex = (*ctxt).controlBufUsed;
                    continue '_get_more;
                } else if *ptr as ::core::ffi::c_int != '\r' as i32 {
                    ptr = ptr.offset(1);
                }
            }
        }
        if !(res < 0 as ::core::ffi::c_int) {
            break;
        }
    }
    (*ctxt).controlBufIndex = ptr
        .offset_from(&raw mut (*ctxt).controlBuf as *mut ::core::ffi::c_char)
        as ::core::ffi::c_long as ::core::ffi::c_int;
    return res / 100 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPGetResponse(
    mut ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut res: ::core::ffi::c_int = 0;
    res = xmlNanoFTPReadResponse(ctx);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPCheckResponse(
    mut ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctxt: xmlNanoFTPCtxtPtr = ctx as xmlNanoFTPCtxtPtr;
    let mut rfd: fd_set = fd_set {
        __fds_bits: [0; 16],
    };
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    if ctxt.is_null() || (*ctxt).controlFd == INVALID_SOCKET {
        return -(1 as ::core::ffi::c_int);
    }
    tv.tv_sec = 0 as __time_t;
    tv.tv_usec = 0 as __suseconds_t;
    let mut __i: ::core::ffi::c_uint = 0;
    let mut __arr: *mut fd_set = &raw mut rfd;
    __i = 0 as ::core::ffi::c_uint;
    while (__i as usize)
        < (::core::mem::size_of::<fd_set>() as usize)
            .wrapping_div(::core::mem::size_of::<__fd_mask>() as usize)
    {
        (*__arr).__fds_bits[__i as usize] = 0 as __fd_mask;
        __i = __i.wrapping_add(1);
    }
    rfd.__fds_bits[((*ctxt).controlFd / __NFDBITS) as usize] |=
        ((1 as ::core::ffi::c_ulong) << (*ctxt).controlFd % __NFDBITS) as __fd_mask;
    match select(
        (*ctxt).controlFd + 1 as ::core::ffi::c_int,
        &raw mut rfd,
        ::core::ptr::null_mut::<fd_set>(),
        ::core::ptr::null_mut::<fd_set>(),
        &raw mut tv,
    ) {
        0 => return 0 as ::core::ffi::c_int,
        -1 => {
            __xmlIOErr(
                XML_FROM_FTP as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"select\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        _ => {}
    }
    return xmlNanoFTPReadResponse(ctx);
}
unsafe extern "C" fn xmlNanoFTPSendUser(mut ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut ctxt: xmlNanoFTPCtxtPtr = ctx as xmlNanoFTPCtxtPtr;
    let mut buf: [::core::ffi::c_char; 200] = [0; 200];
    let mut len: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 0;
    if (*ctxt).user.is_null() {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 200]>() as size_t,
            b"USER anonymous\r\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 200]>() as size_t,
            b"USER %s\r\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*ctxt).user,
        );
    }
    buf[(::core::mem::size_of::<[::core::ffi::c_char; 200]>() as usize).wrapping_sub(1 as usize)
        as usize] = 0 as ::core::ffi::c_char;
    len = strlen(&raw mut buf as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
    res = send(
        (*ctxt).controlFd,
        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        len as size_t,
        0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    if res < 0 as ::core::ffi::c_int {
        __xmlIOErr(
            XML_FROM_FTP as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            b"send failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return res;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlNanoFTPSendPasswd(mut ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut ctxt: xmlNanoFTPCtxtPtr = ctx as xmlNanoFTPCtxtPtr;
    let mut buf: [::core::ffi::c_char; 200] = [0; 200];
    let mut len: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 0;
    if (*ctxt).passwd.is_null() {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 200]>() as size_t,
            b"PASS anonymous@\r\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 200]>() as size_t,
            b"PASS %s\r\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*ctxt).passwd,
        );
    }
    buf[(::core::mem::size_of::<[::core::ffi::c_char; 200]>() as usize).wrapping_sub(1 as usize)
        as usize] = 0 as ::core::ffi::c_char;
    len = strlen(&raw mut buf as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
    res = send(
        (*ctxt).controlFd,
        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        len as size_t,
        0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    if res < 0 as ::core::ffi::c_int {
        __xmlIOErr(
            XML_FROM_FTP as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            b"send failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return res;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPQuit(mut ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut ctxt: xmlNanoFTPCtxtPtr = ctx as xmlNanoFTPCtxtPtr;
    let mut buf: [::core::ffi::c_char; 200] = [0; 200];
    let mut len: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 0;
    if ctxt.is_null() || (*ctxt).controlFd == INVALID_SOCKET {
        return -(1 as ::core::ffi::c_int);
    }
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 200]>() as size_t,
        b"QUIT\r\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    len = strlen(&raw mut buf as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
    res = send(
        (*ctxt).controlFd,
        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        len as size_t,
        0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    if res < 0 as ::core::ffi::c_int {
        __xmlIOErr(
            XML_FROM_FTP as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            b"send failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return res;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPConnect(
    mut ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctxt: xmlNanoFTPCtxtPtr = ctx as xmlNanoFTPCtxtPtr;
    let mut hp: *mut hostent = ::core::ptr::null_mut::<hostent>();
    let mut port: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 0;
    let mut addrlen: ::core::ffi::c_int =
        ::core::mem::size_of::<sockaddr_in>() as ::core::ffi::c_int;
    if ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*ctxt).hostname.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if !proxy.is_null() {
        port = proxyPort;
    } else {
        port = (*ctxt).port;
    }
    if port == 0 as ::core::ffi::c_int {
        port = 21 as ::core::ffi::c_int;
    }
    memset(
        &raw mut (*ctxt).ftpAddr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_storage>() as size_t,
    );
    if have_ipv6() != 0 {
        let mut hints: addrinfo = addrinfo {
            ai_flags: 0,
            ai_family: 0,
            ai_socktype: 0,
            ai_protocol: 0,
            ai_addrlen: 0,
            ai_addr: ::core::ptr::null_mut::<sockaddr>(),
            ai_canonname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            ai_next: ::core::ptr::null_mut::<addrinfo>(),
        };
        let mut tmp: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
        let mut result: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
        result = ::core::ptr::null_mut::<addrinfo>();
        memset(
            &raw mut hints as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<addrinfo>() as size_t,
        );
        hints.ai_socktype = SOCK_STREAM as ::core::ffi::c_int;
        if !proxy.is_null() {
            if getaddrinfo(
                proxy,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut hints,
                &raw mut result,
            ) != 0 as ::core::ffi::c_int
            {
                __xmlIOErr(
                    XML_FROM_FTP as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    b"getaddrinfo failed\0" as *const u8 as *const ::core::ffi::c_char,
                );
                return -(1 as ::core::ffi::c_int);
            }
        } else if getaddrinfo(
            (*ctxt).hostname,
            ::core::ptr::null::<::core::ffi::c_char>(),
            &raw mut hints,
            &raw mut result,
        ) != 0 as ::core::ffi::c_int
        {
            __xmlIOErr(
                XML_FROM_FTP as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"getaddrinfo failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        tmp = result;
        while !tmp.is_null() {
            if (*tmp).ai_family == AF_INET || (*tmp).ai_family == AF_INET6 {
                break;
            }
            tmp = (*tmp).ai_next;
        }
        if tmp.is_null() {
            if !result.is_null() {
                freeaddrinfo(result);
            }
            __xmlIOErr(
                XML_FROM_FTP as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"getaddrinfo failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if (*tmp).ai_addrlen as size_t > ::core::mem::size_of::<sockaddr_storage>() as usize {
            if !result.is_null() {
                freeaddrinfo(result);
            }
            __xmlIOErr(
                XML_FROM_FTP as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"gethostbyname address mismatch\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if (*tmp).ai_family == AF_INET6 {
            memcpy(
                &raw mut (*ctxt).ftpAddr as *mut ::core::ffi::c_void,
                (*tmp).ai_addr as *const ::core::ffi::c_void,
                (*tmp).ai_addrlen as size_t,
            );
            (*(&raw mut (*ctxt).ftpAddr as *mut sockaddr_in6)).sin6_port =
                htons(port as uint16_t) as in_port_t;
            (*ctxt).controlFd = socket(
                AF_INET6,
                SOCK_STREAM as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        } else {
            memcpy(
                &raw mut (*ctxt).ftpAddr as *mut ::core::ffi::c_void,
                (*tmp).ai_addr as *const ::core::ffi::c_void,
                (*tmp).ai_addrlen as size_t,
            );
            (*(&raw mut (*ctxt).ftpAddr as *mut sockaddr_in)).sin_port =
                htons(port as uint16_t) as in_port_t;
            (*ctxt).controlFd = socket(
                AF_INET,
                SOCK_STREAM as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        }
        addrlen = (*tmp).ai_addrlen as ::core::ffi::c_int;
        freeaddrinfo(result);
    } else {
        if !proxy.is_null() {
            hp = gethostbyname(proxy);
        } else {
            hp = gethostbyname((*ctxt).hostname);
        }
        if hp.is_null() {
            __xmlIOErr(
                XML_FROM_FTP as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"gethostbyname failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if (*hp).h_length as ::core::ffi::c_uint as usize
            > ::core::mem::size_of::<in_addr>() as usize
        {
            __xmlIOErr(
                XML_FROM_FTP as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"gethostbyname address mismatch\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        (*(&raw mut (*ctxt).ftpAddr as *mut sockaddr_in)).sin_family = AF_INET as sa_family_t;
        memcpy(
            &raw mut (*(&raw mut (*ctxt).ftpAddr as *mut sockaddr_in)).sin_addr
                as *mut ::core::ffi::c_void,
            *(*hp).h_addr_list.offset(0 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_void,
            (*hp).h_length as size_t,
        );
        (*(&raw mut (*ctxt).ftpAddr as *mut sockaddr_in)).sin_port =
            htons(port as uint16_t) as ::core::ffi::c_ushort as in_port_t;
        (*ctxt).controlFd = socket(
            AF_INET,
            SOCK_STREAM as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        addrlen = ::core::mem::size_of::<sockaddr_in>() as ::core::ffi::c_int;
    }
    if (*ctxt).controlFd == INVALID_SOCKET {
        __xmlIOErr(
            XML_FROM_FTP as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            b"socket failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if connect(
        (*ctxt).controlFd,
        &raw mut (*ctxt).ftpAddr as *mut sockaddr,
        addrlen as socklen_t,
    ) < 0 as ::core::ffi::c_int
    {
        __xmlIOErr(
            XML_FROM_FTP as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            b"Failed to create a connection\0" as *const u8 as *const ::core::ffi::c_char,
        );
        close((*ctxt).controlFd);
        (*ctxt).controlFd = INVALID_SOCKET;
        (*ctxt).controlFd = INVALID_SOCKET;
        return -(1 as ::core::ffi::c_int);
    }
    res = xmlNanoFTPGetResponse(ctxt as *mut ::core::ffi::c_void);
    if res != 2 as ::core::ffi::c_int {
        close((*ctxt).controlFd);
        (*ctxt).controlFd = INVALID_SOCKET;
        (*ctxt).controlFd = INVALID_SOCKET;
        return -(1 as ::core::ffi::c_int);
    }
    if !proxy.is_null() {
        let mut len: ::core::ffi::c_int = 0;
        let mut buf: [::core::ffi::c_char; 400] = [0; 400];
        if !proxyUser.is_null() {
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 400]>() as size_t,
                b"USER %s\r\n\0" as *const u8 as *const ::core::ffi::c_char,
                proxyUser,
            );
            buf[(::core::mem::size_of::<[::core::ffi::c_char; 400]>() as usize)
                .wrapping_sub(1 as usize) as usize] = 0 as ::core::ffi::c_char;
            len = strlen(&raw mut buf as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
            res = send(
                (*ctxt).controlFd,
                &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                len as size_t,
                0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int;
            if res < 0 as ::core::ffi::c_int {
                __xmlIOErr(
                    XML_FROM_FTP as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    b"send failed\0" as *const u8 as *const ::core::ffi::c_char,
                );
                close((*ctxt).controlFd);
                (*ctxt).controlFd = INVALID_SOCKET;
                return res;
            }
            res = xmlNanoFTPGetResponse(ctxt as *mut ::core::ffi::c_void);
            let mut current_block_118: u64;
            match res {
                2 => {
                    if proxyPasswd.is_null() {
                        current_block_118 = 7072655752890836508;
                    } else {
                        current_block_118 = 4819681734069557026;
                    }
                }
                3 => {
                    current_block_118 = 4819681734069557026;
                }
                1 => {
                    current_block_118 = 7072655752890836508;
                }
                4 | 5 | -1 | _ => {
                    close((*ctxt).controlFd);
                    (*ctxt).controlFd = INVALID_SOCKET;
                    return -(1 as ::core::ffi::c_int);
                }
            }
            match current_block_118 {
                4819681734069557026 => {
                    if !proxyPasswd.is_null() {
                        snprintf(
                            &raw mut buf as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 400]>() as size_t,
                            b"PASS %s\r\n\0" as *const u8 as *const ::core::ffi::c_char,
                            proxyPasswd,
                        );
                    } else {
                        snprintf(
                            &raw mut buf as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 400]>() as size_t,
                            b"PASS anonymous@\r\n\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    }
                    buf[(::core::mem::size_of::<[::core::ffi::c_char; 400]>() as usize)
                        .wrapping_sub(1 as usize) as usize] = 0 as ::core::ffi::c_char;
                    len = strlen(&raw mut buf as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
                    res = send(
                        (*ctxt).controlFd,
                        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                        len as size_t,
                        0 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int;
                    if res < 0 as ::core::ffi::c_int {
                        __xmlIOErr(
                            XML_FROM_FTP as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            b"send failed\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        close((*ctxt).controlFd);
                        (*ctxt).controlFd = INVALID_SOCKET;
                        return res;
                    }
                    res = xmlNanoFTPGetResponse(ctxt as *mut ::core::ffi::c_void);
                    if res > 3 as ::core::ffi::c_int {
                        close((*ctxt).controlFd);
                        (*ctxt).controlFd = INVALID_SOCKET;
                        return -(1 as ::core::ffi::c_int);
                    }
                }
                _ => {}
            }
        }
        's_698: {
            let mut current_block_186: u64;
            match proxyType {
                0 | 1 => {
                    snprintf(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 400]>() as size_t,
                        b"SITE %s\r\n\0" as *const u8 as *const ::core::ffi::c_char,
                        (*ctxt).hostname,
                    );
                    buf[(::core::mem::size_of::<[::core::ffi::c_char; 400]>() as usize)
                        .wrapping_sub(1 as usize) as usize] = 0 as ::core::ffi::c_char;
                    len = strlen(&raw mut buf as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
                    res = send(
                        (*ctxt).controlFd,
                        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                        len as size_t,
                        0 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int;
                    if res < 0 as ::core::ffi::c_int {
                        __xmlIOErr(
                            XML_FROM_FTP as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            b"send failed\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        close((*ctxt).controlFd);
                        (*ctxt).controlFd = INVALID_SOCKET;
                        (*ctxt).controlFd = INVALID_SOCKET;
                        return res;
                    }
                    res = xmlNanoFTPGetResponse(ctxt as *mut ::core::ffi::c_void);
                    if res == 2 as ::core::ffi::c_int {
                        proxyType = 1 as ::core::ffi::c_int;
                        break 's_698;
                    } else if proxyType == 1 as ::core::ffi::c_int {
                        close((*ctxt).controlFd);
                        (*ctxt).controlFd = INVALID_SOCKET;
                        (*ctxt).controlFd = INVALID_SOCKET;
                        return -(1 as ::core::ffi::c_int);
                    }
                    current_block_186 = 3477261730121048860;
                }
                2 => {
                    current_block_186 = 3477261730121048860;
                }
                3 | _ => {
                    current_block_186 = 832791514371635938;
                }
            }
            match current_block_186 {
                3477261730121048860 => {
                    if (*ctxt).user.is_null() {
                        snprintf(
                            &raw mut buf as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 400]>() as size_t,
                            b"USER anonymous@%s\r\n\0" as *const u8 as *const ::core::ffi::c_char,
                            (*ctxt).hostname,
                        );
                    } else {
                        snprintf(
                            &raw mut buf as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 400]>() as size_t,
                            b"USER %s@%s\r\n\0" as *const u8 as *const ::core::ffi::c_char,
                            (*ctxt).user,
                            (*ctxt).hostname,
                        );
                    }
                    buf[(::core::mem::size_of::<[::core::ffi::c_char; 400]>() as usize)
                        .wrapping_sub(1 as usize) as usize] = 0 as ::core::ffi::c_char;
                    len = strlen(&raw mut buf as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
                    res = send(
                        (*ctxt).controlFd,
                        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                        len as size_t,
                        0 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int;
                    if res < 0 as ::core::ffi::c_int {
                        __xmlIOErr(
                            XML_FROM_FTP as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            b"send failed\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        close((*ctxt).controlFd);
                        (*ctxt).controlFd = INVALID_SOCKET;
                        (*ctxt).controlFd = INVALID_SOCKET;
                        return res;
                    }
                    res = xmlNanoFTPGetResponse(ctxt as *mut ::core::ffi::c_void);
                    if res == 1 as ::core::ffi::c_int || res == 2 as ::core::ffi::c_int {
                        proxyType = 2 as ::core::ffi::c_int;
                        return 0 as ::core::ffi::c_int;
                    }
                    if (*ctxt).passwd.is_null() {
                        snprintf(
                            &raw mut buf as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 400]>() as size_t,
                            b"PASS anonymous@\r\n\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                        snprintf(
                            &raw mut buf as *mut ::core::ffi::c_char,
                            ::core::mem::size_of::<[::core::ffi::c_char; 400]>() as size_t,
                            b"PASS %s\r\n\0" as *const u8 as *const ::core::ffi::c_char,
                            (*ctxt).passwd,
                        );
                    }
                    buf[(::core::mem::size_of::<[::core::ffi::c_char; 400]>() as usize)
                        .wrapping_sub(1 as usize) as usize] = 0 as ::core::ffi::c_char;
                    len = strlen(&raw mut buf as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
                    res = send(
                        (*ctxt).controlFd,
                        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                        len as size_t,
                        0 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_int;
                    if res < 0 as ::core::ffi::c_int {
                        __xmlIOErr(
                            XML_FROM_FTP as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            b"send failed\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        close((*ctxt).controlFd);
                        (*ctxt).controlFd = INVALID_SOCKET;
                        (*ctxt).controlFd = INVALID_SOCKET;
                        return res;
                    }
                    res = xmlNanoFTPGetResponse(ctxt as *mut ::core::ffi::c_void);
                    if res == 1 as ::core::ffi::c_int || res == 2 as ::core::ffi::c_int {
                        proxyType = 2 as ::core::ffi::c_int;
                        return 0 as ::core::ffi::c_int;
                    }
                    if proxyType == 2 as ::core::ffi::c_int {
                        close((*ctxt).controlFd);
                        (*ctxt).controlFd = INVALID_SOCKET;
                        (*ctxt).controlFd = INVALID_SOCKET;
                        return -(1 as ::core::ffi::c_int);
                    }
                }
                _ => {}
            }
            close((*ctxt).controlFd);
            (*ctxt).controlFd = INVALID_SOCKET;
            (*ctxt).controlFd = INVALID_SOCKET;
            return -(1 as ::core::ffi::c_int);
        }
    }
    res = xmlNanoFTPSendUser(ctxt as *mut ::core::ffi::c_void);
    if res < 0 as ::core::ffi::c_int {
        close((*ctxt).controlFd);
        (*ctxt).controlFd = INVALID_SOCKET;
        (*ctxt).controlFd = INVALID_SOCKET;
        return -(1 as ::core::ffi::c_int);
    }
    res = xmlNanoFTPGetResponse(ctxt as *mut ::core::ffi::c_void);
    match res {
        2 => return 0 as ::core::ffi::c_int,
        3 => {}
        1 | 4 | 5 | -1 | _ => {
            close((*ctxt).controlFd);
            (*ctxt).controlFd = INVALID_SOCKET;
            (*ctxt).controlFd = INVALID_SOCKET;
            return -(1 as ::core::ffi::c_int);
        }
    }
    res = xmlNanoFTPSendPasswd(ctxt as *mut ::core::ffi::c_void);
    if res < 0 as ::core::ffi::c_int {
        close((*ctxt).controlFd);
        (*ctxt).controlFd = INVALID_SOCKET;
        (*ctxt).controlFd = INVALID_SOCKET;
        return -(1 as ::core::ffi::c_int);
    }
    res = xmlNanoFTPGetResponse(ctxt as *mut ::core::ffi::c_void);
    's_825: {
        match res {
            2 => {
                break 's_825;
            }
            3 => {
                __xmlIOErr(
                    XML_FROM_FTP as ::core::ffi::c_int,
                    XML_FTP_ACCNT as ::core::ffi::c_int,
                    b"FTP server asking for ACCNT on anonymous\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            1 | 4 | 5 | -1 | _ => {}
        }
        close((*ctxt).controlFd);
        (*ctxt).controlFd = INVALID_SOCKET;
        (*ctxt).controlFd = INVALID_SOCKET;
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPConnectTo(
    mut server: *const ::core::ffi::c_char,
    mut port: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut ctxt: xmlNanoFTPCtxtPtr = ::core::ptr::null_mut::<xmlNanoFTPCtxt>();
    let mut res: ::core::ffi::c_int = 0;
    if deny_network_host(server) {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    xmlNanoFTPInit();
    if server.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if port <= 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    ctxt = xmlNanoFTPNewCtxt(::core::ptr::null::<::core::ffi::c_char>()) as xmlNanoFTPCtxtPtr;
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    (*ctxt).hostname = xmlMemStrdup.expect("non-null function pointer")(server);
    if (*ctxt).hostname.is_null() {
        xmlNanoFTPFreeCtxt(ctxt as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    (*ctxt).port = port;
    res = xmlNanoFTPConnect(ctxt as *mut ::core::ffi::c_void);
    if res < 0 as ::core::ffi::c_int {
        xmlNanoFTPFreeCtxt(ctxt as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return ctxt as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPCwd(
    mut ctx: *mut ::core::ffi::c_void,
    mut directory: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ctxt: xmlNanoFTPCtxtPtr = ctx as xmlNanoFTPCtxtPtr;
    let mut buf: [::core::ffi::c_char; 400] = [0; 400];
    let mut len: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 0;
    if ctxt.is_null() || (*ctxt).controlFd == INVALID_SOCKET {
        return -(1 as ::core::ffi::c_int);
    }
    if directory.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 400]>() as size_t,
        b"CWD %s\r\n\0" as *const u8 as *const ::core::ffi::c_char,
        directory,
    );
    buf[(::core::mem::size_of::<[::core::ffi::c_char; 400]>() as usize).wrapping_sub(1 as usize)
        as usize] = 0 as ::core::ffi::c_char;
    len = strlen(&raw mut buf as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
    res = send(
        (*ctxt).controlFd,
        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        len as size_t,
        0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    if res < 0 as ::core::ffi::c_int {
        __xmlIOErr(
            XML_FROM_FTP as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            b"send failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return res;
    }
    res = xmlNanoFTPGetResponse(ctxt as *mut ::core::ffi::c_void);
    if res == 4 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if res == 2 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    if res == 5 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPDele(
    mut ctx: *mut ::core::ffi::c_void,
    mut file: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ctxt: xmlNanoFTPCtxtPtr = ctx as xmlNanoFTPCtxtPtr;
    let mut buf: [::core::ffi::c_char; 400] = [0; 400];
    let mut len: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 0;
    if ctxt.is_null() || (*ctxt).controlFd == INVALID_SOCKET || file.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 400]>() as size_t,
        b"DELE %s\r\n\0" as *const u8 as *const ::core::ffi::c_char,
        file,
    );
    buf[(::core::mem::size_of::<[::core::ffi::c_char; 400]>() as usize).wrapping_sub(1 as usize)
        as usize] = 0 as ::core::ffi::c_char;
    len = strlen(&raw mut buf as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
    res = send(
        (*ctxt).controlFd,
        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        len as size_t,
        0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    if res < 0 as ::core::ffi::c_int {
        __xmlIOErr(
            XML_FROM_FTP as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            b"send failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return res;
    }
    res = xmlNanoFTPGetResponse(ctxt as *mut ::core::ffi::c_void);
    if res == 4 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if res == 2 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    if res == 5 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPGetConnection(
    mut ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctxt: xmlNanoFTPCtxtPtr = ctx as xmlNanoFTPCtxtPtr;
    let mut buf: [::core::ffi::c_char; 200] = [0; 200];
    let mut cur: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 0;
    let mut ad: [::core::ffi::c_uchar; 6] = [0; 6];
    let mut adp: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut portp: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut temp: [::core::ffi::c_uint; 6] = [0; 6];
    let mut dataAddr: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut dataAddrLen: socklen_t = 0;
    if ctxt.is_null() {
        return INVALID_SOCKET;
    }
    memset(
        &raw mut dataAddr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_storage>() as size_t,
    );
    if (*ctxt).ftpAddr.ss_family as ::core::ffi::c_int == AF_INET6 {
        (*ctxt).dataFd = socket(
            AF_INET6,
            SOCK_STREAM as ::core::ffi::c_int,
            IPPROTO_TCP as ::core::ffi::c_int,
        );
        (*(&raw mut dataAddr as *mut sockaddr_in6)).sin6_family = AF_INET6 as sa_family_t;
        dataAddrLen = ::core::mem::size_of::<sockaddr_in6>() as socklen_t;
    } else {
        (*ctxt).dataFd = socket(
            AF_INET,
            SOCK_STREAM as ::core::ffi::c_int,
            IPPROTO_TCP as ::core::ffi::c_int,
        );
        (*(&raw mut dataAddr as *mut sockaddr_in)).sin_family = AF_INET as sa_family_t;
        dataAddrLen = ::core::mem::size_of::<sockaddr_in>() as socklen_t;
    }
    if (*ctxt).dataFd == INVALID_SOCKET {
        __xmlIOErr(
            XML_FROM_FTP as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            b"socket failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return INVALID_SOCKET;
    }
    if (*ctxt).passive != 0 {
        if (*ctxt).ftpAddr.ss_family as ::core::ffi::c_int == AF_INET6 {
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 200]>() as size_t,
                b"EPSV\r\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        } else {
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 200]>() as size_t,
                b"PASV\r\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        len = strlen(&raw mut buf as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
        res = send(
            (*ctxt).controlFd,
            &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            len as size_t,
            0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        if res < 0 as ::core::ffi::c_int {
            __xmlIOErr(
                XML_FROM_FTP as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"send failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
            close((*ctxt).dataFd);
            (*ctxt).dataFd = INVALID_SOCKET;
            return INVALID_SOCKET;
        }
        res = xmlNanoFTPReadResponse(ctx);
        if res != 2 as ::core::ffi::c_int {
            if res == 5 as ::core::ffi::c_int {
                close((*ctxt).dataFd);
                (*ctxt).dataFd = INVALID_SOCKET;
                return INVALID_SOCKET;
            } else {
                close((*ctxt).dataFd);
                (*ctxt).dataFd = INVALID_SOCKET;
                (*ctxt).passive = 0 as ::core::ffi::c_int;
            }
        }
        cur = (&raw mut (*ctxt).controlBuf as *mut ::core::ffi::c_char)
            .offset((*ctxt).controlBufAnswer as isize) as *mut ::core::ffi::c_char;
        while ((*cur as ::core::ffi::c_int) < '0' as i32 || *cur as ::core::ffi::c_int > '9' as i32)
            && *cur as ::core::ffi::c_int != '\0' as i32
        {
            cur = cur.offset(1);
        }
        if (*ctxt).ftpAddr.ss_family as ::core::ffi::c_int == AF_INET6 {
            if sscanf(
                cur,
                b"%u\0" as *const u8 as *const ::core::ffi::c_char,
                (&raw mut temp as *mut ::core::ffi::c_uint).offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_uint,
            ) != 1 as ::core::ffi::c_int
            {
                __xmlIOErr(
                    XML_FROM_FTP as ::core::ffi::c_int,
                    XML_FTP_EPSV_ANSWER as ::core::ffi::c_int,
                    b"Invalid answer to EPSV\n\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if (*ctxt).dataFd != INVALID_SOCKET {
                    close((*ctxt).dataFd);
                    (*ctxt).dataFd = INVALID_SOCKET;
                }
                return INVALID_SOCKET;
            }
            memcpy(
                &raw mut (*(&raw mut dataAddr as *mut sockaddr_in6)).sin6_addr
                    as *mut ::core::ffi::c_void,
                &raw mut (*(&raw mut (*ctxt).ftpAddr as *mut sockaddr_in6)).sin6_addr
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<in6_addr>() as size_t,
            );
            (*(&raw mut dataAddr as *mut sockaddr_in6)).sin6_port =
                htons(temp[0 as ::core::ffi::c_int as usize] as uint16_t) as in_port_t;
        } else {
            if sscanf(
                cur,
                b"%u,%u,%u,%u,%u,%u\0" as *const u8 as *const ::core::ffi::c_char,
                (&raw mut temp as *mut ::core::ffi::c_uint).offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_uint,
                (&raw mut temp as *mut ::core::ffi::c_uint).offset(1 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_uint,
                (&raw mut temp as *mut ::core::ffi::c_uint).offset(2 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_uint,
                (&raw mut temp as *mut ::core::ffi::c_uint).offset(3 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_uint,
                (&raw mut temp as *mut ::core::ffi::c_uint).offset(4 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_uint,
                (&raw mut temp as *mut ::core::ffi::c_uint).offset(5 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_uint,
            ) != 6 as ::core::ffi::c_int
            {
                __xmlIOErr(
                    XML_FROM_FTP as ::core::ffi::c_int,
                    XML_FTP_PASV_ANSWER as ::core::ffi::c_int,
                    b"Invalid answer to PASV\n\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if (*ctxt).dataFd != INVALID_SOCKET {
                    close((*ctxt).dataFd);
                    (*ctxt).dataFd = INVALID_SOCKET;
                }
                return INVALID_SOCKET;
            }
            i = 0 as ::core::ffi::c_int;
            while i < 6 as ::core::ffi::c_int {
                ad[i as usize] =
                    (temp[i as usize] & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_uchar;
                i += 1;
            }
            memcpy(
                &raw mut (*(&raw mut dataAddr as *mut sockaddr_in)).sin_addr
                    as *mut ::core::ffi::c_void,
                (&raw mut ad as *mut ::core::ffi::c_uchar).offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                4 as size_t,
            );
            memcpy(
                &raw mut (*(&raw mut dataAddr as *mut sockaddr_in)).sin_port
                    as *mut ::core::ffi::c_void,
                (&raw mut ad as *mut ::core::ffi::c_uchar).offset(4 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_uchar as *const ::core::ffi::c_void,
                2 as size_t,
            );
        }
        if connect(
            (*ctxt).dataFd,
            &raw mut dataAddr as *mut sockaddr,
            dataAddrLen,
        ) < 0 as ::core::ffi::c_int
        {
            __xmlIOErr(
                XML_FROM_FTP as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"Failed to create a data connection\0" as *const u8 as *const ::core::ffi::c_char,
            );
            close((*ctxt).dataFd);
            (*ctxt).dataFd = INVALID_SOCKET;
            return INVALID_SOCKET;
        }
    } else {
        getsockname(
            (*ctxt).dataFd,
            &raw mut dataAddr as *mut sockaddr,
            &raw mut dataAddrLen,
        );
        if (*ctxt).ftpAddr.ss_family as ::core::ffi::c_int == AF_INET6 {
            (*(&raw mut dataAddr as *mut sockaddr_in6)).sin6_port = 0 as in_port_t;
        } else {
            (*(&raw mut dataAddr as *mut sockaddr_in)).sin_port = 0 as in_port_t;
        }
        if bind(
            (*ctxt).dataFd,
            &raw mut dataAddr as *mut sockaddr,
            dataAddrLen,
        ) < 0 as ::core::ffi::c_int
        {
            __xmlIOErr(
                XML_FROM_FTP as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"bind failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
            close((*ctxt).dataFd);
            (*ctxt).dataFd = INVALID_SOCKET;
            return INVALID_SOCKET;
        }
        getsockname(
            (*ctxt).dataFd,
            &raw mut dataAddr as *mut sockaddr,
            &raw mut dataAddrLen,
        );
        if listen((*ctxt).dataFd, 1 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            __xmlIOErr(
                XML_FROM_FTP as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"listen failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
            close((*ctxt).dataFd);
            (*ctxt).dataFd = INVALID_SOCKET;
            return INVALID_SOCKET;
        }
        if (*ctxt).ftpAddr.ss_family as ::core::ffi::c_int == AF_INET6 {
            let mut buf6: [::core::ffi::c_char; 46] = [0; 46];
            inet_ntop(
                AF_INET6,
                &raw mut (*(&raw mut dataAddr as *mut sockaddr_in6)).sin6_addr
                    as *const ::core::ffi::c_void,
                &raw mut buf6 as *mut ::core::ffi::c_char,
                INET6_ADDRSTRLEN as socklen_t,
            );
            adp = &raw mut buf6 as *mut ::core::ffi::c_char as *mut ::core::ffi::c_uchar;
            portp = &raw mut (*(&raw mut dataAddr as *mut sockaddr_in6)).sin6_port
                as *mut ::core::ffi::c_uchar;
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 200]>() as size_t,
                b"EPRT |2|%s|%s|\r\n\0" as *const u8 as *const ::core::ffi::c_char,
                adp,
                portp,
            );
        } else {
            adp = &raw mut (*(&raw mut dataAddr as *mut sockaddr_in)).sin_addr
                as *mut ::core::ffi::c_uchar;
            portp = &raw mut (*(&raw mut dataAddr as *mut sockaddr_in)).sin_port
                as *mut ::core::ffi::c_uchar;
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 200]>() as size_t,
                b"PORT %d,%d,%d,%d,%d,%d\r\n\0" as *const u8 as *const ::core::ffi::c_char,
                *adp.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int,
                *adp.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int,
                *adp.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int,
                *adp.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int,
                *portp.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int,
                *portp.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0xff as ::core::ffi::c_int,
            );
        }
        buf[(::core::mem::size_of::<[::core::ffi::c_char; 200]>() as usize).wrapping_sub(1 as usize)
            as usize] = 0 as ::core::ffi::c_char;
        len = strlen(&raw mut buf as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
        res = send(
            (*ctxt).controlFd,
            &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            len as size_t,
            0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
        if res < 0 as ::core::ffi::c_int {
            __xmlIOErr(
                XML_FROM_FTP as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"send failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
            close((*ctxt).dataFd);
            (*ctxt).dataFd = INVALID_SOCKET;
            return INVALID_SOCKET;
        }
        res = xmlNanoFTPGetResponse(ctxt as *mut ::core::ffi::c_void);
        if res != 2 as ::core::ffi::c_int {
            close((*ctxt).dataFd);
            (*ctxt).dataFd = INVALID_SOCKET;
            return INVALID_SOCKET;
        }
    }
    return (*ctxt).dataFd;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPCloseConnection(
    mut ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctxt: xmlNanoFTPCtxtPtr = ctx as xmlNanoFTPCtxtPtr;
    let mut res: ::core::ffi::c_int = 0;
    let mut rfd: fd_set = fd_set {
        __fds_bits: [0; 16],
    };
    let mut efd: fd_set = fd_set {
        __fds_bits: [0; 16],
    };
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    if ctxt.is_null() || (*ctxt).controlFd == INVALID_SOCKET {
        return -(1 as ::core::ffi::c_int);
    }
    close((*ctxt).dataFd);
    (*ctxt).dataFd = INVALID_SOCKET;
    tv.tv_sec = 15 as __time_t;
    tv.tv_usec = 0 as __suseconds_t;
    let mut __i: ::core::ffi::c_uint = 0;
    let mut __arr: *mut fd_set = &raw mut rfd;
    __i = 0 as ::core::ffi::c_uint;
    while (__i as usize)
        < (::core::mem::size_of::<fd_set>() as usize)
            .wrapping_div(::core::mem::size_of::<__fd_mask>() as usize)
    {
        (*__arr).__fds_bits[__i as usize] = 0 as __fd_mask;
        __i = __i.wrapping_add(1);
    }
    rfd.__fds_bits[((*ctxt).controlFd / __NFDBITS) as usize] |=
        ((1 as ::core::ffi::c_ulong) << (*ctxt).controlFd % __NFDBITS) as __fd_mask;
    let mut __i_0: ::core::ffi::c_uint = 0;
    let mut __arr_0: *mut fd_set = &raw mut efd;
    __i_0 = 0 as ::core::ffi::c_uint;
    while (__i_0 as usize)
        < (::core::mem::size_of::<fd_set>() as usize)
            .wrapping_div(::core::mem::size_of::<__fd_mask>() as usize)
    {
        (*__arr_0).__fds_bits[__i_0 as usize] = 0 as __fd_mask;
        __i_0 = __i_0.wrapping_add(1);
    }
    efd.__fds_bits[((*ctxt).controlFd / __NFDBITS) as usize] |=
        ((1 as ::core::ffi::c_ulong) << (*ctxt).controlFd % __NFDBITS) as __fd_mask;
    res = select(
        (*ctxt).controlFd + 1 as ::core::ffi::c_int,
        &raw mut rfd,
        ::core::ptr::null_mut::<fd_set>(),
        &raw mut efd,
        &raw mut tv,
    );
    if res < 0 as ::core::ffi::c_int {
        close((*ctxt).controlFd);
        (*ctxt).controlFd = INVALID_SOCKET;
        return -(1 as ::core::ffi::c_int);
    }
    if res == 0 as ::core::ffi::c_int {
        close((*ctxt).controlFd);
        (*ctxt).controlFd = INVALID_SOCKET;
    } else {
        res = xmlNanoFTPGetResponse(ctxt as *mut ::core::ffi::c_void);
        if res != 2 as ::core::ffi::c_int {
            close((*ctxt).controlFd);
            (*ctxt).controlFd = INVALID_SOCKET;
            return -(1 as ::core::ffi::c_int);
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn xmlNanoFTPParseList(
    mut list: *const ::core::ffi::c_char,
    mut callback: ftpListCallback,
    mut userData: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut cur: *const ::core::ffi::c_char = list;
    let mut filename: [::core::ffi::c_char; 151] = [0; 151];
    let mut attrib: [::core::ffi::c_char; 11] = [0; 11];
    let mut owner: [::core::ffi::c_char; 11] = [0; 11];
    let mut group: [::core::ffi::c_char; 11] = [0; 11];
    let mut month: [::core::ffi::c_char; 4] = [0; 4];
    let mut year: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut minute: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut hour: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut day: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut size: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut links: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    if strncmp(
        cur,
        b"total\0" as *const u8 as *const ::core::ffi::c_char,
        5 as size_t,
    ) == 0
    {
        cur = cur.offset(5 as ::core::ffi::c_int as isize);
        while *cur as ::core::ffi::c_int == ' ' as i32 {
            cur = cur.offset(1);
        }
        while *cur as ::core::ffi::c_int >= '0' as i32 && *cur as ::core::ffi::c_int <= '9' as i32 {
            let fresh0 = cur;
            cur = cur.offset(1);
            links = links * 10 as ::core::ffi::c_int + (*fresh0 as ::core::ffi::c_int - '0' as i32);
        }
        while *cur as ::core::ffi::c_int == ' ' as i32
            || *cur as ::core::ffi::c_int == '\n' as i32
            || *cur as ::core::ffi::c_int == '\r' as i32
        {
            cur = cur.offset(1);
        }
        return cur.offset_from(list) as ::core::ffi::c_long as ::core::ffi::c_int;
    } else if *list as ::core::ffi::c_int == '+' as i32 {
        return 0 as ::core::ffi::c_int;
    } else {
        while *cur as ::core::ffi::c_int == ' ' as i32
            || *cur as ::core::ffi::c_int == '\n' as i32
            || *cur as ::core::ffi::c_int == '\r' as i32
        {
            cur = cur.offset(1);
        }
        if *cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while *cur as ::core::ffi::c_int != ' ' as i32 {
            if i < 10 as ::core::ffi::c_int {
                let fresh1 = i;
                i = i + 1;
                attrib[fresh1 as usize] = *cur;
            }
            cur = cur.offset(1);
            if *cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
        }
        attrib[10 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        while *cur as ::core::ffi::c_int == ' ' as i32 {
            cur = cur.offset(1);
        }
        if *cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        while *cur as ::core::ffi::c_int >= '0' as i32 && *cur as ::core::ffi::c_int <= '9' as i32 {
            let fresh2 = cur;
            cur = cur.offset(1);
            links = links * 10 as ::core::ffi::c_int + (*fresh2 as ::core::ffi::c_int - '0' as i32);
        }
        while *cur as ::core::ffi::c_int == ' ' as i32 {
            cur = cur.offset(1);
        }
        if *cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while *cur as ::core::ffi::c_int != ' ' as i32 {
            if i < 10 as ::core::ffi::c_int {
                let fresh3 = i;
                i = i + 1;
                owner[fresh3 as usize] = *cur;
            }
            cur = cur.offset(1);
            if *cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
        }
        owner[i as usize] = 0 as ::core::ffi::c_char;
        while *cur as ::core::ffi::c_int == ' ' as i32 {
            cur = cur.offset(1);
        }
        if *cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while *cur as ::core::ffi::c_int != ' ' as i32 {
            if i < 10 as ::core::ffi::c_int {
                let fresh4 = i;
                i = i + 1;
                group[fresh4 as usize] = *cur;
            }
            cur = cur.offset(1);
            if *cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
        }
        group[i as usize] = 0 as ::core::ffi::c_char;
        while *cur as ::core::ffi::c_int == ' ' as i32 {
            cur = cur.offset(1);
        }
        if *cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        while *cur as ::core::ffi::c_int >= '0' as i32 && *cur as ::core::ffi::c_int <= '9' as i32 {
            let fresh5 = cur;
            cur = cur.offset(1);
            size = size
                .wrapping_mul(10 as ::core::ffi::c_ulong)
                .wrapping_add((*fresh5 as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_ulong);
        }
        while *cur as ::core::ffi::c_int == ' ' as i32 {
            cur = cur.offset(1);
        }
        if *cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while *cur as ::core::ffi::c_int != ' ' as i32 {
            if i < 3 as ::core::ffi::c_int {
                let fresh6 = i;
                i = i + 1;
                month[fresh6 as usize] = *cur;
            }
            cur = cur.offset(1);
            if *cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
        }
        month[i as usize] = 0 as ::core::ffi::c_char;
        while *cur as ::core::ffi::c_int == ' ' as i32 {
            cur = cur.offset(1);
        }
        if *cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        while *cur as ::core::ffi::c_int >= '0' as i32 && *cur as ::core::ffi::c_int <= '9' as i32 {
            let fresh7 = cur;
            cur = cur.offset(1);
            day = day * 10 as ::core::ffi::c_int + (*fresh7 as ::core::ffi::c_int - '0' as i32);
        }
        while *cur as ::core::ffi::c_int == ' ' as i32 {
            cur = cur.offset(1);
        }
        if *cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        if *cur.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
            || *cur.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        if *cur.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ':' as i32
            || *cur.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == ':' as i32
        {
            while *cur as ::core::ffi::c_int >= '0' as i32
                && *cur as ::core::ffi::c_int <= '9' as i32
            {
                let fresh8 = cur;
                cur = cur.offset(1);
                hour =
                    hour * 10 as ::core::ffi::c_int + (*fresh8 as ::core::ffi::c_int - '0' as i32);
            }
            if *cur as ::core::ffi::c_int == ':' as i32 {
                cur = cur.offset(1);
            }
            while *cur as ::core::ffi::c_int >= '0' as i32
                && *cur as ::core::ffi::c_int <= '9' as i32
            {
                let fresh9 = cur;
                cur = cur.offset(1);
                minute = minute * 10 as ::core::ffi::c_int
                    + (*fresh9 as ::core::ffi::c_int - '0' as i32);
            }
        } else {
            while *cur as ::core::ffi::c_int >= '0' as i32
                && *cur as ::core::ffi::c_int <= '9' as i32
            {
                let fresh10 = cur;
                cur = cur.offset(1);
                year =
                    year * 10 as ::core::ffi::c_int + (*fresh10 as ::core::ffi::c_int - '0' as i32);
            }
        }
        while *cur as ::core::ffi::c_int == ' ' as i32 {
            cur = cur.offset(1);
        }
        if *cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            return 0 as ::core::ffi::c_int;
        }
        i = 0 as ::core::ffi::c_int;
        while *cur as ::core::ffi::c_int != '\n' as i32 && *cur as ::core::ffi::c_int != '\r' as i32
        {
            if i < 150 as ::core::ffi::c_int {
                let fresh11 = i;
                i = i + 1;
                filename[fresh11 as usize] = *cur;
            }
            cur = cur.offset(1);
            if *cur as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
        }
        filename[i as usize] = 0 as ::core::ffi::c_char;
        if *cur as ::core::ffi::c_int != '\n' as i32 && *cur as ::core::ffi::c_int != '\r' as i32 {
            return 0 as ::core::ffi::c_int;
        }
        while *cur as ::core::ffi::c_int == '\n' as i32 || *cur as ::core::ffi::c_int == '\r' as i32
        {
            cur = cur.offset(1);
        }
    }
    if callback.is_some() {
        callback.expect("non-null function pointer")(
            userData,
            &raw mut filename as *mut ::core::ffi::c_char,
            &raw mut attrib as *mut ::core::ffi::c_char,
            &raw mut owner as *mut ::core::ffi::c_char,
            &raw mut group as *mut ::core::ffi::c_char,
            size,
            links,
            year,
            &raw mut month as *mut ::core::ffi::c_char,
            day,
            hour,
            minute,
        );
    }
    return cur.offset_from(list) as ::core::ffi::c_long as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPList(
    mut ctx: *mut ::core::ffi::c_void,
    mut callback: ftpListCallback,
    mut userData: *mut ::core::ffi::c_void,
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ctxt: xmlNanoFTPCtxtPtr = ctx as xmlNanoFTPCtxtPtr;
    let mut buf: [::core::ffi::c_char; 4097] = [0; 4097];
    let mut len: ::core::ffi::c_int = 0;
    let mut res: ::core::ffi::c_int = 0;
    let mut indx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut base: ::core::ffi::c_int = 0;
    let mut rfd: fd_set = fd_set {
        __fds_bits: [0; 16],
    };
    let mut efd: fd_set = fd_set {
        __fds_bits: [0; 16],
    };
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    if ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if filename.is_null() {
        if xmlNanoFTPCwd(ctxt as *mut ::core::ffi::c_void, (*ctxt).path) < 1 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        (*ctxt).dataFd = xmlNanoFTPGetConnection(ctxt as *mut ::core::ffi::c_void);
        if (*ctxt).dataFd == INVALID_SOCKET {
            return -(1 as ::core::ffi::c_int);
        }
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4097]>() as size_t,
            b"LIST -L\r\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        if *filename.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '/' as i32 {
            if xmlNanoFTPCwd(ctxt as *mut ::core::ffi::c_void, (*ctxt).path)
                < 1 as ::core::ffi::c_int
            {
                return -(1 as ::core::ffi::c_int);
            }
        }
        (*ctxt).dataFd = xmlNanoFTPGetConnection(ctxt as *mut ::core::ffi::c_void);
        if (*ctxt).dataFd == INVALID_SOCKET {
            return -(1 as ::core::ffi::c_int);
        }
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4097]>() as size_t,
            b"LIST -L %s\r\n\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
        );
    }
    buf[(::core::mem::size_of::<[::core::ffi::c_char; 4097]>() as usize).wrapping_sub(1 as usize)
        as usize] = 0 as ::core::ffi::c_char;
    len = strlen(&raw mut buf as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
    res = send(
        (*ctxt).controlFd,
        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        len as size_t,
        0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    if res < 0 as ::core::ffi::c_int {
        __xmlIOErr(
            XML_FROM_FTP as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            b"send failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        close((*ctxt).dataFd);
        (*ctxt).dataFd = INVALID_SOCKET;
        return res;
    }
    res = xmlNanoFTPReadResponse(ctxt as *mut ::core::ffi::c_void);
    if res != 1 as ::core::ffi::c_int {
        close((*ctxt).dataFd);
        (*ctxt).dataFd = INVALID_SOCKET;
        return -res;
    }
    loop {
        tv.tv_sec = 1 as __time_t;
        tv.tv_usec = 0 as __suseconds_t;
        let mut __i: ::core::ffi::c_uint = 0;
        let mut __arr: *mut fd_set = &raw mut rfd;
        __i = 0 as ::core::ffi::c_uint;
        while (__i as usize)
            < (::core::mem::size_of::<fd_set>() as usize)
                .wrapping_div(::core::mem::size_of::<__fd_mask>() as usize)
        {
            (*__arr).__fds_bits[__i as usize] = 0 as __fd_mask;
            __i = __i.wrapping_add(1);
        }
        rfd.__fds_bits[((*ctxt).dataFd / __NFDBITS) as usize] |=
            ((1 as ::core::ffi::c_ulong) << (*ctxt).dataFd % __NFDBITS) as __fd_mask;
        let mut __i_0: ::core::ffi::c_uint = 0;
        let mut __arr_0: *mut fd_set = &raw mut efd;
        __i_0 = 0 as ::core::ffi::c_uint;
        while (__i_0 as usize)
            < (::core::mem::size_of::<fd_set>() as usize)
                .wrapping_div(::core::mem::size_of::<__fd_mask>() as usize)
        {
            (*__arr_0).__fds_bits[__i_0 as usize] = 0 as __fd_mask;
            __i_0 = __i_0.wrapping_add(1);
        }
        efd.__fds_bits[((*ctxt).dataFd / __NFDBITS) as usize] |=
            ((1 as ::core::ffi::c_ulong) << (*ctxt).dataFd % __NFDBITS) as __fd_mask;
        res = select(
            (*ctxt).dataFd + 1 as ::core::ffi::c_int,
            &raw mut rfd,
            ::core::ptr::null_mut::<fd_set>(),
            &raw mut efd,
            &raw mut tv,
        );
        if res < 0 as ::core::ffi::c_int {
            close((*ctxt).dataFd);
            (*ctxt).dataFd = INVALID_SOCKET;
            return -(1 as ::core::ffi::c_int);
        }
        if res == 0 as ::core::ffi::c_int {
            res = xmlNanoFTPCheckResponse(ctxt as *mut ::core::ffi::c_void);
            if res < 0 as ::core::ffi::c_int {
                close((*ctxt).dataFd);
                (*ctxt).dataFd = INVALID_SOCKET;
                (*ctxt).dataFd = INVALID_SOCKET;
                return -(1 as ::core::ffi::c_int);
            }
            if res == 2 as ::core::ffi::c_int {
                close((*ctxt).dataFd);
                (*ctxt).dataFd = INVALID_SOCKET;
                return 0 as ::core::ffi::c_int;
            }
        } else {
            len = recv(
                (*ctxt).dataFd,
                (&raw mut buf as *mut ::core::ffi::c_char).offset(indx as isize)
                    as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                (::core::mem::size_of::<[::core::ffi::c_char; 4097]>() as size_t)
                    .wrapping_sub((indx + 1 as ::core::ffi::c_int) as size_t),
                0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int;
            if len < 0 as ::core::ffi::c_int {
                __xmlIOErr(
                    XML_FROM_FTP as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    b"recv\0" as *const u8 as *const ::core::ffi::c_char,
                );
                close((*ctxt).dataFd);
                (*ctxt).dataFd = INVALID_SOCKET;
                (*ctxt).dataFd = INVALID_SOCKET;
                return -(1 as ::core::ffi::c_int);
            }
            indx += len;
            buf[indx as usize] = 0 as ::core::ffi::c_char;
            base = 0 as ::core::ffi::c_int;
            loop {
                res = xmlNanoFTPParseList(
                    (&raw mut buf as *mut ::core::ffi::c_char).offset(base as isize)
                        as *mut ::core::ffi::c_char,
                    callback,
                    userData,
                );
                base += res;
                if !(res > 0 as ::core::ffi::c_int) {
                    break;
                }
            }
            memmove(
                (&raw mut buf as *mut ::core::ffi::c_char).offset(0 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                (&raw mut buf as *mut ::core::ffi::c_char).offset(base as isize)
                    as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                (indx - base) as size_t,
            );
            indx -= base;
        }
        if !(len != 0 as ::core::ffi::c_int) {
            break;
        }
    }
    xmlNanoFTPCloseConnection(ctxt as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPGetSocket(
    mut ctx: *mut ::core::ffi::c_void,
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ctxt: xmlNanoFTPCtxtPtr = ctx as xmlNanoFTPCtxtPtr;
    let mut buf: [::core::ffi::c_char; 300] = [0; 300];
    let mut res: ::core::ffi::c_int = 0;
    let mut len: ::core::ffi::c_int = 0;
    if ctx.is_null() {
        return INVALID_SOCKET;
    }
    if filename.is_null() && (*ctxt).path.is_null() {
        return INVALID_SOCKET;
    }
    (*ctxt).dataFd = xmlNanoFTPGetConnection(ctxt as *mut ::core::ffi::c_void);
    if (*ctxt).dataFd == INVALID_SOCKET {
        return INVALID_SOCKET;
    }
    snprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 300]>() as size_t,
        b"TYPE I\r\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    len = strlen(&raw mut buf as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
    res = send(
        (*ctxt).controlFd,
        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        len as size_t,
        0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    if res < 0 as ::core::ffi::c_int {
        __xmlIOErr(
            XML_FROM_FTP as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            b"send failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        close((*ctxt).dataFd);
        (*ctxt).dataFd = INVALID_SOCKET;
        return INVALID_SOCKET;
    }
    res = xmlNanoFTPReadResponse(ctxt as *mut ::core::ffi::c_void);
    if res != 2 as ::core::ffi::c_int {
        close((*ctxt).dataFd);
        (*ctxt).dataFd = INVALID_SOCKET;
        return INVALID_SOCKET;
    }
    if filename.is_null() {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 300]>() as size_t,
            b"RETR %s\r\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*ctxt).path,
        );
    } else {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 300]>() as size_t,
            b"RETR %s\r\n\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
        );
    }
    buf[(::core::mem::size_of::<[::core::ffi::c_char; 300]>() as usize).wrapping_sub(1 as usize)
        as usize] = 0 as ::core::ffi::c_char;
    len = strlen(&raw mut buf as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
    res = send(
        (*ctxt).controlFd,
        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        len as size_t,
        0 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    if res < 0 as ::core::ffi::c_int {
        __xmlIOErr(
            XML_FROM_FTP as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            b"send failed\0" as *const u8 as *const ::core::ffi::c_char,
        );
        close((*ctxt).dataFd);
        (*ctxt).dataFd = INVALID_SOCKET;
        return INVALID_SOCKET;
    }
    res = xmlNanoFTPReadResponse(ctxt as *mut ::core::ffi::c_void);
    if res != 1 as ::core::ffi::c_int {
        close((*ctxt).dataFd);
        (*ctxt).dataFd = INVALID_SOCKET;
        return INVALID_SOCKET;
    }
    return (*ctxt).dataFd;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPGet(
    mut ctx: *mut ::core::ffi::c_void,
    mut callback: ftpDataCallback,
    mut userData: *mut ::core::ffi::c_void,
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ctxt: xmlNanoFTPCtxtPtr = ctx as xmlNanoFTPCtxtPtr;
    let mut buf: [::core::ffi::c_char; 4096] = [0; 4096];
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut res: ::core::ffi::c_int = 0;
    let mut rfd: fd_set = fd_set {
        __fds_bits: [0; 16],
    };
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    if ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if filename.is_null() && (*ctxt).path.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if callback.is_none() {
        return -(1 as ::core::ffi::c_int);
    }
    if xmlNanoFTPGetSocket(ctxt as *mut ::core::ffi::c_void, filename) == INVALID_SOCKET {
        return -(1 as ::core::ffi::c_int);
    }
    loop {
        tv.tv_sec = 1 as __time_t;
        tv.tv_usec = 0 as __suseconds_t;
        let mut __i: ::core::ffi::c_uint = 0;
        let mut __arr: *mut fd_set = &raw mut rfd;
        __i = 0 as ::core::ffi::c_uint;
        while (__i as usize)
            < (::core::mem::size_of::<fd_set>() as usize)
                .wrapping_div(::core::mem::size_of::<__fd_mask>() as usize)
        {
            (*__arr).__fds_bits[__i as usize] = 0 as __fd_mask;
            __i = __i.wrapping_add(1);
        }
        rfd.__fds_bits[((*ctxt).dataFd / __NFDBITS) as usize] |=
            ((1 as ::core::ffi::c_ulong) << (*ctxt).dataFd % __NFDBITS) as __fd_mask;
        res = select(
            (*ctxt).dataFd + 1 as ::core::ffi::c_int,
            &raw mut rfd,
            ::core::ptr::null_mut::<fd_set>(),
            ::core::ptr::null_mut::<fd_set>(),
            &raw mut tv,
        );
        if res < 0 as ::core::ffi::c_int {
            close((*ctxt).dataFd);
            (*ctxt).dataFd = INVALID_SOCKET;
            return -(1 as ::core::ffi::c_int);
        }
        if res == 0 as ::core::ffi::c_int {
            res = xmlNanoFTPCheckResponse(ctxt as *mut ::core::ffi::c_void);
            if res < 0 as ::core::ffi::c_int {
                close((*ctxt).dataFd);
                (*ctxt).dataFd = INVALID_SOCKET;
                (*ctxt).dataFd = INVALID_SOCKET;
                return -(1 as ::core::ffi::c_int);
            }
            if res == 2 as ::core::ffi::c_int {
                close((*ctxt).dataFd);
                (*ctxt).dataFd = INVALID_SOCKET;
                return 0 as ::core::ffi::c_int;
            }
        } else {
            len = recv(
                (*ctxt).dataFd,
                &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>() as size_t,
                0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int;
            if len < 0 as ::core::ffi::c_int {
                __xmlIOErr(
                    XML_FROM_FTP as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    b"recv failed\0" as *const u8 as *const ::core::ffi::c_char,
                );
                callback.expect("non-null function pointer")(
                    userData,
                    &raw mut buf as *mut ::core::ffi::c_char,
                    len,
                );
                close((*ctxt).dataFd);
                (*ctxt).dataFd = INVALID_SOCKET;
                return -(1 as ::core::ffi::c_int);
            }
            callback.expect("non-null function pointer")(
                userData,
                &raw mut buf as *mut ::core::ffi::c_char,
                len,
            );
        }
        if !(len != 0 as ::core::ffi::c_int) {
            break;
        }
    }
    return xmlNanoFTPCloseConnection(ctxt as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPRead(
    mut ctx: *mut ::core::ffi::c_void,
    mut dest: *mut ::core::ffi::c_void,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ctxt: xmlNanoFTPCtxtPtr = ctx as xmlNanoFTPCtxtPtr;
    if ctx.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*ctxt).dataFd == INVALID_SOCKET {
        return 0 as ::core::ffi::c_int;
    }
    if dest.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if len <= 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    len = recv((*ctxt).dataFd, dest, len as size_t, 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if len <= 0 as ::core::ffi::c_int {
        if len < 0 as ::core::ffi::c_int {
            __xmlIOErr(
                XML_FROM_FTP as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"recv failed\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
        xmlNanoFTPCloseConnection(ctxt as *mut ::core::ffi::c_void);
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPOpen(
    mut URL: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    let mut ctxt: xmlNanoFTPCtxtPtr = ::core::ptr::null_mut::<xmlNanoFTPCtxt>();
    let mut sock: ::core::ffi::c_int = 0;
    if unsafe { deny_network_uri(URL) } {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    xmlNanoFTPInit();
    if URL.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if strncmp(
        b"ftp://\0" as *const u8 as *const ::core::ffi::c_char,
        URL,
        6 as size_t,
    ) != 0
    {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    ctxt = xmlNanoFTPNewCtxt(URL) as xmlNanoFTPCtxtPtr;
    if ctxt.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    if xmlNanoFTPConnect(ctxt as *mut ::core::ffi::c_void) < 0 as ::core::ffi::c_int {
        xmlNanoFTPFreeCtxt(ctxt as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    sock = xmlNanoFTPGetSocket(ctxt as *mut ::core::ffi::c_void, (*ctxt).path);
    if sock == INVALID_SOCKET {
        xmlNanoFTPFreeCtxt(ctxt as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return ctxt as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn xmlNanoFTPClose(mut ctx: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut ctxt: xmlNanoFTPCtxtPtr = ctx as xmlNanoFTPCtxtPtr;
    if ctxt.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if (*ctxt).dataFd != INVALID_SOCKET {
        close((*ctxt).dataFd);
        (*ctxt).dataFd = INVALID_SOCKET;
    }
    if (*ctxt).controlFd != INVALID_SOCKET {
        xmlNanoFTPQuit(ctxt as *mut ::core::ffi::c_void);
        close((*ctxt).controlFd);
        (*ctxt).controlFd = INVALID_SOCKET;
    }
    xmlNanoFTPFreeCtxt(ctxt as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
