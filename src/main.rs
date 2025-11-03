// Import the Result type from `color_eyre`. This is a custom Result that provides
// better error messages with backtraces when something goes wrong.
use color_eyre::eyre::Result;

// Import types and functions from the local `fruitdata` crate
use fruitdata::{initialise_fruit_catalogue, load_catalogue, save_catalogue};

// Import UI and app modules
mod app;
mod error;
mod ui;

use app::{AppEvent, AppState};
use app::state::AppMode;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyModifiers};

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
        return Err(e);
    }

    // If the app exited normally, restore the terminal
    ratatui::restore();

    Ok(())
}

// The run() function contains the main application logic and event loop
fn run(terminal: &mut ratatui::DefaultTerminal) -> Result<()> {
    // Load the list of fruits from "fruits.json"
    let fruits = load_catalogue("fruits.json").unwrap_or_else(|_| initialise_fruit_catalogue());

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
            // Check for Ctrl+S to save first (independent of mode)
            if key.code == KeyCode::Char('s') && key.modifiers.contains(KeyModifiers::CONTROL) {
                match save_catalogue(&state.fruits, "fruits.json") {
                    Ok(_) => {
                        state.dirty = false;
                        // Silent success - don't show message that would block quit
                    }
                    Err(e) => {
                        state.set_error(format!("Failed to save: {}", e));
                    }
                }
                continue;
            }

            // In modal/filter modes, let them handle all keys including q/Esc
            if state.mode != AppMode::Normal {
                app::handle_event(&mut state, AppEvent::KeyPress(key))?;
                continue;
            }

            // In Normal mode, handle q/Esc specially for quit logic
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
                    // Process other key events through the event handler
                    app::handle_event(&mut state, AppEvent::KeyPress(key))?;
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
