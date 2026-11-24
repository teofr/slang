//! Literal typing and compile-time constant folding: number and string
//! literals, the arithmetic folded over them, and the folded values consumed as
//! array lengths and storage base slots.

use num_bigint::{BigInt, BigUint};
use num_rational::BigRational;

use super::expression;
use crate::types::{AddressType, LiteralKind, Type};

#[test]
fn test_value_bearing_integer_literal_types() {
    let (type_, _) = expression("127").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(127)
        })
    );

    let (type_, _) = expression("-128").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(-128)
        })
    );

    // Hex literals carry source byte width as `HexInteger`, distinct from
    // decimal `Integer` so the byte-array conversion rule can fire.
    let (type_, _) = expression("0xff").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::HexInteger {
            value: BigUint::from(255u32),
            bytes: 1,
        })
    );

    // Source byte width is preserved across leading zeros.
    let (type_, _) = expression("0x0012").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::HexInteger {
            value: BigUint::from(18u32),
            bytes: 2,
        })
    );

    // An odd digit count spans no whole number of bytes: a plain `Integer`.
    let (type_, _) = expression("0x123").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(291)
        })
    );

    // Folding a hex literal demotes it to a plain `Integer` (provenance lost).
    let (type_, _) = expression("0x10 + 0").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(16)
        })
    );
}

#[test]
fn test_address_literal_types_as_address() {
    let address = Some(Type::Address(AddressType { is_payable: false }));
    let (type_, _) =
        expression("0xb701a286753ADe3704e1DcFD93507454A2307641").into_type_and_diagnostic();
    assert_eq!(type_, address);
    let (type_, _) =
        expression("(0xb701a286753ADe3704e1DcFD93507454A2307641)").into_type_and_diagnostic();
    assert_eq!(type_, address);

    // `address` has no arithmetic, so the literal does not fold as a number.
    let (type_, _) =
        expression("0xb701a286753ADe3704e1DcFD93507454A2307641 + 1").into_type_and_diagnostic();
    assert_eq!(type_, None);
}

#[test]
fn test_binary_arithmetic_folds_to_narrowed_literal() {
    // Addition.
    let (type_, _) = expression("1 + 1").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(2)
        })
    );

    // Multiplication.
    let (type_, _) = expression("3 * 4").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(12)
        })
    );

    // Power.
    let (type_, _) = expression("2 ** 10").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(1024)
        })
    );

    // Shift.
    let (type_, _) = expression("1 << 32").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(1u64 << 32)
        })
    );

    // Reducible rational arithmetic normalises back to an integer.
    let (type_, _) = expression("1.5 * 2").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(3)
        })
    );

    // Non-reducing rational division stays rational.
    let (type_, _) = expression("5 / 2").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Rational {
            value: BigRational::new(BigInt::from(5), BigInt::from(2))
        })
    );

    // Negation of a folded constant.
    let (type_, _) = expression("-(1 + 1)").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(-2)
        })
    );
}

#[test]
fn test_binary_bitwise_folds_to_literal() {
    // OR
    let (type_, _) = expression("1 | 2").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(3)
        })
    );

    // AND
    let (type_, _) = expression("12 & 10").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(8)
        })
    );

    // XOR
    let (type_, _) = expression("6 ^ 3").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(5)
        })
    );

    // Folding hex operands demotes the result to a plain `Integer`
    // (mirroring the additive folding behaviour).
    let (type_, _) = expression("0xf0 | 0x0f").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(0xff)
        })
    );

    // Bitwise AND with a negative literal: BigInt uses arbitrary-precision
    // two's-complement, so `-1 & 0xff` masks to the low byte.
    let (type_, _) = expression("(-1) & 0xff").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(0xff)
        })
    );

    // Bitwise OR of a folded constant feeds further folding.
    let (type_, _) = expression("(1 | 2) ^ 4").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(7)
        })
    );
}

#[test]
fn test_bitwise_not_folds_to_literal() {
    // ~x = -x - 1 (two's complement on an infinite-precision integer).
    let (type_, _) = expression("~1").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(-2)
        })
    );

    let (type_, _) = expression("~0").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(-1)
        })
    );

    // Double-complement returns to the original value.
    let (type_, _) = expression("~(-1)").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(0)
        })
    );

    // Folding `~hex` demotes the result to a plain `Integer`.
    let (type_, _) = expression("~0xff").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(-256)
        })
    );

    // `~` of a folded constant.
    let (type_, _) = expression("~(1 | 2)").into_resolved_type();
    assert_eq!(
        type_,
        Type::Literal(LiteralKind::Integer {
            value: BigInt::from(-4)
        })
    );
}

