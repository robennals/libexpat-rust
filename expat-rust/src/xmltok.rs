//! Token types, encoding detection, and XML declaration parsing.
//!
//! Ported from expat's `xmltok.c` / `xmltok_ns.c`. This module provides the
//! encoding-specific [`Encoding`] implementations
//! (UTF-8, UTF-16, Latin-1) and utilities for BOM detection, XML/text-declaration
//! scanning, and character encoding/decoding. It is the interface layer between
//! the main parser ([`xmlparse`](crate::xmlparse)) and the core tokenizer
//! ([`xmltok_impl`](crate::xmltok_impl)).

use crate::char_tables::ByteType;
use crate::xmltok_impl::Encoding;

/// Constants for UTF-8 encoding first byte patterns
const UTF8_CVAL1: u8 = 0x00;
const UTF8_CVAL2: u8 = 0xc0;
const UTF8_CVAL3: u8 = 0xe0;
const UTF8_CVAL4: u8 = 0xf0;

/// Maximum buffer sizes for encoding functions
pub const XML_UTF8_ENCODE_MAX: usize = 4;
pub const XML_UTF16_ENCODE_MAX: usize = 2;

/// UTF-8 encoding implementation
pub struct Utf8Encoding;

impl Encoding for Utf8Encoding {
    fn byte_type(&self, data: &[u8], pos: usize) -> ByteType {
        if pos >= data.len() {
            return ByteType::NONXML;
        }
        let byte = data[pos];
        if byte < 0x80 {
            crate::char_tables::ASCII_BYTE_TYPES[byte as usize]
        } else {
            crate::char_tables::UTF8_BYTE_TYPES[(byte & 0x7f) as usize]
        }
    }

    fn char_matches(&self, data: &[u8], pos: usize, c: u8) -> bool {
        if pos >= data.len() {
            return false;
        }
        data[pos] == c
    }

    fn min_bytes_per_char(&self) -> usize {
        1
    }

    fn byte_to_ascii(&self, data: &[u8], pos: usize) -> u8 {
        if pos >= data.len() {
            0
        } else {
            data[pos]
        }
    }
}

/// Latin-1 (ISO-8859-1) encoding implementation
pub struct Latin1Encoding;

impl Encoding for Latin1Encoding {
    fn byte_type(&self, data: &[u8], pos: usize) -> ByteType {
        if pos >= data.len() {
            return ByteType::NONXML;
        }
        let byte = data[pos];
        if byte < 0x80 {
            crate::char_tables::ASCII_BYTE_TYPES[byte as usize]
        } else {
            crate::char_tables::LATIN1_BYTE_TYPES[(byte & 0x7f) as usize]
        }
    }

    fn char_matches(&self, data: &[u8], pos: usize, c: u8) -> bool {
        if pos >= data.len() {
            return false;
        }
        data[pos] == c
    }

    fn min_bytes_per_char(&self) -> usize {
        1
    }

    fn byte_to_ascii(&self, data: &[u8], pos: usize) -> u8 {
        if pos >= data.len() {
            0
        } else {
            data[pos]
        }
    }
}

/// ASCII encoding implementation
pub struct AsciiEncoding;

impl Encoding for AsciiEncoding {
    fn byte_type(&self, data: &[u8], pos: usize) -> ByteType {
        if pos >= data.len() {
            return ByteType::NONXML;
        }
        let byte = data[pos];
        crate::char_tables::ASCII_BYTE_TYPES[byte as usize]
    }

    fn char_matches(&self, data: &[u8], pos: usize, c: u8) -> bool {
        if pos >= data.len() {
            return false;
        }
        data[pos] == c
    }

    fn min_bytes_per_char(&self) -> usize {
        1
    }

    fn byte_to_ascii(&self, data: &[u8], pos: usize) -> u8 {
        if pos >= data.len() {
            0
        } else {
            data[pos]
        }
    }
}

/// UTF-16 Little-Endian encoding implementation
pub struct Utf16LeEncoding;

