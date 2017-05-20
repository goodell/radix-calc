mod calc {
    include!(concat!(env!("OUT_DIR"), "/calc.rs"));
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let expr = args[1..].join(" ");
    println!("{:?}", calc::expr(&*expr).unwrap());

    // TODO:
    // - add "--alfred2" mode to emit Alfred2-style XML output
    //   + emit 4 options: decimal, hex, octal, binary
    // - add "--help" support
    // - convert "infix_arith" to a more general name
    // - package the Alfred workflow properly, including icon
    // - add a README.md
    // - add unary operators:
    //   + integer negation ("-")
    //   + bitwise negation ("~")
    // - add parentheses support
}
