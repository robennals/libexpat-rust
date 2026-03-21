use expat_rust::xmltok_impl::{self, TokenResult};
use expat_rust::xmltok;
use expat_rust::xmlparse::*;

fn main() {
    let enc = xmltok::Utf8Encoding;
    let data = b"<!DOCTYPE doc [\n<!ELEMENT doc (chapter)>\n<!ELEMENT chapter (#PCDATA)>\n]>\n<doc><chapter>Wombats are go</chapter></doc>";
    let end = data.len();
    let mut pos = 0;
    for i in 0..50 {
        if pos >= end { break; }
        let r = xmltok_impl::prolog_tok(&enc, data, pos, end);
        match r {
            Ok(TokenResult { token, next_pos }) => {
                let text = String::from_utf8_lossy(&data[pos..next_pos.min(end)]);
                let display: String = text.chars().take(40).collect();
                println!("{:3}: {:?} -> {:?} text={:?}", i, pos, token, display);
                if next_pos <= pos { break; }
                pos = next_pos;
            }
            Err(e) => {
                println!("{:3}: {:?} -> Err({}) byte=0x{:02X}", i, pos, e, data[e.min(end-1)]);
                break;
            }
        }
    }
    
    println!("\n--- Full parser test ---");
    let mut parser = Parser::new(None).expect("Parser creation failed");
    let result = parser.parse(data, true);
    println!("result: {:?}, error: {:?}", result, parser.error_code());
}
