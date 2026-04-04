#define IN_LIBXML
#include "libxml.h"

#include <stdarg.h>
#include <stdio.h>
#include <string.h>

#include <libxml/xmlmemory.h>
#include <libxml/xmlwriter.h>

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

static xmlChar *
safeXmlTextWriterVSprintf(const char *format, va_list argptr) {
    int size = BUFSIZ;
    int count;
    xmlChar *buf;
    va_list locarg;

    buf = xmlMalloc(size);
    if (buf == NULL)
        return NULL;

    VA_COPY(locarg, argptr);
    while (((count = vsnprintf((char *) buf, size, format, locarg)) < 0) ||
           (count == size - 1) || (count == size) || (count > size)) {
        va_end(locarg);
        xmlFree(buf);
        size += BUFSIZ;
        buf = xmlMalloc(size);
        if (buf == NULL)
            return NULL;
        VA_COPY(locarg, argptr);
    }
    va_end(locarg);

    return buf;
}

int
xmlTextWriterWriteVFormatComment(xmlTextWriterPtr writer, const char *format,
                                 va_list argptr) {
    int ret;
    xmlChar *buf;

    if (writer == NULL)
        return -1;

    buf = safeXmlTextWriterVSprintf(format, argptr);
    if (buf == NULL)
        return -1;

    ret = xmlTextWriterWriteComment(writer, buf);
    xmlFree(buf);
    return ret;
}

int
xmlTextWriterWriteFormatComment(xmlTextWriterPtr writer, const char *format, ...) {
    int ret;
    va_list args;

    va_start(args, format);
    ret = xmlTextWriterWriteVFormatComment(writer, format, args);
    va_end(args);
    return ret;
}

int
xmlTextWriterWriteVFormatElement(xmlTextWriterPtr writer, const xmlChar *name,
                                 const char *format, va_list argptr) {
    int ret;
    xmlChar *buf;

    if (writer == NULL)
        return -1;

    buf = safeXmlTextWriterVSprintf(format, argptr);
    if (buf == NULL)
        return -1;

    ret = xmlTextWriterWriteElement(writer, name, buf);
    xmlFree(buf);
    return ret;
}

int
xmlTextWriterWriteFormatElement(xmlTextWriterPtr writer, const xmlChar *name,
                                const char *format, ...) {
    int ret;
    va_list args;

    va_start(args, format);
    ret = xmlTextWriterWriteVFormatElement(writer, name, format, args);
    va_end(args);
    return ret;
}

int
xmlTextWriterWriteVFormatElementNS(xmlTextWriterPtr writer, const xmlChar *prefix,
                                   const xmlChar *name,
                                   const xmlChar *namespaceURI,
                                   const char *format, va_list argptr) {
    int ret;
    xmlChar *buf;

    if (writer == NULL)
        return -1;

    buf = safeXmlTextWriterVSprintf(format, argptr);
    if (buf == NULL)
        return -1;

    ret = xmlTextWriterWriteElementNS(writer, prefix, name, namespaceURI, buf);
    xmlFree(buf);
    return ret;
}

int
xmlTextWriterWriteFormatElementNS(xmlTextWriterPtr writer, const xmlChar *prefix,
                                  const xmlChar *name,
                                  const xmlChar *namespaceURI,
                                  const char *format, ...) {
    int ret;
    va_list args;

    va_start(args, format);
    ret = xmlTextWriterWriteVFormatElementNS(writer, prefix, name, namespaceURI,
                                             format, args);
    va_end(args);
    return ret;
}

int
xmlTextWriterWriteVFormatRaw(xmlTextWriterPtr writer, const char *format,
                             va_list argptr) {
    int ret;
    xmlChar *buf;

    if (writer == NULL)
        return -1;

    buf = safeXmlTextWriterVSprintf(format, argptr);
    if (buf == NULL)
        return -1;

    ret = xmlTextWriterWriteRaw(writer, buf);
    xmlFree(buf);
    return ret;
}

