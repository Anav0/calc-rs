use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    cursor,
    event::{poll, read, Event, KeyCode, MouseEvent},
    execute,
    terminal::ClearType,
    Result as CrossResult,
};
use std::result::Result as StdResult;

use crossterm::{event::KeyEvent, terminal};
use draw::draw;

mod draw;

fn handle_key<W: Write>(stdout: &mut W, e: KeyEvent) -> CrossResult<bool> {
    match e.code {
        KeyCode::Esc => {
            execute!(
                stdout,
                terminal::Clear(ClearType::All),
                terminal::LeaveAlternateScreen,
                cursor::Show,
            )?;
            terminal::disable_raw_mode().unwrap();

            return Ok(true);
        }
        _ => Ok(false),
    }
}

fn handle_mouse<W: Write>(stdout: &mut W, e: MouseEvent) -> CrossResult<bool> {
    Ok(false)
}

fn handle_resize<W: Write>(stdout: &mut W, height: u16, width: u16) -> CrossResult<bool> {
    Ok(false)
}

fn main() -> StdResult<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();

    execute!(&mut stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    let size = terminal::size()?;

    loop {
        if poll(Duration::from_millis(16))? {
            let exit = match read()? {
                Event::Key(event) => handle_key(&mut stdout, event),
                Event::Mouse(event) => handle_mouse(&mut stdout, event),
                Event::Resize(width, height) => handle_resize(&mut stdout, width, height),
            };

            if exit.is_err() {
                //TODO: handle error
            };

            if exit.unwrap() {
                break;
            }

            draw(&mut stdout)?;

            stdout.flush()?;
        }
    }

    Ok(())
}
