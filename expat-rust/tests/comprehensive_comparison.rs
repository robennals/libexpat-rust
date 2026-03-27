//! Comprehensive comparison tests: systematically generate XML inputs
//! and compare Rust vs C parser behavior on each one.
//!
//! Every test compares full SAX event sequences (not just status codes).

mod sax_compare;
use sax_compare::{compare, compare_incremental};

// ======================== DTD Entity Storage & Expansion ========================
#[test]
fn dtd_entity_simple() {
    compare(
        b"<!DOCTYPE r [<!ENTITY e 'hello'>]><r>&e;</r>",
        "entity simple",
    );
}
#[test]
fn dtd_entity_multi() {
    compare(
        b"<!DOCTYPE r [<!ENTITY a '1'><!ENTITY b '2'>]><r>&a;&b;</r>",
        "multi entity",
    );
}
#[test]
fn dtd_entity_undefined() {
    compare(b"<r>&undefined;</r>", "undefined entity");
}
#[test]
fn dtd_entity_predefined() {
    compare(
        b"<!DOCTYPE r [<!ENTITY e 'x'>]><r>&amp;&e;</r>",
        "predefined + custom",
    );
}

// ======================== DTD Declarations ========================
#[test]
fn dtd_element_pcdata() {
    compare(
        b"<!DOCTYPE r [<!ELEMENT r (#PCDATA)>]><r>text</r>",
        "element PCDATA",
    );
}
#[test]
fn dtd_element_empty() {
    compare(b"<!DOCTYPE r [<!ELEMENT r EMPTY>]><r/>", "element EMPTY");
}
#[test]
fn dtd_element_any() {
    compare(b"<!DOCTYPE r [<!ELEMENT r ANY>]><r/>", "element ANY");
}
#[test]
fn dtd_element_seq() {
    compare(
        b"<!DOCTYPE r [<!ELEMENT r (a,b)><!ELEMENT a EMPTY><!ELEMENT b EMPTY>]><r><a/><b/></r>",
        "element seq",
    );
}
#[test]
fn dtd_attlist_implied() {
    compare(
        b"<!DOCTYPE r [<!ATTLIST r a CDATA #IMPLIED>]><r/>",
        "attlist implied",
    );
}
#[test]
fn dtd_attlist_required() {
    compare(
        b"<!DOCTYPE r [<!ATTLIST r a CDATA #REQUIRED>]><r a='v'/>",
        "attlist required",
    );
}
#[test]
fn dtd_attlist_default() {
    compare(
        b"<!DOCTYPE r [<!ATTLIST r a CDATA 'def'>]><r/>",
        "attlist default",
    );
}
#[test]
fn dtd_attlist_fixed() {
    compare(
        b"<!DOCTYPE r [<!ATTLIST r a CDATA #FIXED 'x'>]><r/>",
        "attlist fixed",
    );
}
#[test]
fn dtd_attlist_enum() {
    compare(
        b"<!DOCTYPE r [<!ATTLIST r a (x|y) #IMPLIED>]><r a='x'/>",
        "attlist enum",
    );
}
#[test]
fn dtd_notation_system() {
    compare(
        b"<!DOCTYPE r [<!NOTATION n SYSTEM 'sys'>]><r/>",
        "notation system",
    );
}
#[test]
fn dtd_notation_public() {
    compare(
        b"<!DOCTYPE r [<!NOTATION n PUBLIC 'pub' 'sys'>]><r/>",
        "notation public",
    );
}
#[test]
fn dtd_doctype_public() {
    compare(b"<!DOCTYPE r PUBLIC 'pub' 'sys'><r/>", "doctype public");
}
#[test]
fn dtd_doctype_system() {
    compare(b"<!DOCTYPE r SYSTEM 'sys'><r/>", "doctype system");
}