impl Encoding for Utf16LeEncoding {
    fn byte_type(&self, data: &[u8], pos: usize) -> ByteType {
        if pos + 1 >= data.len() {
            return ByteType::NONXML;
        }
        // For UTF-16, we need both bytes to determine type
        // If first byte is 0x00, the second byte determines the type
        // Otherwise, it's a continuation/non-ASCII character
        if data[pos] == 0x00 {
            crate::char_tables::ASCII_BYTE_TYPES[data[pos + 1] as usize]
        } else {
            // Non-ASCII Unicode character
            ByteType::OTHER
        }
    }

    fn char_matches(&self, data: &[u8], pos: usize, c: u8) -> bool {
        if pos + 1 >= data.len() {
            return false;
        }
        // Little-endian: char c requires first byte 0x00, second byte c
        data[pos] == 0x00 && data[pos + 1] == c
    }

    fn min_bytes_per_char(&self) -> usize {
        2
    }

    fn byte_to_ascii(&self, data: &[u8], pos: usize) -> u8 {
        if pos + 1 >= data.len() {
            0
        } else if data[pos] == 0x00 {
            data[pos + 1]
        } else {
            0xff // Invalid ASCII
        }
    }
}

/// UTF-16 Big-Endian encoding implementation
pub struct Utf16BeEncoding;

impl Encoding for Utf16BeEncoding {
    fn byte_type(&self, data: &[u8], pos: usize) -> ByteType {
        if pos + 1 >= data.len() {
            return ByteType::NONXML;
        }
        // For big-endian UTF-16, first byte should be 0x00 for ASCII
        if data[pos] == 0x00 {
            crate::char_tables::ASCII_BYTE_TYPES[data[pos + 1] as usize]
        } else {
            // Non-ASCII Unicode character
            ByteType::OTHER
        }
    }

    fn char_matches(&self, data: &[u8], pos: usize, c: u8) -> bool {
        if pos + 1 >= data.len() {
            return false;
        }
        // Big-endian: char c requires first byte 0x00, second byte c
        data[pos] == 0x00 && data[pos + 1] == c
    }

    fn min_bytes_per_char(&self) -> usize {
        2
    }

    fn byte_to_ascii(&self, data: &[u8], pos: usize) -> u8 {
        if pos + 1 >= data.len() {
            0
        } else if data[pos] == 0x00 {
            data[pos + 1]
        } else {
            0xff // Invalid ASCII
        }
    }
}

/// Detect encoding from BOM (Byte Order Mark)
/// Returns the detected encoding and the number of bytes in the BOM (0 if no BOM)
pub fn detect_encoding_from_bom(data: &[u8]) -> (Box<dyn Encoding>, usize) {
    if data.len() >= 4 {
        // Check for 4-byte BOMs
        let first_four = &data[0..4];
        if first_four == b"\x00\x00\xFE\xFF" {
            return (Box::new(Utf16BeEncoding), 4);
        }
        if first_four == b"\xFF\xFE\x00\x00" {
            return (Box::new(Utf16LeEncoding), 4);
        }
    }

    if data.len() >= 3 {
        // Check for 3-byte BOM (UTF-8)
        let first_three = &data[0..3];
        if first_three == b"\xEF\xBB\xBF" {
            return (Box::new(Utf8Encoding), 3);
        }
    }

    if data.len() >= 2 {
        // Check for 2-byte BOMs
        let first_two = &data[0..2];
        if first_two == b"\xFE\xFF" {
            return (Box::new(Utf16BeEncoding), 2);
        }
        if first_two == b"\xFF\xFE" {
            return (Box::new(Utf16LeEncoding), 2);
        }
    }

    // No BOM detected, default to UTF-8
    (Box::new(Utf8Encoding), 0)
}