int
xmlTextWriterWriteFormatRaw(xmlTextWriterPtr writer, const char *format, ...) {
    int ret;
    va_list args;

    va_start(args, format);
    ret = xmlTextWriterWriteVFormatRaw(writer, format, args);
    va_end(args);
    return ret;
}

int
xmlTextWriterWriteVFormatString(xmlTextWriterPtr writer, const char *format,
                                va_list argptr) {
    int ret;
    xmlChar *buf;

    if (writer == NULL)
        return -1;

    buf = safeXmlTextWriterVSprintf(format, argptr);
    if (buf == NULL)
        return -1;

    ret = xmlTextWriterWriteString(writer, buf);
    xmlFree(buf);
    return ret;
}

int
xmlTextWriterWriteFormatString(xmlTextWriterPtr writer, const char *format, ...) {
    int ret;
    va_list args;

    va_start(args, format);
    ret = xmlTextWriterWriteVFormatString(writer, format, args);
    va_end(args);
    return ret;
}

int
xmlTextWriterWriteVFormatAttribute(xmlTextWriterPtr writer, const xmlChar *name,
                                   const char *format, va_list argptr) {
    int ret;
    xmlChar *buf;

    if (writer == NULL)
        return -1;

    buf = safeXmlTextWriterVSprintf(format, argptr);
    if (buf == NULL)
        return -1;

    ret = xmlTextWriterWriteAttribute(writer, name, buf);
    xmlFree(buf);
    return ret;
}

int
xmlTextWriterWriteFormatAttribute(xmlTextWriterPtr writer, const xmlChar *name,
                                  const char *format, ...) {
    int ret;
    va_list args;

    va_start(args, format);
    ret = xmlTextWriterWriteVFormatAttribute(writer, name, format, args);
    va_end(args);
    return ret;
}

int
xmlTextWriterWriteVFormatAttributeNS(xmlTextWriterPtr writer, const xmlChar *prefix,
                                     const xmlChar *name,
                                     const xmlChar *namespaceURI,
                                     const char *format, va_list argptr) {
    int ret;
    xmlChar *buf;

    if (writer == NULL)
        return -1;

    buf = safeXmlTextWriterVSprintf(format, argptr);
    if (buf == NULL)
        return -1;

    ret = xmlTextWriterWriteAttributeNS(writer, prefix, name, namespaceURI,
                                        buf);
    xmlFree(buf);
    return ret;
}

int
xmlTextWriterWriteFormatAttributeNS(xmlTextWriterPtr writer, const xmlChar *prefix,
                                    const xmlChar *name,
                                    const xmlChar *namespaceURI,
                                    const char *format, ...) {
    int ret;
    va_list args;

    va_start(args, format);
    ret = xmlTextWriterWriteVFormatAttributeNS(writer, prefix, name,
                                               namespaceURI, format, args);
    va_end(args);
    return ret;
}

int
xmlTextWriterWriteVFormatPI(xmlTextWriterPtr writer, const xmlChar *target,
                            const char *format, va_list argptr) {
    int ret;
    xmlChar *buf;

    if (writer == NULL)
        return -1;

    buf = safeXmlTextWriterVSprintf(format, argptr);
    if (buf == NULL)
        return -1;

    ret = xmlTextWriterWritePI(writer, target, buf);
    xmlFree(buf);
    return ret;
}

int
xmlTextWriterWriteFormatPI(xmlTextWriterPtr writer, const xmlChar *target,
                           const char *format, ...) {
    int ret;
    va_list args;

    va_start(args, format);
    ret = xmlTextWriterWriteVFormatPI(writer, target, format, args);
    va_end(args);
    return ret;
}

int
xmlTextWriterWriteVFormatCDATA(xmlTextWriterPtr writer, const char *format,
                               va_list argptr) {
    int ret;
    xmlChar *buf;

    if (writer == NULL)
        return -1;

    buf = safeXmlTextWriterVSprintf(format, argptr);
    if (buf == NULL)
        return -1;

    ret = xmlTextWriterWriteCDATA(writer, buf);
    xmlFree(buf);
    return ret;
}

