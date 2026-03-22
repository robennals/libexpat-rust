/**
 * basic_parse.c — Minimal XML parsing example using expat-rust's C API.
 *
 * This is a drop-in replacement for code using libexpat. The API is identical:
 * same headers, same function names, same behavior.
 *
 * Build (after building expat-ffi):
 *   cargo build --release -p expat-ffi
 *   cc -o basic_parse basic_parse.c -L../../target/release -lexpat -Wl,-rpath,../../target/release
 *
 * Run:
 *   ./basic_parse
 */

#include <stdio.h>
#include <string.h>

/* These are the standard libexpat function signatures.
   When linking against expat-rust's libexpat.so, they resolve to Rust code. */

typedef void *XML_Parser;
typedef int XML_Status;
typedef int XML_Error;

extern XML_Parser XML_ParserCreate(const char *encoding);
extern void XML_ParserFree(XML_Parser parser);
extern XML_Status XML_Parse(XML_Parser parser, const char *s, int len, int isFinal);
extern XML_Error XML_GetErrorCode(XML_Parser parser);
extern const char *XML_ErrorString(XML_Error code);
extern unsigned long XML_GetCurrentLineNumber(XML_Parser parser);
extern unsigned long XML_GetCurrentColumnNumber(XML_Parser parser);
extern void XML_SetUserData(XML_Parser parser, void *userData);
extern void XML_SetElementHandler(
    XML_Parser parser,
    void (*start)(void *, const char *, const char **),
    void (*end)(void *, const char *)
);
extern void XML_SetCharacterDataHandler(
    XML_Parser parser,
    void (*handler)(void *, const char *, int)
);

/* --- Handlers --- */

static int depth = 0;

static void start_element(void *userData, const char *name, const char **atts) {
    (void)userData;
    for (int i = 0; i < depth; i++) printf("  ");
    printf("<%s", name);
    for (int i = 0; atts[i]; i += 2) {
        printf(" %s=\"%s\"", atts[i], atts[i + 1]);
    }
    printf(">\n");
    depth++;
}

static void end_element(void *userData, const char *name) {
    (void)userData;
    depth--;
    for (int i = 0; i < depth; i++) printf("  ");
    printf("</%s>\n", name);
}

static void char_data(void *userData, const char *s, int len) {
    (void)userData;
    /* Skip whitespace-only text */
    int all_space = 1;
    for (int i = 0; i < len; i++) {
        if (s[i] != ' ' && s[i] != '\n' && s[i] != '\r' && s[i] != '\t') {
            all_space = 0;
            break;
        }
    }
    if (all_space) return;

    for (int i = 0; i < depth; i++) printf("  ");
    printf("TEXT: %.*s\n", len, s);
}

/* --- Main --- */

int main(void) {
    const char *xml =
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n"
        "<library>\n"
        "  <book id=\"1\" genre=\"fiction\">\n"
        "    <title>The Great Gatsby</title>\n"
        "    <author>F. Scott Fitzgerald</author>\n"
        "  </book>\n"
        "  <book id=\"2\" genre=\"science\">\n"
        "    <title>A Brief History of Time</title>\n"
        "    <author>Stephen Hawking</author>\n"
        "  </book>\n"
        "</library>\n";

    XML_Parser parser = XML_ParserCreate(NULL);
    if (!parser) {
        fprintf(stderr, "Failed to create parser\n");
        return 1;
    }

    XML_SetElementHandler(parser, start_element, end_element);
    XML_SetCharacterDataHandler(parser, char_data);

    XML_Status status = XML_Parse(parser, xml, (int)strlen(xml), 1);
    if (status == 0) {  /* XML_STATUS_ERROR */
        fprintf(stderr, "Parse error at line %lu, column %lu: %s\n",
                XML_GetCurrentLineNumber(parser),
                XML_GetCurrentColumnNumber(parser),
                XML_ErrorString(XML_GetErrorCode(parser)));
        XML_ParserFree(parser);
        return 1;
    }

    printf("\nParsing complete!\n");
    XML_ParserFree(parser);
    return 0;
}
