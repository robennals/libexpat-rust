// AI-generated port of xmltok_impl.c

use crate::char_tables::ByteType;

/// Token type enumeration matching XML_TOK_* constants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum XmlTok {
    // Error/incomplete tokens (negative values)
    TrailingRsqb = -5,
    None = -4,
    TrailingCr = -3,
    PartialChar = -2,
    Partial = -1,

    // Positive tokens
    Invalid = 0,
    StartTagWithAtts = 1,
    StartTagNoAtts = 2,
    EmptyElementWithAtts = 3,
    EmptyElementNoAtts = 4,
    EndTag = 5,
    DataChars = 6,
    DataNewline = 7,
    CdataSectOpen = 8,
    EntityRef = 9,
    CharRef = 10,

    // Prolog/both tokens
    Pi = 11,
    XmlDecl = 12,
    Comment = 13,
    Bom = 14,

    // Prolog-only tokens
    PrologS = 15,
    DeclOpen = 16,
    DeclClose = 17,
    Name = 18,
    Nmtoken = 19,
    PoundName = 20,
    Or = 21,
    Percent = 22,
    OpenParen = 23,
    CloseParen = 24,
    OpenBracket = 25,
    CloseBracket = 26,
    Literal = 27,
    ParamEntityRef = 28,
    InstanceStart = 29,

    // Element type declaration tokens
    NameQuestion = 30,
    NameAsterisk = 31,
    NamePlus = 32,
    CondSectOpen = 33,
    CondSectClose = 34,
    CloseParenQuestion = 35,
    CloseParenAsterisk = 36,
    CloseParenPlus = 37,
    Comma = 38,

    // Attribute value token
    AttributeValueS = 39,

    // CDATA section token
    CdataSectClose = 40,

    // Prefixed name
    PrefixedName = 41,

    // DTD only
    IgnoreSect = 42,
}

/// Result of scanning a token
pub struct TokenResult {
    pub token: XmlTok,
    pub next_pos: usize,
}

/// Encoding trait for handling different character encodings
pub trait Encoding {
    /// Get the byte type at the given position
    fn byte_type(&self, data: &[u8], pos: usize) -> ByteType;

    /// Check if a byte sequence matches an ASCII character
    fn char_matches(&self, data: &[u8], pos: usize, c: u8) -> bool;

    /// Get minimum bytes per character (1 for UTF-8, 2 for UTF-16)
    fn min_bytes_per_char(&self) -> usize;

    /// Get ASCII value of a byte sequence
    fn byte_to_ascii(&self, data: &[u8], pos: usize) -> u8;

    /// Check if position has at least count characters
    fn has_chars(&self, _data: &[u8], pos: usize, end: usize, count: usize) -> bool {
        pos <= end && (end - pos) >= (count * self.min_bytes_per_char())
    }

    /// Check if position has at least 1 character
    fn has_char(&self, data: &[u8], pos: usize, end: usize) -> bool {
        self.has_chars(data, pos, end, 1)
    }
}

// ASCII character constants
const ASCII_MINUS: u8 = 0x2D;
const ASCII_X: u8 = 0x78;
const ASCII_M: u8 = 0x6D;
const ASCII_L: u8 = 0x6C;
const ASCII_GT: u8 = 0x3E;
const ASCII_RSQB: u8 = 0x5D;
const ASCII_LSQB: u8 = 0x5B;
const ASCII_EXCL: u8 = 0x21;
const ASCII_SEMI: u8 = 0x3B;
const ASCII_TAB: u8 = 0x09;
const ASCII_QUOT: u8 = 0x22;
const ASCII_APOS: u8 = 0x27;

/// Scan a comment token starting after "<!-"
pub fn scan_comment<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut pos: usize,
    end: usize,
) -> Result<TokenResult, usize> {
    if !enc.has_char(data, pos, end) {
        return Ok(TokenResult {
            token: XmlTok::Partial,
            next_pos: pos,
        });
    }

    if !enc.char_matches(data, pos, ASCII_MINUS) {
        return Err(pos);
    }

    pos += enc.min_bytes_per_char();

    while enc.has_char(data, pos, end) {
        match enc.byte_type(data, pos) {
            ByteType::NONXML
            | ByteType::MALFORM
            | ByteType::TRAIL => {
                return Err(pos);
            }
            ByteType::MINUS => {
                pos += enc.min_bytes_per_char();
                if !enc.has_char(data, pos, end) {
                    return Ok(TokenResult {
                        token: XmlTok::Partial,
                        next_pos: pos,
                    });
                }
                if enc.char_matches(data, pos, ASCII_MINUS) {
                    pos += enc.min_bytes_per_char();
                    if !enc.has_char(data, pos, end) {
                        return Ok(TokenResult {
                            token: XmlTok::Partial,
                            next_pos: pos,
                        });
                    }
                    if !enc.char_matches(data, pos, ASCII_GT) {
                        return Err(pos);
                    }
                    return Ok(TokenResult {
                        token: XmlTok::Comment,
                        next_pos: pos + enc.min_bytes_per_char(),
                    });
                }
            }
            _ => {
                pos += enc.min_bytes_per_char();
            }
        }
    }

    Ok(TokenResult {
        token: XmlTok::Partial,
        next_pos: pos,
    })
}

/// Scan a declaration starting after "<!"
pub fn scan_decl<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut pos: usize,
    end: usize,
) -> Result<TokenResult, usize> {
    if !enc.has_char(data, pos, end) {
        return Ok(TokenResult {
            token: XmlTok::Partial,
            next_pos: pos,
        });
    }

    match enc.byte_type(data, pos) {
        ByteType::MINUS => {
            return scan_comment(enc, data, pos + enc.min_bytes_per_char(), end);
        }
        ByteType::LSQB => {
            return Ok(TokenResult {
                token: XmlTok::CondSectOpen,
                next_pos: pos + enc.min_bytes_per_char(),
            });
        }
        ByteType::NMSTRT | ByteType::HEX => {
            pos += enc.min_bytes_per_char();
        }
        _ => {
            return Err(pos);
        }
    }

    while enc.has_char(data, pos, end) {
        match enc.byte_type(data, pos) {
            ByteType::PERCNT => {
                if !enc.has_chars(data, pos, end, 2) {
                    return Ok(TokenResult {
                        token: XmlTok::Partial,
                        next_pos: pos,
                    });
                }
                match enc.byte_type(data, pos + enc.min_bytes_per_char()) {
                    ByteType::S | ByteType::CR | ByteType::LF | ByteType::PERCNT => {
                        return Err(pos);
                    }
                    _ => {}
                }
                pos += enc.min_bytes_per_char();
            }
            ByteType::S | ByteType::CR | ByteType::LF => {
                return Ok(TokenResult {
                    token: XmlTok::DeclOpen,
                    next_pos: pos,
                });
            }
            ByteType::NMSTRT | ByteType::HEX => {
                pos += enc.min_bytes_per_char();
            }
            _ => {
                return Err(pos);
            }
        }
    }

    Ok(TokenResult {
        token: XmlTok::Partial,
        next_pos: pos,
    })
}

/// Check PI target name to identify XML declarations
fn check_pi_target<E: Encoding>(enc: &E, data: &[u8], pos: usize, end: usize) -> (bool, XmlTok) {
    let mut upper = false;
    let minbpc = enc.min_bytes_per_char();

    if end - pos != minbpc * 3 {
        return (true, XmlTok::Pi);
    }

    match enc.byte_to_ascii(data, pos) {
        ASCII_X => {}
        b'X' => {
            upper = true;
        }
        _ => {
            return (true, XmlTok::Pi);
        }
    }

    let pos = pos + minbpc;
    match enc.byte_to_ascii(data, pos) {
        ASCII_M => {}
        b'M' => {
            upper = true;
        }
        _ => {
            return (true, XmlTok::Pi);
        }
    }

    let pos = pos + minbpc;
    match enc.byte_to_ascii(data, pos) {
        ASCII_L => {}
        b'L' => {
            upper = true;
        }
        _ => {
            return (true, XmlTok::Pi);
        }
    }

    if upper {
        (true, XmlTok::Pi)
    } else {
        (true, XmlTok::XmlDecl)
    }
}

/// Check if byte type is valid for name continuation
fn is_name_char(bt: ByteType) -> bool {
    matches!(
        bt,
        ByteType::NMSTRT
            | ByteType::HEX
            | ByteType::DIGIT
            | ByteType::NAME
            | ByteType::MINUS
            | ByteType::COLON
    )
}

/// Check if byte type is valid for name start
fn is_nmstrt_char(bt: ByteType) -> bool {
    matches!(bt, ByteType::NMSTRT | ByteType::HEX)
}

/// Scan a PI starting after "<?"
pub fn scan_pi<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut pos: usize,
    end: usize,
) -> Result<TokenResult, usize> {
    if !enc.has_char(data, pos, end) {
        return Ok(TokenResult {
            token: XmlTok::Partial,
            next_pos: pos,
        });
    }

    if !is_nmstrt_char(enc.byte_type(data, pos)) {
        return Err(pos);
    }

    let target = pos;
    pos += enc.min_bytes_per_char();

    while enc.has_char(data, pos, end) {
        match enc.byte_type(data, pos) {
            ByteType::S | ByteType::CR | ByteType::LF => {
                let (valid, tok) = check_pi_target(enc, data, target, pos);
                if !valid {
                    return Err(pos);
                }
                pos += enc.min_bytes_per_char();

                while enc.has_char(data, pos, end) {
                    match enc.byte_type(data, pos) {
                        ByteType::LEAD2 => {
                            if end - pos < 2 { return Ok(TokenResult { token: XmlTok::Partial, next_pos: pos }); }
                            pos += 2;
                        }
                        ByteType::LEAD3 => {
                            if end - pos < 3 { return Ok(TokenResult { token: XmlTok::Partial, next_pos: pos }); }
                            pos += 3;
                        }
                        ByteType::LEAD4 => {
                            if end - pos < 4 { return Ok(TokenResult { token: XmlTok::Partial, next_pos: pos }); }
                            pos += 4;
                        }
                        ByteType::NONXML | ByteType::MALFORM | ByteType::TRAIL => {
                            return Err(pos);
                        }
                        ByteType::QUEST => {
                            pos += enc.min_bytes_per_char();
                            if !enc.has_char(data, pos, end) {
                                return Ok(TokenResult {
                                    token: XmlTok::Partial,
                                    next_pos: pos,
                                });
                            }
                            if enc.char_matches(data, pos, ASCII_GT) {
                                return Ok(TokenResult {
                                    token: tok,
                                    next_pos: pos + enc.min_bytes_per_char(),
                                });
                            }
                        }
                        _ => {
                            pos += enc.min_bytes_per_char();
                        }
                    }
                }
                return Ok(TokenResult {
                    token: XmlTok::Partial,
                    next_pos: pos,
                });
            }
            ByteType::QUEST => {
                let (valid, tok) = check_pi_target(enc, data, target, pos);
                if !valid {
                    return Err(pos);
                }
                pos += enc.min_bytes_per_char();
                if !enc.has_char(data, pos, end) {
                    return Ok(TokenResult {
                        token: XmlTok::Partial,
                        next_pos: pos,
                    });
                }
                if enc.char_matches(data, pos, ASCII_GT) {
                    return Ok(TokenResult {
                        token: tok,
                        next_pos: pos + enc.min_bytes_per_char(),
                    });
                }
                return Err(pos);
            }
            _ if is_name_char(enc.byte_type(data, pos)) => {
                pos += enc.min_bytes_per_char();
            }
            _ => {
                return Err(pos);
            }
        }
    }

    Ok(TokenResult {
        token: XmlTok::Partial,
        next_pos: pos,
    })
}

