// Copyright 2017 Dave Goodell <dave@goodell.io>
// See LICENSE file for license terms (MIT license)

extern crate rustc_serialize;
extern crate docopt;

use std::process;
use docopt::Docopt;

const USAGE: &'static str = concat!("
A programmer's calculator supporting multiple radixes.

Version ", env!("CARGO_PKG_VERSION"), "

Usage: radix-calc [--alfred2] [--all|--bin|--hex|--oct] [--] <expr>...
       radix-calc (-h | --help)

Options:
    -h, --help   Show this screen.
    --alfred2    Emit Alfred2-style workflow XML.
    --all        Format the result in decimal, hex, octal, and binary
    --bin        Format the result in binary (e.g., 0b0110)
    --hex        Format the result in hexadecimal (e.g., 0xcafe)
    --oct        Format the result in Rust-style octal (e.g., 0o755)
");

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_alfred2: bool,
    flag_all: bool,
    flag_bin: bool,
    flag_hex: bool,
    flag_oct: bool,
    arg_expr: Vec<String>,
}

mod radix_calc {
    include!(concat!(env!("OUT_DIR"), "/radix-calc.rs"));

    fn to_pretty(pfx: &str, e_string: &str, interval: usize) -> String {
        let mut buf = String::with_capacity(pfx.len() + (2 * e_string.len()));
        let mut i = e_string.len();

        buf.push_str(pfx);

        for c in e_string.chars() {
            if (i % interval) == 0 && (i != e_string.len()) {
                buf.push('_');
            }
            buf.push(c);
            i -= 1;
        }

        buf
    }

    pub fn to_pretty_dec(expr: i64) -> String {
        to_pretty("", &expr.to_string(), 3)
    }

    pub fn to_pretty_hex(expr: i64) -> String {
        to_pretty("0x", &format!("{:x}", expr), 4)
    }

    pub fn to_pretty_oct(expr: i64) -> String {
        to_pretty("0o", &format!("{:o}", expr), 3)
    }

    pub fn to_pretty_bin(expr: i64) -> String {
        to_pretty("0b", &format!("{:b}", expr), 8)
    }


    #[test]
    fn test_to_pretty_dec() {
        assert_eq!(to_pretty_dec(0),       "0");
        assert_eq!(to_pretty_dec(12),      "12");
        assert_eq!(to_pretty_dec(123),     "123");
        assert_eq!(to_pretty_dec(1234),    "1_234");
        assert_eq!(to_pretty_dec(12345),   "12_345");
        assert_eq!(to_pretty_dec(123456),  "123_456");
        assert_eq!(to_pretty_dec(1234567), "1_234_567");
    }

    #[test]
    fn test_to_pretty_hex() {
        assert_eq!(to_pretty_hex(0x0),         "0x0");
        assert_eq!(to_pretty_hex(0x12),        "0x12");
        assert_eq!(to_pretty_hex(0x123),       "0x123");
        assert_eq!(to_pretty_hex(0x1234),      "0x1234");
        assert_eq!(to_pretty_hex(0x12345),     "0x1_2345");
        assert_eq!(to_pretty_hex(0x123456),    "0x12_3456");
        assert_eq!(to_pretty_hex(0x1234567),   "0x123_4567");
        assert_eq!(to_pretty_hex(0x12345678),  "0x1234_5678");
        assert_eq!(to_pretty_hex(0x123456789), "0x1_2345_6789");
    }

    #[test]
    fn test_to_pretty_oct() {
        assert_eq!(to_pretty_oct(0o0),         "0o0");
        assert_eq!(to_pretty_oct(0o12),        "0o12");
        assert_eq!(to_pretty_oct(0o123),       "0o123");
        assert_eq!(to_pretty_oct(0o1234),      "0o1_234");
        assert_eq!(to_pretty_oct(0o12345),     "0o12_345");
        assert_eq!(to_pretty_oct(0o123456),    "0o123_456");
        assert_eq!(to_pretty_oct(0o1234567),   "0o1_234_567");
        assert_eq!(to_pretty_oct(0o12345670),  "0o12_345_670");
        assert_eq!(to_pretty_oct(0o123456701), "0o123_456_701");
    }

