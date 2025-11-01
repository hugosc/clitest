use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode},
    prelude::Widget,
    widgets::Paragraph,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    run(&mut terminal)?;
    ratatui::restore();
    Ok(())
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame))?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Esc {
                break;
            }
        }
    }
    Ok(())
}

fn render(frame: &mut ratatui::Frame) {
    Paragraph::new("Hello, Ratatui!").render(frame.area(), frame.buffer_mut());
}
