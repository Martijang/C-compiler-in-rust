use anyhow::{Ok, Result};

mod app;
use app::App;

fn main() -> Result<()>{
    App::new().run()?;
    Ok(())
}
