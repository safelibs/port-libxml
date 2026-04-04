#![allow(non_camel_case_types)]

macro_rules! opaque_type {
    ($name:ident) => {
        #[repr(C)]
        pub struct $name {
            _private: [u8; 0],
        }
    };
}

opaque_type!(_IO_wide_data);
opaque_type!(_IO_codecvt);
opaque_type!(_IO_marker);
opaque_type!(_xmlBuf);
opaque_type!(_xmlDict);
opaque_type!(_xmlHashTable);
opaque_type!(_xmlStartTag);
opaque_type!(_xmlAutomataState);
opaque_type!(_xmlAutomata);
opaque_type!(_xmlValidState);
opaque_type!(_xmlRegexp);
opaque_type!(_xmlRegExecCtxt);
opaque_type!(_xmlLink);
opaque_type!(_xmlList);
opaque_type!(_xmlRMutex);
opaque_type!(_xmlRelaxNG);
opaque_type!(_xmlRelaxNGParserCtxt);
opaque_type!(_xmlRelaxNGValidCtxt);
opaque_type!(_xmlSchema);
opaque_type!(_xmlSchemaParserCtxt);
opaque_type!(_xmlSchemaValidCtxt);
opaque_type!(_xmlSchemaSAXPlug);
opaque_type!(_xmlPattern);
opaque_type!(_xmlStreamCtxt);
opaque_type!(_xmlXIncludeCtxt);
opaque_type!(_xmlXPathCompExpr);
