/*
 * Stable Rust cannot define these internal C variadic SAX callbacks directly.
 * Keep the original TODO behavior in a focused shim.
 */

#define IN_LIBXML
#include "libxml.h"

#include <libxml/globals.h>
#include <libxml/parser.h>
#include <libxml/xmlschemas.h>

struct _xmlSchemaSAXPlug {
    unsigned int magic;
    xmlSAXHandlerPtr *user_sax_ptr;
    xmlSAXHandlerPtr user_sax;
    void **user_data_ptr;
    void *user_data;
    xmlSAXHandler schemas_sax;
    xmlSchemaValidCtxtPtr ctxt;
};

void XMLCDECL
safe_xmlSchemaWarningSplit(void *ctx, const char *msg ATTRIBUTE_UNUSED, ...) {
    xmlSchemaSAXPlugPtr ctxt = (xmlSchemaSAXPlugPtr) ctx;

    if ((ctxt != NULL) && (ctxt->user_sax != NULL) &&
        (ctxt->user_sax->warning != NULL)) {
        xmlGenericError(xmlGenericErrorContext,
                        "Unimplemented block at %s:%d\n",
                        __FILE__, __LINE__);
    }
}

void XMLCDECL
safe_xmlSchemaErrorSplit(void *ctx, const char *msg ATTRIBUTE_UNUSED, ...) {
    xmlSchemaSAXPlugPtr ctxt = (xmlSchemaSAXPlugPtr) ctx;

    if ((ctxt != NULL) && (ctxt->user_sax != NULL) &&
        (ctxt->user_sax->error != NULL)) {
        xmlGenericError(xmlGenericErrorContext,
                        "Unimplemented block at %s:%d\n",
                        __FILE__, __LINE__);
    }
}

void XMLCDECL
safe_xmlSchemaFatalErrorSplit(void *ctx, const char *msg ATTRIBUTE_UNUSED, ...) {
    xmlSchemaSAXPlugPtr ctxt = (xmlSchemaSAXPlugPtr) ctx;

    if ((ctxt != NULL) && (ctxt->user_sax != NULL) &&
        (ctxt->user_sax->fatalError != NULL)) {
        xmlGenericError(xmlGenericErrorContext,
                        "Unimplemented block at %s:%d\n",
                        __FILE__, __LINE__);
    }
}
