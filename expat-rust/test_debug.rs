use expat_rust::Parser;
use expat_rust::XmlStatus;
use expat_rust::XmlError;

fn main() {
    let text = b"<doc></doc>\xe2\x82";
    let mut parser = Parser::new(None).expect("Parser creation failed");

    // First parse without final flag should succeed
    let result1 = parser.parse(text, false);
    println!("First parse result: {:?}", result1);
    println!("Error code: {:?}", parser.error_code());

    // Finalizing should fail with partial char error
    let result2 = parser.parse(b"", true);
    println!("Second parse result: {:?}", result2);
    println!("Error code: {:?}", parser.error_code());
}
