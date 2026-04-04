#[cfg(not(all(target_arch = "x86_64", target_os = "linux")))]
compile_error!("parser trampolines currently require x86_64 Linux");

// Public parser-family symbols are emitted from Rust, while the checked-in upstream
// implementation is compiled under a private prefix so it keeps sharing this library's
// globals, tree, I/O, and callback machinery.
macro_rules! alias_function {
    ($public:literal, $target:literal) => {
        core::arch::global_asm!(concat!(
            ".globl ", $public, "\n",
            ".type ", $public, ", @function\n",
            $public, ":\n",
            "  jmp ", $target, "\n",
        ));
    };
}

macro_rules! alias_hidden_function {
    ($public:literal, $target:literal) => {
        core::arch::global_asm!(concat!(
            ".hidden ", $public, "\n",
            ".globl ", $public, "\n",
            ".type ", $public, ", @function\n",
            $public, ":\n",
            "  jmp ", $target, "\n",
        ));
    };
}

pub(crate) use alias_function;
pub(crate) use alias_hidden_function;

alias_function!("inputPop", "__safe_inputPop");
alias_function!("inputPush", "__safe_inputPush");
alias_function!("namePop", "__safe_namePop");
alias_function!("namePush", "__safe_namePush");
alias_function!("nodePop", "__safe_nodePop");
alias_function!("nodePush", "__safe_nodePush");
alias_function!("xmlCheckLanguageID", "__safe_xmlCheckLanguageID");
alias_function!("xmlCleanupParser", "__safe_xmlCleanupParser");
alias_function!("xmlCreateDocParserCtxt", "__safe_xmlCreateDocParserCtxt");
alias_function!("xmlCreateEntityParserCtxt", "__safe_xmlCreateEntityParserCtxt");
alias_function!("xmlCreateFileParserCtxt", "__safe_xmlCreateFileParserCtxt");
alias_function!("xmlCreateIOParserCtxt", "__safe_xmlCreateIOParserCtxt");
alias_function!("xmlCreateMemoryParserCtxt", "__safe_xmlCreateMemoryParserCtxt");
alias_function!("xmlCreatePushParserCtxt", "__safe_xmlCreatePushParserCtxt");
alias_function!("xmlCreateURLParserCtxt", "__safe_xmlCreateURLParserCtxt");
alias_function!("xmlCtxtReadDoc", "__safe_xmlCtxtReadDoc");
alias_function!("xmlCtxtReadFd", "__safe_xmlCtxtReadFd");
alias_function!("xmlCtxtReadFile", "__safe_xmlCtxtReadFile");
alias_function!("xmlCtxtReadIO", "__safe_xmlCtxtReadIO");
alias_function!("xmlCtxtReadMemory", "__safe_xmlCtxtReadMemory");
alias_function!("xmlCtxtReset", "__safe_xmlCtxtReset");
alias_function!("xmlCtxtResetPush", "__safe_xmlCtxtResetPush");
alias_function!("xmlCtxtUseOptions", "__safe_xmlCtxtUseOptions");
alias_function!("xmlHasFeature", "__safe_xmlHasFeature");
alias_function!("xmlIOParseDTD", "__safe_xmlIOParseDTD");
alias_function!("xmlInitParser", "__safe_xmlInitParser");
alias_function!("xmlParseAttValue", "__safe_xmlParseAttValue");
alias_function!("xmlParseAttribute", "__safe_xmlParseAttribute");
alias_function!("xmlParseAttributeListDecl", "__safe_xmlParseAttributeListDecl");
alias_function!("xmlParseAttributeType", "__safe_xmlParseAttributeType");
alias_function!("xmlParseBalancedChunkMemory", "__safe_xmlParseBalancedChunkMemory");
alias_function!("xmlParseBalancedChunkMemoryRecover", "__safe_xmlParseBalancedChunkMemoryRecover");
alias_function!("xmlParseCDSect", "__safe_xmlParseCDSect");
alias_function!("xmlParseCharData", "__safe_xmlParseCharData");
alias_function!("xmlParseCharRef", "__safe_xmlParseCharRef");
alias_function!("xmlParseChunk", "__safe_xmlParseChunk");
alias_function!("xmlParseComment", "__safe_xmlParseComment");
alias_function!("xmlParseContent", "__safe_xmlParseContent");
alias_function!("xmlParseCtxtExternalEntity", "__safe_xmlParseCtxtExternalEntity");
alias_function!("xmlParseDTD", "__safe_xmlParseDTD");
alias_function!("xmlParseDefaultDecl", "__safe_xmlParseDefaultDecl");
alias_function!("xmlParseDoc", "__safe_xmlParseDoc");
alias_function!("xmlParseDocTypeDecl", "__safe_xmlParseDocTypeDecl");
alias_function!("xmlParseDocument", "__safe_xmlParseDocument");
alias_function!("xmlParseElement", "__safe_xmlParseElement");
alias_function!("xmlParseElementChildrenContentDecl", "__safe_xmlParseElementChildrenContentDecl");
alias_function!("xmlParseElementContentDecl", "__safe_xmlParseElementContentDecl");
alias_function!("xmlParseElementDecl", "__safe_xmlParseElementDecl");
alias_function!("xmlParseElementMixedContentDecl", "__safe_xmlParseElementMixedContentDecl");
alias_function!("xmlParseEncName", "__safe_xmlParseEncName");
alias_function!("xmlParseEncodingDecl", "__safe_xmlParseEncodingDecl");
alias_function!("xmlParseEndTag", "__safe_xmlParseEndTag");
alias_function!("xmlParseEntity", "__safe_xmlParseEntity");
alias_function!("xmlParseEntityDecl", "__safe_xmlParseEntityDecl");
alias_function!("xmlParseEntityRef", "__safe_xmlParseEntityRef");
alias_function!("xmlParseEntityValue", "__safe_xmlParseEntityValue");
alias_function!("xmlParseEnumeratedType", "__safe_xmlParseEnumeratedType");
alias_function!("xmlParseEnumerationType", "__safe_xmlParseEnumerationType");
alias_function!("xmlParseExtParsedEnt", "__safe_xmlParseExtParsedEnt");
alias_function!("xmlParseExternalEntity", "__safe_xmlParseExternalEntity");
alias_function!("xmlParseExternalID", "__safe_xmlParseExternalID");
alias_function!("xmlParseExternalSubset", "__safe_xmlParseExternalSubset");
alias_function!("xmlParseFile", "__safe_xmlParseFile");
alias_function!("xmlParseInNodeContext", "__safe_xmlParseInNodeContext");
alias_function!("xmlParseMarkupDecl", "__safe_xmlParseMarkupDecl");
alias_function!("xmlParseMemory", "__safe_xmlParseMemory");
alias_function!("xmlParseMisc", "__safe_xmlParseMisc");
alias_function!("xmlParseName", "__safe_xmlParseName");
alias_function!("xmlParseNmtoken", "__safe_xmlParseNmtoken");
alias_function!("xmlParseNotationDecl", "__safe_xmlParseNotationDecl");
alias_function!("xmlParseNotationType", "__safe_xmlParseNotationType");
alias_function!("xmlParsePEReference", "__safe_xmlParsePEReference");
alias_function!("xmlParsePI", "__safe_xmlParsePI");
alias_function!("xmlParsePITarget", "__safe_xmlParsePITarget");
alias_function!("xmlParsePubidLiteral", "__safe_xmlParsePubidLiteral");
alias_function!("xmlParseReference", "__safe_xmlParseReference");
alias_function!("xmlParseSDDecl", "__safe_xmlParseSDDecl");
alias_function!("xmlParseStartTag", "__safe_xmlParseStartTag");
alias_function!("xmlParseSystemLiteral", "__safe_xmlParseSystemLiteral");
alias_function!("xmlParseTextDecl", "__safe_xmlParseTextDecl");
alias_function!("xmlParseVersionInfo", "__safe_xmlParseVersionInfo");
alias_function!("xmlParseVersionNum", "__safe_xmlParseVersionNum");
alias_function!("xmlParseXMLDecl", "__safe_xmlParseXMLDecl");
alias_function!("xmlParserHandlePEReference", "__safe_xmlParserHandlePEReference");
alias_function!("xmlPopInput", "__safe_xmlPopInput");
alias_function!("xmlPushInput", "__safe_xmlPushInput");
alias_function!("xmlReadDoc", "__safe_xmlReadDoc");
alias_function!("xmlReadFd", "__safe_xmlReadFd");
alias_function!("xmlReadFile", "__safe_xmlReadFile");
alias_function!("xmlReadIO", "__safe_xmlReadIO");
alias_function!("xmlReadMemory", "__safe_xmlReadMemory");
alias_function!("xmlRecoverDoc", "__safe_xmlRecoverDoc");
alias_function!("xmlRecoverFile", "__safe_xmlRecoverFile");
alias_function!("xmlRecoverMemory", "__safe_xmlRecoverMemory");
alias_function!("xmlSAXParseDTD", "__safe_xmlSAXParseDTD");
alias_function!("xmlSAXParseDoc", "__safe_xmlSAXParseDoc");
alias_function!("xmlSAXParseEntity", "__safe_xmlSAXParseEntity");
alias_function!("xmlSAXParseFile", "__safe_xmlSAXParseFile");
alias_function!("xmlSAXParseFileWithData", "__safe_xmlSAXParseFileWithData");
alias_function!("xmlSAXParseMemory", "__safe_xmlSAXParseMemory");
alias_function!("xmlSAXParseMemoryWithData", "__safe_xmlSAXParseMemoryWithData");
alias_function!("xmlSAXUserParseFile", "__safe_xmlSAXUserParseFile");
alias_function!("xmlSAXUserParseMemory", "__safe_xmlSAXUserParseMemory");
alias_function!("xmlSetEntityReferenceFunc", "__safe_xmlSetEntityReferenceFunc");
alias_function!("xmlSetupParserForBuffer", "__safe_xmlSetupParserForBuffer");
alias_function!("xmlSkipBlankChars", "__safe_xmlSkipBlankChars");
alias_function!("xmlSplitQName", "__safe_xmlSplitQName");
alias_function!("xmlStopParser", "__safe_xmlStopParser");
alias_function!("xmlStringDecodeEntities", "__safe_xmlStringDecodeEntities");
alias_function!("xmlStringLenDecodeEntities", "__safe_xmlStringLenDecodeEntities");