/// Scan CDATA section start
pub fn scan_cdata_section<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut pos: usize,
    end: usize,
) -> Result<TokenResult, usize> {
    const CDATA_LSQB: &[u8] = b"CDATA[";

    if !enc.has_chars(data, pos, end, 6) {
        return Ok(TokenResult {
            token: XmlTok::Partial,
            next_pos: pos,
        });
    }

    for &expected in CDATA_LSQB {
        if !enc.char_matches(data, pos, expected) {
            return Err(pos);
        }
        pos += enc.min_bytes_per_char();
    }

    Ok(TokenResult {
        token: XmlTok::CdataSectOpen,
        next_pos: pos,
    })
}

/// Scan content within CDATA section
pub fn cdata_section_tok<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut pos: usize,
    mut end: usize,
) -> Result<TokenResult, usize> {
    let minbpc = enc.min_bytes_per_char();

    if pos >= end {
        return Ok(TokenResult {
            token: XmlTok::None,
            next_pos: pos,
        });
    }

    if minbpc > 1 {
        let n = end - pos;
        if n & (minbpc - 1) != 0 {
            let n = n & !(minbpc - 1);
            if n == 0 {
                return Ok(TokenResult {
                    token: XmlTok::Partial,
                    next_pos: pos,
                });
            }
            end = pos + n;
        }
    }

    match enc.byte_type(data, pos) {
        ByteType::RSQB => {
            pos += minbpc;
            if !enc.has_char(data, pos, end) {
                return Ok(TokenResult {
                    token: XmlTok::TrailingRsqb,
                    next_pos: pos,
                });
            }
            if !enc.char_matches(data, pos, ASCII_RSQB) {
                // Single ] — keep pos past it and fall through to continuation loop
            } else {
                pos += minbpc;
                if !enc.has_char(data, pos, end) {
                    return Ok(TokenResult {
                        token: XmlTok::TrailingRsqb,
                        next_pos: pos,
                    });
                }
                if !enc.char_matches(data, pos, ASCII_GT) {
                    // ]] but no > — back up to second ], fall through
                    pos -= minbpc;
                } else {
                    return Ok(TokenResult {
                        token: XmlTok::CdataSectClose,
                        next_pos: pos + minbpc,
                    });
                }
            }
        }
        ByteType::CR => {
            pos += minbpc;
            if !enc.has_char(data, pos, end) {
                return Ok(TokenResult {
                    token: XmlTok::DataNewline,
                    next_pos: pos,
                });
            }
            if enc.byte_type(data, pos) == ByteType::LF {
                pos += minbpc;
            }
            return Ok(TokenResult {
                token: XmlTok::DataNewline,
                next_pos: pos,
            });
        }
        ByteType::LF => {
            return Ok(TokenResult {
                token: XmlTok::DataNewline,
                next_pos: pos + minbpc,
            });
        }
        ByteType::LEAD2 => {
            if end - pos < 2 { return Ok(TokenResult { token: XmlTok::PartialChar, next_pos: pos }); }
            pos += 2;
        }
        ByteType::LEAD3 => {
            if end - pos < 3 { return Ok(TokenResult { token: XmlTok::PartialChar, next_pos: pos }); }
            pos += 3;
        }
        ByteType::LEAD4 => {
            if end - pos < 4 { return Ok(TokenResult { token: XmlTok::PartialChar, next_pos: pos }); }
            pos += 4;
        }
        ByteType::NONXML | ByteType::MALFORM | ByteType::TRAIL => {
            return Err(pos);
        }
        _ => {
            pos += minbpc;
        }
    }

    while enc.has_char(data, pos, end) {
        match enc.byte_type(data, pos) {
            ByteType::LEAD2 | ByteType::LEAD3 | ByteType::LEAD4 => {
                let n = match enc.byte_type(data, pos) {
                    ByteType::LEAD2 => 2,
                    ByteType::LEAD3 => 3,
                    ByteType::LEAD4 => 4,
                    _ => unreachable!(),
                };
                if end - pos < n {
                    return Ok(TokenResult {
                        token: XmlTok::DataChars,
                        next_pos: pos,
                    });
                }
                pos += n;
            }
            ByteType::NONXML
            | ByteType::MALFORM
            | ByteType::TRAIL
            | ByteType::CR
            | ByteType::LF
            | ByteType::RSQB => {
                return Ok(TokenResult {
                    token: XmlTok::DataChars,
                    next_pos: pos,
                });
            }
            _ => {
                pos += minbpc;
            }
        }
    }

    Ok(TokenResult {
        token: XmlTok::DataChars,
        next_pos: pos,
    })
}

/// Scan an end tag starting after "</"
pub fn scan_end_tag<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut pos: usize,
    end: usize,
) -> Result<TokenResult, usize> {
    if !enc.has_char(data, pos, end) {
        return Ok(TokenResult {
            token: XmlTok::Partial,
            next_pos: pos,
        });
    }

    // Name start character (including multi-byte UTF-8)
    match enc.byte_type(data, pos) {
        ByteType::LEAD2 => {
            if end - pos < 2 { return Ok(TokenResult { token: XmlTok::Partial, next_pos: pos }); }
            pos += 2;
        }
        ByteType::LEAD3 => {
            if end - pos < 3 { return Ok(TokenResult { token: XmlTok::Partial, next_pos: pos }); }
            pos += 3;
        }
        ByteType::LEAD4 => {
            if end - pos < 4 { return Ok(TokenResult { token: XmlTok::Partial, next_pos: pos }); }
            pos += 4;
        }
        _ if is_nmstrt_char(enc.byte_type(data, pos)) => {
            pos += enc.min_bytes_per_char();
        }
        _ => {
            return Err(pos);
        }
    }

    while enc.has_char(data, pos, end) {
        match enc.byte_type(data, pos) {
            ByteType::S | ByteType::CR | ByteType::LF => {
                pos += enc.min_bytes_per_char();
                while enc.has_char(data, pos, end) {
                    match enc.byte_type(data, pos) {
                        ByteType::S | ByteType::CR | ByteType::LF => {
                            pos += enc.min_bytes_per_char();
                        }
                        ByteType::GT => {
                            return Ok(TokenResult {
                                token: XmlTok::EndTag,
                                next_pos: pos + enc.min_bytes_per_char(),
                            });
                        }
                        _ => {
                            return Err(pos);
                        }
                    }
                }
                return Ok(TokenResult {
                    token: XmlTok::Partial,
                    next_pos: pos,
                });
            }
            ByteType::GT => {
                return Ok(TokenResult {
                    token: XmlTok::EndTag,
                    next_pos: pos + enc.min_bytes_per_char(),
                });
            }
            ByteType::LEAD2 => {
                if end - pos < 2 { return Ok(TokenResult { token: XmlTok::Partial, next_pos: pos }); }
                pos += 2;
            }
            ByteType::LEAD3 => {
                if end - pos < 3 { return Ok(TokenResult { token: XmlTok::Partial, next_pos: pos }); }
                pos += 3;
            }
            ByteType::LEAD4 => {
                if end - pos < 4 { return Ok(TokenResult { token: XmlTok::Partial, next_pos: pos }); }
                pos += 4;
            }
            ByteType::COLON => {
                pos += enc.min_bytes_per_char();
            }
            _ if is_name_char(enc.byte_type(data, pos)) => {
                pos += enc.min_bytes_per_char();
            }
            _ => {
                return Err(pos);
            }
        }
    }

    Ok(TokenResult {
        token: XmlTok::Partial,
        next_pos: pos,
    })
}

/// Scan hex character reference starting after "&#X"
pub fn scan_hex_char_ref<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut pos: usize,
    end: usize,
) -> Result<TokenResult, usize> {
    if !enc.has_char(data, pos, end) {
        return Ok(TokenResult {
            token: XmlTok::Partial,
            next_pos: pos,
        });
    }

    match enc.byte_type(data, pos) {
        ByteType::DIGIT | ByteType::HEX => {}
        _ => {
            return Err(pos);
        }
    }

    pos += enc.min_bytes_per_char();

    while enc.has_char(data, pos, end) {
        match enc.byte_type(data, pos) {
            ByteType::DIGIT | ByteType::HEX => {
                pos += enc.min_bytes_per_char();
            }
            ByteType::SEMI => {
                return Ok(TokenResult {
                    token: XmlTok::CharRef,
                    next_pos: pos + enc.min_bytes_per_char(),
                });
            }
            _ => {
                return Err(pos);
            }
        }
    }

    Ok(TokenResult {
        token: XmlTok::Partial,
        next_pos: pos,
    })
}

/// Scan character reference starting after "&#"
pub fn scan_char_ref<E: Encoding>(
    enc: &E,
    data: &[u8],
    pos: usize,
    end: usize,
) -> Result<TokenResult, usize> {
    if !enc.has_char(data, pos, end) {
        return Ok(TokenResult {
            token: XmlTok::Partial,
            next_pos: pos,
        });
    }

    if enc.char_matches(data, pos, ASCII_X) {
        return scan_hex_char_ref(enc, data, pos + enc.min_bytes_per_char(), end);
    }

    match enc.byte_type(data, pos) {
        ByteType::DIGIT => {}
        _ => {
            return Err(pos);
        }
    }

    let mut pos = pos + enc.min_bytes_per_char();

    while enc.has_char(data, pos, end) {
        match enc.byte_type(data, pos) {
            ByteType::DIGIT => {
                pos += enc.min_bytes_per_char();
            }
            ByteType::SEMI => {
                return Ok(TokenResult {
                    token: XmlTok::CharRef,
                    next_pos: pos + enc.min_bytes_per_char(),
                });
            }
            _ => {
                return Err(pos);
            }
        }
    }

    Ok(TokenResult {
        token: XmlTok::Partial,
        next_pos: pos,
    })
}

