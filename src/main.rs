use std::error::Error;

use crate::manipulator::shell;

mod logic;
mod manipulator;

fn main() -> Result<(), Box<dyn Error>> {
    shell()?;
    Ok(())
}