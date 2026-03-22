#!/bin/bash
# Measure C line coverage of libexpat using gcov.
#
# Usage:
#   ./meta/scripts/measure-c-coverage.sh          # C's own test suite
#   ./meta/scripts/measure-c-coverage.sh --quick   # Representative comparison inputs only
#
# Requires: cmake, cc, gcov

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT="$SCRIPT_DIR/../.."
EXPAT_SRC="$ROOT/expat/expat"
BUILD_DIR="$EXPAT_SRC/build-cov"

echo "=== Building C libexpat with coverage instrumentation ==="
mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"
cmake "$EXPAT_SRC" -DCMAKE_C_FLAGS="--coverage" -DCMAKE_BUILD_TYPE=Debug -DEXPAT_BUILD_TESTS=ON -DEXPAT_BUILD_EXAMPLES=OFF 2>&1 | tail -3
make -j4 2>&1 | tail -3

# Clear previous coverage data
find . -name "*.gcda" -delete

if [ "$1" = "--quick" ]; then
    echo ""
    echo "=== Running representative comparison inputs ==="
    # Build a small C program with representative inputs
    cat > /tmp/expat_cov_inputs.c << 'CEOF'
#include <stdio.h>
#include <string.h>
#include "expat.h"
static void se(void*u,const XML_Char*n,const XML_Char**a){}
static void ee(void*u,const XML_Char*n){}
static void cd(void*u,const XML_Char*s,int l){}
static void pi(void*u,const XML_Char*t,const XML_Char*d){}
static void cm(void*u,const XML_Char*d){}
static void sc(void*u){}
static void ec(void*u){}
static void sd(void*u,const XML_Char*n,const XML_Char*s,const XML_Char*p,int h){}
static void ed(void*u){}
static void xd(void*u,const XML_Char*v,const XML_Char*e,int s){}
static void p(const char *xml) {
    XML_Parser pr = XML_ParserCreate(NULL);
    XML_SetElementHandler(pr,se,ee);XML_SetCharacterDataHandler(pr,cd);
    XML_SetProcessingInstructionHandler(pr,pi);XML_SetCommentHandler(pr,cm);
    XML_SetCdataSectionHandler(pr,sc,ec);XML_SetDoctypeDeclHandler(pr,sd,ed);
    XML_SetXmlDeclHandler(pr,xd);
    XML_Parse(pr, xml, strlen(xml), 1);
    XML_ParserFree(pr);
}
int main() {
    p("<doc/>");p("<doc></doc>");p("<doc>text</doc>");
    p("<doc attr='val'/>");p("<doc a='1' b='2'/>");
    p("<a><b><c/></b></a>");p("<a><b><c><d><e/></d></c></b></a>");
    p("<?xml version='1.0'?><doc/>");
    p("<?xml version='1.0' encoding='utf-8'?><doc/>");
    p("<?xml version='1.0' standalone='yes'?><doc/>");
    p("<doc><?target data?></doc>");p("<doc><!-- comment --></doc>");
    p("<?pi before?><doc/>");p("<!-- before --><doc/><!-- after -->");
    p("<doc><![CDATA[text]]></doc>");p("<doc><![CDATA[<>&]]></doc>");
    p("<!DOCTYPE doc><doc/>");
    p("<!DOCTYPE doc SYSTEM 'foo'><doc/>");
    p("<!DOCTYPE doc [<!ENTITY e 'val'>]><doc>&e;</doc>");
    p("<!DOCTYPE doc [<!ELEMENT doc (#PCDATA)>]><doc>text</doc>");
    p("<!DOCTYPE doc [<!ATTLIST doc a CDATA #IMPLIED>]><doc a='v'/>");
    p("<!DOCTYPE doc [<!NOTATION n SYSTEM 'foo'>]><doc/>");
    p("<doc>&amp;&lt;&gt;&apos;&quot;</doc>");
    p("<doc>&#65;&#x42;</doc>");
    p("<doc>\r\n</doc>");p("<doc>\r</doc>");p("<doc>\n</doc>");
    p("<doc><doc>");p("<doc");p("not xml");p("<doc>&undefined;</doc>");
    p("<doc>text<b>bold</b>more</doc>");
    p("<!DOCTYPE doc [\n<!ENTITY e1 'v1'>\n<!ENTITY e2 'v2'>\n<!ELEMENT doc (#PCDATA)*>\n<!ATTLIST doc a CDATA #IMPLIED>\n]>\n<doc>&e1;&e2;</doc>");
    printf("Done (quick mode)\n");
    return 0;
}
CEOF
    # Build static lib with coverage
    cc -c --coverage -I"$EXPAT_SRC/lib" -DHAVE_ARC4RANDOM_BUF -DXML_DEV_URANDOM -DXML_DTD -DXML_GE=1 -DXML_NS -DXML_CONTEXT_BYTES=1024 \
        "$EXPAT_SRC/lib/xmlparse.c" "$EXPAT_SRC/lib/xmltok.c" "$EXPAT_SRC/lib/xmlrole.c"
    ar rcs libexpat_cov.a xmlparse.o xmltok.o xmlrole.o
    cc -o /tmp/expat_cov_run /tmp/expat_cov_inputs.c -I"$EXPAT_SRC/lib" -L. -lexpat_cov --coverage
    /tmp/expat_cov_run
else
    echo ""
    echo "=== Running C's own test suite (4692 tests) ==="
    ./tests/runtests -q 2>&1 | tail -3
fi

echo ""
echo "=== C Line Coverage ==="
if [ "$1" = "--quick" ]; then
    gcov xmlparse.gcda xmltok.gcda xmlrole.gcda 2>&1 | grep "Lines\|File.*xmlparse.c$\|File.*xmltok_impl\|File.*xmlrole.c$"
else
    cd CMakeFiles/runtests.dir/lib
    gcov xmlparse.c.gcda xmltok.c.gcda xmlrole.c.gcda 2>&1 | grep "Lines\|File.*xmlparse.c\|File.*xmltok_impl\|File.*xmlrole.c"
fi

echo ""
echo "Done. Build artifacts are in $BUILD_DIR"
echo "Run 'rm -rf $BUILD_DIR' to clean up."
