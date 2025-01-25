
use std::{io::{Error, ErrorKind}, 
         collections::HashMap,
         rc::Rc,
         cell::RefCell
};

use lexer::*;
use nodes::*;
use parser::*;

mod lexer;
mod nodes;
mod parser;


// this tree assumes that the provided node is the root node generated by the parser, and it will simply evaluate the syntax starting from the root
struct AbstractTree{
    pub root: Option<Box<dyn Node>>,
    vars: HashMap<String, i32>
}
impl AbstractTree{
    pub fn new(root: Option<Box<dyn Node>>) -> AbstractTree {AbstractTree{root, vars: HashMap::new()}}
    pub fn eval(&self) -> Result<i32, Error>{
        self.root.as_ref().unwrap().evaluate()
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

    pub fn evaluate(&mut self, expr: String) -> Result<i32, Error>{
        let tokens = tokenize(expr)?;
        self.parser.set_tokens(tokens);
        self.parser.parse()?;
        if self.parser.node_stack.len() != 1{
            return Err(Error::new(ErrorKind::Other, "Invalid expression (3)"));
        }
        self.ast.root = Some(self.parser.node_stack.pop().unwrap());
        self.ast.eval()
    }

}