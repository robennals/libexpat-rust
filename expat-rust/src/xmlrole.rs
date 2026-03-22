//! Prolog role state machine, ported from expat's `xmlrole.c`.
//!
//! Classifies each token in the XML prolog and DTD into a semantic [`Role`]
//! (e.g., `DoctypeName`, `EntityValue`, `AttlistDeclName`). The parser feeds
//! tokens from [`xmltok_impl`](crate::xmltok_impl) into [`xml_token_role`],
//! which advances a state machine and returns the role for each token.

// Return codes for token roles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Error = -1,
    None = 0,
    XmlDecl,
    InstanceStart,
    DoctypeNone,
    DoctypeName,
    DoctypeSystemId,
    DoctypePublicId,
    DoctypeInternalSubset,
    DoctypeClose,
    GeneralEntityName,
    ParamEntityName,
    EntityNone,
    EntityValue,
    EntitySystemId,
    EntityPublicId,
    EntityComplete,
    EntityNotationName,
    NotationNone,
    NotationName,
    NotationSystemId,
    NotationNoSystemId,
    NotationPublicId,
    AttributeName,
    AttributeTypeCdata,
    AttributeTypeId,
    AttributeTypeIdref,
    AttributeTypeIdrefs,
    AttributeTypeEntity,
    AttributeTypeEntities,
    AttributeTypeNmtoken,
    AttributeTypeNmtokens,
    AttributeEnumValue,
    AttributeNotationValue,
    AttlistNone,
    AttlistElementName,
    ImpliedAttributeValue,
    RequiredAttributeValue,
    DefaultAttributeValue,
    FixedAttributeValue,
    ElementNone,
    ElementName,
    ContentAny,
    ContentEmpty,
    ContentPcdata,
    GroupOpen,
    GroupClose,
    GroupCloseRep,
    GroupCloseOpt,
    GroupClosePlus,
    GroupChoice,
    GroupSequence,
    ContentElement,
    ContentElementRep,
    ContentElementOpt,
    ContentElementPlus,
    Pi,
    Comment,
    TextDecl,
    IgnoreSect,
    InnerParamEntityRef,
    ParamEntityRef,
}

impl From<Role> for i32 {
    fn from(role: Role) -> i32 {
        role as i32
    }
}

// Token types processed by the state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    PrologS,
    XmlDecl,
    Pi,
    Comment,
    Bom,
    DeclOpen,
    InstanceStart,
    Name,
    PrefixedName,
    OpenBracket,
    DeclarationClose,
    Literal,
    Percent,
    CloseBracket,
    CondSectOpen,
    CondSectClose,
    Nmtoken,
    OpenParen,
    CloseParen,
    Or,
    PoundName,
    NameQuestion,
    NameAsterix,
    NamePlus,
    CloseParenAsterix,
    CloseParenQuestion,
    CloseParenPlus,
    Comma,
    None,
    ParamEntityRef,
}

// State machine states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrologState {
    Prolog0,
    Prolog1,
    Prolog2,
    Doctype0,
    Doctype1,
    Doctype2,
    Doctype3,
    Doctype4,
    Doctype5,
    InternalSubset,
    Entity0,
    Entity1,
    Entity2,
    Entity3,
    Entity4,
    Entity5,
    Entity6,
    Entity7,
    Entity8,
    Entity9,
    Entity10,
    Notation0,
    Notation1,
    Notation2,
    Notation3,
    Notation4,
    Attlist0,
    Attlist1,
    Attlist2,
    Attlist3,
    Attlist4,
    Attlist5,
    Attlist6,
    Attlist7,
    Attlist8,
    Attlist9,
    Element0,
    Element1,
    Element2,
    Element3,
    Element4,
    Element5,
    Element6,
    Element7,
    DeclClose,
    Error,
}

pub struct XmlRoleState {
    pub state: PrologState,
    pub level: u32,
    pub role_none: Role,
    pub include_level: u32,
    pub document_entity: bool,
    pub in_entity_value: bool,
}

impl Default for XmlRoleState {
    fn default() -> Self {
        Self {
            state: PrologState::Prolog0,
            level: 0,
            role_none: Role::None,
            include_level: 0,
            document_entity: true,
            in_entity_value: false,
        }
    }
}

impl XmlRoleState {
    pub fn new() -> Self {
        Self::default()
    }
}

fn keyword_matches(text: &[u8], keyword: &str) -> bool {
    if text.len() != keyword.len() {
        return false;
    }
    text.iter()
        .zip(keyword.bytes())
        .all(|(a, b)| a.eq_ignore_ascii_case(&b))
}