// ======================== Error Cases ========================
#[test]
fn err_dup_attr() {
    compare(b"<r a='1' a='2'/>", "dup attr");
}
#[test]
fn err_bad_charref_zero() {
    compare(b"<r>&#0;</r>", "charref zero");
}
#[test]
fn err_unclosed_tag() {
    compare(b"<r>", "unclosed tag");
}
#[test]
fn err_mismatch() {
    compare(b"<r></s>", "tag mismatch");
}
#[test]
fn err_nul() {
    compare(b"<r>\x00</r>", "null byte");
}
#[test]
fn err_lt_in_attr() {
    compare(b"<r a='<'/>", "lt in attr");
}
#[test]
fn err_lone_amp() {
    compare(b"<r>&</r>", "lone amp");
}
#[test]
fn err_unclosed_comment() {
    compare(b"<r><!-- no close", "unclosed comment");
}
#[test]
fn err_unclosed_cdata() {
    compare(b"<r><![CDATA[no close", "unclosed cdata");
}
#[test]
fn err_double_root() {
    compare(b"<r/><s/>", "double root");
}
#[test]
fn err_text_after() {
    compare(b"<r/>text", "text after root");
}
#[test]
fn err_misplaced_xmldecl() {
    compare(b"\n<?xml version='1.0'?><r/>", "misplaced xmldecl");
}
#[test]
fn err_bad_encoding() {
    compare(
        b"<?xml version='1.0' encoding='bogus'?><r/>",
        "bad encoding",
    );
}
#[test]
fn err_utf16_in_utf8() {
    compare(
        b"<?xml version='1.0' encoding='utf-16'?><r/>",
        "utf-16 in utf-8",
    );
}
#[test]
fn err_empty() {
    compare(b"", "empty");
}
#[test]
fn err_ws_only() {
    compare(b"   ", "ws only");
}
#[test]
fn err_partial() {
    compare(b"<r", "partial");
}

// ======================== Content ========================
#[test]
fn content_charref() {
    compare(b"<r>&#65;&#x42;</r>", "charrefs");
}
#[test]
fn content_entities() {
    compare(b"<r>&amp;&lt;&gt;&apos;&quot;</r>", "predefined entities");
}
#[test]
fn content_cdata() {
    compare(b"<r><![CDATA[<>&]]></r>", "cdata");
}
#[test]
fn content_cdata_rsqb() {
    compare(b"<r><![CDATA[a]b]]c]]></r>", "cdata brackets");
}
#[test]
fn content_pi() {
    compare(b"<r><?t d?></r>", "pi");
}
#[test]
fn content_comment() {
    compare(b"<r><!-- c --></r>", "comment");
}
#[test]
fn content_mixed() {
    compare(b"<r>t<a/>m<b>i</b>e</r>", "mixed");
}
#[test]
fn content_deep() {
    compare(b"<a><b><c><d><e/></d></c></b></a>", "deep");
}
#[test]
fn content_crlf() {
    compare(b"<r>\r\n\r</r>", "crlf");
}
#[test]
fn content_long() {
    let t = "x".repeat(5000);
    compare(format!("<r>{}</r>", t).as_bytes(), "long");
}

// ======================== Attributes ========================
#[test]
fn attr_many() {
    compare(
        b"<r a='1' b='2' c='3' d='4' e='5' f='6' g='7' h='8'/>",
        "many attrs",
    );
}
#[test]
fn attr_ws() {
    compare(b"<r a = 'v' />", "ws around eq");
}
#[test]
fn attr_entity() {
    compare(b"<r a='&amp;'/>", "entity in attr");
}
#[test]
fn attr_charref() {
    compare(b"<r a='&#65;'/>", "charref in attr");
}
#[test]
fn attr_utf8() {
    compare(b"<r a='\xc3\xa9'/>", "utf8 in attr");
}
#[test]
fn attr_long() {
    let v = "x".repeat(1024);
    compare(format!("<r a='{}'/>", v).as_bytes(), "long attr");
}

// ======================== Encoding ========================
#[test]
fn enc_utf8_bom() {
    compare(b"\xef\xbb\xbf<r/>", "utf8 bom");
}
#[test]
fn enc_utf16be() {
    compare(b"\xfe\xff\x00<\x00r\x00/\x00>", "utf16be");
}
#[test]
fn enc_utf16le() {
    compare(b"\xff\xfe<\x00r\x00/\x00>\x00", "utf16le");
}
#[test]
fn enc_latin1() {
    compare(b"<?xml version='1.0' encoding='iso-8859-1'?><r/>", "latin1");
}

