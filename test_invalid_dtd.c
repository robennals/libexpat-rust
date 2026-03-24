#include <stdio.h>
#include <string.h>
#include "expat.h"

int external_entity_param(XML_Parser parser, const XML_Char *context,
                         const XML_Char *base, const XML_Char *systemId,
                         const XML_Char *publicId) {
  const char *text1 = "<!ELEMENT doc EMPTY>\n"
                      "<!ENTITY % e1 SYSTEM '004-2.ent'>\n"
                      "<!ENTITY % e2 '%e1;'>\n"
                      "%e1;\n";
  const char *text2 = "<!ELEMENT el EMPTY>\n"
                      "<el/>\n";
  XML_Parser ext_parser;

  if (systemId == NULL)
    return XML_STATUS_OK;

  ext_parser = XML_ExternalEntityParserCreate(parser, context, NULL);
  if (ext_parser == NULL) {
    printf("ERROR: Could not create external entity parser\n");
    return XML_STATUS_ERROR;
  }

  if (strcmp(systemId, "004-1.ent") == 0) {
    printf("Parsing 004-1.ent...\n");
    int result = XML_Parse(ext_parser, text1, strlen(text1), 1);
    int err = XML_GetErrorCode(ext_parser);
    printf("004-1.ent result: %d, error: %d (%s)\n", result, err, XML_ErrorString(err));
    if (result != XML_STATUS_ERROR) {
      printf("FAIL: 004-1.ent should have failed!\n");
    } else if (err != XML_ERROR_EXTERNAL_ENTITY_HANDLING) {
      printf("FAIL: 004-1.ent error should be EXTERNAL_ENTITY_HANDLING, got %d\n", err);
    } else {
      printf("PASS: 004-1.ent failed correctly\n");
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_ERROR;
  } else if (strcmp(systemId, "004-2.ent") == 0) {
    printf("Parsing 004-2.ent...\n");
    int result = XML_Parse(ext_parser, text2, strlen(text2), 1);
    int err = XML_GetErrorCode(ext_parser);
    printf("004-2.ent result: %d, error: %d (%s)\n", result, err, XML_ErrorString(err));
    if (result != XML_STATUS_ERROR) {
      printf("FAIL: 004-2.ent should have failed!\n");
    } else if (err != XML_ERROR_SYNTAX) {
      printf("FAIL: 004-2.ent error should be SYNTAX, got %d\n", err);
    } else {
      printf("PASS: 004-2.ent failed correctly\n");
    }
    XML_ParserFree(ext_parser);
    return XML_STATUS_ERROR;
  }
  printf("ERROR: Unknown system ID: %s\n", systemId);
  return XML_STATUS_ERROR;
}

void test_invalid_tag_in_dtd() {
  const char *text = "<!DOCTYPE doc SYSTEM '004-1.ent'>\n"
                     "<doc></doc>\n";

  XML_Parser parser = XML_ParserCreate(NULL);
  XML_SetParamEntityParsing(parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
  XML_SetExternalEntityRefHandler(parser, external_entity_param);
  
  printf("Starting main parse...\n");
  int result = XML_Parse(parser, text, strlen(text), 1);
  int err = XML_GetErrorCode(parser);
  printf("Main parse result: %d, error: %d (%s)\n", result, err, XML_ErrorString(err));
  if (result == XML_STATUS_ERROR && err == XML_ERROR_EXTERNAL_ENTITY_HANDLING) {
    printf("PASS: Main parse failed with correct error\n");
  } else {
    printf("FAIL: Main parse should have failed with EXTERNAL_ENTITY_HANDLING\n");
  }
  
  XML_ParserFree(parser);
}

int main() {
  test_invalid_tag_in_dtd();
  return 0;
}
