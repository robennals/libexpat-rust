use expat_rust::xmltok_impl::{self, Encoding};
use expat_rust::xmltok;

fn main() {
    let text = b"<?xml version='1.0' \xc3\xa7?>\n<doc/>";
    let enc = xmltok::Utf8Encoding;
    
    match xmltok_impl::prolog_tok(&enc, text, 0, text.len()) {
        Ok(result) => {
            println!("Token: {:?}", result.token);
            println!("Next pos: {}", result.next_pos);
        }
        Err(err_pos) => {
            println!("Error at pos: {}", err_pos);
        }
    }
}
