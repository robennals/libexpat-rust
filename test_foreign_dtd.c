#include <stdio.h>
#include <string.h>
#include "expat.h"

typedef struct {
  const char *dtd;
  const char *systemId;
  const char *publicId;
} ExtTest;

int external_entity_loader(XML_Parser parser, const XML_Char *context,
                          const XML_Char *base, const XML_Char *systemId,
                          const XML_Char *publicId) {
  ExtTest *test_data = (ExtTest *)XML_GetUserData(parser);
  
  printf("external_entity_loader called for context=%s, systemId=%s\n",
         context ? context : "NULL", systemId ? systemId : "NULL");
  
  XML_Parser ext_parser = XML_ExternalEntityParserCreate(parser, context, NULL);
  if (ext_parser == NULL) {
    printf("Could not create external entity parser\n");
    return 0;
  }
  
  int result = XML_Parse(ext_parser, test_data->dtd, strlen(test_data->dtd), XML_TRUE);
  printf("External parse result: %d, error: %d\n", result, XML_GetErrorCode(ext_parser));
  XML_ParserFree(ext_parser);
  return result;
}

void dummy_default_handler(void *userData, const XML_Char *s, int len) {
  // Do nothing
}

void test_set_foreign_dtd() {
  const char *text1 = "<?xml version='1.0' encoding='us-ascii'?>\n";
  const char *text2 = "<doc>&entity;</doc>";
  ExtTest test_data = {"<!ELEMENT doc (#PCDATA)*>", NULL, NULL};

  XML_Parser g_parser = XML_ParserCreate(NULL);
  
  XML_SetHashSalt(g_parser, 0x12345678);
  XML_SetParamEntityParsing(g_parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
  XML_SetUserData(g_parser, &test_data);
  XML_SetExternalEntityRefHandler(g_parser, external_entity_loader);
  XML_SetDefaultHandler(g_parser, dummy_default_handler);
  
  printf("Setting foreign DTD...\n");
  if (XML_UseForeignDTD(g_parser, XML_TRUE) != XML_ERROR_NONE) {
    printf("Could not set foreign DTD\n");
    return;
  }
  
  printf("Parsing text1...\n");
  if (XML_Parse(g_parser, text1, strlen(text1), XML_FALSE) == XML_STATUS_ERROR) {
    printf("text1 parse failed: %d (%s)\n", XML_GetErrorCode(g_parser), XML_ErrorString(XML_GetErrorCode(g_parser)));
    XML_ParserFree(g_parser);
    return;
  }
  
  printf("Checking USB feature after parsing...\n");
  int err = XML_UseForeignDTD(g_parser, XML_TRUE);
  if (err != XML_ERROR_CANT_CHANGE_FEATURE_ONCE_PARSING) {
    printf("Failed to reject late foreign DTD setting: %d\n", err);
  }
  
  printf("Parsing text2...\n");
  if (XML_Parse(g_parser, text2, strlen(text2), XML_TRUE) == XML_STATUS_ERROR) {
    int err2 = XML_GetErrorCode(g_parser);
    printf("text2 parse failed: %d (%s)\n", err2, XML_ErrorString(err2));
    XML_ParserFree(g_parser);
    return;
  }
  
  printf("Parse succeeded!\n");
  XML_ParserFree(g_parser);
}

int main() {
  test_set_foreign_dtd();
  return 0;
}
