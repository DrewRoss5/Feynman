use calc::Interpreter;

mod calc;

fn main() {
    let mut calculator = Interpreter::new();
    match calculator.evaluate("11 - 6 * 2".to_string()){
        Ok(val) => {println!("Value: {}", val)}
        Err(err) => {println!("Err: {}", err)}
    }
}
