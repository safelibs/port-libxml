/*
 * Stable Rust cannot define these exported C variadics directly.
 */

#define IN_LIBXML
#include "libxml.h"

#include <stdarg.h>

#include <libxml/xmlmemory.h>
#include <libxml/xmlstring.h>

int XMLCDECL
xmlStrPrintf(xmlChar *buf, int len, const char *msg, ...) {
    va_list args;
    int ret;

    if ((buf == NULL) || (msg == NULL))
        return(-1);

    va_start(args, msg);
    ret = vsnprintf((char *) buf, len, msg, args);
    va_end(args);
    buf[len - 1] = 0;

    return(ret);
}

int
xmlStrVPrintf(xmlChar *buf, int len, const char *msg, va_list ap) {
    int ret;

    if ((buf == NULL) || (msg == NULL))
        return(-1);

    ret = vsnprintf((char *) buf, len, msg, ap);
    buf[len - 1] = 0;

    return(ret);
}
