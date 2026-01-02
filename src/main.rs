use anyhow::{Ok, Result};

mod app;
use app::App;

fn main() -> Result<()>{
    App::run()?;
    Ok(())
}
