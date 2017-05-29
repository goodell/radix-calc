// Copyright 2017 Dave Goodell <dave@goodell.io>
// See LICENSE file for license terms (MIT license)

mod calc {
    include!(concat!(env!("OUT_DIR"), "/calc.rs"));
}

#[test]
fn test_binary() {
    assert_eq!(calc::expr("0b00"), Ok(0b0));
    assert_eq!(calc::expr("0b01"), Ok(0b1));
    assert_eq!(calc::expr("0b11010"), Ok(0b11010));

    // negative test cases
    assert!(calc::expr("0b5").is_err());
}

#[test]
fn test_decimal() {
    assert_eq!(calc::expr("0"), Ok(0));
    assert_eq!(calc::expr("4"), Ok(4));
    assert_eq!(calc::expr("123456"), Ok(123456));

    // negative test cases
    assert!(calc::expr("1c").is_err());
}

#[test]
fn test_octal() {
    assert_eq!(calc::expr("0o0"), Ok(0o0));
    assert_eq!(calc::expr("0o1"), Ok(0o1));
    assert_eq!(calc::expr("0o755"), Ok(0o755));

    // negative test cases
    assert!(calc::expr("0o789").is_err());
}

#[test]
fn test_hex() {
    assert_eq!(calc::expr("0x0"), Ok(0x0));
    assert_eq!(calc::expr("0x1"), Ok(0x1));
    assert_eq!(calc::expr("0xfeed"), Ok(0xfeed));
    assert_eq!(calc::expr("0xdeadbeef"), Ok(0xdeadbeef));

    // negative test cases
    assert!(calc::expr("0x7g9").is_err());
}

#[test]
fn test_infix() {
    assert_eq!(calc::expr("0 + 0"), Ok(0 + 0));
    assert_eq!(calc::expr("0+ 0"), Ok(0 + 0));
    assert_eq!(calc::expr("0 +0"), Ok(0 + 0));
    assert_eq!(calc::expr("0+0"), Ok(0 + 0));

    assert_eq!(calc::expr("5 * 4"), Ok(5 * 4));
    assert_eq!(calc::expr("5 - 4"), Ok(5 - 4));
    assert_eq!(calc::expr("20 / 4"), Ok(20 / 4));
    assert_eq!(calc::expr("20 % 4"), Ok(20 % 4));
    assert_eq!(calc::expr("20 % 3"), Ok(20 % 3));
    assert_eq!(calc::expr("2**5"), Ok(32));
    assert_eq!(calc::expr("2 ** 5"), Ok(32));

    // negative test cases
    assert!(calc::expr("5 $ 6").is_err());
}

#[test]
fn test_unary() {
    assert_eq!(calc::expr("~5"), Ok(!5));
    assert_eq!(calc::expr("-5"), Ok(-5));
}

#[test]
fn test_precedence() {
    assert_eq!(calc::expr("2 + 5 * 4"), Ok(2 + 5 * 4));
    assert_eq!(calc::expr("2 + 5 * ~4"), Ok(2 + 5 * !4));
    assert_eq!(calc::expr("-2+5*4"), Ok(-2 + 5 * 4));
    assert_eq!(calc::expr("-2 + 5 * 4"), Ok(-2 + 5 * 4));
    assert_eq!(calc::expr("~2 + 5 * 4"), Ok(!2 + 5 * 4));
}

#[test]
fn test_parens() {
    assert_eq!(calc::expr("(5 * 4)"), Ok((5 * 4)));
    assert_eq!(calc::expr("(5 + 4) * 2"), Ok((5 + 4) * 2));
    assert_eq!(calc::expr("(5 + 4) * 2**2"), Ok((5 + 4) * 4));
    assert_eq!(calc::expr("(5**2 + 4) * 2**2"), Ok((25 + 4) * 4));
}

#[test]
fn test_bitwise() {
    assert_eq!(calc::expr("0b11111111 | 0b00000000"), Ok(0b11111111));
    assert_eq!(calc::expr("0b00000000 | 0b00000000"), Ok(0b00000000));
    assert_eq!(calc::expr("0b11111111 & 0b00000000"), Ok(0b00000000));
    assert_eq!(calc::expr("0b00000000 & 0b00000000"), Ok(0b00000000));
    assert_eq!(calc::expr("0b11111111 & 0b11111111"), Ok(0b11111111));
    assert_eq!(calc::expr("0b10101010 ^ 0b11110000"), Ok(0b10101010 ^ 0b11110000));
}
