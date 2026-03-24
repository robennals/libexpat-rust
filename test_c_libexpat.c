#include <stdio.h>
#include <string.h>
#include "expat.h"

int call_count = 0;

void start_element_handler(void *userData, const char *name, const char **atts) {
  printf("START: %s\n", name);
}

void end_element_handler(void *userData, const char *name) {
  printf("END: %s\n", name);
}

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

  printf("Call %d: external_entity_param called for %s\n", ++call_count, systemId ? systemId : "NULL");

  if (systemId == NULL)
    return XML_STATUS_OK;

  ext_parser = XML_ExternalEntityParserCreate(parser, context, NULL);
  if (ext_parser == NULL) {
    fprintf(stderr, "Could not create external entity parser\n");
    return XML_STATUS_ERROR;
  }

  if (strcmp(systemId, "004-1.ent") == 0) {
    printf("Call %d: Parsing 004-1.ent\n", call_count);
    int result = XML_Parse(ext_parser, text1, strlen(text1), 1);
    int err = XML_GetErrorCode(ext_parser);
    printf("Call %d: 004-1.ent parse result: %d, error code: %d (%s)\n", call_count, result, err, XML_ErrorString(err));
    XML_ParserFree(ext_parser);
    return XML_STATUS_ERROR;
  } else if (strcmp(systemId, "004-2.ent") == 0) {
    printf("Call %d: Parsing 004-2.ent\n", call_count);
    int result = XML_Parse(ext_parser, text2, strlen(text2), 1);
    int err = XML_GetErrorCode(ext_parser);
    printf("Call %d: 004-2.ent parse result: %d, error code: %d (%s)\n", call_count, result, err, XML_ErrorString(err));
    XML_ParserFree(ext_parser);
    return XML_STATUS_ERROR;
  }
  printf("Unknown system ID: %s\n", systemId);
  return XML_STATUS_ERROR;
}

void test_invalid_tag_in_dtd() {
  const char *text = "<!DOCTYPE doc SYSTEM '004-1.ent'>\n"
                     "<doc></doc>\n";

  XML_Parser parser = XML_ParserCreate(NULL);
  XML_SetParamEntityParsing(parser, XML_PARAM_ENTITY_PARSING_ALWAYS);
  XML_SetExternalEntityRefHandler(parser, external_entity_param);
  XML_SetElementHandler(parser, start_element_handler, end_element_handler);
  
  printf("Starting main parse\n");
  int result = XML_Parse(parser, text, strlen(text), 1);
  int err = XML_GetErrorCode(parser);
  printf("Main parse result: %d, error code: %d (%s)\n", result, err, XML_ErrorString(err));
  
  XML_ParserFree(parser);
}

int main() {
  test_invalid_tag_in_dtd();
  return 0;
}