/// Encode a Unicode code point to UTF-8
/// Returns the number of bytes written (0-4)
/// Buffer must be at least XML_UTF8_ENCODE_MAX bytes
pub fn utf8_encode(char_num: u32, buf: &mut [u8]) -> usize {
    const MIN2: u32 = 0x80;
    const MIN3: u32 = 0x800;
    const MIN4: u32 = 0x10000;

    if char_num < MIN2 {
        buf[0] = (char_num as u8) | UTF8_CVAL1;
        1
    } else if char_num < MIN3 {
        buf[0] = ((char_num >> 6) as u8) | UTF8_CVAL2;
        buf[1] = ((char_num as u8) & 0x3f) | 0x80;
        2
    } else if char_num < MIN4 {
        buf[0] = ((char_num >> 12) as u8) | UTF8_CVAL3;
        buf[1] = (((char_num >> 6) as u8) & 0x3f) | 0x80;
        buf[2] = ((char_num as u8) & 0x3f) | 0x80;
        3
    } else if char_num < 0x110000 {
        buf[0] = ((char_num >> 18) as u8) | UTF8_CVAL4;
        buf[1] = (((char_num >> 12) as u8) & 0x3f) | 0x80;
        buf[2] = (((char_num >> 6) as u8) & 0x3f) | 0x80;
        buf[3] = ((char_num as u8) & 0x3f) | 0x80;
        4
    } else {
        0 // Invalid code point
    }
}

/// Encode a Unicode code point to UTF-16
/// Returns the number of 16-bit units written (1 or 2)
/// Buffer must be at least XML_UTF16_ENCODE_MAX units
pub fn utf16_encode(char_num: u32, buf: &mut [u16]) -> usize {
    if char_num < 0x10000 {
        buf[0] = char_num as u16;
        1
    } else if char_num < 0x110000 {
        let code = char_num - 0x10000;
        buf[0] = ((code >> 10) as u16) + 0xD800;
        buf[1] = ((code & 0x3FF) as u16) + 0xDC00;
        2
    } else {
        0 // Invalid code point
    }
}

/// Get the UTF-8 internal encoding (for XML declaration parsing)
pub fn get_utf8_internal_encoding() -> Box<dyn Encoding> {
    Box::new(Utf8Encoding)
}

/// Get the UTF-16 internal encoding based on system byte order
pub fn get_utf16_internal_encoding() -> Box<dyn Encoding> {
    #[cfg(target_endian = "little")]
    {
        Box::new(Utf16LeEncoding)
    }
    #[cfg(target_endian = "big")]
    {
        Box::new(Utf16BeEncoding)
    }
}

/// Encoding selector for initialization
pub fn select_encoding(name: Option<&str>) -> Option<Box<dyn Encoding>> {
    match name {
        None => Some(Box::new(Utf8Encoding)),
        Some(n) => {
            let lower = n.to_lowercase();
            match lower.as_str() {
                "utf-8" | "utf8" => Some(Box::new(Utf8Encoding)),
                "utf-16" | "utf16" => {
                    #[cfg(target_endian = "little")]
                    {
                        Some(Box::new(Utf16LeEncoding))
                    }
                    #[cfg(target_endian = "big")]
                    {
                        Some(Box::new(Utf16BeEncoding))
                    }
                }
                "utf-16le" | "utf16le" => Some(Box::new(Utf16LeEncoding)),
                "utf-16be" | "utf16be" => Some(Box::new(Utf16BeEncoding)),
                "iso-8859-1" | "iso8859-1" | "latin1" => Some(Box::new(Latin1Encoding)),
                "ascii" | "us-ascii" => Some(Box::new(AsciiEncoding)),
                _ => None,
            }
        }
    }
}

/// Check if a character is whitespace (space, tab, CR, LF)
fn is_space(c: u8) -> bool {
    matches!(c, 0x20 | 0x09 | 0x0D | 0x0A)
}

/// Convert a position in the buffer to its ASCII value using an encoding
/// Returns -1 if the sequence is incomplete or not ASCII
fn to_ascii(enc: &dyn Encoding, data: &[u8], pos: usize, end: usize) -> i32 {
    if pos >= end {
        return -1;
    }

    // For single-byte encodings, just return the byte
    if enc.min_bytes_per_char() == 1 {
        let byte = enc.byte_to_ascii(data, pos);
        if byte < 0x80 {
            return byte as i32;
        } else {
            return -1;
        }
    }

    // For multi-byte encodings (UTF-16), need at least 2 bytes
    if pos + enc.min_bytes_per_char() > end {
        return -1;
    }

    let byte = enc.byte_to_ascii(data, pos);
    if byte != 0xff {
        byte as i32
    } else {
        -1
    }
}

