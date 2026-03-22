/**
 * test_ffi.c — C integration tests for expat-rust's FFI layer.
 *
 * Proves that the Rust library is callable from real C code with the standard
 * libexpat API. Each test function returns 0 on success, 1 on failure.
 *
 * Build:
 *   cargo build --release -p expat-ffi
 *   cc -o test_ffi test_ffi.c -L../../target/release -lexpat -Wl,-rpath,../../target/release
 *
 * Run:
 *   ./test_ffi
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* --- libexpat API declarations (same as you'd get from expat.h) --- */

typedef void *XML_Parser;
typedef int XML_Status;
typedef int XML_Error;
typedef char XML_Char;
typedef char XML_Bool;

#define XML_STATUS_ERROR    0
#define XML_STATUS_OK       1
#define XML_STATUS_SUSPENDED 2

#define XML_ERROR_NONE           0
#define XML_ERROR_SYNTAX         2
#define XML_ERROR_TAG_MISMATCH   7

extern XML_Parser XML_ParserCreate(const char *encoding);
extern XML_Parser XML_ParserCreateNS(const char *encoding, char separator);
extern XML_Bool   XML_ParserReset(XML_Parser parser, const char *encoding);
extern void       XML_ParserFree(XML_Parser parser);
extern XML_Status XML_Parse(XML_Parser parser, const char *s, int len, int isFinal);
extern XML_Status XML_StopParser(XML_Parser parser, XML_Bool resumable);
extern XML_Status XML_ResumeParser(XML_Parser parser);
extern XML_Error  XML_GetErrorCode(XML_Parser parser);
extern const char *XML_ErrorString(XML_Error code);
extern unsigned long XML_GetCurrentLineNumber(XML_Parser parser);
extern unsigned long XML_GetCurrentColumnNumber(XML_Parser parser);
extern long       XML_GetCurrentByteIndex(XML_Parser parser);
extern int        XML_GetCurrentByteCount(XML_Parser parser);
extern void       XML_SetUserData(XML_Parser parser, void *userData);
extern XML_Status XML_SetEncoding(XML_Parser parser, const char *encoding);
extern XML_Status XML_SetBase(XML_Parser parser, const char *base);
extern const char *XML_GetBase(XML_Parser parser);
extern int        XML_SetHashSalt(XML_Parser parser, unsigned long salt);
extern void       XML_SetReturnNSTriplet(XML_Parser parser, int do_nst);
extern XML_Bool   XML_SetReparseDeferralEnabled(XML_Parser parser, XML_Bool enabled);
extern const char *XML_ExpatVersion(void);
extern int        XML_GetSpecifiedAttributeCount(XML_Parser parser);
extern int        XML_GetIdAttributeIndex(XML_Parser parser);

extern void XML_SetElementHandler(
    XML_Parser parser,
    void (*start)(void *, const char *, const char **),
    void (*end)(void *, const char *)
);
extern void XML_SetCharacterDataHandler(
    XML_Parser parser,
    void (*handler)(void *, const char *, int)
);
extern void XML_SetProcessingInstructionHandler(
    XML_Parser parser,
    void (*handler)(void *, const char *, const char *)
);
extern void XML_SetCommentHandler(
    XML_Parser parser,
    void (*handler)(void *, const char *)
);
extern void XML_SetStartCdataSectionHandler(
    XML_Parser parser,
    void (*handler)(void *)
);
extern void XML_SetEndCdataSectionHandler(
    XML_Parser parser,
    void (*handler)(void *)
);
extern void XML_SetDefaultHandler(
    XML_Parser parser,
    void (*handler)(void *, const char *, int)
);
extern void XML_SetXmlDeclHandler(
    XML_Parser parser,
    void (*handler)(void *, const char *, const char *, int)
);
extern void XML_SetStartDoctypeDeclHandler(
    XML_Parser parser,
    void (*handler)(void *, const char *, const char *, const char *, int)
);
extern void XML_SetEndDoctypeDeclHandler(
    XML_Parser parser,
    void (*handler)(void *)
);

/* --- Test infrastructure --- */

static int tests_run = 0;
static int tests_passed = 0;

