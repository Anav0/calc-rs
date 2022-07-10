use calc_core::Sheet;
use crossterm::{queue, style, Result};
use std::io::Write;

pub fn draw<W: Write>(stdout: &mut W, sheets: Vec<&mut Sheet>) -> Result<()> {
    queue!(stdout, style::Print("HELLO"))?;

    draw_sheet(stdout, sheets[0])?;

    Ok(())
}

fn draw_sheet<W: Write>(stdout: &mut W, sheet: &Sheet) -> Result<()> {
    Ok(())
}
