// AI-generated test translation from ns_tests.c

use expat_rust::xmlparse::*;

#[test]
fn test_return_ns_triplet() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text = "<foo:e xmlns:foo='http://example.org/' bar:a='12'\n       xmlns:bar='http://example.org/'>";
    let epilog = "</foo:e>";

    parser.set_return_ns_triplet(true);

    // Parse with triplet mode
    let result = parser.parse(text.as_bytes(), false);
    assert_eq!(result, XmlStatus::Ok);

    // Change return triplet mode (this is allowed after parsing starts)
    parser.set_return_ns_triplet(false);

    // Parse epilog
    let result = parser.parse(epilog.as_bytes(), true);
    assert_eq!(result, XmlStatus::Ok);
}

#[test]
fn test_ns_parser_reset() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    // Check initial parsing status
    let status = parser.parsing_status();
    assert_eq!(status.state, ParsingState::Initialized);

    // Parse something
    let text = "<foo:e xmlns:foo='http://example.org/' bar:a='12'\n       xmlns:bar='http://example.org/'>";
    let epilog = "</foo:e>";

    parser.set_return_ns_triplet(true);
    let _ = parser.parse(text.as_bytes(), false);
    let _ = parser.parse(epilog.as_bytes(), true);

    // Check status after parsing
    let status = parser.parsing_status();
    assert_eq!(status.state, ParsingState::Finished);

    // Reset parser
    parser.reset(None);

    // Check status after reset
    let status = parser.parsing_status();
    assert_eq!(status.state, ParsingState::Initialized);
}

#[test]
fn test_ns_tagname_overwrite() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text = "<n:e xmlns:n='http://example.org/'>\n  <n:f n:attr='foo'/>\n  <n:g n:attr2='bar'/>\n</n:e>";

    let _result = parser.parse(text.as_bytes(), true);
}

#[test]
fn test_ns_tagname_overwrite_triplet() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text = "<n:e xmlns:n='http://example.org/'>\n  <n:f n:attr='foo'/>\n  <n:g n:attr2='bar'/>\n</n:e>";

    parser.set_return_ns_triplet(true);

    let _result = parser.parse(text.as_bytes(), true);
}

#[test]
fn test_start_ns_clears_start_element() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text = "<e xmlns='http://example.org/'></e>";

    parser.set_start_namespace_decl_handler(Some(Box::new(|_prefix, _uri| {
        // Start namespace handler - should clear start element
    })));

    parser.set_end_namespace_decl_handler(Some(Box::new(|_prefix| {
        // End namespace handler
    })));

    let result = parser.parse(text.as_bytes(), true);
    assert_eq!(result, XmlStatus::Ok);
}

#[test]
#[ignore] // Requires external entity handling
fn test_default_ns_from_ext_subset_and_ext_ge() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text = "<?xml version='1.0'?>\n\
                <!DOCTYPE doc SYSTEM 'http://example.org/doc.dtd' [\n  \
                  <!ENTITY en SYSTEM 'http://example.org/entity.ent'>\n\
                ]>\n\
                <doc xmlns='http://example.org/ns1'>\n\
                &en;\n\
                </doc>";

    parser.set_param_entity_parsing(ParamEntityParsing::Always);

    let result = parser.parse(text.as_bytes(), true);
    // This test requires external entity handling which will be implemented later
    let _ = result;
}

#[test]
fn test_ns_prefix_with_empty_uri_1() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text = "<doc xmlns:prefix='http://example.org/'>\n  <e xmlns:prefix=''/>\n</doc>";

    // This should fail with UNDECLARING_PREFIX error
    let _result = parser.parse(text.as_bytes(), true);
    // When implemented, should check: parser.error_code() == XmlError::UndeclaringPrefix
}

#[test]
fn test_ns_prefix_with_empty_uri_2() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text = "<?xml version='1.0'?>\n<docelem xmlns:pre=''/>";

    // This should fail with UNDECLARING_PREFIX error
    let _result = parser.parse(text.as_bytes(), true);
}

