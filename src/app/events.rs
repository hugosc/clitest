use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crate::error::Result;
use super::state::{AppState, AppMode};

/// Represents an event that can change the application state
#[derive(Debug, Clone)]
pub enum AppEvent {
    /// User pressed a key
    KeyPress(KeyEvent),
    /// Quit the application
    Quit,
}

/// Handle keyboard input and update state accordingly
pub fn handle_event(state: &mut AppState, event: AppEvent) -> Result<bool> {
    match event {
        AppEvent::Quit => return Ok(true),
        AppEvent::KeyPress(key) => handle_key_press(state, key)?,
    }
    Ok(false)
}

fn handle_key_press(state: &mut AppState, key: KeyEvent) -> Result<()> {
    match state.mode {
        AppMode::Normal => handle_normal_mode(state, key)?,
        AppMode::Filter => handle_filter_mode(state, key)?,
        AppMode::ConfirmDelete => handle_delete_confirm(state, key)?,
        AppMode::AddFruit => handle_add_fruit_modal(state, key)?,
        AppMode::EditFruit => handle_edit_fruit_modal(state, key)?,
        AppMode::Help => handle_help_modal(state, key)?,
    }
    Ok(())
}

fn handle_normal_mode(state: &mut AppState, key: KeyEvent) -> Result<()> {
    // Check for Ctrl+S to save
    if key.code == KeyCode::Char('s') && key.modifiers.contains(KeyModifiers::CONTROL) {
        // Save will be handled in main.rs
        return Ok(());
    }

    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => {
            if state.dirty {
                state.set_error("Unsaved changes! Press Ctrl+S to save or press 'q' again to discard".to_string());
            } else {
                return Ok(()); // Will be caught by main loop
            }
        }
        KeyCode::Up | KeyCode::Char('k') => state.select_previous(),
        KeyCode::Down | KeyCode::Char('j') => state.select_next(),
        KeyCode::Char('/') => state.mode = AppMode::Filter,
        KeyCode::Char('a') => {
            state.modal = Some(crate::ui::modal::ModalState::new());
            state.mode = AppMode::AddFruit;
        }
        KeyCode::Char('e') => {
            if let Some(fruit) = state.selected_fruit() {
                state.modal = Some(crate::ui::modal::ModalState::from_fruit(fruit));
                state.mode = AppMode::EditFruit;
            }
        }
        KeyCode::Char('d') => state.mode = AppMode::ConfirmDelete,
        KeyCode::Char('?') => {
            state.mode = AppMode::Help;
        }
        _ => {}
    }
    Ok(())
}

fn handle_filter_mode(state: &mut AppState, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc => {
            state.mode = AppMode::Normal;
            state.clear_filter();
        }
        KeyCode::Backspace => {
            state.filter_query.pop();
            let query = state.filter_query.clone();
            state.update_filter(&query);
        }
        KeyCode::Char(c) => {
            state.filter_query.push(c);
            let query = state.filter_query.clone();
            state.update_filter(&query);
        }
        KeyCode::Enter => {
            state.mode = AppMode::Normal;
        }
        _ => {}
    }
    Ok(())
}

fn handle_delete_confirm(state: &mut AppState, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Char('y') => {
            if let Some(idx) = state.selected_fruit_index() {
                state.delete_fruit(idx)?;
                state.clear_error();
            }
            state.mode = AppMode::Normal;
        }
        KeyCode::Char('n') | KeyCode::Esc => {
            state.mode = AppMode::Normal;
            state.clear_error();
        }
        _ => {}
    }
    Ok(())
}

fn handle_add_fruit_modal(state: &mut AppState, key: KeyEvent) -> Result<()> {
    if let Some(modal) = &mut state.modal {
        match key.code {
            // Handle close commands first (before character input)
            KeyCode::Esc | KeyCode::Char('q') => {
                state.modal = None;
                state.mode = AppMode::Normal;
            }
            KeyCode::Tab => modal.next_field(),
            KeyCode::BackTab => modal.prev_field(),
            KeyCode::Backspace => modal.backspace(),
            KeyCode::Char(c) => modal.insert_char(c),
            KeyCode::Enter => {
                match modal.validate_and_build() {
                    Ok(fruit) => {
                        state.add_fruit(fruit)?;
                        state.modal = None;
                        state.mode = AppMode::Normal;
                    }
                    Err(_) => {
                        // Keep modal open with error showing
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn handle_edit_fruit_modal(state: &mut AppState, key: KeyEvent) -> Result<()> {
    if let Some(modal) = &mut state.modal {
        match key.code {
            // Handle close commands first (before character input)
            KeyCode::Esc | KeyCode::Char('q') => {
                state.modal = None;
                state.mode = AppMode::Normal;
            }
            KeyCode::Tab => modal.next_field(),
            KeyCode::BackTab => modal.prev_field(),
            KeyCode::Backspace => modal.backspace(),
            KeyCode::Char(c) => modal.insert_char(c),
            KeyCode::Enter => {
                match modal.validate_and_build() {
                    Ok(fruit) => {
                        if let Some(idx) = state.selected_fruit_index() {
                            state.update_fruit(idx, fruit)?;
                        }
                        state.modal = None;
                        state.mode = AppMode::Normal;
                    }
                    Err(_) => {
                        // Keep modal open with error showing
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn handle_help_modal(state: &mut AppState, key: KeyEvent) -> Result<()> {
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('?') | KeyCode::Enter => {
            state.mode = AppMode::Normal;
        }
        _ => {}
    }
    Ok(())
}