/// Scan entity reference starting after "&"
pub fn scan_ref<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut pos: usize,
    end: usize,
) -> Result<TokenResult, usize> {
    if !enc.has_char(data, pos, end) {
        return Ok(TokenResult {
            token: XmlTok::Partial,
            next_pos: pos,
        });
    }

    match enc.byte_type(data, pos) {
        ByteType::NUM => {
            return scan_char_ref(enc, data, pos + enc.min_bytes_per_char(), end);
        }
        _ if is_nmstrt_char(enc.byte_type(data, pos)) => {
            pos += enc.min_bytes_per_char();
        }
        _ => {
            return Err(pos);
        }
    }

    while enc.has_char(data, pos, end) {
        match enc.byte_type(data, pos) {
            ByteType::SEMI => {
                return Ok(TokenResult {
                    token: XmlTok::EntityRef,
                    next_pos: pos + enc.min_bytes_per_char(),
                });
            }
            _ if is_name_char(enc.byte_type(data, pos)) => {
                pos += enc.min_bytes_per_char();
            }
            _ => {
                return Err(pos);
            }
        }
    }

    Ok(TokenResult {
        token: XmlTok::Partial,
        next_pos: pos,
    })
}

/// Scan attributes starting after first character of attribute name
/// Scan attributes in a start tag — 1:1 port of C scanAtts()
/// pos points to the first attribute name start character
pub fn scan_atts<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut pos: usize,
    end: usize,
) -> Result<TokenResult, usize> {
    let minbpc = enc.min_bytes_per_char();

    // Outer loop: scan attribute name characters
    while enc.has_char(data, pos, end) {
        match enc.byte_type(data, pos) {
            // Name characters — continue scanning attr name
            _ if is_name_char(enc.byte_type(data, pos))
                && !matches!(enc.byte_type(data, pos), ByteType::S | ByteType::CR | ByteType::LF
                    | ByteType::GT | ByteType::SOL | ByteType::EQUALS | ByteType::QUOT | ByteType::APOS) =>
            {
                pos += minbpc;
            }

            // Whitespace after attr name — find '='
            ByteType::S | ByteType::CR | ByteType::LF => {
                loop {
                    pos += minbpc;
                    if !enc.has_char(data, pos, end) {
                        return Ok(TokenResult { token: XmlTok::Partial, next_pos: pos });
                    }
                    let t = enc.byte_type(data, pos);
                    if t == ByteType::EQUALS {
                        break;
                    }
                    match t {
                        ByteType::S | ByteType::LF | ByteType::CR => {}
                        _ => return Err(pos),
                    }
                }
                // Fall through to EQUALS handling below
                pos = scan_attr_value(enc, data, pos, end)?;
            }

            // '=' after attr name — parse value
            ByteType::EQUALS => {
                pos = scan_attr_value(enc, data, pos, end)?;
            }

            ByteType::GT => {
                return Ok(TokenResult {
                    token: XmlTok::StartTagWithAtts,
                    next_pos: pos + minbpc,
                });
            }
            ByteType::SOL => {
                pos += minbpc;
                if !enc.has_char(data, pos, end) {
                    return Ok(TokenResult { token: XmlTok::Partial, next_pos: pos });
                }
                if !enc.char_matches(data, pos, ASCII_GT) {
                    return Err(pos);
                }
                return Ok(TokenResult {
                    token: XmlTok::EmptyElementWithAtts,
                    next_pos: pos + minbpc,
                });
            }
            _ => {
                return Err(pos);
            }
        }
    }

    Ok(TokenResult {
        token: XmlTok::Partial,
        next_pos: pos,
    })
}

/// Helper: scan attribute value starting from '=' position.
/// Returns the position after the closing quote and any following whitespace/delimiter.
fn scan_attr_value<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut pos: usize,
    end: usize,
) -> Result<usize, usize> {
    let minbpc = enc.min_bytes_per_char();

    // pos points to '=' — skip it and find opening quote
    let open;
    loop {
        pos += minbpc;
        if !enc.has_char(data, pos, end) {
            return Ok(pos); // Partial — will be detected by caller
        }
        let bt = enc.byte_type(data, pos);
        if bt == ByteType::QUOT || bt == ByteType::APOS {
            open = bt;
            break;
        }
        match bt {
            ByteType::S | ByteType::LF | ByteType::CR => {}
            _ => return Err(pos),
        }
    }

    // Skip opening quote
    pos += minbpc;

    // Scan attribute value content until matching closing quote
    loop {
        if !enc.has_char(data, pos, end) {
            return Ok(pos); // Partial
        }
        let t = enc.byte_type(data, pos);
        if t == open {
            break; // Found closing quote
        }
        match t {
            ByteType::LEAD2 => {
                if end - pos < 2 { return Ok(pos); } // Partial
                pos += 2;
            }
            ByteType::LEAD3 => {
                if end - pos < 3 { return Ok(pos); } // Partial
                pos += 3;
            }
            ByteType::LEAD4 => {
                if end - pos < 4 { return Ok(pos); } // Partial
                pos += 4;
            }
            ByteType::NONXML | ByteType::MALFORM | ByteType::TRAIL => return Err(pos),
            ByteType::AMP => {
                let result = scan_ref(enc, data, pos + minbpc, end)?;
                pos = result.next_pos;
            }
            ByteType::LT => return Err(pos),
            _ => {
                pos += minbpc;
            }
        }
    }

    // Skip closing quote
    pos += minbpc;

    // After closing quote: expect whitespace, '/', '>', or next attr name
    if !enc.has_char(data, pos, end) {
        return Ok(pos); // Partial
    }
    match enc.byte_type(data, pos) {
        ByteType::S | ByteType::CR | ByteType::LF => {
            // Skip whitespace, then look for next attr or end of tag
            loop {
                pos += minbpc;
                if !enc.has_char(data, pos, end) {
                    return Ok(pos);
                }
                match enc.byte_type(data, pos) {
                    ByteType::S | ByteType::CR | ByteType::LF => {}
                    _ => break, // found next token
                }
            }
            Ok(pos) // return to outer loop to handle GT/SOL/name
        }
        _ => Ok(pos), // GT/SOL/name — return to outer loop
    }
}

/// Scan a start tag starting after "<"
pub fn scan_lt<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut pos: usize,
    end: usize,
) -> Result<TokenResult, usize> {
    if !enc.has_char(data, pos, end) {
        return Ok(TokenResult {
            token: XmlTok::Partial,
            next_pos: pos,
        });
    }

    match enc.byte_type(data, pos) {
        ByteType::EXCL => {
            pos += enc.min_bytes_per_char();
            if !enc.has_char(data, pos, end) {
                return Ok(TokenResult {
                    token: XmlTok::Partial,
                    next_pos: pos,
                });
            }
            match enc.byte_type(data, pos) {
                ByteType::MINUS => {
                    return scan_comment(enc, data, pos + enc.min_bytes_per_char(), end);
                }
                ByteType::LSQB => {
                    return scan_cdata_section(enc, data, pos + enc.min_bytes_per_char(), end);
                }
                _ => {
                    return Err(pos);
                }
            }
        }
        ByteType::QUEST => {
            return scan_pi(enc, data, pos + enc.min_bytes_per_char(), end);
        }
        ByteType::SOL => {
            return scan_end_tag(enc, data, pos + enc.min_bytes_per_char(), end);
        }
        ByteType::LEAD2 => {
            if end - pos < 2 { return Ok(TokenResult { token: XmlTok::Partial, next_pos: pos }); }
            // Multi-byte name start — advance by n and fall through to name loop
            pos += 2;
        }
        ByteType::LEAD3 => {
            if end - pos < 3 { return Ok(TokenResult { token: XmlTok::Partial, next_pos: pos }); }
            pos += 3;
        }
        ByteType::LEAD4 => {
            if end - pos < 4 { return Ok(TokenResult { token: XmlTok::Partial, next_pos: pos }); }
            pos += 4;
        }
        _ if is_nmstrt_char(enc.byte_type(data, pos)) => {
            // Start tag - advance past first char and fall through to name loop
            pos += enc.min_bytes_per_char();
        }
        _ => {
            return Err(pos);
        }
    }

    while enc.has_char(data, pos, end) {
        match enc.byte_type(data, pos) {
            ByteType::S | ByteType::CR | ByteType::LF => {
                pos += enc.min_bytes_per_char();
                while enc.has_char(data, pos, end) {
                    match enc.byte_type(data, pos) {
                        ByteType::S | ByteType::CR | ByteType::LF => {
                            pos += enc.min_bytes_per_char();
                        }
                        ByteType::GT => {
                            return Ok(TokenResult {
                                token: XmlTok::StartTagNoAtts,
                                next_pos: pos + enc.min_bytes_per_char(),
                            });
                        }
                        ByteType::SOL => {
                            pos += enc.min_bytes_per_char();
                            if !enc.has_char(data, pos, end) {
                                return Ok(TokenResult {
                                    token: XmlTok::Partial,
                                    next_pos: pos,
                                });
                            }
                            if !enc.char_matches(data, pos, ASCII_GT) {
                                return Err(pos);
                            }
                            return Ok(TokenResult {
                                token: XmlTok::EmptyElementNoAtts,
                                next_pos: pos + enc.min_bytes_per_char(),
                            });
                        }
                        _ if is_nmstrt_char(enc.byte_type(data, pos)) => {
                            return scan_atts(enc, data, pos, end);
                        }
                        _ => {
                            return Err(pos);
                        }
                    }
                }
                return Ok(TokenResult {
                    token: XmlTok::Partial,
                    next_pos: pos,
                });
            }
            ByteType::GT => {
                return Ok(TokenResult {
                    token: XmlTok::StartTagNoAtts,
                    next_pos: pos + enc.min_bytes_per_char(),
                });
            }
            ByteType::SOL => {
                pos += enc.min_bytes_per_char();
                if !enc.has_char(data, pos, end) {
                    return Ok(TokenResult {
                        token: XmlTok::Partial,
                        next_pos: pos,
                    });
                }
                if !enc.char_matches(data, pos, ASCII_GT) {
                    return Err(pos);
                }
                return Ok(TokenResult {
                    token: XmlTok::EmptyElementNoAtts,
                    next_pos: pos + enc.min_bytes_per_char(),
                });
            }
            ByteType::LEAD2 => {
                if end - pos < 2 { return Ok(TokenResult { token: XmlTok::Partial, next_pos: pos }); }
                pos += 2;
            }
            ByteType::LEAD3 => {
                if end - pos < 3 { return Ok(TokenResult { token: XmlTok::Partial, next_pos: pos }); }
                pos += 3;
            }
            ByteType::LEAD4 => {
                if end - pos < 4 { return Ok(TokenResult { token: XmlTok::Partial, next_pos: pos }); }
                pos += 4;
            }
            ByteType::COLON => {
                pos += enc.min_bytes_per_char();
            }
            _ if is_name_char(enc.byte_type(data, pos)) => {
                pos += enc.min_bytes_per_char();
            }
            _ => {
                return Err(pos);
            }
        }
    }

    Ok(TokenResult {
        token: XmlTok::Partial,
        next_pos: pos,
    })
}