fn set_top_level(state: &mut XmlRoleState) {
    // External entity processing not implemented — always use internal subset
    state.state = PrologState::InternalSubset;
}

fn common(state: &mut XmlRoleState, _tok: Token) -> Role {
    if !state.document_entity && _tok == Token::ParamEntityRef {
        state.state = PrologState::Error;
        return Role::InnerParamEntityRef;
    }

    state.state = PrologState::Error;
    Role::Error
}

pub fn xml_token_role(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match state.state {
        PrologState::Prolog0 => prolog0(state, tok, _ptr, _end),
        PrologState::Prolog1 => prolog1(state, tok, _ptr, _end),
        PrologState::Prolog2 => prolog2(state, tok, _ptr, _end),
        PrologState::Doctype0 => doctype0(state, tok, _ptr, _end),
        PrologState::Doctype1 => doctype1(state, tok, _ptr, _end),
        PrologState::Doctype2 => doctype2(state, tok, _ptr, _end),
        PrologState::Doctype3 => doctype3(state, tok, _ptr, _end),
        PrologState::Doctype4 => doctype4(state, tok, _ptr, _end),
        PrologState::Doctype5 => doctype5(state, tok, _ptr, _end),
        PrologState::InternalSubset => internal_subset(state, tok, _ptr, _end),
        PrologState::Entity0 => entity0(state, tok, _ptr, _end),
        PrologState::Entity1 => entity1(state, tok, _ptr, _end),
        PrologState::Entity2 => entity2(state, tok, _ptr, _end),
        PrologState::Entity3 => entity3(state, tok, _ptr, _end),
        PrologState::Entity4 => entity4(state, tok, _ptr, _end),
        PrologState::Entity5 => entity5(state, tok, _ptr, _end),
        PrologState::Entity6 => entity6(state, tok, _ptr, _end),
        PrologState::Entity7 => entity7(state, tok, _ptr, _end),
        PrologState::Entity8 => entity8(state, tok, _ptr, _end),
        PrologState::Entity9 => entity9(state, tok, _ptr, _end),
        PrologState::Entity10 => entity10(state, tok, _ptr, _end),
        PrologState::Notation0 => notation0(state, tok, _ptr, _end),
        PrologState::Notation1 => notation1(state, tok, _ptr, _end),
        PrologState::Notation2 => notation2(state, tok, _ptr, _end),
        PrologState::Notation3 => notation3(state, tok, _ptr, _end),
        PrologState::Notation4 => notation4(state, tok, _ptr, _end),
        PrologState::Attlist0 => attlist0(state, tok, _ptr, _end),
        PrologState::Attlist1 => attlist1(state, tok, _ptr, _end),
        PrologState::Attlist2 => attlist2(state, tok, _ptr, _end),
        PrologState::Attlist3 => attlist3(state, tok, _ptr, _end),
        PrologState::Attlist4 => attlist4(state, tok, _ptr, _end),
        PrologState::Attlist5 => attlist5(state, tok, _ptr, _end),
        PrologState::Attlist6 => attlist6(state, tok, _ptr, _end),
        PrologState::Attlist7 => attlist7(state, tok, _ptr, _end),
        PrologState::Attlist8 => attlist8(state, tok, _ptr, _end),
        PrologState::Attlist9 => attlist9(state, tok, _ptr, _end),
        PrologState::Element0 => element0(state, tok, _ptr, _end),
        PrologState::Element1 => element1(state, tok, _ptr, _end),
        PrologState::Element2 => element2(state, tok, _ptr, _end),
        PrologState::Element3 => element3(state, tok, _ptr, _end),
        PrologState::Element4 => element4(state, tok, _ptr, _end),
        PrologState::Element5 => element5(state, tok, _ptr, _end),
        PrologState::Element6 => element6(state, tok, _ptr, _end),
        PrologState::Element7 => element7(state, tok, _ptr, _end),
        PrologState::DeclClose => decl_close(state, tok, _ptr, _end),
        PrologState::Error => Role::None,
    }
}

fn prolog0(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => {
            state.state = PrologState::Prolog1;
            Role::None
        }
        Token::XmlDecl => {
            state.state = PrologState::Prolog1;
            Role::XmlDecl
        }
        Token::Pi => {
            state.state = PrologState::Prolog1;
            Role::Pi
        }
        Token::Comment => {
            state.state = PrologState::Prolog1;
            Role::Comment
        }
        Token::Bom => Role::None,
        Token::DeclOpen => {
            if keyword_matches(_ptr, "DOCTYPE") {
                state.state = PrologState::Doctype0;
                Role::DoctypeNone
            } else {
                common(state, tok)
            }
        }
        Token::InstanceStart => {
            state.state = PrologState::Error;
            Role::InstanceStart
        }
        _ => common(state, tok),
    }
}

