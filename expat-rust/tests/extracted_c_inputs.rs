//! Tests extracted from C basic_tests.c - compare Rust vs C on real test inputs.
//! Auto-generated. DO NOT EDIT.

use expat_rust::xmlparse::Parser;
use expat_sys::CParser;

fn compare(xml: &[u8], desc: &str) {
    let mut r_parser = Parser::new(None).unwrap();
    let r_status = r_parser.parse(xml, true) as u32;
    let r_error = r_parser.error_code() as u32;

    let c_parser = CParser::new(None).unwrap();
    let (c_status, c_error) = c_parser.parse(xml, true);

    assert!(r_status == c_status && r_error == c_error,
        "MISMATCH {desc}: Rust status={r_status} err={r_error}, C status={c_status} err={c_error}, input={:?}",
        std::str::from_utf8(xml).unwrap_or("<binary>"));
}

#[test]
fn c_nul_byte() { compare(b"<doc>\0</doc>", "test_nul_byte"); }

#[test]
fn c_danish_latin1() { compare(b"<?xml version='1.0' encoding='iso-8859-1'?>\n<e>J\xF8rgen \xE6\xF8\xE5\xC6\xD8\xC5</e>", "test_danish_latin1"); }

#[test]
fn c_french_charref_hexidecimal() { compare(b"<?xml version='1.0' encoding='iso-8859-1'?>\n<doc>&#xE9;&#xE8;&#xE0;&#xE7;&#xEA;&#xC8;</doc>", "test_french_charref_hexidecimal"); }

#[test]
fn c_french_charref_decimal() { compare(b"<?xml version='1.0' encoding='iso-8859-1'?>\n<doc>&#233;&#232;&#224;&#231;&#234;&#200;</doc>", "test_french_charref_decimal"); }

#[test]
fn c_french_latin1() { compare(b"<?xml version='1.0' encoding='iso-8859-1'?>\n<doc>\xE9\xE8\xE0\xE7\xEa\xC8</doc>", "test_french_latin1"); }

#[test]
fn c_french_utf8() { compare(b"<?xml version='1.0' encoding='utf-8'?>\n<doc>\xC3\xA9</doc>", "test_french_utf8"); }

#[test]
fn c_utf8_false_rejection() { compare(b"<doc>\xEF\xBA\xBF</doc>", "test_utf8_false_rejection"); }

#[test]
fn c_not_utf16() { compare(b"<?xml version='1.0' encoding='utf-16'?><doc>Hi</doc>", "test_not_utf16"); }

#[test]
fn c_bad_encoding() { compare(b"<doc>Hi</doc>", "test_bad_encoding"); }

#[test]
fn c_latin1_umlauts() { compare(b"<?xml version='1.0' encoding='iso-8859-1'?>\n<e a='\xE4 \xF6 \xFC &#228; &#246; &#252; &#x00E4; &#x0F6; &#xFC; >'\n  >\xE4 \xF6 \xFC &#228; &#246; &#252; &#x00E4; &#x0F6; &#xFC; ></e>", "test_latin1_umlauts"); }

#[test]
fn c_line_number_after_parse() { compare(b"<tag>\n\n\n</tag>", "test_line_number_after_parse"); }

#[test]
fn c_column_number_after_parse() { compare(b"<tag></tag>", "test_column_number_after_parse"); }

#[test]
fn c_line_number_after_error() { compare(b"<a>\n  <b>\n  </a>", "test_line_number_after_error"); }

#[test]
fn c_column_number_after_error() { compare(b"<a>\n  <b>\n  </a>", "test_column_number_after_error"); }

#[test]
fn c_end_element_events() { compare(b"<a><b><c/></b><d><f/></d></a>", "test_end_element_events"); }

#[test]
fn c_unknown_encoding_internal_enti() { compare(b"<?xml version='1.0' encoding='unsupported-encoding'?>\n<!DOCTYPE test [<!ENTITY foo 'bar'>]>\n<test a='&foo;'/>", "test_unknown_encoding_internal_entity"); }

#[test]
fn c_unrecognised_encoding_internal() { compare(b"<?xml version='1.0' encoding='unsupported-encoding'?>\n<!DOCTYPE test [<!ENTITY foo 'bar'>]>\n<test a='&foo;'/>", "test_unrecognised_encoding_internal_entity"); }