#[test]
fn test_bitwise_operations_unresolved_for_rationals() {
    // Bitwise binary operators don't apply to non-reducing rationals.
    let (type_, _) = expression("1.5 | 1").into_type();
    assert_eq!(type_, None);

    let (type_, _) = expression("1 & 0.5").into_type();
    assert_eq!(type_, None);

    let (type_, _) = expression("0.5 ^ 0.25").into_type();
    assert_eq!(type_, None);

    // Likewise for the unary bitwise NOT.
    let (type_, _) = expression("~0.5").into_type();
    assert_eq!(type_, None);
}

fn string_literal(value: &[u8]) -> Type {
    Type::Literal(LiteralKind::String {
        value: value.into(),
    })
}

fn hex_string_literal(value: &[u8]) -> Type {
    Type::Literal(LiteralKind::HexString {
        value: value.into(),
    })
}

#[test]
fn test_string_literal_value_with_escapes() {
    // Plain ASCII: one byte per char.
    let (type_, _) = expression(r#""abc""#).into_resolved_type();
    assert_eq!(type_, string_literal(b"abc"));

    // Each `\n`, `\t`, etc. decodes to a single byte.
    let (type_, _) = expression(r#""\n\t\\""#).into_resolved_type();
    assert_eq!(type_, string_literal(b"\n\t\\"));

    // `\xNN` escapes decode to one byte each, regardless of the 4-char source
    // length per escape.
    let (type_, _) = expression(r#""\x41\x42""#).into_resolved_type();
    assert_eq!(type_, string_literal(b"AB"));

    // Line continuations (`\<newline>`) decode to nothing.
    let (type_, _) = expression("\"a\\\nb\"").into_resolved_type();
    assert_eq!(type_, string_literal(b"ab"));

    // Concatenated string literals: the pieces are joined.
    let (type_, _) = expression(r#""abc" "de""#).into_resolved_type();
    assert_eq!(type_, string_literal(b"abcde"));
}

#[test]
fn test_unicode_string_literal_value() {
    // ASCII unicode-string literal: one byte per char.
    let (type_, _) = expression(r#"unicode"abc""#).into_resolved_type();
    assert_eq!(type_, string_literal(b"abc"));

    // Multi-byte UTF-8 passes through as its encoding: `€` is 3 bytes.
    let (type_, _) = expression(r#"unicode"€""#).into_resolved_type();
    assert_eq!(type_, string_literal("€".as_bytes()));

    // `\uNNNN` escapes decode to their UTF-8 encoding:
    // `\u20AC` (€) → 3 bytes, `\u00A2` (¢) → 2 bytes, `\u0024` ($) → 1 byte.
    let (type_, _) = expression(r#"unicode"\u20AC\u00A2\u0024""#).into_resolved_type();
    assert_eq!(type_, string_literal("€¢$".as_bytes()));
}

#[test]
fn test_hex_string_literal_value() {
    // Pairs of hex digits, no separators: one byte per pair.
    let (type_, _) = expression(r#"hex"414243""#).into_resolved_type();
    assert_eq!(type_, hex_string_literal(b"ABC"));

    // Underscore separators don't contribute to the decoded value.
    let (type_, _) = expression(r#"hex"41_42""#).into_resolved_type();
    assert_eq!(type_, hex_string_literal(b"AB"));

    // Concatenated hex string literals: the pieces are joined.
    let (type_, _) = expression(r#"hex"4142" hex"43""#).into_resolved_type();
    assert_eq!(type_, hex_string_literal(b"ABC"));
}

/// The value a literal expression folds to, or `None` if it has no type, as
/// when it is past solc's precision limit for constants.
fn folded_integer(expression_text: &str) -> Option<BigInt> {
    match expression(expression_text).into_type().0? {
        Type::Literal(LiteralKind::Integer { value }) => Some(value),
        other => panic!("expected an integer literal type, got {other:?}"),
    }
}

/// `2^exponent`.
fn power_of_two(exponent: u32) -> BigInt {
    BigInt::from(2).pow(exponent)
}

/// solc limits a constant's numerator and denominator to have their most
/// significant bit at bit 4096 or below, after every binary operator.
#[test]
fn test_folding_is_limited_to_4096_bits() {
    // `2^4096` has its most significant bit at bit 4096.
    assert_eq!(
        folded_integer("2 ** 2048 * 2 ** 2048"),
        Some(power_of_two(4096))
    );
    assert_eq!(folded_integer("2 ** 2048 * 2 ** 2048 * 2"), None);
    assert_eq!(
        folded_integer("2 ** 2048 * 2 ** 2048 + 2 ** 2048 * 2 ** 2048"),
        None
    );
    assert_eq!(
        folded_integer("2 ** 2048 * 2 ** 2048 - 1"),
        Some(power_of_two(4096) - 1)
    );
    // A chain of multiplications stops at the limit rather than growing.
    assert_eq!(folded_integer("1e1233 * 1e1233 * 1e1233 * 1e1233"), None);
    assert!(
        expression("1e-1233 * 1e-1233 * 1e-1233 * 1e-1233")
            .into_type()
            .0
            .is_none()
    );
}

/// solc estimates the size of a power before computing it, as
/// `exponent * (msb(base) + 1)` bits.
#[test]
fn test_exponentiation_is_limited_before_computing() {
    assert_eq!(folded_integer("2 ** 2048"), Some(power_of_two(2048)));
    assert_eq!(folded_integer("2 ** 2049"), None);
    // `3^2049` has 3248 bits, but the estimate is `2049 * 2`.
    assert_eq!(folded_integer("3 ** 2048"), Some(BigInt::from(3).pow(2048)));
    assert_eq!(folded_integer("3 ** 2049"), None);
    // These would take 512 MB each to compute.
    assert_eq!(folded_integer("2 ** 4294967295"), None);
    assert_eq!(folded_integer("2 ** 4294967296"), None);
    // Zero, one and minus one to any power are computed without a limit, and
    // anything to the power of zero is one.
    assert_eq!(folded_integer("1 ** (2 ** 40)"), Some(BigInt::from(1)));
    assert_eq!(folded_integer("0 ** (2 ** 40)"), Some(BigInt::from(0)));
    assert_eq!(
        folded_integer("(-1) ** (2 ** 40 + 1)"),
        Some(BigInt::from(-1))
    );
    assert_eq!(folded_integer("(2 ** 2048) ** 0"), Some(BigInt::from(1)));
    // Including a negative power of zero, which solc folds to zero.
    assert_eq!(folded_integer("0 ** -1"), Some(BigInt::from(0)));
}

#[test]
fn test_left_shift_is_limited_before_computing() {
    // The estimate is `msb + amount + 1` bits, one more than the result has.
    assert_eq!(folded_integer("1 << 4095"), Some(power_of_two(4095)));
    assert_eq!(folded_integer("1 << 4096"), None);
    assert_eq!(folded_integer("1 << 4294967295"), None);
    assert_eq!(folded_integer("1 << 4294967296"), None);
    // Shifting zero never grows it.
    assert_eq!(folded_integer("0 << 4294967295"), Some(BigInt::from(0)));
    // A right shift can only shrink its operand.
    assert_eq!(folded_integer("1 >> 4294967295"), Some(BigInt::from(0)));
    assert_eq!(folded_integer("-1 >> 4294967295"), Some(BigInt::from(-1)));
}

/// A literal with an exponent is limited before computing `10^exponent`.
#[test]
fn test_literal_exponent_is_limited_before_computing() {
    // `10^1233` is just under `2^4096`.
    assert_eq!(folded_integer("1e1233"), Some(BigInt::from(10).pow(1233)));
    assert_eq!(folded_integer("1e1234"), None);
    assert_eq!(folded_integer("2e1233"), None);
    // This would take minutes to compute.
    assert_eq!(folded_integer("1e400000000"), None);
    // The exponent must fit in 32 bits.
    assert_eq!(folded_integer("1e4294967296"), None);
    // The check is on the mantissa in lowest terms.
    assert_eq!(folded_integer("1.0e1233"), Some(BigInt::from(10).pow(1233)));
    assert_eq!(
        folded_integer("0.5e1233"),
        Some(BigInt::from(5) * BigInt::from(10).pow(1232))
    );
    assert_eq!(folded_integer("1.5e1233"), None);
    // A negative exponent checks the denominator.
    assert!(matches!(
        expression("1e-1233").into_type().0,
        Some(Type::Literal(LiteralKind::Rational { .. }))
    ));
    assert!(expression("1e-1234").into_type().0.is_none());
    // A zero mantissa is zero, whatever the exponent.
    assert_eq!(
        folded_integer("0e99999999999999999999"),
        Some(BigInt::from(0))
    );
    // The unit is applied after the check.
    assert_eq!(
        folded_integer("1e1233 ether"),
        Some(BigInt::from(10).pow(1233 + 18))
    );
    // A literal without an exponent is not limited, as in solc.
    assert_eq!(
        folded_integer(&format!("1{}", "0".repeat(1300))),
        Some(BigInt::from(10).pow(1300))
    );
    // But arithmetic on it is, unless the result is zero.
    let large = format!("1{}", "0".repeat(1300));
    assert_eq!(
        folded_integer(&format!("{large} * 0")),
        Some(BigInt::from(0))
    );
    assert_eq!(
        folded_integer(&format!("{large} - {large}")),
        Some(BigInt::from(0))
    );
    assert_eq!(folded_integer(&format!("{large} >> 0")), None);
    assert_eq!(folded_integer(&format!("{large} | 0")), None);
}