    #[test]
    fn test_to_pretty_bin() {
        assert_eq!(to_pretty_bin(0b0),                   "0b0");
        assert_eq!(to_pretty_bin(0b11),                  "0b11");
        assert_eq!(to_pretty_bin(0b111),                 "0b111");
        assert_eq!(to_pretty_bin(0b1111),                "0b1111");
        assert_eq!(to_pretty_bin(0b11111),               "0b11111");
        assert_eq!(to_pretty_bin(0b111111),              "0b111111");
        assert_eq!(to_pretty_bin(0b1111111),             "0b1111111");
        assert_eq!(to_pretty_bin(0b11111111),            "0b11111111");
        assert_eq!(to_pretty_bin(0b1_11111111),          "0b1_11111111");
        assert_eq!(to_pretty_bin(0b11_11111111),         "0b11_11111111");
        assert_eq!(to_pretty_bin(0b111_11111111),        "0b111_11111111");
        assert_eq!(to_pretty_bin(0b1111_11111111),       "0b1111_11111111");
        assert_eq!(to_pretty_bin(0b11111111_11111111),   "0b11111111_11111111");
        assert_eq!(to_pretty_bin(0b1_11111111_11111111), "0b1_11111111_11111111");
    }
}

fn emit_alfred2(expr_str: &str) {
    match radix_calc::expr(&*expr_str) {
        Ok(expr) => {
            let pretty_dec = radix_calc::to_pretty_dec(expr);
            let pretty_hex = radix_calc::to_pretty_hex(expr);
            let pretty_oct = radix_calc::to_pretty_oct(expr);
            let pretty_bin = radix_calc::to_pretty_bin(expr);

            // the Alfred2 Script Filter XML output format is documented here:
            // https://www.alfredapp.com/help/workflows/inputs/script-filter/xml/
            print!("\
<?xml version=\"1.0\"?>
<items>
  <item arg=\"{expr}\" valid=\"YES\" autocomplete=\"{pretty_dec}\" type=\"default\">
    <title>{pretty_dec}</title>
    <subtitle>copy+paste as \"{expr}\"</subtitle>
    <mod key=\"shift\" subtitle=\"copy+paste as &quot;{pretty_dec}&quot;\" valid=\"yes\" arg=\"{pretty_dec}\"/>
    <icon>dec.png</icon>
  </item>
  <item arg=\"0x{expr:x}\" valid=\"YES\" autocomplete=\"{pretty_hex}\" type=\"default\">
    <title>{pretty_hex}</title>
    <subtitle>copy+paste as \"0x{expr:x}\"</subtitle>
    <mod key=\"shift\" subtitle=\"copy+paste as &quot;{pretty_hex}&quot;\" valid=\"yes\" arg=\"{pretty_hex}\"/>
    <icon>hex.png</icon>
  </item>
  <item arg=\"0o{expr:o}\" valid=\"YES\" autocomplete=\"{pretty_oct}\" type=\"default\">
    <title>{pretty_oct}</title>
    <subtitle>copy+paste as \"0o{expr:o}\"</subtitle>
    <mod key=\"shift\" subtitle=\"copy+paste as &quot;{pretty_oct}&quot;\" valid=\"yes\" arg=\"{pretty_oct}\"/>
    <icon>oct.png</icon>
  </item>
  <item arg=\"0b{expr:b}\" valid=\"YES\" autocomplete=\"{pretty_bin}\" type=\"default\">
    <title>{pretty_bin}</title>
    <subtitle>copy+paste as \"0b{expr:b}\"</subtitle>
    <mod key=\"shift\" subtitle=\"copy+paste as &quot;{pretty_bin}&quot;\" valid=\"yes\" arg=\"{pretty_bin}\"/>
    <icon>bin.png</icon>
  </item>
</items>
",
                    expr=expr,
                    pretty_dec=pretty_dec,
                    pretty_hex=pretty_hex,
                    pretty_oct=pretty_oct,
                    pretty_bin=pretty_bin);
            }
            Err(err) => {
                print!("\
<?xml version=\"1.0\"?>
<items>
  <item arg=\"{0}\" valid=\"NO\" autocomplete=\"{0}\" type=\"default\">
    <title>{0}</title>
    <subtitle><![CDATA[{1}]]></subtitle>
  </item>
</items>
",
                    "...",
                    err);
            // don't "process::exit(1)" here, it makes Alfred mildly angry
        }
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let expr_str = args.arg_expr.join(" ");

    if args.flag_alfred2 {
        emit_alfred2(&expr_str);
    } else {
        match radix_calc::expr(&*expr_str) {
            Ok(expr) => {
                if args.flag_all {
                    println!("{:}", expr);
                    println!("0x{:x}", expr);
                    println!("0o{:o}", expr);
                    println!("0b{:b}", expr);
                } else if args.flag_bin {
                    println!("0b{:b}", expr);
                } else if args.flag_hex {
                    println!("0x{:x}", expr);
                } else if args.flag_oct {
                    println!("0o{:o}", expr);
                } else {
                    println!("{:}", expr);
                }
            }
            Err(err) => {
                println!("Error: {}", err);
                process::exit(1);
            }
        }
    }
}