#define ASSERT(cond, msg) do { \
    if (!(cond)) { \
        fprintf(stderr, "  FAIL: %s (line %d)\n", msg, __LINE__); \
        return 1; \
    } \
} while(0)

/* --- Handler state for capturing callbacks --- */

typedef struct {
    int start_count;
    int end_count;
    int char_count;
    int comment_count;
    int pi_count;
    int cdata_start_count;
    int cdata_end_count;
    int xmldecl_count;
    int doctype_start_count;
    int doctype_end_count;
    char last_element[256];
    char last_text[1024];
    char last_comment[256];
    int total_text_len;
} HandlerState;

static void start_handler(void *ud, const char *name, const char **atts) {
    HandlerState *s = (HandlerState *)ud;
    s->start_count++;
    strncpy(s->last_element, name, sizeof(s->last_element) - 1);
    s->last_element[sizeof(s->last_element) - 1] = '\0';
    (void)atts;
}

static void end_handler(void *ud, const char *name) {
    HandlerState *s = (HandlerState *)ud;
    s->end_count++;
    (void)name;
}

static void char_handler(void *ud, const char *data, int len) {
    HandlerState *s = (HandlerState *)ud;
    s->char_count++;
    s->total_text_len += len;
    int copy_len = len < (int)sizeof(s->last_text) - 1 ? len : (int)sizeof(s->last_text) - 1;
    memcpy(s->last_text, data, copy_len);
    s->last_text[copy_len] = '\0';
}

static void comment_handler(void *ud, const char *data) {
    HandlerState *s = (HandlerState *)ud;
    s->comment_count++;
    strncpy(s->last_comment, data, sizeof(s->last_comment) - 1);
    s->last_comment[sizeof(s->last_comment) - 1] = '\0';
}

static void pi_handler(void *ud, const char *target, const char *data) {
    HandlerState *s = (HandlerState *)ud;
    s->pi_count++;
    (void)target;
    (void)data;
}

static void cdata_start_handler(void *ud) {
    HandlerState *s = (HandlerState *)ud;
    s->cdata_start_count++;
}

static void cdata_end_handler(void *ud) {
    HandlerState *s = (HandlerState *)ud;
    s->cdata_end_count++;
}

static void xmldecl_handler(void *ud, const char *version, const char *encoding, int standalone) {
    HandlerState *s = (HandlerState *)ud;
    s->xmldecl_count++;
    (void)version; (void)encoding; (void)standalone;
}

static void doctype_start_handler(void *ud, const char *name, const char *sysid, const char *pubid, int has_internal) {
    HandlerState *s = (HandlerState *)ud;
    s->doctype_start_count++;
    (void)name; (void)sysid; (void)pubid; (void)has_internal;
}

static void doctype_end_handler(void *ud) {
    HandlerState *s = (HandlerState *)ud;
    s->doctype_end_count++;
}

static void init_state(HandlerState *s) {
    memset(s, 0, sizeof(*s));
}

/* --- Tests --- */

static int test_create_and_free(void) {
    XML_Parser p = XML_ParserCreate(NULL);
    ASSERT(p != NULL, "XML_ParserCreate returned NULL");
    XML_ParserFree(p);
    return 0;
}

static int test_create_with_encoding(void) {
    XML_Parser p = XML_ParserCreate("UTF-8");
    ASSERT(p != NULL, "XML_ParserCreate(UTF-8) returned NULL");
    XML_ParserFree(p);
    return 0;
}

static int test_create_ns(void) {
    XML_Parser p = XML_ParserCreateNS(NULL, '|');
    ASSERT(p != NULL, "XML_ParserCreateNS returned NULL");
    XML_ParserFree(p);
    return 0;
}

static int test_simple_parse(void) {
    XML_Parser p = XML_ParserCreate(NULL);
    ASSERT(p != NULL, "parser create");

    const char *xml = "<root><child>hello</child></root>";
    XML_Status s = XML_Parse(p, xml, (int)strlen(xml), 1);
    ASSERT(s == XML_STATUS_OK, "parse should succeed");
    ASSERT(XML_GetErrorCode(p) == XML_ERROR_NONE, "no error expected");

    XML_ParserFree(p);
    return 0;
}

