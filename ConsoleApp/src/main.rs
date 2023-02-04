// import packages!
use std::io::{stdin, stdout, Write};

fn main() {
    // let keywork just as in js
    let mut line = String::new();

    println!("Type a number:");

    let _ = stdout().flush(); // clean stdout, _ user to avoid warnings by the compiler

    // read line and put it into &line
    // match used to handle errors as patter matching, neato
    // https://doc.rust-lang.org/rust-by-example/flow_control/match.html
    match stdin().read_line(&mut line) {
        Ok(n) => {
            println!("{n} bytes read");
            println!("as js {line}"); // format read line into parser to pint to stdout
            println!("as C {}", line); // format read line into parser to pint to stdout
        }
        Err(e) => {
            println!("{}", e);
        }
    };
}
