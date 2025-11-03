use fruitdata::FruitDimensions;
use crate::error::Result;
use crate::ui::modal::ModalState;

/// Represents the current application mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppMode {
    /// Normal browsing/navigation mode
    Normal,
    /// Adding a new fruit
    AddFruit,
    /// Editing an existing fruit
    EditFruit,
    /// Filtering/searching fruits
    Filter,
    /// Confirming a delete action
    ConfirmDelete,
    /// Showing help modal
    Help,
}

/// Main application state
#[derive(Debug, Clone)]
pub struct AppState {
    /// All fruits in the catalogue
    pub fruits: Vec<FruitDimensions>,
    /// Index of currently selected fruit
    pub selected_index: usize,
    /// Current application mode
    pub mode: AppMode,
    /// Dirty flag: has data been modified but not saved?
    pub dirty: bool,
    /// Filter query for search
    pub filter_query: String,
    /// Filtered fruit indices (when filtering)
    pub filtered_indices: Vec<usize>,
    /// Error message to display
    pub error_message: Option<String>,
    /// Modal state for add/edit operations
    pub modal: Option<ModalState>,
}

impl AppState {
    /// Create a new app state with the given fruits
    pub fn new(fruits: Vec<FruitDimensions>) -> Self {
        let filtered_indices = (0..fruits.len()).collect();
        Self {
            fruits,
            selected_index: 0,
            mode: AppMode::Normal,
            dirty: false,
            filter_query: String::new(),
            filtered_indices,
            error_message: None,
            modal: None,
        }
    }

    /// Get the currently selected fruit
    pub fn selected_fruit(&self) -> Option<&FruitDimensions> {
        if self.is_filtering() {
            self.filtered_indices.get(self.selected_index).and_then(|&i| self.fruits.get(i))
        } else {
            self.fruits.get(self.selected_index)
        }
    }

    /// Get the actual index of the selected fruit in the main fruits vec
    pub fn selected_fruit_index(&self) -> Option<usize> {
        if self.is_filtering() {
            self.filtered_indices.get(self.selected_index).copied()
        } else {
            Some(self.selected_index)
        }
    }

    /// Get the display list (either all fruits or filtered)
    pub fn display_fruits(&self) -> Vec<&FruitDimensions> {
        if self.is_filtering() {
            self.filtered_indices.iter().filter_map(|&i| self.fruits.get(i)).collect()
        } else {
            self.fruits.iter().collect()
        }
    }

    /// Move selection up
    pub fn select_previous(&mut self) {
        let display_len = if self.is_filtering() {
            self.filtered_indices.len()
        } else {
            self.fruits.len()
        };

        if display_len > 0 && self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Move selection down
    pub fn select_next(&mut self) {
        let display_len = if self.is_filtering() {
            self.filtered_indices.len()
        } else {
            self.fruits.len()
        };

        if display_len > 0 && self.selected_index < display_len - 1 {
            self.selected_index += 1;
        }
    }

    /// Check if currently in filter mode
    pub fn is_filtering(&self) -> bool {
        !self.filter_query.is_empty()
    }

    /// Update the filter and rebuild filtered_indices
    pub fn update_filter(&mut self, query: &str) {
        self.filter_query = query.to_lowercase();
        self.filtered_indices = self.fruits
            .iter()
            .enumerate()
            .filter(|(_, fruit)| fruit.name.to_lowercase().contains(&self.filter_query))
            .map(|(i, _)| i)
            .collect();
        self.selected_index = 0;
    }

    /// Clear the filter
    pub fn clear_filter(&mut self) {
        self.filter_query.clear();
        self.filtered_indices = (0..self.fruits.len()).collect();
    }

    /// Add a new fruit
    pub fn add_fruit(&mut self, fruit: FruitDimensions) -> Result<()> {
        self.fruits.push(fruit);
        self.dirty = true;
        Ok(())
    }

    /// Update an existing fruit by index
    pub fn update_fruit(&mut self, index: usize, fruit: FruitDimensions) -> Result<()> {
        if index >= self.fruits.len() {
            return Err(crate::error::AppError::Validation("Invalid fruit index".to_string()));
        }
        self.fruits[index] = fruit;
        self.dirty = true;
        Ok(())
    }

    /// Delete a fruit by index
    pub fn delete_fruit(&mut self, index: usize) -> Result<()> {
        if index >= self.fruits.len() {
            return Err(crate::error::AppError::Validation("Invalid fruit index".to_string()));
        }
        self.fruits.remove(index);
        self.dirty = true;

        // Adjust selected index if needed
        if self.selected_index >= self.fruits.len() && self.fruits.len() > 0 {
            self.selected_index = self.fruits.len() - 1;
        }

        Ok(())
    }

    /// Set an error message
    pub fn set_error(&mut self, msg: impl Into<String>) {
        self.error_message = Some(msg.into());
    }

    /// Clear the error message
    pub fn clear_error(&mut self) {
        self.error_message = None;
    }
}
