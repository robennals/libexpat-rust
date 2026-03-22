
use expat_rust::xmlparse::{Parser, XmlStatus, XmlError};

#[test]
fn test_unbound_prefix_element() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();
    let xml = b"<a:doc/>";
    let status = parser.parse(xml, true);

    println!("Status: {:?}", status);
    println!("Error code: {:?}", parser.error_code());

    assert_eq!(status, XmlStatus::Error, "Expected parse to fail");
    assert_eq!(parser.error_code(), XmlError::UnboundPrefix, "Expected UnboundPrefix error");
}

#[test]
fn test_unbound_prefix_attribute() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();
    let xml = b"<doc a:attr=''/>";
    let status = parser.parse(xml, true);

    println!("Attribute test Status: {:?}", status);
    println!("Attribute test Error code: {:?}", parser.error_code());

    assert_eq!(status, XmlStatus::Error, "Expected parse to fail");
    assert_eq!(parser.error_code(), XmlError::UnboundPrefix, "Expected UnboundPrefix error");
}

#[test]
fn test_return_ns_triplet() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();
    parser.set_return_ns_triplet(true);

    let xml1 = b"<foo:e xmlns:foo='http://example.org/' bar:a='12'\n       xmlns:bar='http://example.org/'>";
    let status1 = parser.parse(xml1, false);
    println!("First parse status: {:?}", status1);
    println!("First parse error: {:?}", parser.error_code());

    let xml2 = b"</foo:e>";
    let status2 = parser.parse(xml2, true);
    println!("Second parse status: {:?}", status2);
    println!("Second parse error: {:?}", parser.error_code());

    assert_eq!(status1, XmlStatus::Ok, "First parse should succeed");
    assert_eq!(status2, XmlStatus::Ok, "Second parse should succeed");
}

#[test]
fn test_start_ns_clears_start_element() {
    let mut parser = Parser::new_ns(None, ' ').unwrap();

    let xml = b"<e xmlns='http://example.org/'></e>";
    let status = parser.parse(xml, true);
    println!("Parse status: {:?}", status);
    println!("Parse error: {:?}", parser.error_code());

    assert_eq!(status, XmlStatus::Ok, "Parse should succeed");
}
