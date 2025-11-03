// Import the Result type from `color_eyre`. This is a custom Result that provides
// better error messages with backtraces when something goes wrong.
use color_eyre::eyre::Result;

// Import types and functions from the local `fruitdata` crate:
// - FruitDimensions: represents a single fruit's size information (length, width, height)
// - initialise_fruit_catalogue: creates a default list of fruits
// - load_catalogue: reads a list of fruits from a JSON file
use fruitdata::{FruitDimensions, initialise_fruit_catalogue, load_catalogue};

// Import UI components from `ratatui`, a library for building terminal user interfaces (TUIs).
// Think of these like building blocks for creating a fancy text-based interface.
use ratatui::{
    // DefaultTerminal: the main object that controls drawing on the terminal screen
    DefaultTerminal,
    // crossterm::event: handles keyboard input (when the user presses keys)
    // Event: represents something that happened (like a keypress)
    // KeyCode: identifies which key was pressed (Up, Down, 'q', etc.)
    crossterm::event::{self, Event, KeyCode},
    // Layout: helps divide the terminal screen into sections (left panel, right panel, etc.)
    // Constraint: specifies how large each section should be (e.g., 60% width)
    // Direction: determines if sections are arranged horizontally or vertically
    layout::{Constraint, Direction, Layout},
    // Widgets: the visual components we can draw on screen
    // Block: a box with a border and title
    // Borders: creates visual lines around a widget
    // List: displays a scrollable list of items (like the fruit names)
    // ListItem: a single item in a list
    // Paragraph: displays text content
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

// The main() function is where every Rust program starts executing.
// It returns Result<()>, meaning it can either succeed (Ok) or fail (Err) with an error message.
fn main() -> Result<()> {
    // Install the `color_eyre` error handler. This makes error messages more readable
    // and includes a backtrace if something goes wrong. The '?' operator will use this.
    color_eyre::install()?;

    // Initialize the terminal for drawing. This:
    // - Switches to "raw mode" (reads keypresses directly)
    // - Switches to an alternate screen (so we don't overwrite your terminal history)
    // - Returns a mutable terminal object we can draw on
    let mut terminal = ratatui::init();

    // Call the run() function to start the main app loop.
    // If run() returns an error, we catch it with 'if let Err(e)'.
    if let Err(e) = run(&mut terminal) {
        // If an error occurred, restore the terminal to normal mode before showing the error.
        // This prevents leaving the user's terminal in a broken state.
        ratatui::restore();
        // Return the error so it gets printed and the program exits
        return Err(e);
    }

    // If the app exited normally (user pressed q/Esc), restore the terminal
    // so the user can see their normal terminal prompt again.
    ratatui::restore();

    // Return Ok(()) to indicate the program succeeded
    Ok(())
}

// The run() function contains the main application logic and event loop.
// It takes a mutable reference to the terminal so it can draw on it.
fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    // Load the list of fruits from "fruits.json". If the file doesn't exist or fails to load,
    // unwrap_or_else() runs the closure (the |_| { } block) which creates a default list.
    let fruits: Vec<FruitDimensions> =
        load_catalogue("fruits.json").unwrap_or_else(|_| initialise_fruit_catalogue());

    // Initialize variables that track the state of the UI:

    // selected: which fruit in the list is currently highlighted (starts at 0, the first fruit)
    let mut selected: usize = 0;

    // list_state: ratatui's internal state for tracking which list item is selected
    // This is separate from 'selected' because ratatui needs to manage its own state
    let mut list_state = ratatui::widgets::ListState::default();

    // command_buffer: stores characters the user types (not actively used here, but available)
    let mut command_buffer = String::new();

    // If there are fruits in the list, mark the first one (index 0) as selected.
    // Some() wraps the index because ListState::select() expects Option<usize>
    // (it could be None if we want nothing selected)
    if !fruits.is_empty() {
        list_state.select(Some(selected));
    }

    // Main event loop: this loop runs repeatedly, drawing the UI and handling user input.
    loop {
        // terminal.draw() takes a closure (a block of code) that describes what to draw.
        // The 'frame' object is our canvas for drawing on the terminal.
        terminal.draw(|frame| {
            // Split the terminal screen into two sections (left and right):
            // - Left side: 60% of the width (for the fruit list)
            // - Right side: 40% of the width (for detailed info about the selected fruit)
            // margin(1) adds 1 space of padding around all edges
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
                .split(frame.area());

            // Convert the fruits vector into a list of ListItem widgets.
            // .iter() loops through each fruit, .map() transforms each one,
            // and .collect() gathers them into a Vec<ListItem>
            let items: Vec<ListItem> = fruits
                .iter()
                .map(|f| ListItem::new(f.name.clone()))
                .collect();

            // Create the left-side list widget with:
            // - The items (fruit names)
            // - A title and border around the list
            // - A ">> " symbol to highlight the selected item
            let list = List::new(items)
                .block(Block::default().title("Fruits — (Up/Down/j/k: navigate, Enter: none, Esc/q: quit)").borders(Borders::ALL))
                .highlight_symbol(">> ");

            // Render the list widget in the left section (chunks[0]) and update list_state
            // to show which item is currently selected
            frame.render_stateful_widget(list, chunks[0], &mut list_state);

            // Create the right-side details pane.
            // Use an if-else expression to show either:
            // - "No fruits available" if the list is empty
            // - Detailed information about the selected fruit otherwise
            let details = if fruits.is_empty() {
                // Empty case: show a message
                Paragraph::new("No fruits available").block(Block::default().title("Details").borders(Borders::ALL))
            } else {
                // Non-empty case: display info about the currently selected fruit
                let f = &fruits[selected];

                // Format a string with the fruit's information.
                // {:.2} means "print this number with 2 decimal places"
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

            // Render the details widget in the right section (chunks[1])
            frame.render_widget(details, chunks[1]);
        })?;

        // Handle keyboard input.
        // event::read()? blocks and waits for the next keyboard input from the user.
        if let Event::Key(key) = event::read()? {
            // Match checks which key was pressed and reacts accordingly.
            match key.code {
                // Quit the app if user presses Escape or the 'q' key
                // 'break' exits the loop, ending the run() function
                KeyCode::Esc | KeyCode::Char('q') => break,

                // Move selection up if user presses Up arrow or 'k' (vim-style)
                KeyCode::Up | KeyCode::Char('k') => {
                    if selected > 0 {
                        // If we're not already at the top, move up (decrease the index)
                        selected -= 1;
                    } else {
                        // If we're at the top, stay at the top (don't go negative)
                        selected = 0;
                    }
                    // Update the list state so ratatui redraws with the new selection
                    list_state.select(Some(selected));
                }

                // Move selection down if user presses Down arrow or 'j' (vim-style)
                KeyCode::Down | KeyCode::Char('j') => {
                    // Only move down if there are fruits and we're not already at the bottom
                    if !fruits.is_empty() && selected + 1 < fruits.len() {
                        selected += 1;
                        list_state.select(Some(selected));
                    }
                }

                // The ':' key initiates a command (vim-style), so clear any previous input
                KeyCode::Char(':') => {
                    command_buffer.clear();
                }

                // Handle other regular character input
                KeyCode::Char(c) => {
                    // Only add to the buffer if the buffer is already being used (e.g., after ':')
                    if !command_buffer.is_empty() {
                        command_buffer.push(c);
                        // Check if the user typed 'q' to quit
                        if command_buffer == "q" {
                            break;
                        }
                    }
                }

                // Handle backspace: delete the last character from the command buffer
                KeyCode::Backspace => {
                    if !command_buffer.is_empty() {
                        command_buffer.pop();
                    }
                }

                // Clear the command buffer when user presses Enter
                KeyCode::Enter => {
                    command_buffer.clear();
                }

                // Ignore all other keys (like Ctrl, Alt, function keys, etc.)
                _ => {}
            }
        }
    }

    // Return Ok(()) to indicate the run() function succeeded
    Ok(())
}