alias_hidden_function!("inputPop__internal_alias", "__safe_inputPop");
alias_hidden_function!("inputPush__internal_alias", "__safe_inputPush");
alias_hidden_function!("namePop__internal_alias", "__safe_namePop");
alias_hidden_function!("namePush__internal_alias", "__safe_namePush");
alias_hidden_function!("nodePop__internal_alias", "__safe_nodePop");
alias_hidden_function!("nodePush__internal_alias", "__safe_nodePush");
alias_hidden_function!("xmlCheckLanguageID__internal_alias", "__safe_xmlCheckLanguageID");
alias_hidden_function!("xmlCleanupParser__internal_alias", "__safe_xmlCleanupParser");
alias_hidden_function!("xmlCreateDocParserCtxt__internal_alias", "__safe_xmlCreateDocParserCtxt");
alias_hidden_function!("xmlCreateEntityParserCtxt__internal_alias", "__safe_xmlCreateEntityParserCtxt");
alias_hidden_function!("xmlCreateFileParserCtxt__internal_alias", "__safe_xmlCreateFileParserCtxt");
alias_hidden_function!("xmlCreateIOParserCtxt__internal_alias", "__safe_xmlCreateIOParserCtxt");
alias_hidden_function!("xmlCreateMemoryParserCtxt__internal_alias", "__safe_xmlCreateMemoryParserCtxt");
alias_hidden_function!("xmlCreatePushParserCtxt__internal_alias", "__safe_xmlCreatePushParserCtxt");
alias_hidden_function!("xmlCreateURLParserCtxt__internal_alias", "__safe_xmlCreateURLParserCtxt");
alias_hidden_function!("xmlCtxtReadDoc__internal_alias", "__safe_xmlCtxtReadDoc");
alias_hidden_function!("xmlCtxtReadFd__internal_alias", "__safe_xmlCtxtReadFd");
alias_hidden_function!("xmlCtxtReadFile__internal_alias", "__safe_xmlCtxtReadFile");
alias_hidden_function!("xmlCtxtReadIO__internal_alias", "__safe_xmlCtxtReadIO");
alias_hidden_function!("xmlCtxtReadMemory__internal_alias", "__safe_xmlCtxtReadMemory");
alias_hidden_function!("xmlCtxtResetPush__internal_alias", "__safe_xmlCtxtResetPush");
alias_hidden_function!("xmlCtxtReset__internal_alias", "__safe_xmlCtxtReset");
alias_hidden_function!("xmlCtxtUseOptions__internal_alias", "__safe_xmlCtxtUseOptions");
alias_hidden_function!("xmlHasFeature__internal_alias", "__safe_xmlHasFeature");
alias_hidden_function!("xmlIOParseDTD__internal_alias", "__safe_xmlIOParseDTD");
alias_hidden_function!("xmlInitParser__internal_alias", "__safe_xmlInitParser");
alias_hidden_function!("xmlParseAttValue__internal_alias", "__safe_xmlParseAttValue");
alias_hidden_function!("xmlParseAttributeListDecl__internal_alias", "__safe_xmlParseAttributeListDecl");
alias_hidden_function!("xmlParseAttributeType__internal_alias", "__safe_xmlParseAttributeType");
alias_hidden_function!("xmlParseAttribute__internal_alias", "__safe_xmlParseAttribute");
alias_hidden_function!("xmlParseBalancedChunkMemoryRecover__internal_alias", "__safe_xmlParseBalancedChunkMemoryRecover");
alias_hidden_function!("xmlParseBalancedChunkMemory__internal_alias", "__safe_xmlParseBalancedChunkMemory");
alias_hidden_function!("xmlParseCDSect__internal_alias", "__safe_xmlParseCDSect");
alias_hidden_function!("xmlParseCharData__internal_alias", "__safe_xmlParseCharData");
alias_hidden_function!("xmlParseCharRef__internal_alias", "__safe_xmlParseCharRef");
alias_hidden_function!("xmlParseChunk__internal_alias", "__safe_xmlParseChunk");
alias_hidden_function!("xmlParseComment__internal_alias", "__safe_xmlParseComment");
alias_hidden_function!("xmlParseContent__internal_alias", "__safe_xmlParseContent");
alias_hidden_function!("xmlParseCtxtExternalEntity__internal_alias", "__safe_xmlParseCtxtExternalEntity");
alias_hidden_function!("xmlParseDTD__internal_alias", "__safe_xmlParseDTD");
alias_hidden_function!("xmlParseDefaultDecl__internal_alias", "__safe_xmlParseDefaultDecl");
alias_hidden_function!("xmlParseDocTypeDecl__internal_alias", "__safe_xmlParseDocTypeDecl");
alias_hidden_function!("xmlParseDoc__internal_alias", "__safe_xmlParseDoc");
alias_hidden_function!("xmlParseDocument__internal_alias", "__safe_xmlParseDocument");
alias_hidden_function!("xmlParseElementChildrenContentDecl__internal_alias", "__safe_xmlParseElementChildrenContentDecl");
alias_hidden_function!("xmlParseElementContentDecl__internal_alias", "__safe_xmlParseElementContentDecl");
alias_hidden_function!("xmlParseElementDecl__internal_alias", "__safe_xmlParseElementDecl");
alias_hidden_function!("xmlParseElementMixedContentDecl__internal_alias", "__safe_xmlParseElementMixedContentDecl");
alias_hidden_function!("xmlParseElement__internal_alias", "__safe_xmlParseElement");
alias_hidden_function!("xmlParseEncName__internal_alias", "__safe_xmlParseEncName");
alias_hidden_function!("xmlParseEncodingDecl__internal_alias", "__safe_xmlParseEncodingDecl");
alias_hidden_function!("xmlParseEndTag__internal_alias", "__safe_xmlParseEndTag");
alias_hidden_function!("xmlParseEntityDecl__internal_alias", "__safe_xmlParseEntityDecl");
alias_hidden_function!("xmlParseEntityRef__internal_alias", "__safe_xmlParseEntityRef");
alias_hidden_function!("xmlParseEntityValue__internal_alias", "__safe_xmlParseEntityValue");
alias_hidden_function!("xmlParseEntity__internal_alias", "__safe_xmlParseEntity");
alias_hidden_function!("xmlParseEnumeratedType__internal_alias", "__safe_xmlParseEnumeratedType");
alias_hidden_function!("xmlParseEnumerationType__internal_alias", "__safe_xmlParseEnumerationType");
alias_hidden_function!("xmlParseExtParsedEnt__internal_alias", "__safe_xmlParseExtParsedEnt");
alias_hidden_function!("xmlParseExternalEntity__internal_alias", "__safe_xmlParseExternalEntity");
alias_hidden_function!("xmlParseExternalID__internal_alias", "__safe_xmlParseExternalID");
alias_hidden_function!("xmlParseExternalSubset__internal_alias", "__safe_xmlParseExternalSubset");
alias_hidden_function!("xmlParseFile__internal_alias", "__safe_xmlParseFile");
alias_hidden_function!("xmlParseInNodeContext__internal_alias", "__safe_xmlParseInNodeContext");
alias_hidden_function!("xmlParseMarkupDecl__internal_alias", "__safe_xmlParseMarkupDecl");
alias_hidden_function!("xmlParseMemory__internal_alias", "__safe_xmlParseMemory");
alias_hidden_function!("xmlParseMisc__internal_alias", "__safe_xmlParseMisc");
alias_hidden_function!("xmlParseName__internal_alias", "__safe_xmlParseName");
alias_hidden_function!("xmlParseNmtoken__internal_alias", "__safe_xmlParseNmtoken");
alias_hidden_function!("xmlParseNotationDecl__internal_alias", "__safe_xmlParseNotationDecl");
alias_hidden_function!("xmlParseNotationType__internal_alias", "__safe_xmlParseNotationType");
alias_hidden_function!("xmlParsePEReference__internal_alias", "__safe_xmlParsePEReference");
alias_hidden_function!("xmlParsePITarget__internal_alias", "__safe_xmlParsePITarget");
alias_hidden_function!("xmlParsePI__internal_alias", "__safe_xmlParsePI");
alias_hidden_function!("xmlParsePubidLiteral__internal_alias", "__safe_xmlParsePubidLiteral");
alias_hidden_function!("xmlParseReference__internal_alias", "__safe_xmlParseReference");
alias_hidden_function!("xmlParseSDDecl__internal_alias", "__safe_xmlParseSDDecl");
alias_hidden_function!("xmlParseStartTag__internal_alias", "__safe_xmlParseStartTag");
alias_hidden_function!("xmlParseSystemLiteral__internal_alias", "__safe_xmlParseSystemLiteral");
alias_hidden_function!("xmlParseTextDecl__internal_alias", "__safe_xmlParseTextDecl");
alias_hidden_function!("xmlParseVersionInfo__internal_alias", "__safe_xmlParseVersionInfo");
alias_hidden_function!("xmlParseVersionNum__internal_alias", "__safe_xmlParseVersionNum");
alias_hidden_function!("xmlParseXMLDecl__internal_alias", "__safe_xmlParseXMLDecl");
alias_hidden_function!("xmlParserHandlePEReference__internal_alias", "__safe_xmlParserHandlePEReference");
alias_hidden_function!("xmlPopInput__internal_alias", "__safe_xmlPopInput");
alias_hidden_function!("xmlPushInput__internal_alias", "__safe_xmlPushInput");
alias_hidden_function!("xmlReadDoc__internal_alias", "__safe_xmlReadDoc");
alias_hidden_function!("xmlReadFd__internal_alias", "__safe_xmlReadFd");
alias_hidden_function!("xmlReadFile__internal_alias", "__safe_xmlReadFile");
alias_hidden_function!("xmlReadIO__internal_alias", "__safe_xmlReadIO");
alias_hidden_function!("xmlReadMemory__internal_alias", "__safe_xmlReadMemory");
alias_hidden_function!("xmlRecoverDoc__internal_alias", "__safe_xmlRecoverDoc");
alias_hidden_function!("xmlRecoverFile__internal_alias", "__safe_xmlRecoverFile");
alias_hidden_function!("xmlRecoverMemory__internal_alias", "__safe_xmlRecoverMemory");
alias_hidden_function!("xmlSAXParseDTD__internal_alias", "__safe_xmlSAXParseDTD");
alias_hidden_function!("xmlSAXParseDoc__internal_alias", "__safe_xmlSAXParseDoc");
alias_hidden_function!("xmlSAXParseEntity__internal_alias", "__safe_xmlSAXParseEntity");
alias_hidden_function!("xmlSAXParseFileWithData__internal_alias", "__safe_xmlSAXParseFileWithData");
alias_hidden_function!("xmlSAXParseFile__internal_alias", "__safe_xmlSAXParseFile");
alias_hidden_function!("xmlSAXParseMemoryWithData__internal_alias", "__safe_xmlSAXParseMemoryWithData");
alias_hidden_function!("xmlSAXParseMemory__internal_alias", "__safe_xmlSAXParseMemory");
alias_hidden_function!("xmlSAXUserParseFile__internal_alias", "__safe_xmlSAXUserParseFile");
alias_hidden_function!("xmlSAXUserParseMemory__internal_alias", "__safe_xmlSAXUserParseMemory");
alias_hidden_function!("xmlSetEntityReferenceFunc__internal_alias", "__safe_xmlSetEntityReferenceFunc");
alias_hidden_function!("xmlSetupParserForBuffer__internal_alias", "__safe_xmlSetupParserForBuffer");
alias_hidden_function!("xmlSkipBlankChars__internal_alias", "__safe_xmlSkipBlankChars");
alias_hidden_function!("xmlSplitQName__internal_alias", "__safe_xmlSplitQName");
alias_hidden_function!("xmlStopParser__internal_alias", "__safe_xmlStopParser");
alias_hidden_function!("xmlStringDecodeEntities__internal_alias", "__safe_xmlStringDecodeEntities");
alias_hidden_function!("xmlStringLenDecodeEntities__internal_alias", "__safe_xmlStringLenDecodeEntities");
