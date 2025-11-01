use color_eyre::eyre::Result;
use fruitdata::{FruitDimensions, initialise_fruit_catalogue, load_catalogue};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Direction, Layout},
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    if let Err(e) = run(&mut terminal) {
        // Restore terminal before propagating an error
        ratatui::restore();
        return Err(e);
    }
    ratatui::restore();
    Ok(())
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    // Load fruits (fallback to defaults)
    let fruits: Vec<FruitDimensions> =
        load_catalogue("fruits.json").unwrap_or_else(|_| initialise_fruit_catalogue());

    // UI state
    let mut selected: usize = 0;
    let mut list_state = ratatui::widgets::ListState::default();
    if !fruits.is_empty() {
        list_state.select(Some(selected));
    }

    loop {
        terminal.draw(|frame| {
            // Layout: left = list, right = details
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(frame.area());

            // Build list items
            let items: Vec<ListItem> = fruits
                .iter()
                .map(|f| ListItem::new(f.name.clone()))
                .collect();

            let list = List::new(items)
                .block(Block::default().title("Fruits (Up/Down: navigate, Enter: none, Esc/q: quit)").borders(Borders::ALL))
                .highlight_symbol(">> ");

            frame.render_stateful_widget(list, chunks[0], &mut list_state);

            // Details pane
            let details = if fruits.is_empty() {
                Paragraph::new("No fruits available").block(Block::default().title("Details").borders(Borders::ALL))
            } else {
                let f = &fruits[selected];
                let txt = format!(
                    "Name: {}\n\nDimensions:\n  Length: {}\n  Width : {}\n  Height: {}\n\nVolume: {:.2}",
                    f.name,
                    f.length,
                    f.width,
                    f.height,
                    f.volume()
                );
                Paragraph::new(txt).block(Block::default().title("Details").borders(Borders::ALL))
            };
            frame.render_widget(details, chunks[1]);
        })?;

        // Event handling
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => break,
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    } else {
                        selected = 0;
                    }
                    list_state.select(Some(selected));
                }
                KeyCode::Down => {
                    if !fruits.is_empty() && selected + 1 < fruits.len() {
                        selected += 1;
                        list_state.select(Some(selected));
                    }
                }
                KeyCode::Enter => {
                    // For this learning demo, Enter does nothing special — details show on right.
                    // You could extend this to open a modal or trigger an action.
                }
                _ => {}
            }
        }
    }

    Ok(())
}