static int test_element_handlers(void) {
    XML_Parser p = XML_ParserCreate(NULL);
    HandlerState state;
    init_state(&state);

    XML_SetUserData(p, &state);
    XML_SetElementHandler(p, start_handler, end_handler);

    const char *xml = "<a><b/><c><d/></c></a>";
    XML_Status s = XML_Parse(p, xml, (int)strlen(xml), 1);
    ASSERT(s == XML_STATUS_OK, "parse should succeed");
    ASSERT(state.start_count == 4, "expected 4 start elements");
    ASSERT(state.end_count == 4, "expected 4 end elements");

    XML_ParserFree(p);
    return 0;
}

static int test_character_data(void) {
    XML_Parser p = XML_ParserCreate(NULL);
    HandlerState state;
    init_state(&state);

    XML_SetUserData(p, &state);
    XML_SetCharacterDataHandler(p, char_handler);

    const char *xml = "<root>Hello, world!</root>";
    XML_Status s = XML_Parse(p, xml, (int)strlen(xml), 1);
    ASSERT(s == XML_STATUS_OK, "parse should succeed");
    ASSERT(state.char_count > 0, "expected character data callbacks");
    ASSERT(state.total_text_len == 13, "expected 13 bytes of text");

    XML_ParserFree(p);
    return 0;
}

static int test_comment_handler(void) {
    XML_Parser p = XML_ParserCreate(NULL);
    HandlerState state;
    init_state(&state);

    XML_SetUserData(p, &state);
    XML_SetCommentHandler(p, comment_handler);

    const char *xml = "<root><!-- a comment --></root>";
    XML_Status s = XML_Parse(p, xml, (int)strlen(xml), 1);
    ASSERT(s == XML_STATUS_OK, "parse should succeed");
    ASSERT(state.comment_count == 1, "expected 1 comment");

    XML_ParserFree(p);
    return 0;
}

static int test_pi_handler(void) {
    XML_Parser p = XML_ParserCreate(NULL);
    HandlerState state;
    init_state(&state);

    XML_SetUserData(p, &state);
    XML_SetProcessingInstructionHandler(p, pi_handler);

    const char *xml = "<root><?mypi data?></root>";
    XML_Status s = XML_Parse(p, xml, (int)strlen(xml), 1);
    ASSERT(s == XML_STATUS_OK, "parse should succeed");
    ASSERT(state.pi_count == 1, "expected 1 PI");

    XML_ParserFree(p);
    return 0;
}

static int test_cdata_handlers(void) {
    XML_Parser p = XML_ParserCreate(NULL);
    HandlerState state;
    init_state(&state);

    XML_SetUserData(p, &state);
    XML_SetStartCdataSectionHandler(p, cdata_start_handler);
    XML_SetEndCdataSectionHandler(p, cdata_end_handler);
    XML_SetCharacterDataHandler(p, char_handler);

    const char *xml = "<root><![CDATA[raw <data>]]></root>";
    XML_Status s = XML_Parse(p, xml, (int)strlen(xml), 1);
    ASSERT(s == XML_STATUS_OK, "parse should succeed");
    ASSERT(state.cdata_start_count == 1, "expected 1 CDATA start");
    ASSERT(state.cdata_end_count == 1, "expected 1 CDATA end");

    XML_ParserFree(p);
    return 0;
}

static int test_xmldecl_handler(void) {
    XML_Parser p = XML_ParserCreate(NULL);
    HandlerState state;
    init_state(&state);

    XML_SetUserData(p, &state);
    XML_SetXmlDeclHandler(p, xmldecl_handler);

    const char *xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><root/>";
    XML_Status s = XML_Parse(p, xml, (int)strlen(xml), 1);
    ASSERT(s == XML_STATUS_OK, "parse should succeed");
    ASSERT(state.xmldecl_count == 1, "expected 1 XML declaration");

    XML_ParserFree(p);
    return 0;
}