// ======================== Prolog/Epilog ========================
#[test]
fn prolog_all() {
    compare(
        b"<?xml version='1.0' encoding='utf-8' standalone='yes'?><r/>",
        "all xmldecl",
    );
}
#[test]
fn prolog_comment() {
    compare(b"<!-- c --><r/>", "prolog comment");
}
#[test]
fn prolog_pi() {
    compare(b"<?t d?><r/>", "prolog pi");
}
#[test]
fn epilog_ws() {
    compare(b"<r/>\n\r\n  ", "epilog ws");
}
#[test]
fn epilog_comment() {
    compare(b"<r/><!-- e -->", "epilog comment");
}
#[test]
fn epilog_pi() {
    compare(b"<r/><?t d?>", "epilog pi");
}

// ======================== UTF-8 ========================
#[test]
fn utf8_2byte() {
    compare(b"<r>\xc3\xa9</r>", "2byte");
}
#[test]
fn utf8_3byte() {
    compare(b"<r>\xe4\xb8\xad</r>", "3byte");
}
#[test]
fn utf8_in_name() {
    compare(b"<r\xc3\xa9/>", "utf8 name");
}
#[test]
fn utf8_in_end() {
    compare(b"<r\xc3\xa9></r\xc3\xa9>", "utf8 end");
}
#[test]
fn utf8_invalid() {
    compare(b"<r>\x80</r>", "invalid utf8");
}

// ======================== Namespace-like ========================
#[test]
fn ns_colon() {
    compare(b"<a:b/>", "colon in name");
}
#[test]
fn ns_xmlns() {
    compare(b"<r xmlns='http://example.com'/>", "xmlns");
}
#[test]
fn ns_prefix() {
    compare(b"<r xmlns:x='http://example.com' x:a='1'/>", "prefix");
}
#[test]
fn ns_elem() {
    compare(b"<x:r xmlns:x='http://example.com'/>", "prefixed elem");
}

// ======================== Incremental ========================
#[test]
fn incr_simple() {
    compare_incremental(b"<r/>", "incr simple");
}
#[test]
fn incr_content() {
    compare_incremental(b"<r>hello</r>", "incr content");
}
#[test]
fn incr_xmldecl() {
    compare_incremental(b"<?xml version='1.0'?><r/>", "incr xmldecl");
}
#[test]
fn incr_doctype() {
    compare_incremental(b"<!DOCTYPE r><r/>", "incr doctype");
}
#[test]
fn incr_cdata() {
    compare_incremental(b"<r><![CDATA[test]]></r>", "incr cdata");
}
#[test]
fn incr_entity() {
    compare_incremental(b"<!DOCTYPE r [<!ENTITY e 'v'>]><r>&e;</r>", "incr entity");
}
#[test]
fn incr_comment() {
    compare_incremental(b"<r><!-- c --></r>", "incr comment");
}
#[test]
fn incr_pi() {
    compare_incremental(b"<r><?t d?></r>", "incr pi");
}
#[test]
fn incr_attrs() {
    compare_incremental(b"<r a='1' b='2'/>", "incr attrs");
}
#[test]
fn incr_nested() {
    compare_incremental(b"<a><b/></a>", "incr nested");
}
#[test]
fn incr_epilog() {
    compare_incremental(b"<r/>\n<!-- x -->\n", "incr epilog");
}
#[test]
fn incr_utf8() {
    compare_incremental(b"<r>\xc3\xa9</r>", "incr utf8");
}
#[test]
fn incr_complex() {
    compare_incremental(
        b"<?xml version='1.0'?><!DOCTYPE r [<!ENTITY e 'v'>]><r a='1'>&amp;&e;<!-- c --><![CDATA[d]]></r>\n",
        "incr complex"
    );
}