/// Scan content token
pub fn content_tok<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut pos: usize,
    mut end: usize,
) -> Result<TokenResult, usize> {
    let minbpc = enc.min_bytes_per_char();

    if pos >= end {
        return Ok(TokenResult {
            token: XmlTok::None,
            next_pos: pos,
        });
    }

    if minbpc > 1 {
        let n = end - pos;
        if n & (minbpc - 1) != 0 {
            let n = n & !(minbpc - 1);
            if n == 0 {
                return Ok(TokenResult {
                    token: XmlTok::Partial,
                    next_pos: pos,
                });
            }
            end = pos + n;
        }
    }

    match enc.byte_type(data, pos) {
        ByteType::LT => {
            return scan_lt(enc, data, pos + minbpc, end);
        }
        ByteType::AMP => {
            return scan_ref(enc, data, pos + minbpc, end);
        }
        ByteType::CR => {
            pos += minbpc;
            if !enc.has_char(data, pos, end) {
                return Ok(TokenResult {
                    token: XmlTok::TrailingCr,
                    next_pos: pos,
                });
            }
            if enc.byte_type(data, pos) == ByteType::LF {
                pos += minbpc;
            }
            return Ok(TokenResult {
                token: XmlTok::DataNewline,
                next_pos: pos,
            });
        }
        ByteType::LF => {
            return Ok(TokenResult {
                token: XmlTok::DataNewline,
                next_pos: pos + minbpc,
            });
        }
        ByteType::RSQB => {
            pos += minbpc;
            if !enc.has_char(data, pos, end) {
                return Ok(TokenResult {
                    token: XmlTok::TrailingRsqb,
                    next_pos: pos,
                });
            }
            if !enc.char_matches(data, pos, ASCII_RSQB) {
                pos -= minbpc;
            } else {
                pos += minbpc;
                if !enc.has_char(data, pos, end) {
                    return Ok(TokenResult {
                        token: XmlTok::TrailingRsqb,
                        next_pos: pos,
                    });
                }
                if !enc.char_matches(data, pos, ASCII_GT) {
                    pos -= minbpc;
                } else {
                    return Ok(TokenResult {
                        token: XmlTok::Invalid,
                        next_pos: pos,
                    });
                }
            }
        }
        ByteType::LEAD2 => {
            if end - pos < 2 {
                return Ok(TokenResult { token: XmlTok::PartialChar, next_pos: pos });
            }
            pos += 2;
        }
        ByteType::LEAD3 => {
            if end - pos < 3 {
                return Ok(TokenResult { token: XmlTok::PartialChar, next_pos: pos });
            }
            pos += 3;
        }
        ByteType::LEAD4 => {
            if end - pos < 4 {
                return Ok(TokenResult { token: XmlTok::PartialChar, next_pos: pos });
            }
            pos += 4;
        }
        ByteType::NONXML | ByteType::MALFORM | ByteType::TRAIL => {
            return Err(pos);
        }
        _ => {
            pos += minbpc;
        }
    }

    while enc.has_char(data, pos, end) {
        match enc.byte_type(data, pos) {
            ByteType::LEAD2 | ByteType::LEAD3 | ByteType::LEAD4 => {
                let n = match enc.byte_type(data, pos) {
                    ByteType::LEAD2 => 2,
                    ByteType::LEAD3 => 3,
                    ByteType::LEAD4 => 4,
                    _ => unreachable!(),
                };
                if end - pos < n {
                    return Ok(TokenResult {
                        token: XmlTok::DataChars,
                        next_pos: pos,
                    });
                }
                pos += n;
            }
            ByteType::RSQB => {
                if enc.has_chars(data, pos, end, 2) {
                    if !enc.char_matches(data, pos + minbpc, ASCII_RSQB) {
                        pos += minbpc;
                    } else if enc.has_chars(data, pos, end, 3) {
                        if !enc.char_matches(data, pos + 2 * minbpc, ASCII_GT) {
                            pos += minbpc;
                        } else {
                            return Ok(TokenResult {
                                token: XmlTok::Invalid,
                                next_pos: pos + 2 * minbpc,
                            });
                        }
                    }
                }
                return Ok(TokenResult {
                    token: XmlTok::DataChars,
                    next_pos: pos,
                });
            }
            ByteType::AMP
            | ByteType::LT
            | ByteType::NONXML
            | ByteType::MALFORM
            | ByteType::TRAIL
            | ByteType::CR
            | ByteType::LF => {
                return Ok(TokenResult {
                    token: XmlTok::DataChars,
                    next_pos: pos,
                });
            }
            _ => {
                pos += minbpc;
            }
        }
    }

    Ok(TokenResult {
        token: XmlTok::DataChars,
        next_pos: pos,
    })
}

/// Scan percent token
pub fn scan_percent<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut pos: usize,
    end: usize,
) -> Result<TokenResult, usize> {
    if !enc.has_char(data, pos, end) {
        return Ok(TokenResult {
            token: XmlTok::Partial,
            next_pos: pos,
        });
    }

    match enc.byte_type(data, pos) {
        ByteType::S | ByteType::LF | ByteType::CR | ByteType::PERCNT => {
            return Ok(TokenResult {
                token: XmlTok::Percent,
                next_pos: pos,
            });
        }
        _ if is_nmstrt_char(enc.byte_type(data, pos)) => {
            pos += enc.min_bytes_per_char();
        }
        _ => {
            return Err(pos);
        }
    }

    while enc.has_char(data, pos, end) {
        match enc.byte_type(data, pos) {
            ByteType::SEMI => {
                return Ok(TokenResult {
                    token: XmlTok::ParamEntityRef,
                    next_pos: pos + enc.min_bytes_per_char(),
                });
            }
            _ if is_name_char(enc.byte_type(data, pos)) => {
                pos += enc.min_bytes_per_char();
            }
            _ => {
                return Err(pos);
            }
        }
    }

    Ok(TokenResult {
        token: XmlTok::Partial,
        next_pos: pos,
    })
}

/// Scan #name token
pub fn scan_pound_name<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut pos: usize,
    end: usize,
) -> Result<TokenResult, usize> {
    if !enc.has_char(data, pos, end) {
        return Ok(TokenResult {
            token: XmlTok::Partial,
            next_pos: pos,
        });
    }

    if !is_nmstrt_char(enc.byte_type(data, pos)) {
        return Err(pos);
    }

    pos += enc.min_bytes_per_char();

    while enc.has_char(data, pos, end) {
        match enc.byte_type(data, pos) {
            ByteType::CR
            | ByteType::LF
            | ByteType::S
            | ByteType::RPAR
            | ByteType::GT
            | ByteType::PERCNT
            | ByteType::VERBAR => {
                return Ok(TokenResult {
                    token: XmlTok::PoundName,
                    next_pos: pos,
                });
            }
            _ if is_name_char(enc.byte_type(data, pos)) => {
                pos += enc.min_bytes_per_char();
            }
            _ => {
                return Err(pos);
            }
        }
    }

    Ok(TokenResult {
        token: XmlTok::PoundName,
        next_pos: pos,
    })
}

/// Scan literal (quoted string)
pub fn scan_lit<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut pos: usize,
    end: usize,
    open: ByteType,
) -> Result<TokenResult, usize> {
    while enc.has_char(data, pos, end) {
        let t = enc.byte_type(data, pos);
        match t {
            ByteType::NONXML | ByteType::MALFORM | ByteType::TRAIL => {
                return Err(pos);
            }
            ByteType::QUOT | ByteType::APOS => {
                pos += enc.min_bytes_per_char();
                if t != open {
                    continue;
                }
                if !enc.has_char(data, pos, end) {
                    return Ok(TokenResult {
                        token: XmlTok::Literal,
                        next_pos: pos,
                    });
                }
                match enc.byte_type(data, pos) {
                    ByteType::S | ByteType::CR | ByteType::LF | ByteType::GT | ByteType::PERCNT | ByteType::LSQB => {
                        return Ok(TokenResult {
                            token: XmlTok::Literal,
                            next_pos: pos,
                        });
                    }
                    _ => {
                        return Err(pos);
                    }
                }
            }
            _ => {
                pos += enc.min_bytes_per_char();
            }
        }
    }

    Ok(TokenResult {
        token: XmlTok::Partial,
        next_pos: pos,
    })
}

