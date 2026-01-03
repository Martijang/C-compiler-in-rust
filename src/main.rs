use anyhow::{Ok, Result};

mod lexer;
mod parser;
mod app;
use app::App;

fn main() -> Result<()>{
    App::new().run()?;
    Ok(())
}