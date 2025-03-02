
use std::io::{Error, ErrorKind};

use lexer::*;
use nodes::*;
use parser::*;

mod lexer;
mod nodes;
mod parser;


// this tree assumes that the provided node is the root node generated by the parser, and it will simply evaluate the syntax starting from the root
struct AbstractTree{
    pub root: Option<Box<dyn Node>>,
}
impl AbstractTree{
    pub fn new(root: Option<Box<dyn Node>>) -> AbstractTree {AbstractTree{root}}
    pub fn eval(&mut self) -> Result<f64, Error>{
        self.root.as_mut().unwrap().evaluate()
    }
}

// this effectively just combines the lexer, parser, and AST
pub struct Interpreter{
    ast: AbstractTree,
    parser: Parser
}
impl Interpreter{
    pub fn new() -> Interpreter{
        let ast = AbstractTree::new(None);
        let parser = Parser::new(Vec::new());
        Interpreter{ast, parser}
    }

    pub fn evaluate(&mut self, expr: String) -> Result<f64, Error>{
        let tokens = tokenize(expr)?;
        self.parser.set_tokens(tokens);
        self.parser.parse()?;
        if !self.parser.validate(){
            return Err(Error::new(ErrorKind::Other, "Invalid expression (3)"));
        }
        self.ast.root = Some(self.parser.get_tree().unwrap());
        self.ast.eval()
    }

}