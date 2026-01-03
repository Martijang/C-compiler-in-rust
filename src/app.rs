use clap::Parser;
use anyhow::{Ok, Result, anyhow};
use std::{fs, path::Path};

#[path ="lexer.rs"]
mod lexer;
use lexer::Lexer;

#[path ="lexer.rs"]
mod parser;

/// ec lang compiler
#[derive(Parser, Debug)]
#[command(about, long_about = None)]
struct Cli{
    ///path of the input
    input_file: String,
}

pub struct App{
    input_file: String,
}

impl App{
    pub fn new() -> Self{
        let cli = Cli::parse();
        App { 
            input_file: cli.input_file 
        }
    }

    pub fn run(&mut self) -> Result<()>{
        let content = self.get_input_file()?;
        let mut lexer = Lexer::new(content);
        let tvec = lexer.run();
        for t in tvec{
            println!("{t:?}")
        }
        Ok(())
    }

    fn get_input_file(&self) -> Result<String>{
        let path = Path::new(&self.input_file);
        if let Some(extension) = path.extension(){
            if extension == "c" {
                Ok(fs::read_to_string(&self.input_file)?)
            }else{
                Err(anyhow!("extension of the input file must be .c"))
            }
        }else{
            Err(anyhow!("cannot find any file extension"))
        }
    }
}