/// Scan prolog token
pub fn prolog_tok<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut pos: usize,
    mut end: usize,
) -> Result<TokenResult, usize> {
    let minbpc = enc.min_bytes_per_char();

    if pos >= end {
        return Ok(TokenResult {
            token: XmlTok::None,
            next_pos: pos,
        });
    }

    if minbpc > 1 {
        let n = end - pos;
        if n & (minbpc - 1) != 0 {
            let n = n & !(minbpc - 1);
            if n == 0 {
                return Ok(TokenResult {
                    token: XmlTok::Partial,
                    next_pos: pos,
                });
            }
            end = pos + n;
        }
    }

    match enc.byte_type(data, pos) {
        ByteType::QUOT => {
            return scan_lit(enc, data, pos + minbpc, end, ByteType::QUOT);
        }
        ByteType::APOS => {
            return scan_lit(enc, data, pos + minbpc, end, ByteType::APOS);
        }
        ByteType::LT => {
            pos += minbpc;
            if !enc.has_char(data, pos, end) {
                return Ok(TokenResult {
                    token: XmlTok::Partial,
                    next_pos: pos,
                });
            }
            match enc.byte_type(data, pos) {
                ByteType::EXCL => {
                    return scan_decl(enc, data, pos + minbpc, end);
                }
                ByteType::QUEST => {
                    return scan_pi(enc, data, pos + minbpc, end);
                }
                ByteType::NMSTRT
                | ByteType::HEX
                | ByteType::LEAD2
                | ByteType::LEAD3
                | ByteType::LEAD4 => {
                    return Ok(TokenResult {
                        token: XmlTok::InstanceStart,
                        next_pos: pos - minbpc,
                    });
                }
                _ => {
                    return Err(pos);
                }
            }
        }
        ByteType::CR => {
            if pos + minbpc == end {
                return Ok(TokenResult {
                    token: XmlTok::TrailingCr,
                    next_pos: end,
                });
            }
            // Fall through to whitespace scanning loop
            pos += minbpc;
            // Scan remaining whitespace
            loop {
                if !enc.has_char(data, pos, end) {
                    break;
                }
                match enc.byte_type(data, pos) {
                    ByteType::S | ByteType::LF => {
                        pos += minbpc;
                    }
                    ByteType::CR => {
                        // Don't split CR/LF pair
                        if pos + minbpc == end {
                            break;
                        }
                        pos += minbpc;
                    }
                    _ => {
                        return Ok(TokenResult {
                            token: XmlTok::PrologS,
                            next_pos: pos,
                        });
                    }
                }
            }
            return Ok(TokenResult {
                token: XmlTok::PrologS,
                next_pos: pos,
            });
        }
        ByteType::S | ByteType::LF => {
            pos += minbpc;
            // Scan remaining whitespace
            loop {
                if !enc.has_char(data, pos, end) {
                    break;
                }
                match enc.byte_type(data, pos) {
                    ByteType::S | ByteType::LF => {
                        pos += minbpc;
                    }
                    ByteType::CR => {
                        // Don't split CR/LF pair
                        if pos + minbpc == end {
                            break;
                        }
                        pos += minbpc;
                    }
                    _ => {
                        return Ok(TokenResult {
                            token: XmlTok::PrologS,
                            next_pos: pos,
                        });
                    }
                }
            }
            return Ok(TokenResult {
                token: XmlTok::PrologS,
                next_pos: pos,
            });
        }
        ByteType::PERCNT => {
            return scan_percent(enc, data, pos + minbpc, end);
        }
        ByteType::COMMA => {
            return Ok(TokenResult {
                token: XmlTok::Comma,
                next_pos: pos + minbpc,
            });
        }
        ByteType::LSQB => {
            return Ok(TokenResult {
                token: XmlTok::OpenBracket,
                next_pos: pos + minbpc,
            });
        }
        ByteType::RSQB => {
            pos += minbpc;
            if !enc.has_char(data, pos, end) {
                return Ok(TokenResult {
                    token: XmlTok::CloseBracket,
                    next_pos: pos,
                });
            }
            if enc.char_matches(data, pos, ASCII_RSQB)
                && enc.has_chars(data, pos, end, 2)
                && enc.char_matches(data, pos + minbpc, ASCII_GT)
            {
                return Ok(TokenResult {
                    token: XmlTok::CondSectClose,
                    next_pos: pos + 2 * minbpc,
                });
            }
            return Ok(TokenResult {
                token: XmlTok::CloseBracket,
                next_pos: pos,
            });
        }
        ByteType::LPAR => {
            return Ok(TokenResult {
                token: XmlTok::OpenParen,
                next_pos: pos + minbpc,
            });
        }
        ByteType::RPAR => {
            pos += minbpc;
            if !enc.has_char(data, pos, end) {
                return Ok(TokenResult {
                    token: XmlTok::CloseParen,
                    next_pos: pos,
                });
            }
            match enc.byte_type(data, pos) {
                ByteType::AST => {
                    return Ok(TokenResult {
                        token: XmlTok::CloseParenAsterisk,
                        next_pos: pos + minbpc,
                    });
                }
                ByteType::QUEST => {
                    return Ok(TokenResult {
                        token: XmlTok::CloseParenQuestion,
                        next_pos: pos + minbpc,
                    });
                }
                ByteType::PLUS => {
                    return Ok(TokenResult {
                        token: XmlTok::CloseParenPlus,
                        next_pos: pos + minbpc,
                    });
                }
                ByteType::CR | ByteType::LF | ByteType::S | ByteType::GT | ByteType::COMMA | ByteType::VERBAR | ByteType::RPAR => {
                    return Ok(TokenResult {
                        token: XmlTok::CloseParen,
                        next_pos: pos,
                    });
                }
                _ => {
                    return Err(pos);
                }
            }
        }
        ByteType::VERBAR => {
            return Ok(TokenResult {
                token: XmlTok::Or,
                next_pos: pos + minbpc,
            });
        }
        ByteType::GT => {
            return Ok(TokenResult {
                token: XmlTok::DeclClose,
                next_pos: pos + minbpc,
            });
        }
        ByteType::NUM => {
            return scan_pound_name(enc, data, pos + minbpc, end);
        }
        _ if is_nmstrt_char(enc.byte_type(data, pos)) => {
            pos += minbpc;
        }
        _ => {
            return Err(pos);
        }
    }

    // Skip whitespace/parse name
    let start_pos = pos;
    let mut is_name = matches!(enc.byte_type(data, start_pos - minbpc), ByteType::NMSTRT | ByteType::HEX);

    while enc.has_char(data, pos, end) {
        match enc.byte_type(data, pos) {
            ByteType::GT
            | ByteType::RPAR
            | ByteType::COMMA
            | ByteType::VERBAR
            | ByteType::LSQB
            | ByteType::PERCNT
            | ByteType::S
            | ByteType::CR
            | ByteType::LF => {
                return Ok(TokenResult {
                    token: if is_name { XmlTok::Name } else { XmlTok::Nmtoken },
                    next_pos: pos,
                });
            }
            ByteType::PLUS => {
                if !is_name {
                    return Err(pos);
                }
                return Ok(TokenResult {
                    token: XmlTok::NamePlus,
                    next_pos: pos + minbpc,
                });
            }
            ByteType::AST => {
                if !is_name {
                    return Err(pos);
                }
                return Ok(TokenResult {
                    token: XmlTok::NameAsterisk,
                    next_pos: pos + minbpc,
                });
            }
            ByteType::QUEST => {
                if !is_name {
                    return Err(pos);
                }
                return Ok(TokenResult {
                    token: XmlTok::NameQuestion,
                    next_pos: pos + minbpc,
                });
            }
            ByteType::NMSTRT | ByteType::HEX => {
                pos += minbpc;
            }
            ByteType::DIGIT | ByteType::NAME | ByteType::MINUS => {
                is_name = false;
                pos += minbpc;
            }
            ByteType::COLON => {
                pos += minbpc;
            }
            ByteType::LEAD2 => {
                if end - pos < 2 { return Ok(TokenResult { token: XmlTok::PartialChar, next_pos: pos }); }
                pos += 2;
            }
            ByteType::LEAD3 => {
                if end - pos < 3 { return Ok(TokenResult { token: XmlTok::PartialChar, next_pos: pos }); }
                pos += 3;
            }
            ByteType::LEAD4 => {
                if end - pos < 4 { return Ok(TokenResult { token: XmlTok::PartialChar, next_pos: pos }); }
                pos += 4;
            }
            _ => {
                return Err(pos);
            }
        }
    }

    Ok(TokenResult {
        token: if is_name { XmlTok::Name } else { XmlTok::Nmtoken },
        next_pos: pos,
    })
}

/// Scan attribute value token
pub fn attribute_value_tok<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut pos: usize,
    end: usize,
) -> Result<TokenResult, usize> {
    if pos >= end {
        return Ok(TokenResult {
            token: XmlTok::None,
            next_pos: pos,
        });
    }

    if !enc.has_char(data, pos, end) {
        return Ok(TokenResult {
            token: XmlTok::Partial,
            next_pos: pos,
        });
    }

    let start = pos;
    while enc.has_char(data, pos, end) {
        match enc.byte_type(data, pos) {
            ByteType::LEAD2 | ByteType::LEAD3 | ByteType::LEAD4 => {
                let n = match enc.byte_type(data, pos) {
                    ByteType::LEAD2 => 2,
                    ByteType::LEAD3 => 3,
                    ByteType::LEAD4 => 4,
                    _ => unreachable!(),
                };
                pos += n;
            }
            ByteType::AMP => {
                if pos == start {
                    return scan_ref(enc, data, pos + enc.min_bytes_per_char(), end);
                }
                return Ok(TokenResult {
                    token: XmlTok::DataChars,
                    next_pos: pos,
                });
            }
            ByteType::LT => {
                return Err(pos);
            }
            ByteType::LF => {
                if pos == start {
                    return Ok(TokenResult {
                        token: XmlTok::DataNewline,
                        next_pos: pos + enc.min_bytes_per_char(),
                    });
                }
                return Ok(TokenResult {
                    token: XmlTok::DataChars,
                    next_pos: pos,
                });
            }
            ByteType::CR => {
                if pos == start {
                    pos += enc.min_bytes_per_char();
                    if !enc.has_char(data, pos, end) {
                        return Ok(TokenResult {
                            token: XmlTok::TrailingCr,
                            next_pos: pos,
                        });
                    }
                    if enc.byte_type(data, pos) == ByteType::LF {
                        pos += enc.min_bytes_per_char();
                    }
                    return Ok(TokenResult {
                        token: XmlTok::DataNewline,
                        next_pos: pos,
                    });
                }
                return Ok(TokenResult {
                    token: XmlTok::DataChars,
                    next_pos: pos,
                });
            }
            ByteType::S => {
                if pos == start {
                    return Ok(TokenResult {
                        token: XmlTok::AttributeValueS,
                        next_pos: pos + enc.min_bytes_per_char(),
                    });
                }
                return Ok(TokenResult {
                    token: XmlTok::DataChars,
                    next_pos: pos,
                });
            }
            _ => {
                pos += enc.min_bytes_per_char();
            }
        }
    }

    Ok(TokenResult {
        token: XmlTok::DataChars,
        next_pos: pos,
    })
}

