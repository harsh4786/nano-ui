mod app;
use std::{error::Error, f32::consts::E, time::Duration, io};

use app::{run_app, App};
use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen}, execute, event::{EnableMouseCapture, DisableMouseCapture}};
use tui::{backend::{self, CrosstermBackend}, Terminal};
fn main() -> Result<(), Box<dyn Error>>{
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnableMouseCapture, EnterAlternateScreen)?;
    let backend =  CrosstermBackend::new(stdout);   
    let mut terminal = Terminal::new(backend)?;
    let tict_rate =  Duration::from_millis(250);
    let app = App::new();
    let res = run_app(&mut terminal, app, tict_rate);
     disable_raw_mode()?;

     execute!(
        terminal.backend_mut(),
        DisableMouseCapture,
        LeaveAlternateScreen,
     )?;
     terminal.show_cursor()?;
     if let Err(e) = res{
        println!("{:?}", e)
     }
    Ok(())
}
