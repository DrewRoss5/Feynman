
use std::{io::{Error, ErrorKind}, collections::HashMap};

enum Token{
    Add,
    Sub,
    Mul,
    Div,
    OpenParen, 
    CloseParen,
    Int(i32)
}
impl Clone for Token{
    fn clone(&self) -> Token{
        match self{
            Token::Add => {Token::Add}
            Token::Sub => {Token::Sub}
            Token::Mul => {Token::Mul}
            Token::Div => {Token::Div}
            Token::OpenParen => {Token::OpenParen}
            Token::CloseParen => {Token::OpenParen}
            Token::Int(val) => {Token::Int(val.clone())}
        }
    }
}

fn tokenize(expr: String) -> Result<Vec<Token>, Error>{
    let chars: Vec<char> = expr.chars().collect();
    let mut str_pos: usize = 0;
    let mut tokens: Vec<Token> = Vec::new();
    
    while str_pos < chars.len(){
        let mut char = chars[str_pos];
        if '0' <= char && char <= '9'{
            let mut num_str = String::new();
            while '0' <= char && char <= '9'{
                num_str.push(char);
                str_pos += 1;
                if str_pos == chars.len(){
                    break;
                }
                char = chars[str_pos];
            }
            let num: i32 = num_str.parse().unwrap();
            tokens.push(Token::Int(num));
        }
        else{
            match char {
                '+' => {tokens.push(Token::Add)}
                '-' => {tokens.push(Token::Sub)}
                '*' => {tokens.push(Token::Mul)}
                '/' => {tokens.push(Token::Div)}
                '(' => {tokens.push(Token::OpenParen)}
                ')' => {tokens.push(Token::CloseParen)}
                ' ' => {}
                _ => {
                    return Err(Error::new(ErrorKind::Other, "Unrecognized Token"));
                }
            }
            str_pos += 1
        }
    }
    Ok(tokens)
}

// this is a root trait for every type of the node in the tree. Because this a calculator, we assume every type of node can eventually be resolved to a number 
pub trait Node{
    fn evaluate(&self) -> Result<i32, Error>;
}

// this is the simplest type of node, it only represents a literal number, and has no children
struct NumNode{
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
struct OperatorNode{
    operator: Token,
    left: Option<Box<dyn Node>>,
    right: Option<Box<dyn Node>>,
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

// this node is used to define a "block" to be evaluated in parentheses
struct BlockNode{
    pub body: Option<Box<dyn Node>>
}
impl BlockNode{
    pub fn new() -> BlockNode{
        BlockNode { body: None }
    }
}
impl Node for BlockNode{
    fn evaluate(&self) -> Result<i32, Error>{
        if self.body.is_none(){
            return Err(Error::new(ErrorKind::Other, "Invalid parenthesis"));
        }
        self.body.as_ref().unwrap().evaluate()
    }
}

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

struct Parser{
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
                Token::OpenParen => {
                    let mut block_node = BlockNode::new();
                    self.pos += 1;
                     // get the current number of parenthesis on the stack
                    self.paren_stack.push(Token::OpenParen);
                    self.parse()?;
                    // ensure a body for the parenthesis was parsed
                    if self.node_stack.len() == 0{
                        return  Err(Error::new(ErrorKind::Other, "Invalid Parenthesis"));
                    }
                    block_node.body = self.node_stack.pop();
                    self.node_stack.push(Box::new(block_node));    
                }
                Token::CloseParen => {
                    // ensure that the last seen parenthesis was an opening parenthesis
                    if self.paren_stack.len() == 0{
                        return  Err(Error::new(ErrorKind::Other, "Invalid Parenthesis"));
                    }
                    match self.paren_stack.pop().unwrap(){
                        Token::OpenParen => {
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
