mod calc {
    include!(concat!(env!("OUT_DIR"), "/calc.rs"));
}

#[test]
fn test_binary() {
    assert_eq!(calc::expr("00"), Ok(0b0));
    assert_eq!(calc::expr("01"), Ok(0b1));
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
    assert_eq!(calc::expr("00"), Ok(0o0));
    assert_eq!(calc::expr("01"), Ok(0o1));
    assert_eq!(calc::expr("0o755"), Ok(0o755));

    // negative test cases
    assert!(calc::expr("0o789").is_err());
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
