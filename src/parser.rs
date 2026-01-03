use anyhow::{Ok, Result, anyhow};
use crate::lexer::{Lexer,Token};

enum IrTok{

}

pub struct Parser{
    ir_vec: Vec<IrTok>,
}
impl Parser {
    pub fn new() -> Self{
        Parser { ir_vec: Vec::new() }
    }
    
    pub fn parse_expression(lexer: &mut Lexer) -> Result<()>{
        let _lhs = match lexer.next(){
            Token::Keywords(k) => todo!(),
            t => return Err(anyhow!("UNKNOWN token found: {t:?}"))
        };
        Ok(())
    }
}