/// Scan entity value token
pub fn entity_value_tok<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut pos: usize,
    end: usize,
) -> Result<TokenResult, usize> {
    if pos >= end {
        return Ok(TokenResult {
            token: XmlTok::None,
            next_pos: pos,
        });
    }

    if !enc.has_char(data, pos, end) {
        return Ok(TokenResult {
            token: XmlTok::Partial,
            next_pos: pos,
        });
    }

    let start = pos;
    while enc.has_char(data, pos, end) {
        match enc.byte_type(data, pos) {
            ByteType::LEAD2 | ByteType::LEAD3 | ByteType::LEAD4 => {
                let n = match enc.byte_type(data, pos) {
                    ByteType::LEAD2 => 2,
                    ByteType::LEAD3 => 3,
                    ByteType::LEAD4 => 4,
                    _ => unreachable!(),
                };
                pos += n;
            }
            ByteType::AMP => {
                if pos == start {
                    return scan_ref(enc, data, pos + enc.min_bytes_per_char(), end);
                }
                return Ok(TokenResult {
                    token: XmlTok::DataChars,
                    next_pos: pos,
                });
            }
            ByteType::PERCNT => {
                if pos == start {
                    let result = scan_percent(enc, data, pos + enc.min_bytes_per_char(), end)?;
                    if result.token == XmlTok::Percent {
                        return Err(pos);
                    }
                    return Ok(result);
                }
                return Ok(TokenResult {
                    token: XmlTok::DataChars,
                    next_pos: pos,
                });
            }
            ByteType::LF => {
                if pos == start {
                    return Ok(TokenResult {
                        token: XmlTok::DataNewline,
                        next_pos: pos + enc.min_bytes_per_char(),
                    });
                }
                return Ok(TokenResult {
                    token: XmlTok::DataChars,
                    next_pos: pos,
                });
            }
            ByteType::CR => {
                if pos == start {
                    pos += enc.min_bytes_per_char();
                    if !enc.has_char(data, pos, end) {
                        return Ok(TokenResult {
                            token: XmlTok::TrailingCr,
                            next_pos: pos,
                        });
                    }
                    if enc.byte_type(data, pos) == ByteType::LF {
                        pos += enc.min_bytes_per_char();
                    }
                    return Ok(TokenResult {
                        token: XmlTok::DataNewline,
                        next_pos: pos,
                    });
                }
                return Ok(TokenResult {
                    token: XmlTok::DataChars,
                    next_pos: pos,
                });
            }
            _ => {
                pos += enc.min_bytes_per_char();
            }
        }
    }

    Ok(TokenResult {
        token: XmlTok::DataChars,
        next_pos: pos,
    })
}

/// Scan ignore section token (DTD only)
pub fn ignore_section_tok<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut pos: usize,
    mut end: usize,
) -> Result<TokenResult, usize> {
    let minbpc = enc.min_bytes_per_char();

    if minbpc > 1 {
        let n = end - pos;
        if n & (minbpc - 1) != 0 {
            let n = n & !(minbpc - 1);
            end = pos + n;
        }
    }

    let mut level = 0;

    while enc.has_char(data, pos, end) {
        match enc.byte_type(data, pos) {
            ByteType::NONXML | ByteType::MALFORM | ByteType::TRAIL => {
                return Err(pos);
            }
            ByteType::LT => {
                pos += minbpc;
                if !enc.has_char(data, pos, end) {
                    return Ok(TokenResult {
                        token: XmlTok::Partial,
                        next_pos: pos,
                    });
                }
                if enc.char_matches(data, pos, ASCII_EXCL) {
                    pos += minbpc;
                    if !enc.has_char(data, pos, end) {
                        return Ok(TokenResult {
                            token: XmlTok::Partial,
                            next_pos: pos,
                        });
                    }
                    if enc.char_matches(data, pos, ASCII_LSQB) {
                        level += 1;
                        pos += minbpc;
                    }
                }
            }
            ByteType::RSQB => {
                pos += minbpc;
                if !enc.has_char(data, pos, end) {
                    return Ok(TokenResult {
                        token: XmlTok::Partial,
                        next_pos: pos,
                    });
                }
                if enc.char_matches(data, pos, ASCII_RSQB) {
                    pos += minbpc;
                    if !enc.has_char(data, pos, end) {
                        return Ok(TokenResult {
                            token: XmlTok::Partial,
                            next_pos: pos,
                        });
                    }
                    if enc.char_matches(data, pos, ASCII_GT) {
                        pos += minbpc;
                        if level == 0 {
                            return Ok(TokenResult {
                                token: XmlTok::IgnoreSect,
                                next_pos: pos,
                            });
                        }
                        level -= 1;
                    }
                }
            }
            _ => {
                pos += minbpc;
            }
        }
    }

    Ok(TokenResult {
        token: XmlTok::Partial,
        next_pos: pos,
    })
}

/// Check if a character is valid in a public ID
pub fn is_public_id<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut ptr: usize,
    mut end: usize,
) -> (bool, usize) {
    let minbpc = enc.min_bytes_per_char();
    ptr += minbpc;
    end -= minbpc;

    while enc.has_char(data, ptr, end) {
        match enc.byte_type(data, ptr) {
            ByteType::DIGIT
            | ByteType::HEX
            | ByteType::MINUS
            | ByteType::APOS
            | ByteType::LPAR
            | ByteType::RPAR
            | ByteType::PLUS
            | ByteType::COMMA
            | ByteType::SOL
            | ByteType::EQUALS
            | ByteType::QUEST
            | ByteType::CR
            | ByteType::LF
            | ByteType::SEMI
            | ByteType::EXCL
            | ByteType::AST
            | ByteType::PERCNT
            | ByteType::NUM => {
                ptr += minbpc;
            }
            ByteType::S => {
                if enc.char_matches(data, ptr, ASCII_TAB) {
                    return (false, ptr);
                }
                ptr += minbpc;
            }
            ByteType::NAME | ByteType::NMSTRT => {
                if (enc.byte_to_ascii(data, ptr) & 0x80) == 0 {
                    ptr += minbpc;
                } else {
                    return (false, ptr);
                }
            }
            _ => {
                let c = enc.byte_to_ascii(data, ptr);
                match c {
                    0x24 | 0x40 => {
                        ptr += minbpc;
                    }
                    _ => {
                        return (false, ptr);
                    }
                }
            }
        }
    }

    (true, ptr)
}

/// Get character reference number
pub fn char_ref_number<E: Encoding>(enc: &E, data: &[u8], mut pos: usize) -> i32 {
    let mut result: i32 = 0;
    let minbpc = enc.min_bytes_per_char();

    pos += 2 * minbpc; // skip &#

    if enc.char_matches(data, pos, ASCII_X) {
        pos += minbpc;
        while !enc.char_matches(data, pos, ASCII_SEMI) {
            let c = enc.byte_to_ascii(data, pos);
            result <<= 4;
            match c {
                b'0'..=b'9' => {
                    result |= (c - b'0') as i32;
                }
                b'A'..=b'F' => {
                    result += 10 + (c - b'A') as i32;
                }
                b'a'..=b'f' => {
                    result += 10 + (c - b'a') as i32;
                }
                _ => {}
            }
            if result >= 0x110000 {
                return -1;
            }
            pos += minbpc;
        }
    } else {
        while !enc.char_matches(data, pos, ASCII_SEMI) {
            let c = enc.byte_to_ascii(data, pos);
            result *= 10;
            result += (c - b'0') as i32;
            if result >= 0x110000 {
                return -1;
            }
            pos += minbpc;
        }
    }

    check_char_ref_number(result)
}

/// Validate character reference number
fn check_char_ref_number(num: i32) -> i32 {
    if !((0..=0x10FFFF).contains(&num)) {
        return -1;
    }
    // Check for NONXML characters (from ASCII_BYTE_TYPES)
    // These are control characters that are invalid in XML: 0x00-0x08, 0x0B-0x0C, 0x0E-0x1F, 0x7F
    if num < 0x20 && num != 0x09 && num != 0x0A && num != 0x0D {
        return -1;
    }
    if num == 0x7F {
        return -1;
    }
    if (num & 0xFFFE) == 0xFFFE {
        return -1;
    }
    if (0xD800..0xE000).contains(&num) || (0xFDD0..0xFDF0).contains(&num) {
        return -1;
    }
    num
}

/// Get predefined entity name
pub fn predefined_entity_name<E: Encoding>(
    enc: &E,
    data: &[u8],
    ptr: usize,
    end: usize,
) -> u8 {
    let minbpc = enc.min_bytes_per_char();
    let len = (end - ptr) / minbpc;

    match len {
        2 => {
            if enc.char_matches(data, ptr + minbpc, b't') {
                match enc.byte_to_ascii(data, ptr) {
                    b'l' => ASCII_LT,
                    b'g' => ASCII_GT,
                    _ => 0,
                }
            } else {
                0
            }
        }
        3 => {
            if enc.char_matches(data, ptr, b'a')
                && enc.char_matches(data, ptr + minbpc, b'm')
                && enc.char_matches(data, ptr + 2 * minbpc, b'p')
            {
                return ASCII_AMP;
            }
            0
        }
        4 => match enc.byte_to_ascii(data, ptr) {
            b'q' if enc.char_matches(data, ptr + minbpc, b'u')
                && enc.char_matches(data, ptr + 2 * minbpc, b'o')
                && enc.char_matches(data, ptr + 3 * minbpc, b't') =>
            {
                ASCII_QUOT
            }
            b'a' if enc.char_matches(data, ptr + minbpc, b'p')
                && enc.char_matches(data, ptr + 2 * minbpc, b'o')
                && enc.char_matches(data, ptr + 3 * minbpc, b's') =>
            {
                ASCII_APOS
            }
            _ => 0,
        },
        _ => 0,
    }
}

/// Check if name matches ASCII string
pub fn name_matches_ascii<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut ptr1: usize,
    end1: usize,
    ptr2: &[u8],
) -> bool {
    let minbpc = enc.min_bytes_per_char();

    for &c in ptr2 {
        if end1 - ptr1 < minbpc {
            return false;
        }
        if !enc.char_matches(data, ptr1, c) {
            return false;
        }
        ptr1 += minbpc;
    }

    ptr1 == end1
}

/// Get length of a name
pub fn name_length<E: Encoding>(enc: &E, data: &[u8], mut ptr: usize) -> usize {
    let start = ptr;
    let minbpc = enc.min_bytes_per_char();

    loop {
        match enc.byte_type(data, ptr) {
            ByteType::LEAD2 | ByteType::LEAD3 | ByteType::LEAD4 => {
                let n = match enc.byte_type(data, ptr) {
                    ByteType::LEAD2 => 2,
                    ByteType::LEAD3 => 3,
                    ByteType::LEAD4 => 4,
                    _ => unreachable!(),
                };
                ptr += n;
            }
            ByteType::NMSTRT
            | ByteType::HEX
            | ByteType::DIGIT
            | ByteType::NAME
            | ByteType::MINUS => {
                ptr += minbpc;
            }
            _ => {
                return ptr - start;
            }
        }
    }
}

/// Skip whitespace
pub fn skip_s<E: Encoding>(enc: &E, data: &[u8], mut ptr: usize) -> usize {
    let minbpc = enc.min_bytes_per_char();

    loop {
        match enc.byte_type(data, ptr) {
            ByteType::LF | ByteType::CR | ByteType::S => {
                ptr += minbpc;
            }
            _ => {
                return ptr;
            }
        }
    }
}

/// Attribute information (for internal parsing)
pub struct Attribute {
    pub name: usize,
    pub name_end: usize,
    pub value_ptr: usize,
    pub value_end: usize,
    pub normalized: bool,
}

