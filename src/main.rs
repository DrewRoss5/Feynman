use std::io::{self, Write};
use interpreter::Interpreter;

mod interpreter;

fn main() {
    let mut calculator = Interpreter::new();
    loop{
        print!("Feynman> ");
        io::stdout().flush().unwrap();
        let expr = io::stdin().lines().next().unwrap().unwrap().replace("\n", "");
        if expr == "exit"{
            break;
        }
        match calculator.evaluate(expr) {
            Ok(val) => {println!("= {}", val)}
            Err(err) => {println!("Error: {}", err)}
        }
    }

}