fn prolog1(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::None,
        Token::Pi => Role::Pi,
        Token::Comment => Role::Comment,
        Token::Bom => Role::None,
        Token::DeclOpen => {
            if keyword_matches(_ptr, "DOCTYPE") {
                state.state = PrologState::Doctype0;
                Role::DoctypeNone
            } else {
                common(state, tok)
            }
        }
        Token::InstanceStart => {
            state.state = PrologState::Error;
            Role::InstanceStart
        }
        _ => common(state, tok),
    }
}

fn prolog2(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::None,
        Token::Pi => Role::Pi,
        Token::Comment => Role::Comment,
        Token::InstanceStart => {
            state.state = PrologState::Error;
            Role::InstanceStart
        }
        _ => common(state, tok),
    }
}

fn doctype0(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::DoctypeNone,
        Token::Name | Token::PrefixedName => {
            state.state = PrologState::Doctype1;
            Role::DoctypeName
        }
        _ => common(state, tok),
    }
}

fn doctype1(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::DoctypeNone,
        Token::OpenBracket => {
            state.state = PrologState::InternalSubset;
            Role::DoctypeInternalSubset
        }
        Token::DeclarationClose => {
            state.state = PrologState::Prolog2;
            Role::DoctypeClose
        }
        Token::Name => {
            if keyword_matches(_ptr, "SYSTEM") {
                state.state = PrologState::Doctype3;
                Role::DoctypeNone
            } else if keyword_matches(_ptr, "PUBLIC") {
                state.state = PrologState::Doctype2;
                Role::DoctypeNone
            } else {
                common(state, tok)
            }
        }
        _ => common(state, tok),
    }
}

fn doctype2(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::DoctypeNone,
        Token::Literal => {
            state.state = PrologState::Doctype3;
            Role::DoctypePublicId
        }
        _ => common(state, tok),
    }
}

fn doctype3(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::DoctypeNone,
        Token::Literal => {
            state.state = PrologState::Doctype4;
            Role::DoctypeSystemId
        }
        _ => common(state, tok),
    }
}

fn doctype4(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::DoctypeNone,
        Token::OpenBracket => {
            state.state = PrologState::InternalSubset;
            Role::DoctypeInternalSubset
        }
        Token::DeclarationClose => {
            state.state = PrologState::Prolog2;
            Role::DoctypeClose
        }
        _ => common(state, tok),
    }
}

fn doctype5(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::DoctypeNone,
        Token::DeclarationClose => {
            state.state = PrologState::Prolog2;
            Role::DoctypeClose
        }
        _ => common(state, tok),
    }
}

fn internal_subset(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::None,
        Token::DeclOpen => {
            if keyword_matches(_ptr, "ENTITY") {
                state.state = PrologState::Entity0;
                Role::EntityNone
            } else if keyword_matches(_ptr, "ATTLIST") {
                state.state = PrologState::Attlist0;
                Role::AttlistNone
            } else if keyword_matches(_ptr, "ELEMENT") {
                state.state = PrologState::Element0;
                Role::ElementNone
            } else if keyword_matches(_ptr, "NOTATION") {
                state.state = PrologState::Notation0;
                Role::NotationNone
            } else {
                common(state, tok)
            }
        }
        Token::Pi => Role::Pi,
        Token::Comment => Role::Comment,
        Token::ParamEntityRef => Role::ParamEntityRef,
        Token::CloseBracket => {
            state.state = PrologState::Doctype5;
            Role::DoctypeNone
        }
        Token::None => Role::None,
        _ => common(state, tok),
    }
}

fn entity0(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::EntityNone,
        Token::Percent => {
            state.state = PrologState::Entity1;
            Role::EntityNone
        }
        Token::Name => {
            state.state = PrologState::Entity2;
            Role::GeneralEntityName
        }
        _ => common(state, tok),
    }
}

fn entity1(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::EntityNone,
        Token::Name => {
            state.state = PrologState::Entity7;
            Role::ParamEntityName
        }
        _ => common(state, tok),
    }
}

