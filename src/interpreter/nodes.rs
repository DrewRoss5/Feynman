use std::{io::{Error, ErrorKind}, rc::Rc, cell::RefCell};
use crate::interpreter::lexer::*;

// this is a root trait for every type of the node in the tree. Because this a calculator, we assume every type of node can eventually be resolved to a number 
pub trait Node{
    fn evaluate(&self) -> Result<i32, Error>;
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
}

// this node holds a smart pointer to a user defined variable
pub struct VarNode{
    name: String, 
    ptr: Option<Rc<RefCell<i32>>>
}
impl VarNode{
    pub fn new(name: String) -> VarNode{
        VarNode{name, ptr: None}
    }

    pub fn intialize(&mut self, ptr: Rc<RefCell<i32>>){
        self.ptr = Some(ptr)
    }

    pub fn set_val(&mut self, new_val: i32) -> Result<(), Error>{
        if self.ptr.is_none(){
            return Err(Error::new(ErrorKind::Other, format!("The variable {} has not been initalized", self.name) ));
        }
        let val = self.ptr.as_mut().unwrap();
        val.replace(new_val);
        Ok(())

    }
}
impl Node for VarNode{
    fn evaluate(&self) -> Result<i32, Error>{
        if self.ptr.is_none(){
            return Err(Error::new(ErrorKind::Other, format!("The variable {} has not been initalized", self.name) ));
        }
        let val =self.ptr.as_ref().unwrap().borrow(); 
        Ok(val.clone())
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
}