/// Position information
pub struct Position {
    pub line_number: usize,
    pub column_number: usize,
}

/// Get attributes from a tag
/// Parses attribute name=value pairs from a start tag, tracking normalized state
/// Returns (number_of_attributes, attribute_vector)
pub fn get_atts<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut pos: usize,
    atts_max: usize,
) -> (usize, Vec<Attribute>) {
    enum State {
        Other,
        InName,
        InValue,
    }

    let minbpc = enc.min_bytes_per_char();
    let mut state = State::InName;
    let mut nAtts = 0;
    let mut atts = Vec::with_capacity(atts_max);
    let mut open = ByteType::QUOT; // delimiter type (QUOT or APOS)

    pos += minbpc; // skip opening < or space after tag name

    while pos < data.len() {
        let byte_type = enc.byte_type(data, pos);

        match byte_type {
            // Start of name: multi-byte lead characters
            ByteType::LEAD2 | ByteType::LEAD3 | ByteType::LEAD4 => {
                if matches!(state, State::Other) {
                    if nAtts < atts_max {
                        atts.push(Attribute {
                            name: pos,
                            name_end: 0,
                            value_ptr: 0,
                            value_end: 0,
                            normalized: true,
                        });
                    }
                    state = State::InName;
                }
                let n = match byte_type {
                    ByteType::LEAD2 => 2,
                    ByteType::LEAD3 => 3,
                    ByteType::LEAD4 => 4,
                    _ => unreachable!(),
                };
                pos += n - minbpc;
            }
            // Start of name: single-byte name start characters (letters, etc.)
            ByteType::NMSTRT | ByteType::HEX => {
                if matches!(state, State::Other) {
                    if nAtts < atts_max {
                        atts.push(Attribute {
                            name: pos,
                            name_end: 0,
                            value_ptr: 0,
                            value_end: 0,
                            normalized: true,
                        });
                    }
                    state = State::InName;
                }
            }
            // Quote delimiter
            ByteType::QUOT => {
                if !matches!(state, State::InValue) {
                    if nAtts < atts_max {
                        if let Some(att) = atts.last_mut() {
                            att.value_ptr = pos + minbpc;
                        }
                    }
                    state = State::InValue;
                    open = ByteType::QUOT;
                } else if open == ByteType::QUOT {
                    state = State::Other;
                    if nAtts < atts_max {
                        if let Some(att) = atts.last_mut() {
                            att.value_end = pos;
                        }
                    }
                    nAtts += 1;
                }
            }
            // Apostrophe delimiter
            ByteType::APOS => {
                if !matches!(state, State::InValue) {
                    if nAtts < atts_max {
                        if let Some(att) = atts.last_mut() {
                            att.value_ptr = pos + minbpc;
                        }
                    }
                    state = State::InValue;
                    open = ByteType::APOS;
                } else if open == ByteType::APOS {
                    state = State::Other;
                    if nAtts < atts_max {
                        if let Some(att) = atts.last_mut() {
                            att.value_end = pos;
                        }
                    }
                    nAtts += 1;
                }
            }
            // Entity reference (means value is not normalized)
            ByteType::AMP => {
                if nAtts < atts_max {
                    if let Some(att) = atts.last_mut() {
                        att.normalized = false;
                    }
                }
            }
            // Whitespace
            ByteType::S => {
                if matches!(state, State::InName) {
                    if nAtts < atts_max {
                        if let Some(att) = atts.last_mut() {
                            att.name_end = pos;
                        }
                    }
                    state = State::Other;
                } else if matches!(state, State::InValue) && nAtts < atts_max {
                    if let Some(att) = atts.last_mut() {
                        // Check if this whitespace makes the value non-normalized
                        if att.normalized
                            && (pos == att.value_ptr
                                || (pos + minbpc < data.len()
                                    && enc.byte_type(data, pos + minbpc) == ByteType::S)
                                || (pos + minbpc < data.len()
                                    && enc.byte_type(data, pos + minbpc) == open))
                        {
                            att.normalized = false;
                        }
                    }
                }
            }
            // Carriage return or line feed
            ByteType::CR | ByteType::LF => {
                if matches!(state, State::InName) {
                    if nAtts < atts_max {
                        if let Some(att) = atts.last_mut() {
                            att.name_end = pos;
                        }
                    }
                    state = State::Other;
                } else if matches!(state, State::InValue) && nAtts < atts_max {
                    if let Some(att) = atts.last_mut() {
                        att.normalized = false;
                    }
                }
            }
            // Equals sign — end of attribute name
            ByteType::EQUALS => {
                if matches!(state, State::InName) {
                    if nAtts < atts_max {
                        if let Some(att) = atts.last_mut() {
                            att.name_end = pos;
                        }
                    }
                    state = State::Other;
                }
            }
            // End of tag
            ByteType::GT | ByteType::SOL => {
                if !matches!(state, State::InValue) {
                    return (nAtts, atts);
                }
            }
            _ => {}
        }

        pos += minbpc;
    }

    (nAtts, atts)
}

/// Update position tracking based on character data
pub fn update_position<E: Encoding>(
    enc: &E,
    data: &[u8],
    mut pos: usize,
    end: usize,
) -> Position {
    let minbpc = enc.min_bytes_per_char();
    let mut line_number = 0;
    let mut column_number = 0;

    while enc.has_char(data, pos, end) {
        match enc.byte_type(data, pos) {
            ByteType::LEAD2 | ByteType::LEAD3 | ByteType::LEAD4 => {
                let n = match enc.byte_type(data, pos) {
                    ByteType::LEAD2 => 2,
                    ByteType::LEAD3 => 3,
                    ByteType::LEAD4 => 4,
                    _ => unreachable!(),
                };
                pos += n;
                column_number += 1;
            }
            ByteType::LF => {
                column_number = 0;
                line_number += 1;
                pos += minbpc;
            }
            ByteType::CR => {
                line_number += 1;
                pos += minbpc;
                if enc.has_char(data, pos, end) && enc.byte_type(data, pos) == ByteType::LF {
                    pos += minbpc;
                }
                column_number = 0;
            }
            _ => {
                pos += minbpc;
                column_number += 1;
            }
        }
    }

    Position {
        line_number,
        column_number,
    }
}

