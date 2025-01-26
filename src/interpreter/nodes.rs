use std::{io::{Error, ErrorKind}, rc::Rc, cell::RefCell};
use crate::interpreter::lexer::*;

pub enum NodeType{
    Num,
    Op, 
    Sym,
    Var,
    Block,
}

// this is a root trait for every type of the node in the tree. Because this a calculator, we assume every type of node can eventually be resolved to a number 
pub trait Node{
    fn evaluate(&self) -> Result<i32, Error>;
    fn node_type(&self) -> NodeType;
    fn text(&self) -> String;
    fn assign(&self, val: Box<dyn Node>) -> Result<(), Error>;
}

// this is the simplest type of node, it only represents a literal number, and has no children
pub struct NumNode{
    val: i32
}
impl NumNode{
    pub fn new(val: i32) -> NumNode {NumNode{val}}
}
impl Node for NumNode{
    fn evaluate(&self) -> Result<i32, Error>{
        Ok(self.val)
    }
    fn node_type(&self) -> NodeType {
        NodeType::Num
    }
    fn text(&self) -> String {
        self.val.to_string()
    }
    fn assign(&self, val: Box<dyn Node>) -> Result<(), Error> {
        return Err(Error::new(ErrorKind::Other, "Cannot assign to literal expression"));
    }
}

// this node represents any binary operator, it evaluates by performing the specified operation on its two children
pub struct OperatorNode{
    operator: Token,
    pub left: Option<Box<dyn Node>>,
    pub right: Option<Box<dyn Node>>,
}
impl OperatorNode{
    pub fn new(operator: Token) -> OperatorNode{
        OperatorNode{operator, left: None, right: None}
    }
}
impl Node for OperatorNode{
    fn evaluate(&self) -> Result<i32, Error>{
        if self.left.is_none() || self.right.is_none(){
            Err(Error::new(ErrorKind::Other, "Incomplete expression (1)"))
        }
        else{
            let left_val = self.left.as_ref().unwrap().evaluate()?;
            let right_val = self.right.as_ref().unwrap().evaluate()?;
            match self.operator {
                Token::Add => {Ok(left_val + right_val)}
                Token::Sub => {Ok(left_val - right_val)}
                Token::Mul => {Ok(left_val * right_val)}
                Token::Div => {Ok(left_val / right_val)}
                _ => {Err(Error::new(ErrorKind::Other, "If this error appears, something has gone terribly wrong"))}
            }
        }
    }
    fn node_type(&self) -> NodeType {
        NodeType::Op
    }
    fn text(&self) -> String {
        if self.left.is_none() || self.right.is_none(){
            return "INVALID_EXPR".to_string();
        }
        let op_char = match self.operator {
            Token::Add => {"+".to_string()}
            Token::Sub => {"-".to_string()}
            Token::Mul => {"*".to_string()}
            Token::Div => {"/".to_string()}
            _ => {String::new()}
        };
        format!("{} {} {}", self.left.as_ref().unwrap().text(), op_char, self.right.as_ref().unwrap().text())
    }

    fn assign(&self, val: Box<dyn Node>) -> Result<(), Error> {
        Err(Error::new(ErrorKind::Other, "Cannot assign to operator"))
    }
}

// this node represents the name of a  user-defined symbol, such as a function or a variable
pub struct SymNode{
    pub sym: String
}
impl SymNode{
    pub fn new(sym: String) -> SymNode{
        SymNode{sym}
    }
}
impl Node for SymNode{
    fn evaluate(&self) -> Result<i32, Error>{
        Err(Error::new(ErrorKind::Other, format!("The value \"{}\" is undefined", self.sym)))
    }
    fn node_type(&self) -> NodeType {
        NodeType::Sym
    }
    fn text(&self) -> String {
        self.sym.clone()
    }
    fn assign(&self, val: Box<dyn Node>) -> Result<(), Error> {
        Err(Error::new(ErrorKind::Other, format!("Cannot assign to unitialized value \"{}\"", &self.sym)))
    }
}

// this node holds a smart pointer to a user defined variable
pub struct VarNode{
    name: String, 
    ptr: Rc<RefCell<Option<i32>>>
}
impl VarNode{
    pub fn new(name: String, ptr: Rc<RefCell<Option<i32>>>) -> VarNode{
        VarNode{name, ptr}
    }
}
impl Node for VarNode{
    fn evaluate(&self) -> Result<i32, Error>{
        let val = self.ptr.borrow();
        if val.is_none(){
            return Err(Error::new(ErrorKind::Other, format!("The variable {} has not been initalized", self.name) ));
        }
        let val = self.ptr.borrow();
        Ok(val.unwrap())
    }
    fn node_type(&self) -> NodeType {
        NodeType::Var
    }
    fn text(&self) -> String {
        self.name.clone()
    }
    fn assign(&self, val: Box<dyn Node>) -> Result<(), Error> {
        let new_val = val.evaluate()?;
        self.ptr.replace(Some(new_val));
        Ok(())
    }
}

// this node is used to define a "block" to be evaluated in brackets
pub struct Block{
    pub body: Option<Box<dyn Node>>
}
impl Block{
    pub fn new() -> Block{
        Block { body: None }
    }
}
impl Node for Block{
    fn evaluate(&self) -> Result<i32, Error>{
        if self.body.is_none(){
            return Err(Error::new(ErrorKind::Other, "Invalid parenthesis"));
        }
        self.body.as_ref().unwrap().evaluate()
    }
    fn node_type(&self) -> NodeType {
        NodeType::Block
    }
    fn text(&self) -> String {
        let mut body_str = String::new();
        match  &self.body {
            Some(body_node) => {body_str = body_node.text()}
            None => {body_str = " ".to_string()}
        }
        format!("[{}]", body_str)
    }
    fn assign(&self, val: Box<dyn Node>) -> Result<(), Error> {
        Err(Error::new(ErrorKind::Other, "Cannot assign to block literal"))
    }
}
