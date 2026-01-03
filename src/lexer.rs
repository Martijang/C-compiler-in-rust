use std::collections::HashMap;


#[derive(Debug, PartialEq, Clone)]
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
    keywords: HashMap<&'static str, Token>,
    tvec: Vec<Token>
}

impl Lexer{
    pub fn new(input: String) -> Self{
        Lexer { input: input, position: usize::default() , keywords: HashMap::new(), tvec: Vec::new() }
    }

    pub fn run(&mut self){
        self.init_keywords();

        while self.position < self.input.len(){
            let c = char::from(self.input.as_bytes()[self.position]);

            if c.is_ascii_whitespace() {
                self.position+=1;
                continue;
            }

            if c.is_alphabetic() {
                if let Some(word) = self.get_next_word(){
                    if self.keywords.contains_key(&word as &str) {
                        self.tvec.push(Token::Keywords(word));
                    }else{
                        self.tvec.push(Token::Identifier(word));
                    }
                }
            }else if c.is_ascii_digit(){
                if let Some(num) = self.get_next_number(){
                    if num.contains("."){
                        self.tvec.push(Token::Float(num));
                    }else{
                        self.tvec.push(Token::Interger(num));
                    }
                }
            }else if c == '+' || c == '-' ||
                    c == '*' || c == '/' {
                    self.tvec.push(Token::Op(c));
                    self.position+=1;
            }else if c == '{' || c == '}' ||
                     c == '(' || c == ')'
                {
                    self.tvec.push(Token::Punctuator(c));
                    self.position+=1;
                }else if c == ';'{
                    self.tvec.push(Token::Semicolon(c));
                    self.position+=1;
                }else{
                self.tvec.push(Token::Uknown(c));
                self.position+=1;
            }
        }
        self.tvec.reverse(); //Eof will appear first
        
    }
    pub fn get_tvec(&mut self) -> &Vec<Token>{
        &self.tvec
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

    pub fn peek(&mut self) -> Token{
        self.tvec.last().unwrap_or(&Token::Eof).clone()
    }

    pub fn next(&mut self) -> Token{
        self.tvec.pop().unwrap_or(Token::Eof)
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


#[test]
fn lexer_test(){
    let mut lexer = Lexer::new(String::from("int main(){return 0;}"));
    lexer.run();
    let tok = lexer.get_tvec();
    let mut v = vec![
    Token::Keywords(String::from("int")),
    Token::Identifier(String::from("main")),
    Token::Punctuator('('),
    Token::Punctuator(')'),
    Token::Punctuator('{'),
    Token::Keywords(String::from("return")),
    Token::Interger(String::from("0")),
    Token::Semicolon(';'),
    Token::Punctuator('}')];
    v.reverse();
    assert_eq!(tok, &v)
}