int
xmlTextWriterWriteFormatCDATA(xmlTextWriterPtr writer, const char *format, ...) {
    int ret;
    va_list args;

    va_start(args, format);
    ret = xmlTextWriterWriteVFormatCDATA(writer, format, args);
    va_end(args);
    return ret;
}

int
xmlTextWriterWriteVFormatDTD(xmlTextWriterPtr writer, const xmlChar *name,
                             const xmlChar *pubid, const xmlChar *sysid,
                             const char *format, va_list argptr) {
    int ret;
    xmlChar *buf;

    if (writer == NULL)
        return -1;

    buf = safeXmlTextWriterVSprintf(format, argptr);
    if (buf == NULL)
        return -1;

    ret = xmlTextWriterWriteDTD(writer, name, pubid, sysid, buf);
    xmlFree(buf);
    return ret;
}

int
xmlTextWriterWriteFormatDTD(xmlTextWriterPtr writer, const xmlChar *name,
                            const xmlChar *pubid, const xmlChar *sysid,
                            const char *format, ...) {
    int ret;
    va_list args;

    va_start(args, format);
    ret = xmlTextWriterWriteVFormatDTD(writer, name, pubid, sysid, format, args);
    va_end(args);
    return ret;
}

int
xmlTextWriterWriteVFormatDTDElement(xmlTextWriterPtr writer, const xmlChar *name,
                                    const char *format, va_list argptr) {
    int ret;
    xmlChar *buf;

    if (writer == NULL)
        return -1;

    buf = safeXmlTextWriterVSprintf(format, argptr);
    if (buf == NULL)
        return -1;

    ret = xmlTextWriterWriteDTDElement(writer, name, buf);
    xmlFree(buf);
    return ret;
}

int
xmlTextWriterWriteFormatDTDElement(xmlTextWriterPtr writer, const xmlChar *name,
                                   const char *format, ...) {
    int ret;
    va_list args;

    va_start(args, format);
    ret = xmlTextWriterWriteVFormatDTDElement(writer, name, format, args);
    va_end(args);
    return ret;
}

int
xmlTextWriterWriteVFormatDTDAttlist(xmlTextWriterPtr writer, const xmlChar *name,
                                    const char *format, va_list argptr) {
    int ret;
    xmlChar *buf;

    if (writer == NULL)
        return -1;

    buf = safeXmlTextWriterVSprintf(format, argptr);
    if (buf == NULL)
        return -1;

    ret = xmlTextWriterWriteDTDAttlist(writer, name, buf);
    xmlFree(buf);
    return ret;
}

int
xmlTextWriterWriteFormatDTDAttlist(xmlTextWriterPtr writer, const xmlChar *name,
                                   const char *format, ...) {
    int ret;
    va_list args;

    va_start(args, format);
    ret = xmlTextWriterWriteVFormatDTDAttlist(writer, name, format, args);
    va_end(args);
    return ret;
}

int
xmlTextWriterWriteVFormatDTDInternalEntity(xmlTextWriterPtr writer, int pe,
                                           const xmlChar *name,
                                           const char *format, va_list argptr) {
    int ret;
    xmlChar *buf;

    if (writer == NULL)
        return -1;

    buf = safeXmlTextWriterVSprintf(format, argptr);
    if (buf == NULL)
        return -1;

    ret = xmlTextWriterWriteDTDInternalEntity(writer, pe, name, buf);
    xmlFree(buf);
    return ret;
}

int
xmlTextWriterWriteFormatDTDInternalEntity(xmlTextWriterPtr writer, int pe,
                                          const xmlChar *name,
                                          const char *format, ...) {
    int ret;
    va_list args;

    va_start(args, format);
    ret = xmlTextWriterWriteVFormatDTDInternalEntity(writer, pe, name, format,
                                                     args);
    va_end(args);
    return ret;
}
