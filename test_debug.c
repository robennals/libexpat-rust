#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef struct {
    const char *external;
    int split;
    int nested_callback_happened;
} bom_testdata;

int main() {
    const char *const external = "\xEF\xBB\xBF<!ATTLIST doc a1 CDATA 'value'>";
    const int len = (int)strlen(external);
    
    printf("External content length: %d\n", len);
    printf("First 10 bytes as hex: ");
    for (int i = 0; i < 10 && i < len; i++) {
        printf("%02x ", (unsigned char)external[i]);
    }
    printf("\n");
    
    printf("Testing split at byte 0:\n");
    int split = 0;
    printf("  First parse: %d bytes\n", split);
    printf("  Second parse: %d bytes from offset %d\n", len - split, split);
    printf("  Second parse starts with: ");
    for (int i = 0; i < 5; i++) {
        printf("%02x ", (unsigned char)external[split + i]);
    }
    printf("\n");
    
    return 0;
}