fn entity2(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::EntityNone,
        Token::Name => {
            if keyword_matches(_ptr, "SYSTEM") {
                state.state = PrologState::Entity4;
                Role::EntityNone
            } else if keyword_matches(_ptr, "PUBLIC") {
                state.state = PrologState::Entity3;
                Role::EntityNone
            } else {
                common(state, tok)
            }
        }
        Token::Literal => {
            state.state = PrologState::DeclClose;
            state.role_none = Role::EntityNone;
            Role::EntityValue
        }
        _ => common(state, tok),
    }
}

fn entity3(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::EntityNone,
        Token::Literal => {
            state.state = PrologState::Entity4;
            Role::EntityPublicId
        }
        _ => common(state, tok),
    }
}

fn entity4(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::EntityNone,
        Token::Literal => {
            state.state = PrologState::Entity5;
            Role::EntitySystemId
        }
        _ => common(state, tok),
    }
}

fn entity5(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::EntityNone,
        Token::DeclarationClose => {
            set_top_level(state);
            Role::EntityComplete
        }
        Token::Name => {
            if keyword_matches(_ptr, "NDATA") {
                state.state = PrologState::Entity6;
                Role::EntityNone
            } else {
                common(state, tok)
            }
        }
        _ => common(state, tok),
    }
}

fn entity6(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::EntityNone,
        Token::Name => {
            state.state = PrologState::DeclClose;
            state.role_none = Role::EntityNone;
            Role::EntityNotationName
        }
        _ => common(state, tok),
    }
}

fn entity7(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::EntityNone,
        Token::Name => {
            if keyword_matches(_ptr, "SYSTEM") {
                state.state = PrologState::Entity9;
                Role::EntityNone
            } else if keyword_matches(_ptr, "PUBLIC") {
                state.state = PrologState::Entity8;
                Role::EntityNone
            } else {
                common(state, tok)
            }
        }
        Token::Literal => {
            state.state = PrologState::DeclClose;
            state.role_none = Role::EntityNone;
            Role::EntityValue
        }
        _ => common(state, tok),
    }
}

fn entity8(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::EntityNone,
        Token::Literal => {
            state.state = PrologState::Entity9;
            Role::EntityPublicId
        }
        _ => common(state, tok),
    }
}

fn entity9(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::EntityNone,
        Token::Literal => {
            state.state = PrologState::Entity10;
            Role::EntitySystemId
        }
        _ => common(state, tok),
    }
}

fn entity10(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::EntityNone,
        Token::DeclarationClose => {
            set_top_level(state);
            Role::EntityComplete
        }
        _ => common(state, tok),
    }
}

fn notation0(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::NotationNone,
        Token::Name => {
            state.state = PrologState::Notation1;
            Role::NotationName
        }
        _ => common(state, tok),
    }
}

fn notation1(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::NotationNone,
        Token::Name => {
            if keyword_matches(_ptr, "SYSTEM") {
                state.state = PrologState::Notation3;
                Role::NotationNone
            } else if keyword_matches(_ptr, "PUBLIC") {
                state.state = PrologState::Notation2;
                Role::NotationNone
            } else {
                common(state, tok)
            }
        }
        _ => common(state, tok),
    }
}

fn notation2(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::NotationNone,
        Token::Literal => {
            state.state = PrologState::Notation4;
            Role::NotationPublicId
        }
        _ => common(state, tok),
    }
}

fn notation3(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::NotationNone,
        Token::Literal => {
            state.state = PrologState::DeclClose;
            state.role_none = Role::NotationNone;
            Role::NotationSystemId
        }
        _ => common(state, tok),
    }
}

fn notation4(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::NotationNone,
        Token::Literal => {
            state.state = PrologState::DeclClose;
            state.role_none = Role::NotationNone;
            Role::NotationSystemId
        }
        Token::DeclarationClose => {
            set_top_level(state);
            Role::NotationNoSystemId
        }
        _ => common(state, tok),
    }
}

fn attlist0(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::AttlistNone,
        Token::Name | Token::PrefixedName => {
            state.state = PrologState::Attlist1;
            Role::AttlistElementName
        }
        _ => common(state, tok),
    }
}

fn attlist1(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::AttlistNone,
        Token::DeclarationClose => {
            set_top_level(state);
            Role::AttlistNone
        }
        Token::Name | Token::PrefixedName => {
            state.state = PrologState::Attlist2;
            Role::AttributeName
        }
        _ => common(state, tok),
    }
}

