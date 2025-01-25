use std::io::{Error, ErrorKind};
use crate::interpreter::{lexer::*, nodes::*};

pub struct Parser{
    pub tokens: Vec<Token>,
    token_count: usize,
    pos: usize,
    paren_stack: Vec<Token>,
    pub node_stack: Vec<Box<dyn Node>>
}
impl Parser{
    pub fn new(tokens: Vec<Token>) -> Parser{
        let token_count = tokens.len();
        Parser{tokens, token_count, node_stack: Vec::new(), paren_stack: Vec::new(), pos: 0}
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
            match self.tokens[self.pos]{
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
                        return  Err(Error::new(ErrorKind::Other, "Invalid Blockthesis"));
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
                    self.node_stack.push(Box::new(NumNode::new(val)));
                    self.pos += 1
                }
                Token::Sym(_) => {
                    return Err(Error::new(ErrorKind::Other, "Token: Sym\nNot implemented yet"));
                }
                Token::DecVar => {
                    return Err(Error::new(ErrorKind::Other, "Token: DecVar\nNot implemented yet"));
                }
                Token::Asgn => {
                    return Err(Error::new(ErrorKind::Other, "Token: Asgn\nNot implemented yet"));
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
