use std::collections::HashMap;


#[derive(Debug)]
pub enum Token{
    Keywords(String),
    Identifier(String),
    Interger(String),
    Float(String),
    Op(char),
    Punctuator(char),
    Semicolon(char),
    Uknown(char),
    Eof,
}


pub struct Lexer{
    input: String,
    position: usize,
    keywords: HashMap<&'static str, Token>
}

impl Lexer{
    pub fn new(input: String) -> Self{
        Lexer { input: input, position: usize::default() , keywords: HashMap::new() }
    }

    pub fn run(&mut self) -> Vec<Token>{
        self.init_keywords();
        let mut tvec: Vec<Token> = Vec::new();

        while self.position < self.input.len(){
            let c = char::from(self.input.as_bytes()[self.position]);

            if c.is_ascii_whitespace() {
                self.position+=1;
                continue;
            }

            if c.is_alphabetic() {
                if let Some(word) = self.get_next_word(){
                    if self.keywords.contains_key(&word as &str) {
                        tvec.push(Token::Keywords(word));
                    }else{
                        tvec.push(Token::Identifier(word));
                    }
                }else{
                    tvec.push(Token::Eof);
                }
            }else if c.is_ascii_digit(){
                if let Some(num) = self.get_next_number(){
                    if num.contains("."){
                        tvec.push(Token::Float(num));
                    }else{
                        tvec.push(Token::Interger(num));
                    }
                }else{
                    tvec.push(Token::Eof);
                }
            }else if c == '+' || c == '-' ||
                    c == '*' || c == '/' {
                    tvec.push(Token::Op(c));
                    self.position+=1;
            }else if c == '{' || c == '}' ||
                     c == '(' || c == ')'
                {
                    tvec.push(Token::Punctuator(c));
                    self.position+=1;
                }else if c == ';'{
                    tvec.push(Token::Semicolon(c));
                    self.position+=1;
                }else{
                tvec.push(Token::Uknown(c));
                self.position+=1;
            }
        }
        tvec
    }

    fn get_next_word(&mut self) -> Option<String> {
        let start = self.position;

        while self.position < self.input.len() {
            let c = self.input[self.position..].chars().next()?;
            if c.is_alphanumeric() {
                self.position += c.len_utf8(); // advance by char length
            } else {
                break;
            }
        }

        Some(self.input[start..self.position].to_string())
    }
    fn get_next_number(&mut self) -> Option<String> {
        let start = self.position;
        let mut has_decimal = false;

        while self.position < self.input.len()
            && (self.input[self.position..].chars().next()?.is_ascii_digit()
                || self.input[self.position..].chars().next()? == '.')
        {
            let c = self.input[self.position..].chars().next()?;

            if c == '.' {
                if has_decimal {
                    break;
                }
                has_decimal = true;
            }

        self.position += c.len_utf8();
    }

    Some(self.input[start..self.position].to_string())
}
    fn init_keywords(&mut self){
        self.keywords.insert("int", Token::Keywords(String::from("int")));
        self.keywords.insert("return", Token::Keywords(String::from("return")));
    }
}