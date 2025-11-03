use ratatui::crossterm::event::KeyCode;
use crate::error::Result;
use super::state::{AppState, AppMode};

/// Represents an event that can change the application state
#[derive(Debug, Clone)]
pub enum AppEvent {
    /// User pressed a key
    KeyPress(KeyCode),
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

fn handle_key_press(state: &mut AppState, key: KeyCode) -> Result<()> {
    match state.mode {
        AppMode::Normal => handle_normal_mode(state, key),
        AppMode::Filter => handle_filter_mode(state, key),
        AppMode::ConfirmDelete => handle_delete_confirm(state, key),
        AppMode::AddFruit | AppMode::EditFruit => {
            // These modes are handled by the modal system
            Ok(())
        }
    }
}

fn handle_normal_mode(state: &mut AppState, key: KeyCode) -> Result<()> {
    match key {
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
        KeyCode::Char('a') => state.mode = AppMode::AddFruit,
        KeyCode::Char('e') => state.mode = AppMode::EditFruit,
        KeyCode::Char('d') => state.mode = AppMode::ConfirmDelete,
        KeyCode::Char('?') => {
            // TODO: Show help modal
        }
        _ => {}
    }
    Ok(())
}

fn handle_filter_mode(state: &mut AppState, key: KeyCode) -> Result<()> {
    match key {
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

fn handle_delete_confirm(state: &mut AppState, key: KeyCode) -> Result<()> {
    match key {
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