fn attlist2(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::AttlistNone,
        Token::Name => {
            let types = [
                "CDATA", "ID", "IDREF", "IDREFS", "ENTITY", "ENTITIES", "NMTOKEN", "NMTOKENS",
            ];
            for (i, ty) in types.iter().enumerate() {
                if keyword_matches(_ptr, ty) {
                    state.state = PrologState::Attlist8;
                    return match i {
                        0 => Role::AttributeTypeCdata,
                        1 => Role::AttributeTypeId,
                        2 => Role::AttributeTypeIdref,
                        3 => Role::AttributeTypeIdrefs,
                        4 => Role::AttributeTypeEntity,
                        5 => Role::AttributeTypeEntities,
                        6 => Role::AttributeTypeNmtoken,
                        7 => Role::AttributeTypeNmtokens,
                        _ => unreachable!(),
                    };
                }
            }
            if keyword_matches(_ptr, "NOTATION") {
                state.state = PrologState::Attlist5;
                Role::AttlistNone
            } else {
                common(state, tok)
            }
        }
        Token::OpenParen => {
            state.state = PrologState::Attlist3;
            Role::AttlistNone
        }
        _ => common(state, tok),
    }
}

fn attlist3(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::AttlistNone,
        Token::Nmtoken | Token::Name | Token::PrefixedName => {
            state.state = PrologState::Attlist4;
            Role::AttributeEnumValue
        }
        _ => common(state, tok),
    }
}

fn attlist4(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::AttlistNone,
        Token::CloseParen => {
            state.state = PrologState::Attlist8;
            Role::AttlistNone
        }
        Token::Or => {
            state.state = PrologState::Attlist3;
            Role::AttlistNone
        }
        _ => common(state, tok),
    }
}

fn attlist5(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::AttlistNone,
        Token::OpenParen => {
            state.state = PrologState::Attlist6;
            Role::AttlistNone
        }
        _ => common(state, tok),
    }
}

fn attlist6(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::AttlistNone,
        Token::Name => {
            state.state = PrologState::Attlist7;
            Role::AttributeNotationValue
        }
        _ => common(state, tok),
    }
}

fn attlist7(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::AttlistNone,
        Token::CloseParen => {
            state.state = PrologState::Attlist8;
            Role::AttlistNone
        }
        Token::Or => {
            state.state = PrologState::Attlist6;
            Role::AttlistNone
        }
        _ => common(state, tok),
    }
}

fn attlist8(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::AttlistNone,
        Token::PoundName => {
            if keyword_matches(_ptr, "IMPLIED") {
                state.state = PrologState::Attlist1;
                Role::ImpliedAttributeValue
            } else if keyword_matches(_ptr, "REQUIRED") {
                state.state = PrologState::Attlist1;
                Role::RequiredAttributeValue
            } else if keyword_matches(_ptr, "FIXED") {
                state.state = PrologState::Attlist9;
                Role::AttlistNone
            } else {
                common(state, tok)
            }
        }
        Token::Literal => {
            state.state = PrologState::Attlist1;
            Role::DefaultAttributeValue
        }
        _ => common(state, tok),
    }
}

fn attlist9(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::AttlistNone,
        Token::Literal => {
            state.state = PrologState::Attlist1;
            Role::FixedAttributeValue
        }
        _ => common(state, tok),
    }
}

fn element0(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::ElementNone,
        Token::Name | Token::PrefixedName => {
            state.state = PrologState::Element1;
            Role::ElementName
        }
        _ => common(state, tok),
    }
}

fn element1(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::ElementNone,
        Token::Name => {
            if keyword_matches(_ptr, "EMPTY") {
                state.state = PrologState::DeclClose;
                state.role_none = Role::ElementNone;
                Role::ContentEmpty
            } else if keyword_matches(_ptr, "ANY") {
                state.state = PrologState::DeclClose;
                state.role_none = Role::ElementNone;
                Role::ContentAny
            } else {
                common(state, tok)
            }
        }
        Token::OpenParen => {
            state.state = PrologState::Element2;
            state.level = 1;
            Role::GroupOpen
        }
        _ => common(state, tok),
    }
}

fn element2(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::ElementNone,
        Token::PoundName => {
            if keyword_matches(_ptr, "PCDATA") {
                state.state = PrologState::Element3;
                Role::ContentPcdata
            } else {
                common(state, tok)
            }
        }
        Token::OpenParen => {
            state.level = 2;
            state.state = PrologState::Element6;
            Role::GroupOpen
        }
        Token::Name | Token::PrefixedName => {
            state.state = PrologState::Element7;
            Role::ContentElement
        }
        Token::NameQuestion => {
            state.state = PrologState::Element7;
            Role::ContentElementOpt
        }
        Token::NameAsterix => {
            state.state = PrologState::Element7;
            Role::ContentElementRep
        }
        Token::NamePlus => {
            state.state = PrologState::Element7;
            Role::ContentElementPlus
        }
        _ => common(state, tok),
    }
}

