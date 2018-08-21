#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(skylark);

mod ast;

#[test]
fn zero() {
    assert_eq!(skylark::IntParser::new().parse("0"), Ok(0));
    assert_eq!(skylark::IntParser::new().parse("00000"), Ok(0));
}

#[test]
fn decimal() {
    assert_eq!(skylark::IntParser::new().parse("8"), Ok(8));
    assert_eq!(skylark::IntParser::new().parse("10"), Ok(10));
    assert!(skylark::IntParser::new().parse("01").is_err());
}

#[test]
fn octal() {
    assert_eq!(skylark::IntParser::new().parse("0o7"), Ok(7));
    assert_eq!(skylark::IntParser::new().parse("0O7"), Ok(7));
    assert_eq!(skylark::IntParser::new().parse("0O777"), Ok(0o777));
}

#[test]
fn hexadecimal() {
    assert_eq!(skylark::IntParser::new().parse("0x7"), Ok(7));
    assert_eq!(skylark::IntParser::new().parse("0X7"), Ok(7));
    assert_eq!(skylark::IntParser::new().parse("0xffe"), Ok(0xffe));
}

#[test]
fn identifier() {
    assert!(skylark::IdentifierParser::new().parse("0x7").is_err());
    assert!(skylark::IdentifierParser::new().parse("foo").is_ok());
    assert!(skylark::IdentifierParser::new().parse("_foo").is_ok());
    assert!(skylark::IdentifierParser::new().parse("__foo").is_ok());
    assert!(skylark::IdentifierParser::new().parse("Foo").is_ok());
    assert!(skylark::IdentifierParser::new().parse("F0ooBar").is_ok());
}