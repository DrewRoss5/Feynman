use std::io::{Error, ErrorKind};

pub enum Token{
    Add,
    Sub,
    Mul,
    Div,
    Asgn, 
    OpenBlock, 
    CloseBlock,
    DecVar,
    Break,
    Int(f64),
    Sym(String),
}
impl Clone for Token{
    fn clone(&self) -> Token{
        match self{
            Token::Add => {Token::Add}
            Token::Sub => {Token::Sub}
            Token::Mul => {Token::Mul}
            Token::Div => {Token::Div}
            Token::Asgn => {Token::Asgn}
            Token::OpenBlock => {Token::OpenBlock}
            Token::CloseBlock => {Token::OpenBlock}
            Token::DecVar => {Token::DecVar}
            Token::Break => {Token::Break}
            Token::Int(val) => {Token::Int(val.clone())}
            Token::Sym(name) => {Token::Sym(name.to_string())}
        }
    }
}

pub fn tokenize(expr: String) -> Result<Vec<Token>, Error>{
    let chars: Vec<char> = expr.chars().collect();
    let mut str_pos: usize = 0;
    let mut tokens: Vec<Token> = Vec::new();
    
    while str_pos < chars.len(){
        let mut char = chars[str_pos];
        let mut radix_encountered = false;
        if '0' <= char && char <= '9'{
            let mut num_str = String::new();
            while ('0' <= char && char <= '9') || char == '.'{
                // if the character is a radix, ensure only one is encountered
                if char == '.'{
                    if radix_encountered{
                        return Err(Error::new(ErrorKind::Other, "Invalid syntax"));
                    }
                    radix_encountered = true;
                }
                // append the digit to the number
                num_str.push(char);
                str_pos += 1;
                if str_pos == chars.len(){
                    break;
                }
                char = chars[str_pos];
            }
            let num: f64 = num_str.parse().unwrap();
            tokens.push(Token::Int(num));
        }
        else{
            match char {
                '+' => {tokens.push(Token::Add)}
                '-' => {tokens.push(Token::Sub)}
                '*' => {tokens.push(Token::Mul)}
                '/' => {tokens.push(Token::Div)}
                '=' => {tokens.push(Token::Asgn)}
                '[' => {tokens.push(Token::OpenBlock)}
                ']' => {tokens.push(Token::CloseBlock)}
                ';' => {tokens.push(Token::Break)}
                ' ' => {}
                _ => {
                   // read the token to either the end of the line or the next space
                   let mut token = String::new();
                   while !"+-*/=[] ".contains(char){
                        token.push(char);
                        str_pos += 1;
                        if str_pos == chars.len(){
                            break;
                        }
                        char = chars[str_pos];
                   }
                   match token.as_str() {
                       "let" => {
                            tokens.push(Token::DecVar)
                       }
                       _ => {
                            // we assume any unrecognized token is a user defined symbol
                            tokens.push(Token::Sym(token))
                       }
                   }
                }
            }
            str_pos += 1
        }
    }
    Ok(tokens)
}