/// Parse a pseudo-attribute from XML declaration
/// Returns (name_start, name_end, value_start, next_position, success)
fn parse_pseudo_attribute(
    enc: &dyn Encoding,
    data: &[u8],
    mut pos: usize,
    end: usize,
) -> (usize, usize, usize, usize, bool) {
    let minbpc = enc.min_bytes_per_char();

    // Check if we're at the end or if there's no name
    if pos >= end {
        return (0, 0, 0, pos, true);
    }

    // Skip whitespace
    while pos < end {
        let c = to_ascii(enc, data, pos, end);
        if c == -1 || !is_space(c as u8) {
            break;
        }
        pos += minbpc;
    }

    if pos >= end {
        return (0, 0, 0, pos, true);
    }

    // Get name start
    let name_start = pos;

    // Scan for name end (whitespace or '=')
    let name_end;
    loop {
        let c = to_ascii(enc, data, pos, end);
        if c == -1 {
            return (0, 0, 0, pos, false);
        }

        if is_space(c as u8) {
            name_end = pos;
            break;
        }

        if c == b'=' as i32 {
            name_end = pos;
            break;
        }

        pos += minbpc;
        if pos > end {
            return (0, 0, 0, pos, false);
        }
    }

    // Skip whitespace after name
    pos = name_end;
    while pos < end {
        let c = to_ascii(enc, data, pos, end);
        if c == -1 {
            return (0, 0, 0, pos, false);
        }
        if !is_space(c as u8) {
            break;
        }
        pos += minbpc;
    }

    // Expect '='
    if pos >= end {
        return (0, 0, 0, pos, false);
    }

    let c = to_ascii(enc, data, pos, end);
    if c != b'=' as i32 {
        return (0, 0, 0, pos, false);
    }

    pos += minbpc;

    // Skip whitespace after '='
    while pos < end {
        let c = to_ascii(enc, data, pos, end);
        if c == -1 {
            return (0, 0, 0, pos, false);
        }
        if !is_space(c as u8) {
            break;
        }
        pos += minbpc;
    }

    // Check for quote
    if pos >= end {
        return (0, 0, 0, pos, false);
    }

    let c = to_ascii(enc, data, pos, end);
    if c != b'"' as i32 && c != b'\'' as i32 {
        return (0, 0, 0, pos, false);
    }

    let quote = c as u8;
    pos += minbpc;
    let value_start = pos;

    // Scan for closing quote
    while pos < end {
        let c = to_ascii(enc, data, pos, end);
        if c == -1 {
            return (0, 0, 0, pos, false);
        }

        if c == quote as i32 {
            pos += minbpc;
            return (name_start, name_end, value_start, pos, true);
        }

        // Validate attribute value characters
        if !((c as u8).is_ascii_lowercase()
            || (c as u8).is_ascii_uppercase()
            || (c as u8).is_ascii_digit()
            || c == b'.' as i32
            || c == b'-' as i32
            || c == b'_' as i32)
        {
            return (0, 0, 0, pos, false);
        }

        pos += minbpc;
    }

    (0, 0, 0, pos, false)
}

/// XML Declaration Result
#[derive(Debug, Clone)]
pub struct XmlDeclInfo {
    pub version_start: usize,
    pub version_end: usize,
    pub encoding_start: usize,
    pub encoding_end: usize,
    pub standalone: Option<bool>,
}

