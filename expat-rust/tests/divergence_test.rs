// Quick test to verify the 3 divergences

use expat_rust::xmlparse::{Parser, XmlStatus, XmlError};
use expat_sys::CParser;

fn test_divergence(xml: &[u8], name: &str) {
    let mut rust_parser = Parser::new(None).unwrap();
    let rust_status = rust_parser.parse(xml, true);
    let rust_error = rust_parser.error_code();
    
    let c_parser = CParser::new(None).unwrap();
    let (c_status, c_error) = c_parser.parse(xml, true);
    
    let rust_s = match rust_status {
        XmlStatus::Error => 0,
        XmlStatus::Ok => 1,
        XmlStatus::Suspended => 2,
    };
    let rust_e = rust_error as u32;
    
    println!("Test: {}", name);
    println!("  Rust: status={} error={}", rust_s, rust_e);
    println!("  C:    status={} error={}", c_status, c_error);
    println!("  Input: {:?}", std::str::from_utf8(xml).unwrap_or("<non-utf8>"));
    if rust_s == c_status && rust_e == c_error {
        println!("  MATCH!");
    } else {
        println!("  DIVERGENCE!");
    }
    println!();
}

#[test]
fn test_all_divergences() {
    println!("\nTesting 3 specific behavioral divergences\n");
    
    // Test 1: Junk before root element
    test_divergence(b"xxx<r/>", "Junk before root (xxx<r/>)");
    
    // Test 2: XML decl after root element  
    test_divergence(b"<r/><?xml version='1.0'?>", "XML decl after root (<r/><?xml version='1.0'?>)");
    
    // Test 3: Unclosed entity ref
    test_divergence(b"<r>&amp</r>", "Unclosed entity ref (<r>&amp</r>)");
}
