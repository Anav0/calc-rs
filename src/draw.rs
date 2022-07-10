use crossterm::{queue, style, Result};
use std::io::Write;

pub fn draw<W: Write>(stdout: &mut W) -> Result<()> {
    queue!(stdout, style::Print("HELLO"))?;

    Ok(())
}
