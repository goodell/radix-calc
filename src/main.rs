//use std::env::args;

mod calc {
    include!(concat!(env!("OUT_DIR"), "/calc.rs"));
}

fn main() {
    //let args = std::env::args().collect::<Vec<String>>();
    //let expr = std::env::args().collect::<Vec<String>>().join(" ");
    let args = std::env::args().collect::<Vec<String>>();
    let expr = args[1..].join(" ");
    //println!("args: {:?}", expr);
    println!("{:?}", calc::infix_arith(&*expr).unwrap());
}
