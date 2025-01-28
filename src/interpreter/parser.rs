use std::{io::{Error, ErrorKind}, collections::HashMap, rc::Rc, cell::RefCell};
use crate::interpreter::{lexer::*, nodes::*};

pub struct Parser{
    pub tokens: Vec<Token>,
    token_count: usize,
    pos: usize,
    paren_stack: Vec<Token>,
    node_stack: Vec<Box<dyn Node>>,
    pub vars: HashMap<String, Rc<RefCell<Option<f64>>>>
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
        // delete any unitialized variables 
        let symbols: Vec<&String> = self.vars.keys().collect();
        let mut new_map: HashMap<String, Rc<RefCell<Option<f64>>>> = HashMap::new();
        for i in symbols{
          if self.vars[i].borrow().is_some(){
            new_map.insert(i.clone(), self.vars[i].clone());
          }
        }
        self.vars = new_map;
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
                        return  Err(Error::new(ErrorKind::Other, "Invalid brackets"));
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
                    // determine if the name is associated with a defined variable
                    if self.vars.contains_key(name){
                        let var_node = VarNode::new(name.clone(), self.vars[name].clone());
                        self.node_stack.push(Box::new(var_node));
                        self.pos += 1;
                    }
                    else{
                        // if the node is not a defined variable, insert is a leaf SymNode
                        self.node_stack.push(Box::new(SymNode::new(name.clone())));
                        self.pos += 1;
                        return Ok(());
                    }
                }
                Token::Break => {
                    // this clears the node stack, essentially preparing the parser for a new statement
                    self.node_stack.clear();
                    self.pos += 1;
                }
                Token::DecVar => {
                    let init_len = self.node_stack.len();
                    self.pos += 1;
                    self.parse()?;
                    // check that exactly one more node was added
                    if self.node_stack.len() != (init_len + 1){
                        return Err(Error::new(ErrorKind::Other, "Invalid variable declaration"));
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
                        return Err(Error::new(ErrorKind::Other, "Incomplete Expression (5)"));
                    }
                    // assign lhs to rhs
                    let mut new_node = AsgnNode::new();
                    new_node.right = self.node_stack.pop();
                    new_node.left = self.node_stack.pop();
                    self.node_stack.push(Box::new(new_node));
                }
            }
        }
        Ok(())
    }  
    // returns if the parser is in a state that would indicate a valid expression was generated (exactly one node left on the stack and no open block)
    pub fn validate(&self) -> bool{
        self.node_stack.len() == 1 && self.paren_stack.len() == 0
    }
    // returns the top node of the stack (the root of an AST if an expression was just parsed)
    pub fn get_tree(&mut self) -> Option<Box<dyn Node>>{
        let ret = self.node_stack.pop();
        ret
    }
}  