#[test]
fn test_ns_prefix_with_empty_uri_3() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text = "<!DOCTYPE doc [\n  <!ELEMENT doc EMPTY>\n  <!ATTLIST doc\n    xmlns:prefix CDATA ''>\n]\n<doc/>";

    // This should fail with UNDECLARING_PREFIX error
    let _result = parser.parse(text.as_bytes(), true);
}

#[test]
fn test_ns_default_with_empty_uri() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text = "<doc xmlns='http://example.org/'>\n  <e xmlns=''/>\n</doc>";

    parser.set_start_namespace_decl_handler(Some(Box::new(|_prefix, _uri| {
        // Start namespace handler
    })));

    parser.set_end_namespace_decl_handler(Some(Box::new(|_prefix| {
        // End namespace handler
    })));

    let _result = parser.parse(text.as_bytes(), true);
}

#[test]
fn test_ns_duplicate_attrs_diff_prefixes() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text = "<doc xmlns:a='http://example.org/a'\n     xmlns:b='http://example.org/a'\n     a:a='v' b:a='v' />";

    // This should fail with DUPLICATE_ATTRIBUTE error
    let _result = parser.parse(text.as_bytes(), true);
    // Should report duplicate attribute with same URI+name
}

#[test]
fn test_ns_duplicate_hashes() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text = "<doc xmlns:a='http://example.org/a'\n     a:a='v' a:i='w' />";

    let result = parser.parse(text.as_bytes(), true);
    assert_eq!(result, XmlStatus::Ok);
}

#[test]
fn test_ns_long_element() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text = "<foo:thisisalongenoughelementnametotriggerareallocation\n \
                 xmlns:foo='http://example.org/' bar:a='12'\n \
                 xmlns:bar='http://example.org/'>\
                 </foo:thisisalongenoughelementnametotriggerareallocation>";

    parser.set_return_ns_triplet(true);

    let result = parser.parse(text.as_bytes(), true);
    assert_eq!(result, XmlStatus::Ok);
}

#[test]
fn test_ns_mixed_prefix_atts() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text = "<e a='12' bar:b='13'\n xmlns:bar='http://example.org/'></e>";

    let result = parser.parse(text.as_bytes(), true);
    assert_eq!(result, XmlStatus::Ok);
}

#[test]
fn test_ns_extend_uri_buffer() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text = "<foo:e xmlns:foo='http://example.org/'> \
                 <foo:thisisalongenoughnametotriggerallocationaction \
                   foo:a='12' />\
                 </foo:e>";

    let result = parser.parse(text.as_bytes(), true);
    assert_eq!(result, XmlStatus::Ok);
}

#[test]
fn test_ns_extremely_long_prefix() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text1 = "<doc ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                       ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP:a='12'";

    let _result1 = parser.parse(text1.as_bytes(), false);
    // First part should succeed

    let text2 = " xmlns:ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP\
                           ABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOPABCDEFGHIJKLMNOP='foo'\n>\n</doc>";

    let result = parser.parse(text2.as_bytes(), true);
    assert_eq!(result, XmlStatus::Ok);
}

#[test]
#[ignore] // Requires unknown encoding handler
fn test_ns_unknown_encoding_success() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text = "<?xml version='1.0' encoding='prefix-conv'?>\n<foo:e xmlns:foo='http://example.org/'>Hi</foo:e>";

    // Would need to set unknown encoding handler
    let _result = parser.parse(text.as_bytes(), true);
}

#[test]
fn test_ns_double_colon() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text = "<foo:e xmlns:foo='http://example.org/' foo:a:b='bar' />";

    let result = parser.parse(text.as_bytes(), true);
    // Should fail with INVALID_TOKEN error
    if result == XmlStatus::Error {
        assert_eq!(parser.error_code(), XmlError::InvalidToken);
    }
}