static int test_doctype_handlers(void) {
    XML_Parser p = XML_ParserCreate(NULL);
    HandlerState state;
    init_state(&state);

    XML_SetUserData(p, &state);
    XML_SetStartDoctypeDeclHandler(p, doctype_start_handler);
    XML_SetEndDoctypeDeclHandler(p, doctype_end_handler);

    const char *xml = "<!DOCTYPE root [ ]><root/>";
    XML_Status s = XML_Parse(p, xml, (int)strlen(xml), 1);
    ASSERT(s == XML_STATUS_OK, "parse should succeed");
    ASSERT(state.doctype_start_count == 1, "expected 1 DOCTYPE start");
    ASSERT(state.doctype_end_count == 1, "expected 1 DOCTYPE end");

    /* Also test that DOCTYPE without internal subset at least parses OK */
    XML_Parser p2 = XML_ParserCreate(NULL);
    const char *xml2 = "<!DOCTYPE root SYSTEM \"test.dtd\"><root/>";
    s = XML_Parse(p2, xml2, (int)strlen(xml2), 1);
    ASSERT(s == XML_STATUS_OK, "DOCTYPE with SYSTEM ID should parse OK");
    XML_ParserFree(p2);

    XML_ParserFree(p);
    return 0;
}

static int test_error_detection(void) {
    XML_Parser p = XML_ParserCreate(NULL);

    const char *xml = "<root><unclosed>";
    XML_Status s = XML_Parse(p, xml, (int)strlen(xml), 1);
    ASSERT(s == XML_STATUS_ERROR, "malformed XML should fail");

    XML_Error err = XML_GetErrorCode(p);
    ASSERT(err != XML_ERROR_NONE, "error code should be set");

    XML_ParserFree(p);
    return 0;
}

static int test_tag_mismatch(void) {
    XML_Parser p = XML_ParserCreate(NULL);

    const char *xml = "<root></wrong>";
    XML_Status s = XML_Parse(p, xml, (int)strlen(xml), 1);
    ASSERT(s == XML_STATUS_ERROR, "mismatched tags should fail");
    ASSERT(XML_GetErrorCode(p) == XML_ERROR_TAG_MISMATCH, "expected TAG_MISMATCH error");

    XML_ParserFree(p);
    return 0;
}

static int test_error_string(void) {
    const char *msg = XML_ErrorString(XML_ERROR_SYNTAX);
    ASSERT(msg != NULL, "error string should not be NULL");
    ASSERT(strlen(msg) > 0, "error string should not be empty");
    return 0;
}

static int test_position_info(void) {
    XML_Parser p = XML_ParserCreate(NULL);

    const char *xml = "<root>\n  <child/>\n</root>";
    XML_Parse(p, xml, (int)strlen(xml), 1);

    /* After parsing, line/column should be non-zero */
    unsigned long line = XML_GetCurrentLineNumber(p);
    ASSERT(line > 0, "line number should be > 0");

    XML_ParserFree(p);
    return 0;
}

static int test_version_string(void) {
    const char *ver = XML_ExpatVersion();
    ASSERT(ver != NULL, "version should not be NULL");
    ASSERT(strlen(ver) > 0, "version should not be empty");
    return 0;
}

static int test_parser_reset(void) {
    XML_Parser p = XML_ParserCreate(NULL);

    /* Parse once */
    const char *xml1 = "<a/>";
    XML_Status s = XML_Parse(p, xml1, (int)strlen(xml1), 1);
    ASSERT(s == XML_STATUS_OK, "first parse should succeed");

    /* Reset */
    XML_Bool reset = XML_ParserReset(p, NULL);
    ASSERT(reset != 0, "reset should succeed");

    /* Parse again */
    const char *xml2 = "<b/>";
    s = XML_Parse(p, xml2, (int)strlen(xml2), 1);
    ASSERT(s == XML_STATUS_OK, "parse after reset should succeed");

    XML_ParserFree(p);
    return 0;
}

static int test_incremental_parse(void) {
    XML_Parser p = XML_ParserCreate(NULL);
    HandlerState state;
    init_state(&state);

    XML_SetUserData(p, &state);
    XML_SetElementHandler(p, start_handler, end_handler);

    /* Feed XML in chunks */
    const char *chunk1 = "<root><chi";
    const char *chunk2 = "ld/></root>";

    XML_Status s = XML_Parse(p, chunk1, (int)strlen(chunk1), 0);
    ASSERT(s == XML_STATUS_OK, "first chunk should succeed");

    s = XML_Parse(p, chunk2, (int)strlen(chunk2), 1);
    ASSERT(s == XML_STATUS_OK, "second chunk should succeed");

    ASSERT(state.start_count == 2, "expected 2 start elements");
    ASSERT(state.end_count == 2, "expected 2 end elements");

    XML_ParserFree(p);
    return 0;
}