/// Parse an XML declaration (<?xml ...?>)
/// Returns ParseXmlDeclResult with parsed information
pub fn parse_xml_decl(data: &[u8], is_text_decl: bool) -> Result<XmlDeclInfo, usize> {
    let enc = &Utf8Encoding as &dyn Encoding;
    let minbpc = enc.min_bytes_per_char();

    let mut pos = 0;
    let end = data.len();

    // Skip "<?xml"
    if end < 5 * minbpc || !data.starts_with(b"<?xml") {
        return Err(pos);
    }

    pos = 5 * minbpc;

    // Skip trailing space/?>
    if end < pos + 2 * minbpc {
        return Err(pos);
    }

    let mut version_start = 0;
    let mut version_end = 0;
    let mut encoding_start = 0;
    let mut encoding_end = 0;
    let mut standalone = None;

    // Parse version attribute
    let (name_start, name_end, val_start, next_pos, success) =
        parse_pseudo_attribute(enc, data, pos, end - 2 * minbpc);

    if !success {
        return Err(if name_start > 0 { name_start } else { next_pos });
    }

    pos = next_pos;

    // Check first attribute is "version"
    if name_start == 0 {
        if !is_text_decl {
            return Err(next_pos);
        }
    } else {
        // Check if name matches "version"
        let name_matches_version = name_end > name_start
            && name_end - name_start == 7
            && &data[name_start..name_end] == b"version";

        if !name_matches_version {
            if !is_text_decl {
                return Err(name_start);
            }
        } else {
            version_start = val_start;
            version_end = next_pos - minbpc; // exclude closing quote

            // Try to parse second attribute
            let (name_start2, name_end2, val_start2, next_pos2, success2) =
                parse_pseudo_attribute(enc, data, pos, end - 2 * minbpc);

            if !success2 {
                return Err(next_pos2);
            }

            if name_start2 == 0 {
                if is_text_decl {
                    return Err(next_pos2);
                }
                return Ok(XmlDeclInfo {
                    version_start,
                    version_end,
                    encoding_start: 0,
                    encoding_end: 0,
                    standalone,
                });
            }

            pos = next_pos2;

            // Check if second attribute is "encoding"
            let name_matches_encoding = name_end2 > name_start2
                && name_end2 - name_start2 == 8
                && &data[name_start2..name_end2] == b"encoding";

            if name_matches_encoding {
                encoding_start = val_start2;
                encoding_end = next_pos2 - minbpc; // exclude closing quote

                // Try to parse third attribute
                let (name_start3, name_end3, val_start3, next_pos3, success3) =
                    parse_pseudo_attribute(enc, data, pos, end - 2 * minbpc);

                if !success3 {
                    return Err(next_pos3);
                }

                if name_start3 == 0 {
                    return Ok(XmlDeclInfo {
                        version_start,
                        version_end,
                        encoding_start,
                        encoding_end,
                        standalone,
                    });
                }

                // Check if third attribute is "standalone"
                let name_matches_standalone = name_end3 > name_start3
                    && name_end3 - name_start3 == 10
                    && &data[name_start3..name_end3] == b"standalone";

                if !name_matches_standalone || is_text_decl {
                    return Err(name_start3);
                }

                // Check standalone value (next_pos3 is past closing quote, so subtract minbpc)
                let val_end3 = next_pos3 - minbpc;
                let val_matches_yes = val_start3 < val_end3
                    && val_end3 - val_start3 == 3
                    && &data[val_start3..val_end3] == b"yes";
                let val_matches_no = val_start3 < val_end3
                    && val_end3 - val_start3 == 2
                    && &data[val_start3..val_end3] == b"no";

                if val_matches_yes {
                    standalone = Some(true);
                } else if val_matches_no {
                    standalone = Some(false);
                } else {
                    return Err(val_start3);
                }

                return Ok(XmlDeclInfo {
                    version_start,
                    version_end,
                    encoding_start,
                    encoding_end,
                    standalone,
                });
            }
        }
    }

    Ok(XmlDeclInfo {
        version_start,
        version_end,
        encoding_start,
        encoding_end,
        standalone,
    })
}

