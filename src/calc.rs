
use std::{io::{Error, ErrorKind}, collections::HashMap};


// represents a node in the AST, eventually, everything has to be able to be evaluated to a numbe
pub trait Node{
    fn evaluate(&self) -> Result<i32, Error>;
}

// represents an integer number, should have no children
struct NumNode{
    val: i32
}

impl NumNode{
    pub fn new(val: i32) -> NumNode{
        NumNode{val}
    }
}

impl Node for NumNode {
    fn evaluate(&self) -> Result<i32, Error> {
        Ok(self.val)
    }
}

struct OpNode{
    operator: char,
    pub left: Option<Box<dyn Node>>,
    pub right: Option<Box<dyn Node>>
}

impl OpNode{
    pub fn new(operator: char) -> OpNode{
        OpNode{operator, left: None, right: None}
    }
}

impl Node for OpNode{
    fn evaluate(&self) -> Result<i32, Error> {
        if self.left.is_none() || self.right.is_none(){
            return Err(Error::new(ErrorKind::Other, "Invalid expression"));
        }
        let left_val = self.left.as_ref().unwrap().evaluate()?;
        let right_val = self.right.as_ref().unwrap().evaluate()?;
        match self.operator {
            '+' => {Ok(left_val + right_val)}
            '-' => {Ok(left_val - right_val)}
            '*' => {Ok(left_val * right_val)}
            '/' => {Ok(left_val / right_val)}
            _ => {Err(Error::new(ErrorKind::Other, "something has gone terribly wrong!"))}
        }   
    }
}

pub struct SyntaxTree{
    node_stack: Vec<Box<dyn Node>>,
    vars: HashMap<String, i32>,
    str_pos: usize,
    str_size: usize,
    expr: Vec<char>
}

impl SyntaxTree{
    pub fn new(expr: String) -> SyntaxTree{
        let chars: Vec<char> = expr.chars().collect();
        SyntaxTree{node_stack: Vec::new(), vars: HashMap::new(), str_pos: 0, str_size: expr.len(), expr: chars}
    }

    // parses the next node from the expression and pushes it onto the node stack
    fn parse_node(&mut self) -> Result<(), Error>{
        while self.str_pos < self.str_size{
            let mut chr = self.expr[self.str_pos];
            if chr >= '0' && chr <= '9'{
                let  mut num_str = String::new();
                while self.str_pos < self.str_size && chr >= '0' && chr <= '9'{
                    num_str.push(chr);
                    self.str_pos += 1;
                    if self.str_pos != self.str_size{
                        chr = self.expr[self.str_pos];
                    }
                }
                let num: i32 = num_str.parse().unwrap();
                self.node_stack.push(Box::new(NumNode::new(num)));

                return self.parse_node()
            }
            else if "+-*/".contains(chr){
                if self.node_stack.len() == 0{
                    return Err(Error::new(ErrorKind::Other, "Invalid expression (1)"));
                }
                let mut op = OpNode::new(chr);
                op.left = Some(self.node_stack.pop().unwrap());
                self.str_pos += 1;
                self.parse_node()?;
                if self.node_stack.len() == 0{
                    return Err(Error::new(ErrorKind::Other, "Invalid expression (2)"));
                }
                op.right = Some(self.node_stack.pop().unwrap());
                self.node_stack.push(Box::new(op));
                println!("Pushed: {}", chr);
                return self.parse_node()
            }
            else if chr == ' '{
                self.str_pos += 1;
                return self.parse_node()
            }
            else{
                return Err(Error::new(ErrorKind::Other, "Invalid token"));
            }
        }
        Ok(())
    }

    pub fn evaluate(&mut self) ->  Result <i32, Error>{
        self.parse_node()?;
        if self.node_stack.len() != 1{
            return Err(Error::new(ErrorKind::Other, "Invalid expression"));
        }
        let root = self.node_stack.pop().unwrap();
        root.evaluate()
    }
}