// Copyright 2017 Dave Goodell <dave@goodell.io>
// See LICENSE file for license terms (MIT license)

// TODO:
// - package the Alfred workflow properly, including icon
// - add a README.md
// - add "<icon>hex.png</icon>" (etc.) to xml output
// - add tests for the binary's behavior
// - convert all signed integer arithmetic to use "wrapping_add()" and friends to avoid undefined
//   behavior?
// - toggle type width/signedness with shift/option/ctrl/cmd modifier keys?
// - add "--binary"/"--hex"/etc. output format options for the default (non-"--alfred2") mode

extern crate rustc_serialize;
extern crate docopt;

use std::process;
use docopt::Docopt;

const USAGE: &'static str = "
Usage: alfred-rust-calc [--alfred2] [--] <expr>...
       alfred-rust-calc (-h | --help)

Options:
    -h,--help    Show this screen.
    --alfred2    Emit Alfred2-style workflow XML.
";

mod calc {
    include!(concat!(env!("OUT_DIR"), "/calc.rs"));
}

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_alfred2: bool,
    arg_expr: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let expr_str = args.arg_expr.join(" ");

    if args.flag_alfred2 {
        match calc::expr(&*expr_str) {
            Ok(expr) => {
                // the Alfred2 Script Filter XML output format is documented here:
                // https://www.alfredapp.com/help/workflows/inputs/script-filter/xml/
                print!("\
<?xml version=\"1.0\"?>
<items>
  <item arg=\"{0}\" valid=\"YES\" autocomplete=\"{0}\" type=\"default\">
    <title>{0}</title>
    <subtitle>(decimal)</subtitle>
  </item>
  <item arg=\"0x{0:x}\" valid=\"YES\" autocomplete=\"0x{0:x}\" type=\"default\">
    <title>0x{0:x}</title>
    <subtitle>(hexadecimal)</subtitle>
  </item>
  <item arg=\"0o{0:o}\" valid=\"YES\" autocomplete=\"0o{0:o}\" type=\"default\">
    <title>0o{0:o}</title>
    <subtitle>(octal)</subtitle>
  </item>
  <item arg=\"0b{0:b}\" valid=\"YES\" autocomplete=\"0b{0:b}\" type=\"default\">
    <title>0b{0:o}</title>
    <subtitle>(binary)</subtitle>
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
        match calc::expr(&*expr_str) {
            Ok(expr) => {
                println!("{:}", expr);
            }
            Err(err) => {
                println!("Error: {}", err);
                process::exit(1);
            }
        }
    }
}
