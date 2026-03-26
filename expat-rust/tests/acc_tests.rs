// Tests for billion laughs attack protection (amplification tracking)
// Translated from acc_tests.c in the C libexpat test suite.

use expat_rust::xmlparse::{Parser, XmlError, XmlStatus};

// Test billion laughs attack protection API
// Corresponds to C test: test_billion_laughs_attack_protection_api
#[test]
fn test_billion_laughs_attack_protection_api() {
    let mut root_parser = Parser::new(None).expect("Failed to create root parser");
    let child_parser = root_parser
        .create_external_entity_parser("entity123", None)
        .expect("Failed to create child parser");

    // === MaximumAmplification error cases ===

    // Call with non-root (child) parser should fail
    // (child_parser is a separate Parser, so we need a mutable reference)
    let mut child = child_parser;
    assert!(
        !child.set_billion_laughs_attack_protection_maximum_amplification(123.0),
        "Call with non-root parser is NOT supposed to succeed"
    );

    // NaN should fail
    assert!(
        !root_parser.set_billion_laughs_attack_protection_maximum_amplification(f32::NAN),
        "Call with NaN limit is NOT supposed to succeed"
    );

    // Negative should fail
    assert!(
        !root_parser.set_billion_laughs_attack_protection_maximum_amplification(-1.0),
        "Call with negative limit is NOT supposed to succeed"
    );

    // Positive < 1.0 should fail
    assert!(
        !root_parser.set_billion_laughs_attack_protection_maximum_amplification(0.9),
        "Call with positive limit <1.0 is NOT supposed to succeed"
    );

    // === MaximumAmplification success cases ===

    assert!(
        root_parser.set_billion_laughs_attack_protection_maximum_amplification(1.0),
        "Call with positive limit >=1.0 is supposed to succeed"
    );
    assert!(
        root_parser.set_billion_laughs_attack_protection_maximum_amplification(123456.789),
        "Call with positive limit >=1.0 is supposed to succeed"
    );
    assert!(
        root_parser.set_billion_laughs_attack_protection_maximum_amplification(f32::INFINITY),
        "Call with positive limit >=1.0 is supposed to succeed"
    );

    // === ActivationThreshold error cases ===

    // Call with non-root (child) parser should fail
    assert!(
        !child.set_billion_laughs_attack_protection_activation_threshold(123),
        "Call with non-root parser is NOT supposed to succeed"
    );

    // === ActivationThreshold success cases ===

    assert!(
        root_parser.set_billion_laughs_attack_protection_activation_threshold(123),
        "Call with non-NULL parentless parser is supposed to succeed"
    );
}

// Test amplification limit detection in isolated external parser
// Corresponds to C test: test_amplification_isolated_external_parser
//
// The test creates a document that is exactly 44 bytes — twice the length of
// "<!ENTITY a SYSTEM 'b'>" (22 bytes) used in accountingGetCurrentAmplification.
// With maximumToleratedAmplification = 2.0, we test thresholds around the
// document length to verify the amplification check triggers correctly.
#[test]
fn test_amplification_isolated_external_parser() {
    //                   1.........1.........1.........1.........1..4 => 44
    let doc = b"<!ENTITY % p1 '123456789_123456789_1234567'>";
    let doc_len = doc.len();
    assert_eq!(doc_len, 44);
    let maximum_tolerated_amplification: f32 = 2.0;

    struct TestCase {
        offset_of_threshold: i32,
        expected_ok: bool,
    }

    let cases = [
        TestCase {
            offset_of_threshold: -2,
            expected_ok: false,
        },
        TestCase {
            offset_of_threshold: -1,
            expected_ok: false,
        },
        TestCase {
            offset_of_threshold: 0,
            expected_ok: false,
        },
        TestCase {
            offset_of_threshold: 1,
            expected_ok: true,
        },
        TestCase {
            offset_of_threshold: 2,
            expected_ok: true,
        },
    ];

    for case in &cases {
        let activation_threshold_bytes = (doc_len as i64 + case.offset_of_threshold as i64) as u64;

        let mut parser = Parser::new(None).expect("Failed to create parser");

        assert!(
            parser.set_billion_laughs_attack_protection_maximum_amplification(
                maximum_tolerated_amplification
            )
        );
        assert!(parser
            .set_billion_laughs_attack_protection_activation_threshold(activation_threshold_bytes));

        let mut ext_parser = parser
            .create_external_entity_parser("", None)
            .expect("Failed to create external entity parser");

        // Parse the document one byte at a time (matches C's _XML_Parse_SINGLE_BYTES)
        let mut actual_ok = true;
        let mut error_code = XmlError::None;
        for i in 0..doc_len {
            let is_final = i == doc_len - 1;
            let status = ext_parser.parse(&doc[i..i + 1], is_final);
            if status != XmlStatus::Ok {
                actual_ok = false;
                error_code = ext_parser.error_code();
                break;
            }
        }

        assert_eq!(
            actual_ok, case.expected_ok,
            "offsetOfThreshold={}, expected_ok={}, actual_ok={}, error={:?}",
            case.offset_of_threshold, case.expected_ok, actual_ok, error_code
        );

        if !actual_ok {
            assert_eq!(
                error_code,
                XmlError::AmplificationLimitBreach,
                "offsetOfThreshold={}: expected AmplificationLimitBreach, got {:?}",
                case.offset_of_threshold,
                error_code
            );
        }
    }
}

// Test that billion laughs attack is actually blocked with default settings
#[test]
fn test_billion_laughs_attack_blocked() {
    // A classic billion laughs payload — exponential entity expansion
    let payload = r#"<?xml version="1.0"?>
<!DOCTYPE lolz [
  <!ENTITY lol "lol">
  <!ENTITY lol2 "&lol;&lol;&lol;&lol;&lol;&lol;&lol;&lol;&lol;&lol;">
  <!ENTITY lol3 "&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;&lol2;">
  <!ENTITY lol4 "&lol3;&lol3;&lol3;&lol3;&lol3;&lol3;&lol3;&lol3;&lol3;&lol3;">
  <!ENTITY lol5 "&lol4;&lol4;&lol4;&lol4;&lol4;&lol4;&lol4;&lol4;&lol4;&lol4;">
  <!ENTITY lol6 "&lol5;&lol5;&lol5;&lol5;&lol5;&lol5;&lol5;&lol5;&lol5;&lol5;">
  <!ENTITY lol7 "&lol6;&lol6;&lol6;&lol6;&lol6;&lol6;&lol6;&lol6;&lol6;&lol6;">
  <!ENTITY lol8 "&lol7;&lol7;&lol7;&lol7;&lol7;&lol7;&lol7;&lol7;&lol7;&lol7;">
  <!ENTITY lol9 "&lol8;&lol8;&lol8;&lol8;&lol8;&lol8;&lol8;&lol8;&lol8;&lol8;">
]>
<lolz>&lol9;</lolz>"#;

    let mut parser = Parser::new(None).expect("Failed to create parser");
    // Use a low activation threshold so the test completes quickly
    parser.set_billion_laughs_attack_protection_activation_threshold(1024);
    parser.set_billion_laughs_attack_protection_maximum_amplification(100.0);

    let status = parser.parse(payload.as_bytes(), true);
    // The parser should abort with an amplification limit breach
    assert_ne!(
        status,
        XmlStatus::Ok,
        "Billion laughs attack should have been blocked"
    );
    assert_eq!(
        parser.error_code(),
        XmlError::AmplificationLimitBreach,
        "Expected AmplificationLimitBreach error"
    );
}