/// Convert UTF-8 to UTF-16
/// Returns the number of UTF-16 units written
pub fn utf8_to_utf16(input: &[u8], output: &mut [u16]) -> Result<usize, usize> {
    let mut in_pos = 0;
    let mut out_pos = 0;

    while in_pos < input.len() && out_pos < output.len() {
        let byte = input[in_pos];

        if byte < 0x80 {
            // Single-byte ASCII
            output[out_pos] = byte as u16;
            out_pos += 1;
            in_pos += 1;
        } else if byte < 0xC0 {
            // Continuation byte at start - invalid
            return Err(in_pos);
        } else if byte < 0xE0 {
            // 2-byte sequence
            if in_pos + 1 >= input.len() {
                return Err(in_pos);
            }
            let byte1 = input[in_pos + 1];
            if (byte1 & 0xC0) != 0x80 {
                return Err(in_pos);
            }
            let code = (((byte & 0x1f) as u16) << 6) | ((byte1 & 0x3f) as u16);
            output[out_pos] = code;
            out_pos += 1;
            in_pos += 2;
        } else if byte < 0xF0 {
            // 3-byte sequence
            if in_pos + 2 >= input.len() {
                return Err(in_pos);
            }
            let byte1 = input[in_pos + 1];
            let byte2 = input[in_pos + 2];
            if (byte1 & 0xC0) != 0x80 || (byte2 & 0xC0) != 0x80 {
                return Err(in_pos);
            }
            let code = (((byte & 0x0f) as u16) << 12)
                | (((byte1 & 0x3f) as u16) << 6)
                | ((byte2 & 0x3f) as u16);
            output[out_pos] = code;
            out_pos += 1;
            in_pos += 3;
        } else if byte < 0xF8 {
            // 4-byte sequence - need surrogate pair
            if in_pos + 3 >= input.len() {
                return Err(in_pos);
            }
            let byte1 = input[in_pos + 1];
            let byte2 = input[in_pos + 2];
            let byte3 = input[in_pos + 3];
            if (byte1 & 0xC0) != 0x80 || (byte2 & 0xC0) != 0x80 || (byte3 & 0xC0) != 0x80 {
                return Err(in_pos);
            }

            if out_pos + 1 >= output.len() {
                return Err(in_pos);
            }

            let code_point = (((byte & 0x07) as u32) << 18)
                | (((byte1 & 0x3f) as u32) << 12)
                | (((byte2 & 0x3f) as u32) << 6)
                | ((byte3 & 0x3f) as u32);

            if code_point >= 0x110000 {
                return Err(in_pos);
            }

            let adjusted = code_point - 0x10000;
            output[out_pos] = ((adjusted >> 10) as u16) + 0xD800;
            output[out_pos + 1] = ((adjusted & 0x3FF) as u16) + 0xDC00;
            out_pos += 2;
            in_pos += 4;
        } else {
            // Invalid UTF-8
            return Err(in_pos);
        }
    }

    if in_pos < input.len() {
        Err(in_pos) // Incomplete sequence
    } else {
        Ok(out_pos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utf8_encode_single_byte() {
        let mut buf = [0u8; 4];
        let len = utf8_encode(0x41, &mut buf);
        assert_eq!(len, 1);
        assert_eq!(buf[0], 0x41);
    }

    #[test]
    fn test_utf8_encode_two_bytes() {
        let mut buf = [0u8; 4];
        let len = utf8_encode(0x00C2, &mut buf);
        assert_eq!(len, 2);
        assert_eq!(buf[0], 0xC3);
        assert_eq!(buf[1], 0x82);
    }

    #[test]
    fn test_utf16_encode_single_unit() {
        let mut buf = [0u16; 2];
        let len = utf16_encode(0x0041, &mut buf);
        assert_eq!(len, 1);
        assert_eq!(buf[0], 0x0041);
    }

    #[test]
    fn test_utf16_encode_surrogate_pair() {
        let mut buf = [0u16; 2];
        let len = utf16_encode(0x10000, &mut buf);
        assert_eq!(len, 2);
        assert_eq!(buf[0], 0xD800);
        assert_eq!(buf[1], 0xDC00);
    }

    #[test]
    fn test_detect_utf8_bom() {
        let data = b"\xEF\xBB\xBFHello";
        let (_enc, bom_len) = detect_encoding_from_bom(data);
        assert_eq!(bom_len, 3);
    }

    #[test]
    fn test_utf8_encoding() {
        let enc = Utf8Encoding;
        let data = b"Hello";
        assert_eq!(enc.byte_to_ascii(data, 0), b'H');
        assert!(enc.char_matches(data, 0, b'H'));
    }

    #[test]
    fn test_latin1_encoding() {
        let enc = Latin1Encoding;
        let data = b"Test";
        assert_eq!(enc.min_bytes_per_char(), 1);
        assert!(enc.char_matches(data, 0, b'T'));
    }

    #[test]
    fn test_utf8_to_utf16() {
        let input = b"A";
        let mut output = [0u16; 10];
        let len = utf8_to_utf16(input, &mut output).unwrap();
        assert_eq!(len, 1);
        assert_eq!(output[0], b'A' as u16);
    }
}