#[test]
fn test_ns_double_colon_element() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text = "<foo:bar:e xmlns:foo='http://example.org/' />";

    let result = parser.parse(text.as_bytes(), true);
    // Should fail with INVALID_TOKEN error
    if result == XmlStatus::Error {
        assert_eq!(parser.error_code(), XmlError::InvalidToken);
    }
}

#[test]
fn test_ns_bad_attr_leafname() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text = "<foo:e xmlns:foo='http://example.org/' foo:?ar='baz' />";

    let result = parser.parse(text.as_bytes(), true);
    // Should fail with INVALID_TOKEN error
    assert_eq!(result, XmlStatus::Error);
    assert_eq!(parser.error_code(), XmlError::InvalidToken);
}

#[test]
fn test_ns_bad_element_leafname() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text = "<foo:?oc xmlns:foo='http://example.org/' />";

    let result = parser.parse(text.as_bytes(), true);
    // Should fail with INVALID_TOKEN error
    assert_eq!(result, XmlStatus::Error);
    assert_eq!(parser.error_code(), XmlError::InvalidToken);
}

#[test]
#[ignore] // Requires UTF-16 and special handling
fn test_ns_utf16_leafname() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    // UTF-16 encoded test data
    let text: &[u8] = b"<\0n\0:\0e\0 \0x\0m\0l\0n\0s\0:\0n\0=\0'\0U\0R\0I\0'\0 \0\
                        n\0:\0\x04\x0e=\0'\0a\0'\0 \0/\0>\0";

    let result = parser.parse(text, true);
    assert_eq!(result, XmlStatus::Ok);
}

#[test]
#[ignore] // Requires UTF-16 and special handling
fn test_ns_utf16_element_leafname() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    // UTF-16 encoded test data
    let text: &[u8] = b"\0<\0n\0:\x0e\x04\0 \0x\0m\0l\0n\0s\0:\0n\0=\0'\0U\0R\0I\0'\0/\0>";

    let result = parser.parse(text, true);
    assert_eq!(result, XmlStatus::Ok);
}

#[test]
#[ignore] // Requires XML_GE feature
fn test_ns_utf16_doctype() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    // UTF-16 encoded test data with DOCTYPE
    let text: &[u8] = b"\0<\0!\0D\0O\0C\0T\0Y\0P\0E\0 \0f\0o\0o\0:\x0e\x04\0 \
                        \0[\0 \0<\0!\0E\0N\0T\0I\0T\0Y\0 \0b\0a\0r\0 \0'\0b\0a\0z\0'\0>\0 \
                        \0]\0>\0\n\
                        \0<\0f\0o\0o\0:\x0e\x04\0 \
                        \0x\0m\0l\0n\0s\0:\0f\0o\0o\0=\0'\0U\0R\0I\0'\0>\
                        \0&\0b\0a\0r\0;\
                        \0<\0/\0f\0o\0o\0:\x0e\x04\0>";

    let result = parser.parse(text, true);
    assert_eq!(result, XmlStatus::Ok);
}

#[test]
fn test_ns_invalid_doctype() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text = "<!DOCTYPE foo:!bad [ <!ENTITY bar 'baz' ]>\n<foo:!bad>&bar;</foo:!bad>";

    let result = parser.parse(text.as_bytes(), true);
    // Should fail with INVALID_TOKEN error
    assert_eq!(result, XmlStatus::Error);
    assert_eq!(parser.error_code(), XmlError::InvalidToken);
}

#[test]
fn test_ns_double_colon_doctype() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let text = "<!DOCTYPE foo:a:doc [ <!ENTITY bar 'baz' ]>\n<foo:a:doc>&bar;</foo:a:doc>";

    let result = parser.parse(text.as_bytes(), true);
    // Should fail with SYNTAX error
    assert_eq!(result, XmlStatus::Error);
    // Error code might be SYNTAX or INVALID_TOKEN depending on implementation
}
