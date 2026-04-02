#!/usr/bin/python -u
#
# this tests the Expand() API of the xmlTextReader interface
# this extract the Dragon bibliography entries from the XML specification
#
import libxml2
import sys
try:
    import StringIO
    str_io = StringIO.StringIO
except:
    import io
    str_io = io.StringIO

# Memory debug specific
libxml2.debugMemory(1)

doc="""<spec><bibl id="Aho" key="Aho/Ullman">Aho<emph>Compilers</emph></bibl><bibl id="Knu">Knuth</bibl></spec>"""
expect="""<bibl id="Aho" key="Aho/Ullman">Aho<emph>Compilers</emph></bibl>"""

input = libxml2.inputBuffer(str_io(doc))
reader = input.newTextReader("REC")
res=""
while reader.Read() > 0:
    while reader.Name() == 'bibl':
        node = reader.Expand()            # expand the subtree
        if node.xpathEval("@id = 'Aho'"): # use XPath on it
            res = res + node.serialize()
        if reader.Next() != 1:            # skip the subtree
            break;

if res != expect:
    print("Error: didn't get the expected output")
    print("got '%s'" % (res))
    print("expected '%s'" % (expect))
    

#
# cleanup
#
del input
del reader

# Memory debug specific
libxml2.cleanupParser()
if libxml2.debugMemory(1) == 0:
    print("OK")
else:
    print("Memory leak %d bytes" % (libxml2.debugMemory(1)))
    libxml2.dumpMemory()