static int test_set_base(void) {
    XML_Parser p = XML_ParserCreate(NULL);

    XML_Status s = XML_SetBase(p, "http://example.com/");
    ASSERT(s == XML_STATUS_OK, "SetBase should succeed");

    const char *base = XML_GetBase(p);
    ASSERT(base != NULL, "GetBase should return non-NULL");

    XML_ParserFree(p);
    return 0;
}

static int test_attributes(void) {
    XML_Parser p = XML_ParserCreate(NULL);
    HandlerState state;
    init_state(&state);

    XML_SetUserData(p, &state);
    XML_SetElementHandler(p, start_handler, end_handler);

    const char *xml = "<root a=\"1\" b=\"2\" c=\"3\"/>";
    XML_Status s = XML_Parse(p, xml, (int)strlen(xml), 1);
    ASSERT(s == XML_STATUS_OK, "parse with attributes should succeed");
    ASSERT(state.start_count == 1, "expected 1 start element");
    ASSERT(strcmp(state.last_element, "root") == 0, "element name should be 'root'");

    XML_ParserFree(p);
    return 0;
}

static int test_empty_document(void) {
    XML_Parser p = XML_ParserCreate(NULL);

    XML_Status s = XML_Parse(p, "", 0, 1);
    ASSERT(s == XML_STATUS_ERROR, "empty document should fail");

    XML_ParserFree(p);
    return 0;
}

static int test_multiple_handlers(void) {
    XML_Parser p = XML_ParserCreate(NULL);
    HandlerState state;
    init_state(&state);

    XML_SetUserData(p, &state);
    XML_SetElementHandler(p, start_handler, end_handler);
    XML_SetCharacterDataHandler(p, char_handler);
    XML_SetCommentHandler(p, comment_handler);
    XML_SetProcessingInstructionHandler(p, pi_handler);

    const char *xml =
        "<root>"
        "text"
        "<!-- comment -->"
        "<?pi data?>"
        "<child/>"
        "</root>";

    XML_Status s = XML_Parse(p, xml, (int)strlen(xml), 1);
    ASSERT(s == XML_STATUS_OK, "parse should succeed");
    ASSERT(state.start_count == 2, "expected 2 start elements");
    ASSERT(state.end_count == 2, "expected 2 end elements");
    ASSERT(state.char_count > 0, "expected character data");
    ASSERT(state.comment_count == 1, "expected 1 comment");
    ASSERT(state.pi_count == 1, "expected 1 PI");

    XML_ParserFree(p);
    return 0;
}

/* --- Test runner --- */

#define RUN_TEST(fn) do { \
    tests_run++; \
    printf("  %-40s", #fn); \
    if (fn() == 0) { \
        tests_passed++; \
        printf("PASS\n"); \
    } else { \
        printf("FAIL\n"); \
    } \
} while(0)

int main(void) {
    printf("expat-ffi C integration tests\n");
    printf("========================================\n\n");

    RUN_TEST(test_create_and_free);
    RUN_TEST(test_create_with_encoding);
    RUN_TEST(test_create_ns);
    RUN_TEST(test_simple_parse);
    RUN_TEST(test_element_handlers);
    RUN_TEST(test_character_data);
    RUN_TEST(test_comment_handler);
    RUN_TEST(test_pi_handler);
    RUN_TEST(test_cdata_handlers);
    RUN_TEST(test_xmldecl_handler);
    RUN_TEST(test_doctype_handlers);
    RUN_TEST(test_error_detection);
    RUN_TEST(test_tag_mismatch);
    RUN_TEST(test_error_string);
    RUN_TEST(test_position_info);
    RUN_TEST(test_version_string);
    RUN_TEST(test_parser_reset);
    RUN_TEST(test_incremental_parse);
    RUN_TEST(test_set_base);
    RUN_TEST(test_attributes);
    RUN_TEST(test_empty_document);
    RUN_TEST(test_multiple_handlers);

    printf("\n========================================\n");
    printf("Results: %d/%d passed\n", tests_passed, tests_run);

    return (tests_passed == tests_run) ? 0 : 1;
}
