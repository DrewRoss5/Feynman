use std::{io::{Error, ErrorKind}, collections::HashMap, rc::Rc, cell::RefCell};
use crate::interpreter::{lexer::*, nodes::*};

pub struct Parser{
    pub tokens: Vec<Token>,
    token_count: usize,
    pos: usize,
    paren_stack: Vec<Token>,
    pub node_stack: Vec<Box<dyn Node>>,
    pub vars: HashMap<String, Rc<RefCell<Option<i32>>>>
}
impl Parser{
    pub fn new(tokens: Vec<Token>) -> Parser{
        let token_count = tokens.len();
        Parser{tokens, token_count, node_stack: Vec::new(), pos: 0, paren_stack: Vec::new(), vars: HashMap::new()}
    }
    pub fn set_tokens(&mut self, tokens: Vec<Token>){
        self.pos = 0;
        self.tokens = tokens;
        self.token_count = self.tokens.len();
        // clear out the internal stacks
        self.node_stack.clear(); 
        self.paren_stack.clear();
    }
    pub fn parse(&mut self) -> Result<(), Error>{
        while self.pos < self.token_count{
            match &self.tokens[self.pos]{
                Token::Add | Token::Sub | Token::Mul | Token::Div => {
                    let mut operator = OperatorNode::new(self.tokens[self.pos].clone());
                    self.pos += 1;
                    self.parse()?;
                    if self.node_stack.len() < 2{
                        return Err(Error::new(ErrorKind::Other, "Incomplete Expression (2)"));
                    }
                    let right_val = self.node_stack.pop().unwrap();
                    let left_val = self.node_stack.pop().unwrap();  
                    operator.left = Some(left_val);
                    operator.right = Some(right_val);
                    self.node_stack.push(Box::new(operator));
                }
                Token::OpenBlock => {
                    let mut block_node = Block::new();
                    self.pos += 1;
                     // get the current number of parenthesis on the stack
                    self.paren_stack.push(Token::OpenBlock);
                    self.parse()?;
                    // ensure a body for the parenthesis was parsed
                    if self.node_stack.len() == 0{
                        return  Err(Error::new(ErrorKind::Other, "Invalid brackets"));
                    }
                    block_node.body = self.node_stack.pop();
                    self.node_stack.push(Box::new(block_node));    
                }
                Token::CloseBlock => {
                    // ensure that the last seen parenthesis was an opening parenthesis
                    if self.paren_stack.len() == 0{
                        return  Err(Error::new(ErrorKind::Other, "Invalid Blockthesis"));
                    }
                    match self.paren_stack.pop().unwrap(){
                        Token::OpenBlock => {
                            self.pos += 1;
                            return  Ok(());
                        }
                        _ => {return  Err(Error::new(ErrorKind::Other, "Opening parenthesis does not match closing"));}
                    }
                }
                Token::Int(val )=> {
                    self.node_stack.push(Box::new(NumNode::new(val.clone())));
                    self.pos += 1
                }
                Token::Sym(name) => {
                    self.pos += 1;
                    // determine if the name is associated with a defined variable
                    if self.vars.contains_key(name){
                        let var_node = VarNode::new(name.clone(), self.vars[name].clone());
                        self.node_stack.push(Box::new(var_node));
                    }
                    else{
                        // if the node is not a defined variable, insert is a leaf SymNode
                        self.node_stack.push(Box::new(SymNode::new(name.clone())));
                        return Ok(());
                    }
                }
                Token::DecVar => {
                    self.pos += 1;
                    self.parse()?;
                    if self.node_stack.len() < 1{
                        return Err(Error::new(ErrorKind::Other, "Incomplete Expression (4)"));
                    }
                    let sym = self.node_stack.pop().unwrap();
                    match sym.node_type() {
                        NodeType::Sym => {
                            // create a new variable
                            let ptr = Rc::new(RefCell::new(None));
                            self.vars.insert(sym.text(), ptr.clone());
                            // push the new variable on the node stack to be next thign evaluated
                            self.node_stack.push(Box::new(VarNode::new(sym.text(), ptr.clone())))
                        
                        }
                        _ => {return Err(Error::new(ErrorKind::Other, "Cannot assign expression to a variable"))}
                        
                    }
                }
                Token::Asgn => {
                    self.pos += 1;
                    // read the right hand side of the expression
                    self.parse()?;
                    if self.node_stack.len() < 2{
                        return Err(Error::new(ErrorKind::Other, "Incomplete Expression (4)"));
                    }
                    // assign lhs to rhs
                    let rhs = self.node_stack.pop().unwrap();
                    let lhs = self.node_stack.pop().unwrap();
                    lhs.assign(rhs)?;
                    // push the lhs back onto the stack
                    self.node_stack.push(lhs);
                }
            }
        }
        if self.paren_stack.len() != 0{
            Err(Error::new(ErrorKind::Other, "Incomplete expression (2)"))
        }
        else{
            Ok(())
        }
    }        
}  
