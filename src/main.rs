use anyhow::{Ok, Result};

//will be continue one day

mod lexer;
mod parser;
mod app;
use app::App;

fn main() -> Result<()>{
    App::new().run()?;
    Ok(())
}