#[test]
fn c_ext_entity_set_encoding() { compare(b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>", "test_ext_entity_set_encoding"); }

#[test]
fn c_ext_entity_no_handler() { compare(b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>", "test_ext_entity_no_handler"); }

#[test]
fn c_ext_entity_set_bom() { compare(b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>", "test_ext_entity_set_bom"); }

#[test]
fn c_ext_entity_bad_encoding() { compare(b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>", "test_ext_entity_bad_encoding"); }

#[test]
fn c_ext_entity_bad_encoding_2() { compare(b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>", "test_ext_entity_bad_encoding_2"); }

#[test]
fn c_wfc_undeclared_entity_unread_e() { compare(b"<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>", "test_wfc_undeclared_entity_unread_external_subset"); }

#[test]
fn c_wfc_undeclared_entity_standalo() { compare(b"<?xml version='1.0' encoding='us-ascii' standalone='yes'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>", "test_wfc_undeclared_entity_standalone"); }

#[test]
fn c_wfc_undeclared_entity_with_ext() { compare(b"<?xml version='1.0' encoding='us-ascii' standalone='yes'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>", "test_wfc_undeclared_entity_with_external_subset_standalone"); }

#[test]
fn c_entity_with_external_subset_un() { compare(b"<?xml version='1.0' encoding='us-ascii' standalone='yes'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>", "test_entity_with_external_subset_unless_standalone"); }

#[test]
fn c_wfc_undeclared_entity_with_ext_1() { compare(b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>", "test_wfc_undeclared_entity_with_external_subset"); }

#[test]
fn c_not_standalone_handler_reject() { compare(b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>", "test_not_standalone_handler_reject"); }

#[test]
fn c_not_standalone_handler_accept() { compare(b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>", "test_not_standalone_handler_accept"); }

#[test]
fn c_entity_start_tag_level_greater() { compare(b"<!DOCTYPE t1 [\n  <!ENTITY e1 'hello'>\n]>\n<t1>\n  <t2>&e1;</t2>\n</t1>\n", "test_entity_start_tag_level_greater_than_one"); }

#[test]
fn c_wfc_no_recursive_entity_refs() { compare(b"<!DOCTYPE doc [\n  <!ENTITY entity '&#38;entity;'>\n]>\n<doc>&entity;</doc>", "test_wfc_no_recursive_entity_refs"); }

#[test]
fn c_ext_entity_invalid_parse() { compare(b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>", "test_ext_entity_invalid_parse"); }

#[test]
fn c_empty_ns_without_namespaces() { compare(b"<doc xmlns:prefix='http://example.org/'>\n  <e xmlns:prefix=''/>\n</doc>", "test_empty_ns_without_namespaces"); }

#[test]
fn c_ns_in_attribute_default_withou() { compare(b"<!DOCTYPE e:element [\n  <!ATTLIST e:element\n    xmlns:e CDATA 'http://example.org/'>\n      ]>\n<e:element/>", "test_ns_in_attribute_default_without_namespaces"); }

#[test]
fn c_good_cdata_ascii() { compare(b"<a><![CDATA[<greeting>Hello, world!</greeting>]]></a>", "test_good_cdata_ascii"); }

#[test]
fn c_good_cdata_utf16_le() { compare(b"<\0?\0x\0m\0l\0 \0v\0e\0r\0s\0i\0o\0n\0=\0'\0\x31\0.\0\x30\0'\0 \0e\0n\0c\0o\0d\0i\0n\0g\0=\0'\0u\0t\0f\0-\01\06\0'\0?\0>\0\n\0<\0a\0>\0<\0!\0[\0C\0D\0A\0T\0A\0[\0h\0e\0l\0l\0o\0]\0]\0>\0<\0/\0a\0>\0", "test_good_cdata_utf16_le"); }

#[test]
fn c_default_current() { compare(b"<doc>hell]</doc>", "test_default_current"); }

#[test]
fn c_default_current_1() { compare(b"<!DOCTYPE doc [\n<!ENTITY entity '&#37;'>\n]>\n<doc>&entity;</doc>", "test_default_current"); }

#[test]
fn c_dtd_elements() { compare(b"<!DOCTYPE doc [\n<!ELEMENT doc (chapter)>\n<!ELEMENT chapter (#PCDATA)>\n]>\n<doc><chapter>Wombats are go</chapter></doc>", "test_dtd_elements"); }

#[test]
fn c_dtd_elements_nesting() { compare(b"<!DOCTYPE foo [\n<!ELEMENT junk ((bar|foo|xyz+), zebra*)>\n]>\n<foo/>", "test_dtd_elements_nesting"); }

#[test]
fn c_foreign_dtd_not_standalone() { compare(b"<?xml version='1.0' encoding='us-ascii'?>\n<doc>&entity;</doc>", "test_foreign_dtd_not_standalone"); }

#[test]
fn c_invalid_foreign_dtd() { compare(b"<?xml version='1.0' encoding='us-ascii'?>\n<doc>&entity;</doc>", "test_invalid_foreign_dtd"); }

#[test]
fn c_foreign_dtd_without_external_s() { compare(b"<!DOCTYPE doc [<!ENTITY foo 'bar'>]>\n<doc>&foo;</doc>", "test_foreign_dtd_without_external_subset"); }

#[test]
fn c_empty_foreign_dtd() { compare(b"<?xml version='1.0' encoding='us-ascii'?>\n<doc>&entity;</doc>", "test_empty_foreign_dtd"); }

#[test]
fn c_attributes() { compare(b"<!DOCTYPE doc [\n<!ELEMENT doc (tag)>\n<!ATTLIST doc id ID #REQUIRED>\n]><doc a='1' id='one' b='2'><tag c='3'/></doc>", "test_attributes"); }

#[test]
fn c_reset_in_entity() { compare(b"<!DOCTYPE doc [\n<!ENTITY wombat 'wom'>\n<!ENTITY entity 'hi &wom; there'>\n]>\n<doc>&entity;</doc>", "test_reset_in_entity"); }

#[test]
fn c_resume_invalid_parse() { compare(b"<doc>Hello</doc", "test_resume_invalid_parse"); }

#[test]
fn c_resume_resuspended() { compare(b"<doc>Hello<meep/>world</doc>", "test_resume_resuspended"); }

#[test]
fn c_cdata_default() { compare(b"<doc><![CDATA[Hello\nworld]]></doc>", "test_cdata_default"); }

#[test]
fn c_subordinate_reset() { compare(b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>", "test_subordinate_reset"); }

#[test]
fn c_subordinate_suspend() { compare(b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>", "test_subordinate_suspend"); }

#[test]
fn c_subordinate_xdecl_suspend() { compare(b"<!DOCTYPE doc [\n  <!ENTITY entity SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&entity;</doc>", "test_subordinate_xdecl_suspend"); }

#[test]
fn c_subordinate_xdecl_abort() { compare(b"<!DOCTYPE doc [\n  <!ENTITY entity SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&entity;</doc>", "test_subordinate_xdecl_abort"); }

#[test]
fn c_ext_entity_invalid_suspended_p() { compare(b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>", "test_ext_entity_invalid_suspended_parse"); }

#[test]
fn c_trailing_cr() { compare(b"<doc>\r", "test_trailing_cr"); }

#[test]
fn c_ext_entity_trailing_cr() { compare(b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>", "test_ext_entity_trailing_cr"); }

#[test]
fn c_ext_entity_trailing_rsqb() { compare(b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>", "test_ext_entity_trailing_rsqb"); }

#[test]
fn c_ext_entity_good_cdata() { compare(b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>", "test_ext_entity_good_cdata"); }

#[test]
fn c_user_parameters() { compare(b"<?xml version='1.0' encoding='us-ascii'?>\n<!-- Primary parse -->\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;", "test_user_parameters"); }

#[test]
fn c_ext_entity_ref_parameter() { compare(b"<?xml version='1.0' encoding='us-ascii'?>\n<!DOCTYPE doc SYSTEM 'foo'>\n<doc>&entity;</doc>", "test_ext_entity_ref_parameter"); }

#[test]
fn c_empty_parse() { compare(b"<doc></doc>", "test_empty_parse"); }

#[test]
fn c_byte_info_at_end() { compare(b"<doc></doc>", "test_byte_info_at_end"); }

#[test]
fn c_predefined_entities() { compare(b"<doc>&lt;&gt;&amp;&quot;&apos;</doc>", "test_predefined_entities"); }

#[test]
fn c_invalid_tag_in_dtd() { compare(b"<!DOCTYPE doc SYSTEM '004-1.ent'>\n<doc></doc>\n", "test_invalid_tag_in_dtd"); }

#[test]
fn c_ignore_section() { compare(b"<!DOCTYPE doc SYSTEM 'foo'>\n<doc><e>&entity;</e></doc>", "test_ignore_section"); }

#[test]
fn c_bad_ignore_section() { compare(b"<!DOCTYPE doc SYSTEM 'foo'>\n<doc><e>&entity;</e></doc>", "test_bad_ignore_section"); }

#[test]
fn c_external_bom_consumed() { compare(b"<!DOCTYPE doc SYSTEM '004-1.ent'>\n<doc></doc>\n", "test_external_bom_consumed"); }

#[test]
fn c_external_entity_values() { compare(b"<!DOCTYPE doc SYSTEM '004-1.ent'>\n<doc></doc>\n", "test_external_entity_values"); }

#[test]
fn c_ext_entity_not_standalone() { compare(b"<!DOCTYPE doc SYSTEM 'foo'>\n<doc></doc>", "test_ext_entity_not_standalone"); }

#[test]
fn c_ext_entity_value_abort() { compare(b"<!DOCTYPE doc SYSTEM '004-1.ent'>\n<doc></doc>\n", "test_ext_entity_value_abort"); }

#[test]
fn c_bad_public_doctype() { compare(b"<?xml version='1.0' encoding='utf-8'?>\n<!DOCTYPE doc PUBLIC '{BadName}' 'test'>\n<doc></doc>", "test_bad_public_doctype"); }

#[test]
fn c_attribute_enum_value() { compare(b"<?xml version='1.0' standalone='no'?>\n<!DOCTYPE animal SYSTEM 'test.dtd'>\n<animal>This is a \n    <a/>  \n\nyellow tiger</animal>", "test_attribute_enum_value"); }

#[test]
fn c_predefined_entity_redefinition() { compare(b"<!DOCTYPE doc [\n<!ENTITY apos 'foo'>\n]>\n<doc>&apos;</doc>", "test_predefined_entity_redefinition"); }

#[test]
fn c_public_notation_no_sysid() { compare(b"<!DOCTYPE doc [\n<!NOTATION note PUBLIC 'foo'>\n<!ELEMENT doc EMPTY>\n]>\n<doc/>", "test_public_notation_no_sysid"); }

#[test]
fn c_group_choice() { compare(b"<!DOCTYPE doc [\n<!ELEMENT doc (a|b|c)+>\n<!ELEMENT a EMPTY>\n<!ELEMENT b (#PCDATA)>\n<!ELEMENT c ANY>\n]>\n<doc>\n<a/>\n<b attr='foo'>This is a foo</b>\n<c></c>\n</doc>\n", "test_group_choice"); }

#[test]
fn c_skipped_parameter_entity() { compare(b"<?xml version='1.0'?>\n<!DOCTYPE root SYSTEM 'http://example.org/dtd.ent' [\n<!ELEMENT root (#PCDATA|a)* >\n]>\n<root></root>", "test_skipped_parameter_entity"); }

#[test]
fn c_recursive_external_parameter_e() { compare(b"<?xml version='1.0'?>\n<!DOCTYPE root SYSTEM 'http://example.org/dtd.ent' [\n<!ELEMENT root (#PCDATA|a)* >\n]>\n<root></root>", "test_recursive_external_parameter_entity"); }

#[test]
fn c_undefined_ext_entity_in_extern() { compare(b"<!DOCTYPE doc SYSTEM 'foo'>\n<doc></doc>\n", "test_undefined_ext_entity_in_external_dtd"); }

#[test]
fn c_abort_epilog() { compare(b"<doc></doc>\n\r\n", "test_abort_epilog"); }

#[test]
fn c_abort_epilog_2() { compare(b"<doc></doc>\n", "test_abort_epilog_2"); }

#[test]
fn c_suspend_epilog() { compare(b"<doc></doc>\n", "test_suspend_epilog"); }

#[test]
fn c_suspend_in_sole_empty_tag() { compare(b"<doc/>", "test_suspend_in_sole_empty_tag"); }

#[test]
fn c_unfinished_epilog() { compare(b"<doc></doc><", "test_unfinished_epilog"); }

#[test]
fn c_partial_char_in_epilog() { compare(b"<doc></doc>\xe2\x82", "test_partial_char_in_epilog"); }

#[test]
fn c_suspend_resume_internal_entity() { compare(b"<!DOCTYPE doc [\n<!ENTITY foo '<suspend>Hi<suspend>Ho</suspend></suspend>'>\n]>\n<doc>&foo;</doc>\n", "test_suspend_resume_internal_entity"); }

#[test]
fn c_resume_entity_with_syntax_erro() { compare(b"<!DOCTYPE doc [\n<!ENTITY foo '<suspend>Hi</wombat>'>\n]>\n<doc>&foo;</doc>\n", "test_resume_entity_with_syntax_error"); }

#[test]
fn c_restart_on_error() { compare(b"<$doc><doc></doc>", "test_restart_on_error"); }

#[test]
fn c_reject_lt_in_attribute_value() { compare(b"<!DOCTYPE doc [<!ATTLIST doc a CDATA '<bar>'>]>\n<doc></doc>", "test_reject_lt_in_attribute_value"); }

#[test]
fn c_reject_unfinished_param_in_att() { compare(b"<!DOCTYPE doc [<!ATTLIST doc a CDATA '&foo'>]>\n<doc></doc>", "test_reject_unfinished_param_in_att_value"); }

#[test]
fn c_trailing_cr_in_att_value() { compare(b"<doc a='value\r'/>", "test_trailing_cr_in_att_value"); }

#[test]
fn c_skipped_external_entity() { compare(b"<!DOCTYPE doc SYSTEM 'http://example.org/'>\n<doc></doc>\n", "test_skipped_external_entity"); }

#[test]
fn c_skipped_null_loaded_ext_entity() { compare(b"<!DOCTYPE doc SYSTEM 'http://example.org/one.ent'>\n<doc />", "test_skipped_null_loaded_ext_entity"); }

#[test]
fn c_skipped_unloaded_ext_entity() { compare(b"<!DOCTYPE doc SYSTEM 'http://example.org/one.ent'>\n<doc />", "test_skipped_unloaded_ext_entity"); }

#[test]
fn c_param_entity_with_trailing_cr() { compare(b"<!DOCTYPE doc SYSTEM 'http://example.org/'>\n<doc/>", "test_param_entity_with_trailing_cr"); }

#[test]
fn c_invalid_character_entity() { compare(b"<!DOCTYPE doc [\n  <!ENTITY entity '&#x110000;'>\n]>\n<doc>&entity;</doc>", "test_invalid_character_entity"); }

#[test]
fn c_invalid_character_entity_2() { compare(b"<!DOCTYPE doc [\n  <!ENTITY entity '&#xg0;'>\n]>\n<doc>&entity;</doc>", "test_invalid_character_entity_2"); }

#[test]
fn c_pi_handled_in_default() { compare(b"<?test processing instruction?>\n<doc/>", "test_pi_handled_in_default"); }

#[test]
fn c_comment_handled_in_default() { compare(b"<!-- This is a comment -->\n<doc/>", "test_comment_handled_in_default"); }

#[test]
fn c_pi_yml() { compare(b"<?yml something like data?><doc/>", "test_pi_yml"); }

#[test]
fn c_pi_xnl() { compare(b"<?xnl nothing like data?><doc/>", "test_pi_xnl"); }

#[test]
fn c_pi_xmm() { compare(b"<?xmm everything like data?><doc/>", "test_pi_xmm"); }

#[test]
fn c_missing_encoding_conversion_fn() { compare(b"<?xml version='1.0' encoding='no-conv'?>\n<doc>\x81</doc>", "test_missing_encoding_conversion_fn"); }

#[test]
fn c_failing_encoding_conversion_fn() { compare(b"<?xml version='1.0' encoding='failing-conv'?>\n<doc>\x81</doc>", "test_failing_encoding_conversion_fn"); }

#[test]
fn c_unknown_encoding_bad_name() { compare(b"<?xml version='1.0' encoding='prefix-conv'?>\n<\xff\x64oc>Hello, world</\xff\x64oc>", "test_unknown_encoding_bad_name"); }

#[test]
fn c_unknown_encoding_bad_name_2() { compare(b"<?xml version='1.0' encoding='prefix-conv'?>\n<d\xffoc>Hello, world</d\xffoc>", "test_unknown_encoding_bad_name_2"); }

#[test]
fn c_unknown_encoding_long_name_1() { compare(b"<?xml version='1.0' encoding='prefix-conv'?>\n<abcdefghabcdefghabcdefghijkl\x80m\x80n\x80o\x80p>Hi</abcdefghabcdefghabcdefghijkl\x80m\x80n\x80o\x80p>", "test_unknown_encoding_long_name_1"); }

#[test]
fn c_unknown_encoding_long_name_2() { compare(b"<?xml version='1.0' encoding='prefix-conv'?>\n<abcdefghabcdefghabcdefghijklmnop>Hi</abcdefghabcdefghabcdefghijklmnop>", "test_unknown_encoding_long_name_2"); }

#[test]
fn c_invalid_unknown_encoding() { compare(b"<?xml version='1.0' encoding='invalid-9'?>\n<doc>Hello world</doc>", "test_invalid_unknown_encoding"); }

#[test]
fn c_unknown_ascii_encoding_ok() { compare(b"<?xml version='1.0' encoding='ascii-like'?>\n<doc>Hello, world</doc>", "test_unknown_ascii_encoding_ok"); }

#[test]
fn c_unknown_ascii_encoding_fail() { compare(b"<?xml version='1.0' encoding='ascii-like'?>\n<doc>Hello, \x80 world</doc>", "test_unknown_ascii_encoding_fail"); }

#[test]
fn c_unknown_encoding_invalid_lengt() { compare(b"<?xml version='1.0' encoding='invalid-len'?>\n<doc>Hello, world</doc>", "test_unknown_encoding_invalid_length"); }

#[test]
fn c_unknown_encoding_invalid_topbi() { compare(b"<?xml version='1.0' encoding='invalid-a'?>\n<doc>Hello, world</doc>", "test_unknown_encoding_invalid_topbit"); }

#[test]
fn c_unknown_encoding_invalid_surro() { compare(b"<?xml version='1.0' encoding='invalid-surrogate'?>\n<doc>Hello, \x82 world</doc>", "test_unknown_encoding_invalid_surrogate"); }

#[test]
fn c_unknown_encoding_invalid_high() { compare(b"<?xml version='1.0' encoding='invalid-high'?>\n<doc>Hello, world</doc>", "test_unknown_encoding_invalid_high"); }

#[test]
fn c_unknown_encoding_invalid_attr_() { compare(b"<?xml version='1.0' encoding='prefix-conv'?>\n<doc attr='\xff\x30'/>", "test_unknown_encoding_invalid_attr_value"); }

#[test]
fn c_unknown_encoding_user_data_pri() { compare(b"<?xml version='1.0' encoding='x-unk'?>\n<root />\n", "test_unknown_encoding_user_data_primary"); }

#[test]
fn c_ext_entity_latin1_utf16le_bom() { compare(b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>", "test_ext_entity_latin1_utf16le_bom"); }

#[test]
fn c_ext_entity_latin1_utf16be_bom() { compare(b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>", "test_ext_entity_latin1_utf16be_bom"); }

#[test]
fn c_ext_entity_latin1_utf16le_bom2() { compare(b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>", "test_ext_entity_latin1_utf16le_bom2"); }

#[test]
fn c_ext_entity_latin1_utf16be_bom2() { compare(b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>", "test_ext_entity_latin1_utf16be_bom2"); }

#[test]
fn c_ext_entity_utf16_be() { compare(b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>", "test_ext_entity_utf16_be"); }

#[test]
fn c_ext_entity_utf16_le() { compare(b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>", "test_ext_entity_utf16_le"); }

#[test]
fn c_ext_entity_utf16_unknown() { compare(b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>", "test_ext_entity_utf16_unknown"); }

#[test]
fn c_ext_entity_utf8_non_bom() { compare(b"<!DOCTYPE doc [\n  <!ENTITY en SYSTEM 'http://example.org/dummy.ent'>\n]>\n<doc>&en;</doc>", "test_ext_entity_utf8_non_bom"); }

#[test]
fn c_utf8_in_cdata_section() { compare(b"<doc><![CDATA[one \xc3\xa9 two]]></doc>", "test_utf8_in_cdata_section"); }

#[test]
fn c_utf8_in_cdata_section_2() { compare(b"<doc><![CDATA[\xc3\xa9]\xc3\xa9two]]></doc>", "test_utf8_in_cdata_section_2"); }

#[test]
fn c_trailing_spaces_in_elements() { compare(b"<doc   >Hi</doc >", "test_trailing_spaces_in_elements"); }

#[test]
fn c_utf16_second_attr() { compare(b"<\0d\0 \0a\0=\0'\0\x31\0'\0 \0\x04\x0e\x08\x0e=\0'\0\x32\0'\0/\0>\0", "test_utf16_second_attr"); }

#[test]
fn c_attr_after_solidus() { compare(b"<doc attr1='a' / attr2='b'>", "test_attr_after_solidus"); }

#[test]
fn c_bad_attr_desc_keyword() { compare(b"<!DOCTYPE doc [\n  <!ATTLIST doc attr CDATA #!IMPLIED>\n]>\n<doc />", "test_bad_attr_desc_keyword"); }

#[test]
fn c_bad_doctype() { compare(b"<?xml version='1.0' encoding='prefix-conv'?>\n<!DOCTYPE doc [ \x80\x44 ]><doc/>", "test_bad_doctype"); }

#[test]
fn c_bad_doctype_utf8() { compare(b"<!DOCTYPE \xDB\x25doc><doc/>", "test_bad_doctype_utf8"); }

#[test]
fn c_bad_doctype_plus() { compare(b"<!DOCTYPE 1+ [ <!ENTITY foo 'bar'> ]>\n<1+>&foo;</1+>", "test_bad_doctype_plus"); }

#[test]
fn c_bad_doctype_star() { compare(b"<!DOCTYPE 1* [ <!ENTITY foo 'bar'> ]>\n<1*>&foo;</1*>", "test_bad_doctype_star"); }

#[test]
fn c_bad_doctype_query() { compare(b"<!DOCTYPE 1? [ <!ENTITY foo 'bar'> ]>\n<1?>&foo;</1?>", "test_bad_doctype_query"); }

#[test]
fn c_unknown_encoding_bad_ignore() { compare(b"<?xml version='1.0' encoding='prefix-conv'?><!DOCTYPE doc SYSTEM 'foo'><doc><e>&entity;</e></doc>", "test_unknown_encoding_bad_ignore"); }

#[test]
fn c_short_doctype() { compare(b"<!DOCTYPE doc></doc>", "test_short_doctype"); }

#[test]
fn c_short_doctype_2() { compare(b"<!DOCTYPE doc PUBLIC></doc>", "test_short_doctype_2"); }

#[test]
fn c_short_doctype_3() { compare(b"<!DOCTYPE doc SYSTEM></doc>", "test_short_doctype_3"); }

#[test]
fn c_long_doctype() { compare(b"<!DOCTYPE doc PUBLIC 'foo' 'bar' 'baz'></doc>", "test_long_doctype"); }

#[test]
fn c_bad_entity() { compare(b"<!DOCTYPE doc [\n  <!ENTITY foo PUBLIC>\n]>\n<doc/>", "test_bad_entity"); }

#[test]
fn c_bad_notation() { compare(b"<!DOCTYPE doc [\n  <!NOTATION n SYSTEM>\n]>\n<doc/>", "test_bad_notation"); }

#[test]
fn c_default_doctype_handler() { compare(b"<!DOCTYPE doc PUBLIC 'pubname' 'test.dtd' [\n  <!ENTITY foo 'bar'>\n]>\n<doc>&foo;</doc>", "test_default_doctype_handler"); }

#[test]
fn c_empty_element_abort() { compare(b"<abort/>", "test_empty_element_abort"); }

#[test]
fn c_entity_ref_no_elements() { compare(b"<!DOCTYPE foo [\n<!ENTITY e1 \"test\">\n]> <foo>&e1;", "test_entity_ref_no_elements"); }

#[test]
fn c_nested_entity_suspend() { compare(b"<!DOCTYPE a [\n  <!ENTITY e1 '<!--e1-->'>\n  <!ENTITY e2 '<!--e2 head-->&e1;<!--e2 tail-->'>\n  <!ENTITY e3 '<!--e3 head-->&e2;<!--e3 tail-->'>\n]>\n<a><!--start-->&e3;<!--end--></a>", "test_nested_entity_suspend"); }

#[test]
fn c_nested_entity_suspend_2() { compare(b"<!DOCTYPE doc [\n  <!ENTITY ge1 'head1Ztail1'>\n  <!ENTITY ge2 'head2&ge1;tail2'>\n  <!ENTITY ge3 'head3&ge2;tail3'>\n]>\n<doc>&ge3;</doc>", "test_nested_entity_suspend_2"); }

#[test]
fn c_reparse_deferral_is_inherited() { compare(b"<!DOCTYPE document SYSTEM 'something.ext'><document/>", "test_reparse_deferral_is_inherited"); }

#[test]
fn c_empty_ext_param_entity_in_valu() { compare(b"<!DOCTYPE r SYSTEM \"ext.dtd\"><r/>", "test_empty_ext_param_entity_in_value"); }

// Total: 149 tests extracted from C test suite
