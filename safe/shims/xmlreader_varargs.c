#define IN_LIBXML
#include "libxml.h"

#ifdef LIBXML_READER_ENABLED

#include <stdarg.h>
#include <stdio.h>
#include <string.h>

#include <libxml/xmlmemory.h>
#include <libxml/xmlreader.h>
#include <libxml/xmlstring.h>

#ifndef VA_COPY
  #ifdef HAVE_VA_COPY
    #define VA_COPY(dest, src) va_copy(dest, src)
  #else
    #ifdef HAVE___VA_COPY
      #define VA_COPY(dest, src) __va_copy(dest, src)
    #else
      #ifndef VA_LIST_IS_ARRAY
        #define VA_COPY(dest, src) (dest) = (src)
      #else
        #define VA_COPY(dest, src) memcpy((char *)(dest), (char *)(src), sizeof(va_list))
      #endif
    #endif
  #endif
#endif

#define MAX_ERR_MSG_SIZE 64000

extern void xmlTextReaderGenericError(void *ctxt,
                                      xmlParserSeverities severity,
                                      char *str);
extern void safeXmlTextReaderValidityRelayMessage(void *ctx,
                                                  xmlParserSeverities severity,
                                                  char *str);

static char *
safeXmlTextReaderBuildMessage(const char *msg, va_list ap) {
    int size = 0;
    int chars;
    char *larger;
    char *str = NULL;
    va_list aq;

    while (1) {
        VA_COPY(aq, ap);
        chars = vsnprintf(str, size, msg, aq);
        va_end(aq);
        if (chars < 0) {
            xmlFree(str);
            return NULL;
        }
        if ((chars < size) || (size == MAX_ERR_MSG_SIZE))
            break;
        if (chars < MAX_ERR_MSG_SIZE)
            size = chars + 1;
        else
            size = MAX_ERR_MSG_SIZE;
        larger = xmlRealloc(str, size);
        if (larger == NULL) {
            xmlFree(str);
            return NULL;
        }
        str = larger;
    }

    return str;
}

void
xmlTextReaderError(void *ctxt, const char *msg, ...) {
    va_list ap;

    va_start(ap, msg);
    xmlTextReaderGenericError(ctxt, XML_PARSER_SEVERITY_ERROR,
                              safeXmlTextReaderBuildMessage(msg, ap));
    va_end(ap);
}

void
xmlTextReaderWarning(void *ctxt, const char *msg, ...) {
    va_list ap;

    va_start(ap, msg);
    xmlTextReaderGenericError(ctxt, XML_PARSER_SEVERITY_WARNING,
                              safeXmlTextReaderBuildMessage(msg, ap));
    va_end(ap);
}

void
xmlTextReaderValidityError(void *ctxt, const char *msg, ...) {
    int len = xmlStrlen((const xmlChar *) msg);
    va_list ap;

    if ((len > 1) && (msg[len - 2] != ':')) {
        va_start(ap, msg);
        xmlTextReaderGenericError(ctxt, XML_PARSER_SEVERITY_VALIDITY_ERROR,
                                  safeXmlTextReaderBuildMessage(msg, ap));
        va_end(ap);
    }
}

void
xmlTextReaderValidityWarning(void *ctxt, const char *msg, ...) {
    int len = xmlStrlen((const xmlChar *) msg);
    va_list ap;

    if ((len != 0) && (msg[len - 1] != ':')) {
        va_start(ap, msg);
        xmlTextReaderGenericError(ctxt, XML_PARSER_SEVERITY_VALIDITY_WARNING,
                                  safeXmlTextReaderBuildMessage(msg, ap));
        va_end(ap);
    }
}

void
xmlTextReaderValidityErrorRelay(void *ctx, const char *msg, ...) {
    va_list ap;
    char *str;

    va_start(ap, msg);
    str = safeXmlTextReaderBuildMessage(msg, ap);
    va_end(ap);
    safeXmlTextReaderValidityRelayMessage(ctx,
                                          XML_PARSER_SEVERITY_VALIDITY_ERROR,
                                          str);
}

void
xmlTextReaderValidityWarningRelay(void *ctx, const char *msg, ...) {
    va_list ap;
    char *str;

    va_start(ap, msg);
    str = safeXmlTextReaderBuildMessage(msg, ap);
    va_end(ap);
    safeXmlTextReaderValidityRelayMessage(ctx,
                                          XML_PARSER_SEVERITY_VALIDITY_WARNING,
                                          str);
}

#endif