// ASCII constants (additional ones used in functions)
const ASCII_LT: u8 = 0x3C;
const ASCII_AMP: u8 = 0x26;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xmltok::Utf8Encoding;

    // ============ content_tok tests ============

    #[test]
    fn test_content_tok_start_tag_no_atts() {
        let enc = Utf8Encoding;
        let data = b"<doc>";
        let result = content_tok(&enc, data, 0, data.len()).unwrap();
        assert_eq!(result.token, XmlTok::StartTagNoAtts);
        assert_eq!(result.next_pos, 5);
    }

    #[test]
    fn test_content_tok_data_chars() {
        let enc = Utf8Encoding;
        let data = b"hello";
        let result = content_tok(&enc, data, 0, data.len()).unwrap();
        assert_eq!(result.token, XmlTok::DataChars);
        assert_eq!(result.next_pos, 5);
    }

    #[test]
    fn test_content_tok_end_tag() {
        let enc = Utf8Encoding;
        let data = b"</doc>";
        let result = content_tok(&enc, data, 0, data.len()).unwrap();
        assert_eq!(result.token, XmlTok::EndTag);
    }

    #[test]
    fn test_content_tok_empty_element_no_atts() {
        let enc = Utf8Encoding;
        let data = b"<e/>";
        let result = content_tok(&enc, data, 0, data.len()).unwrap();
        assert_eq!(result.token, XmlTok::EmptyElementNoAtts);
    }

    #[test]
    fn test_content_tok_entity_ref() {
        let enc = Utf8Encoding;
        let data = b"&amp;";
        let result = content_tok(&enc, data, 0, data.len()).unwrap();
        assert_eq!(result.token, XmlTok::EntityRef);
    }

    #[test]
    fn test_content_tok_char_ref() {
        let enc = Utf8Encoding;
        let data = b"&#233;";
        let result = content_tok(&enc, data, 0, data.len()).unwrap();
        assert_eq!(result.token, XmlTok::CharRef);
    }

    #[test]
    fn test_content_tok_comment() {
        let enc = Utf8Encoding;
        let data = b"<!-- comment -->";
        let result = content_tok(&enc, data, 0, data.len()).unwrap();
        assert_eq!(result.token, XmlTok::Comment);
    }

    #[test]
    fn test_content_tok_data_newline_lf() {
        let enc = Utf8Encoding;
        let data = b"\ntext";
        let result = content_tok(&enc, data, 0, data.len()).unwrap();
        assert_eq!(result.token, XmlTok::DataNewline);
        assert_eq!(result.next_pos, 1);
    }

    #[test]
    fn test_content_tok_data_newline_crlf() {
        let enc = Utf8Encoding;
        let data = b"\r\ntext";
        let result = content_tok(&enc, data, 0, data.len()).unwrap();
        assert_eq!(result.token, XmlTok::DataNewline);
        assert_eq!(result.next_pos, 2);
    }

    #[test]
    fn test_content_tok_no_data() {
        let enc = Utf8Encoding;
        let data = b"";
        let result = content_tok(&enc, data, 0, 0).unwrap();
        assert_eq!(result.token, XmlTok::None);
    }

    // ============ char_ref_number tests ============

    #[test]
    fn test_char_ref_number_decimal() {
        let enc = Utf8Encoding;
        let data = b"&#233;";
        let result = char_ref_number(&enc, data, 0);
        assert_eq!(result, 233);
    }

    #[test]
    fn test_char_ref_number_hex_lowercase() {
        let enc = Utf8Encoding;
        let data = b"&#xe9;";
        let result = char_ref_number(&enc, data, 0);
        assert_eq!(result, 233);
    }

    #[test]
    fn test_char_ref_number_hex_uppercase() {
        let enc = Utf8Encoding;
        let data = b"&#xE9;";
        let result = char_ref_number(&enc, data, 0);
        assert_eq!(result, 233);
    }

    #[test]
    fn test_char_ref_number_hex_mixed() {
        let enc = Utf8Encoding;
        let data = b"&#xEa;";
        let result = char_ref_number(&enc, data, 0);
        assert_eq!(result, 234);
    }

    #[test]
    fn test_char_ref_number_zero() {
        let enc = Utf8Encoding;
        let data = b"&#0;";
        let result = char_ref_number(&enc, data, 0);
        // NUL character (0) is invalid in XML
        assert_eq!(result, -1);
    }

    #[test]
    fn test_char_ref_number_valid_low() {
        let enc = Utf8Encoding;
        let data = b"&#32;";
        let result = char_ref_number(&enc, data, 0);
        // Space (32) is valid
        assert_eq!(result, 32);
    }

    #[test]
    fn test_char_ref_number_surrogate() {
        let enc = Utf8Encoding;
        let data = b"&#55296;"; // 0xD800 (surrogate)
        let result = char_ref_number(&enc, data, 0);
        assert_eq!(result, -1); // invalid
    }

    #[test]
    fn test_char_ref_number_valid_max() {
        let enc = Utf8Encoding;
        let data = b"&#65;"; // 'A' = 65
        let result = char_ref_number(&enc, data, 0);
        assert_eq!(result, 65);
    }

    // ============ predefined_entity_name tests ============

    #[test]
    fn test_predefined_entity_lt() {
        let enc = Utf8Encoding;
        let data = b"lt";
        let result = predefined_entity_name(&enc, data, 0, 2);
        assert_eq!(result, 0x3C); // '<'
    }

    #[test]
    fn test_predefined_entity_gt() {
        let enc = Utf8Encoding;
        let data = b"gt";
        let result = predefined_entity_name(&enc, data, 0, 2);
        assert_eq!(result, 0x3E); // '>'
    }

    #[test]
    fn test_predefined_entity_amp() {
        let enc = Utf8Encoding;
        let data = b"amp";
        let result = predefined_entity_name(&enc, data, 0, 3);
        assert_eq!(result, 0x26); // '&'
    }

    #[test]
    fn test_predefined_entity_quot() {
        let enc = Utf8Encoding;
        let data = b"quot";
        let result = predefined_entity_name(&enc, data, 0, 4);
        assert_eq!(result, 0x22); // '"'
    }

    #[test]
    fn test_predefined_entity_apos() {
        let enc = Utf8Encoding;
        let data = b"apos";
        let result = predefined_entity_name(&enc, data, 0, 4);
        assert_eq!(result, 0x27); // '\''
    }

    #[test]
    fn test_predefined_entity_unknown() {
        let enc = Utf8Encoding;
        let data = b"foo";
        let result = predefined_entity_name(&enc, data, 0, 3);
        assert_eq!(result, 0); // unknown
    }

    #[test]
    fn test_predefined_entity_wrong_length() {
        let enc = Utf8Encoding;
        let data = b"am";
        let result = predefined_entity_name(&enc, data, 0, 2);
        assert_eq!(result, 0); // not "lt" or "gt"
    }

    // ============ update_position tests ============

    #[test]
    fn test_update_position_simple_text() {
        let enc = Utf8Encoding;
        let data = b"hello";
        let pos = update_position(&enc, data, 0, data.len());
        assert_eq!(pos.line_number, 0);
        assert_eq!(pos.column_number, 5);
    }

    #[test]
    fn test_update_position_with_lf() {
        let enc = Utf8Encoding;
        let data = b"hello\nworld";
        let pos = update_position(&enc, data, 0, data.len());
        assert_eq!(pos.line_number, 1);
        assert_eq!(pos.column_number, 5); // "world"
    }

    #[test]
    fn test_update_position_with_crlf() {
        let enc = Utf8Encoding;
        let data = b"hello\r\nworld";
        let pos = update_position(&enc, data, 0, data.len());
        assert_eq!(pos.line_number, 1);
        assert_eq!(pos.column_number, 5); // "world"
    }

    #[test]
    fn test_update_position_with_cr() {
        let enc = Utf8Encoding;
        let data = b"hello\rworld";
        let pos = update_position(&enc, data, 0, data.len());
        assert_eq!(pos.line_number, 1);
        assert_eq!(pos.column_number, 5); // "world"
    }

    #[test]
    fn test_update_position_multiple_lines() {
        let enc = Utf8Encoding;
        let data = b"line1\nline2\nline3";
        let pos = update_position(&enc, data, 0, data.len());
        assert_eq!(pos.line_number, 2);
        assert_eq!(pos.column_number, 5); // "line3"
    }

    #[test]
    fn test_update_position_empty() {
        let enc = Utf8Encoding;
        let data = b"";
        let pos = update_position(&enc, data, 0, 0);
        assert_eq!(pos.line_number, 0);
        assert_eq!(pos.column_number, 0);
    }

    // ============ name_length tests ============

    #[test]
    fn test_name_length_simple() {
        let enc = Utf8Encoding;
        let data = b"doc>";
        let len = name_length(&enc, data, 0);
        assert_eq!(len, 3); // "doc"
    }

    #[test]
    fn test_name_length_with_namespace() {
        let enc = Utf8Encoding;
        let data = b"ns:tag>";
        let len = name_length(&enc, data, 0);
        // Colon stops the name, so just "ns"
        assert_eq!(len, 2);
    }

    #[test]
    fn test_name_length_with_hyphen() {
        let enc = Utf8Encoding;
        let data = b"my-elem>";
        let len = name_length(&enc, data, 0);
        // Hyphen is part of NAME, so "my-elem"
        assert_eq!(len, 7);
    }

    #[test]
    fn test_name_length_with_digit() {
        let enc = Utf8Encoding;
        let data = b"elem1>";
        let len = name_length(&enc, data, 0);
        assert_eq!(len, 5); // "elem1"
    }

    #[test]
    fn test_name_length_single_char() {
        let enc = Utf8Encoding;
        let data = b"a>";
        let len = name_length(&enc, data, 0);
        assert_eq!(len, 1); // "a"
    }

    // ============ get_atts tests ============

    #[test]
    fn test_get_atts_no_attributes() {
        let enc = Utf8Encoding;
        let data = b"tag>";
        let (count, atts) = get_atts(&enc, data, 0, 10);
        assert_eq!(count, 0);
        assert_eq!(atts.len(), 0);
    }

    #[test]
    fn test_get_atts_single_attribute() {
        let enc = Utf8Encoding;
        let data = b"tag a=\"value\">";
        let (count, atts) = get_atts(&enc, data, 0, 10);
        assert_eq!(count, 1);
        assert_eq!(atts.len(), 1);
        assert_eq!(atts[0].normalized, true); // no entities in value
    }

    #[test]
    fn test_get_atts_multiple_attributes() {
        let enc = Utf8Encoding;
        let data = b"tag a=\"1\" b=\"2\">";
        let (count, atts) = get_atts(&enc, data, 0, 10);
        assert_eq!(count, 2);
        assert_eq!(atts.len(), 2);
    }
}

#[test]
fn test_prolog_tok_instance_start() {
    use crate::xmltok::Utf8Encoding;
    let enc = Utf8Encoding;
    let data = b"<doc>hello</doc>";
    let result = prolog_tok(&enc, data, 0, data.len());
    match result {
        Ok(TokenResult { token, next_pos }) => {
            eprintln!("prolog_tok returned: {:?} at {}", token, next_pos);
            assert_eq!(token, XmlTok::InstanceStart);
            assert_eq!(next_pos, 0); // should point to '<'
        }
        Err(pos) => {
            panic!("prolog_tok failed at pos {}", pos);
        }
    }
}

#[test]
fn test_trace_doc_slash() {
    use crate::xmltok::Utf8Encoding;
    let enc = Utf8Encoding;
    let data = b"<doc/>";
    
    // Step 1: prolog_tok should return InstanceStart at pos 0
    let r1 = prolog_tok(&enc, data, 0, data.len());
    let (tok1, next1) = match r1 {
        Ok(TokenResult { token, next_pos }) => (token, next_pos),
        Err(p) => panic!("prolog_tok error at {}", p),
    };
    eprintln!("prolog_tok: {:?} next={}", tok1, next1);
    assert_eq!(tok1, XmlTok::InstanceStart);
    assert_eq!(next1, 0); // points to '<'
    
    // Step 2: content_tok from pos 0 should return EmptyElementNoAtts
    let r2 = content_tok(&enc, data, 0, data.len());
    let (tok2, next2) = match r2 {
        Ok(TokenResult { token, next_pos }) => (token, next_pos),
        Err(p) => panic!("content_tok error at {}", p),
    };
    eprintln!("content_tok: {:?} next={}", tok2, next2);
    assert_eq!(tok2, XmlTok::EmptyElementNoAtts);
    assert_eq!(next2, 6); // past '>'
    
    // Step 3: content_tok from pos 6 should return None
    let r3 = content_tok(&enc, data, 6, data.len());
    let (tok3, _next3) = match r3 {
        Ok(TokenResult { token, next_pos }) => (token, next_pos),
        Err(p) => panic!("content_tok error at {}", p),
    };
    eprintln!("content_tok #2: {:?}", tok3);
    assert_eq!(tok3, XmlTok::None);
}

#[test]
fn test_trace_doc_with_content() {
    use crate::xmltok::Utf8Encoding;
    let enc = Utf8Encoding;
    let data = b"<doc>hello</doc>";
    
    let r1 = prolog_tok(&enc, data, 0, data.len());
    let (tok1, next1) = match r1 {
        Ok(TokenResult { token, next_pos }) => (token, next_pos),
        Err(p) => panic!("prolog_tok error at {}", p),
    };
    eprintln!("1. prolog_tok: {:?} next={}", tok1, next1);
    
    let mut pos = next1;
    for i in 0..10 {
        let r = content_tok(&enc, data, pos, data.len());
        let (tok, next) = match r {
            Ok(TokenResult { token, next_pos }) => (token, next_pos),
            Err(p) => panic!("content_tok error at {}", p),
        };
        eprintln!("{}. content_tok({}): {:?} next={}", i+2, pos, tok, next);
        if tok == XmlTok::None { break; }
        pos = next;
    }
}

#[test]
fn test_content_tok_with_attrs() {
    use crate::xmltok::Utf8Encoding;
    let enc = Utf8Encoding;
    let data = b"<e a='1'/>";
    let r = content_tok(&enc, data, 0, data.len());
    match r {
        Ok(TokenResult { token, next_pos }) => {
            eprintln!("token: {:?}, next: {}", token, next_pos);
            assert!(matches!(token, XmlTok::EmptyElementWithAtts | XmlTok::EmptyElementNoAtts));
            
            // Test get_atts
            let (count, atts) = get_atts(&enc, data, 0, 10);
            eprintln!("get_atts: count={}, atts.len={}", count, atts.len());
            for (i, att) in atts.iter().enumerate() {
                eprintln!("  attr {}: name={}..{} value={}..{}", i, att.name, att.name_end, att.value_ptr, att.value_end);
                let name = std::str::from_utf8(&data[att.name..att.name_end]).unwrap();
                let value = std::str::from_utf8(&data[att.value_ptr..att.value_end]).unwrap();
                eprintln!("  attr {}: name='{}' value='{}'", i, name, value);
            }
            assert_eq!(count, 1);
        }
        Err(p) => panic!("content_tok error at {}", p),
    }
}
