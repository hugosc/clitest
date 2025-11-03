// Import the Result type from `color_eyre`. This is a custom Result that provides
// better error messages with backtraces when something goes wrong.
use color_eyre::eyre::Result;

// Import types and functions from the local `fruitdata` crate
use fruitdata::{initialise_fruit_catalogue, load_catalogue};

// Import UI and app modules
mod app;
mod error;
mod ui;

use app::{AppEvent, AppState};
use ratatui::crossterm::event::{self, Event, KeyCode};

// The main() function is where every Rust program starts executing.
fn main() -> Result<()> {
    // Install the `color_eyre` error handler
    color_eyre::install()?;

    // Initialize the terminal for drawing
    let mut terminal = ratatui::init();

    // Call the run() function to start the main app loop
    if let Err(e) = run(&mut terminal) {
        // If an error occurred, restore the terminal to normal mode before showing the error
        ratatui::restore();
        return Err(e.into());
    }

    // If the app exited normally, restore the terminal
    ratatui::restore();

    Ok(())
}

// The run() function contains the main application logic and event loop
fn run(terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
    // Load the list of fruits from "fruits.json"
    let fruits = load_catalogue("fruits.json")
        .unwrap_or_else(|_| initialise_fruit_catalogue());

    // Initialize the app state
    let mut state = AppState::new(fruits);

    // Main event loop
    loop {
        // Draw the UI
        terminal.draw(|frame| {
            ui::render(frame, &state);
        })?;

        // Handle user input
        if let Event::Key(key) = event::read()? {
            let should_quit = match key.code {
                KeyCode::Char('q') | KeyCode::Esc => {
                    // Only quit if there's no error message (user cleared the error first)
                    if state.error_message.is_none() && !state.dirty {
                        true
                    } else if state.error_message.is_some() {
                        state.clear_error();
                        false
                    } else {
                        // dirty but no error shown yet, show error
                        false
                    }
                }
                _ => {
                    // Process other key events
                    app::handle_event(&mut state, AppEvent::KeyPress(key.code))?;
                    false
                }
            };

            if should_quit {
                break;
            }
        }
    }

    Ok(())
}
