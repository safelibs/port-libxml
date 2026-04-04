#include <stdarg.h>
#include <libxml/xmlwriter.h>

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
xmlTextWriterWriteFormatRaw(xmlTextWriterPtr writer, const char *format, ...) {
    int ret;
    va_list args;

    va_start(args, format);
    ret = xmlTextWriterWriteVFormatRaw(writer, format, args);
    va_end(args);
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
xmlTextWriterWriteFormatCDATA(xmlTextWriterPtr writer, const char *format, ...) {
    int ret;
    va_list args;

    va_start(args, format);
    ret = xmlTextWriterWriteVFormatCDATA(writer, format, args);
    va_end(args);
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
