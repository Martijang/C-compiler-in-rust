use anyhow::{Ok, Result, anyhow};
use crate::lexer::{Lexer,Token};

#[derive(PartialEq, Eq, PartialOrd, Debug)]
enum IrTok{
    I32,
    Ident(String),
    F32,
    Return,
    Delimiter,
    Op(char, u8), //u8 is a binding power
    End,
}

pub struct Parser{
    ir_vec: Vec<IrTok>,
}

impl Parser {
    pub fn new() -> Self{
        Parser { ir_vec: Vec::new() }
    }
    
    pub fn parse_expression(&mut self,lexer: &mut Lexer) -> Result<()>{
        match lexer.next(){
            Token::Keywords(k) => {
                let str = k.as_str();
                match str {
                    "int" => self.ir_vec.push(IrTok::I32),
                    "return" => self.ir_vec.push(IrTok::Return),
                    "float" => self.ir_vec.push(IrTok::F32),
                    _ => return Err(anyhow!("Unknown keywords: {str}"))
                }
                self.expect_identifier()?;
                self.parse_expression(lexer)
            },
            Token::Identifier(id) => {
                self.ir_vec.push(IrTok::Ident(id));
                self.parse_expression(lexer)
            }
            Token::Op(op) => {
                match op{
                    '+' => self.ir_vec.push(IrTok::Op(op, 1)),
                    '-' => self.ir_vec.push(IrTok::Op(op, 1)),
                    '*' => self.ir_vec.push(IrTok::Op(op, 2)),
                    '/' => self.ir_vec.push(IrTok::Op(op, 2)),
                    _ => return Err(anyhow!("UNKNOWN Opeartor found: {op}"))
                }
                self.parse_expression(lexer)
            }
            Token::Eof => {self.ir_vec.push(IrTok::End); Ok(())},
            t => return Err(anyhow!("UNKNOWN token found: {t:?}"))
        }?;
        Ok(())
    }
    fn expect_identifier(&mut self) -> Result<()> {
        if let Some(t) = self.ir_vec.last() {
            if !matches!(t, IrTok::Ident(_)) {
                return Err(anyhow!("Expected identifier, found: {t:?}"));
            }
        }
        Ok(())
    }

}