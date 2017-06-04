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

mod radix_calc {
    include!(concat!(env!("OUT_DIR"), "/radix-calc.rs"));
}

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_alfred2: bool,
    flag_all: bool,
    flag_bin: bool,
    flag_hex: bool,
    flag_oct: bool,
    arg_expr: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let expr_str = args.arg_expr.join(" ");

    if args.flag_alfred2 {
        match radix_calc::expr(&*expr_str) {
            Ok(expr) => {
                // the Alfred2 Script Filter XML output format is documented here:
                // https://www.alfredapp.com/help/workflows/inputs/script-filter/xml/
                print!("\
<?xml version=\"1.0\"?>
<items>
  <item arg=\"{0}\" valid=\"YES\" autocomplete=\"{0}\" type=\"default\">
    <title>{0}</title>
    <subtitle>(decimal)</subtitle>
    <icon>dec.png</icon>
  </item>
  <item arg=\"0x{0:x}\" valid=\"YES\" autocomplete=\"0x{0:x}\" type=\"default\">
    <title>0x{0:x}</title>
    <subtitle>(hexadecimal)</subtitle>
    <icon>hex.png</icon>
  </item>
  <item arg=\"0o{0:o}\" valid=\"YES\" autocomplete=\"0o{0:o}\" type=\"default\">
    <title>0o{0:o}</title>
    <subtitle>(octal)</subtitle>
    <icon>oct.png</icon>
  </item>
  <item arg=\"0b{0:b}\" valid=\"YES\" autocomplete=\"0b{0:b}\" type=\"default\">
    <title>0b{0:b}</title>
    <subtitle>(binary)</subtitle>
    <icon>bin.png</icon>
  </item>
</items>
",
                       expr);
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