fn element3(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::ElementNone,
        Token::CloseParen => {
            state.state = PrologState::DeclClose;
            state.role_none = Role::ElementNone;
            Role::GroupClose
        }
        Token::CloseParenAsterix => {
            state.state = PrologState::DeclClose;
            state.role_none = Role::ElementNone;
            Role::GroupCloseRep
        }
        Token::Or => {
            state.state = PrologState::Element4;
            Role::GroupChoice
        }
        _ => common(state, tok),
    }
}

fn element4(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::ElementNone,
        Token::Name | Token::PrefixedName => {
            // Element name in mixed content model (#PCDATA|name)*
            state.state = PrologState::Element5;
            Role::ContentElement
        }
        Token::PoundName => {
            if keyword_matches(_ptr, "PCDATA") {
                state.state = PrologState::Element3;
                Role::ContentPcdata
            } else {
                common(state, tok)
            }
        }
        _ => common(state, tok),
    }
}

/// element5: after element name in mixed content (#PCDATA|name)
/// Matches C element5 — only accepts CloseParenAsterisk and Or
fn element5(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::ElementNone,
        Token::CloseParenAsterix => {
            state.state = PrologState::DeclClose;
            state.role_none = Role::ElementNone;
            Role::GroupCloseRep
        }
        Token::Or => {
            // Return to element4 for more element names in mixed content
            state.state = PrologState::Element4;
            Role::ElementNone
        }
        _ => common(state, tok),
    }
}

fn element6(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::ElementNone,
        Token::OpenParen => {
            state.level += 1;
            state.state = PrologState::Element6;
            Role::GroupOpen
        }
        Token::Name | Token::PrefixedName => {
            state.state = PrologState::Element7;
            Role::ContentElement
        }
        Token::NameQuestion => {
            state.state = PrologState::Element7;
            Role::ContentElementOpt
        }
        Token::NameAsterix => {
            state.state = PrologState::Element7;
            Role::ContentElementRep
        }
        Token::NamePlus => {
            state.state = PrologState::Element7;
            Role::ContentElementPlus
        }
        _ => common(state, tok),
    }
}

fn element7(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => Role::ElementNone,
        Token::OpenParen => {
            state.level += 1;
            state.state = PrologState::Element6;
            Role::GroupOpen
        }
        Token::CloseParen => {
            state.level -= 1;
            if state.level == 0 {
                state.state = PrologState::DeclClose;
                state.role_none = Role::ElementNone;
            } else {
                state.state = PrologState::Element7;
            }
            Role::GroupClose
        }
        Token::CloseParenAsterix => {
            state.level -= 1;
            if state.level == 0 {
                state.state = PrologState::DeclClose;
                state.role_none = Role::ElementNone;
            } else {
                state.state = PrologState::Element7;
            }
            Role::GroupCloseRep
        }
        Token::CloseParenQuestion => {
            state.level -= 1;
            if state.level == 0 {
                state.state = PrologState::DeclClose;
                state.role_none = Role::ElementNone;
            } else {
                state.state = PrologState::Element7;
            }
            Role::GroupCloseOpt
        }
        Token::CloseParenPlus => {
            state.level -= 1;
            if state.level == 0 {
                state.state = PrologState::DeclClose;
                state.role_none = Role::ElementNone;
            } else {
                state.state = PrologState::Element7;
            }
            Role::GroupClosePlus
        }
        Token::Or | Token::Comma => {
            if tok == Token::Or {
                state.state = PrologState::Element6;
                Role::GroupChoice
            } else {
                state.state = PrologState::Element6;
                Role::GroupSequence
            }
        }
        _ => common(state, tok),
    }
}

fn decl_close(state: &mut XmlRoleState, tok: Token, _ptr: &[u8], _end: &[u8]) -> Role {
    match tok {
        Token::PrologS => state.role_none,
        Token::DeclarationClose => {
            set_top_level(state);
            state.role_none
        }
        _ => common(state, tok),
    }
}

pub fn prolog_state_init(state: &mut XmlRoleState) {
    state.state = PrologState::Prolog0;
    state.document_entity = true;
    state.include_level = 0;
    state.in_entity_value = false;
}
