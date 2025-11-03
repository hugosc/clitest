use crate::error::Result;
use fruitdata::FruitDimensions;

/// Represents the current input field in a modal
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputField {
    Name,
    Length,
    Width,
    Height,
}

impl InputField {
    /// Move to the next field (Tab navigation)
    pub fn next(self) -> Self {
        match self {
            InputField::Name => InputField::Length,
            InputField::Length => InputField::Width,
            InputField::Width => InputField::Height,
            InputField::Height => InputField::Name,
        }
    }

    /// Move to the previous field (Shift+Tab navigation)
    pub fn prev(self) -> Self {
        match self {
            InputField::Name => InputField::Height,
            InputField::Length => InputField::Name,
            InputField::Width => InputField::Length,
            InputField::Height => InputField::Width,
        }
    }
}

/// Modal state for adding or editing fruits
#[derive(Debug, Clone)]
pub struct ModalState {
    /// The fruit being edited (or template for new fruit)
    pub name: String,
    pub length: String,
    pub width: String,
    pub height: String,
    /// Which field is currently focused
    pub focused_field: InputField,
    /// Error message within the modal
    pub error: Option<String>,
}

impl ModalState {
    /// Create a new modal for adding a fruit
    pub fn new() -> Self {
        Self {
            name: String::new(),
            length: String::new(),
            width: String::new(),
            height: String::new(),
            focused_field: InputField::Name,
            error: None,
        }
    }

    /// Create a modal pre-filled with an existing fruit's data
    pub fn from_fruit(fruit: &FruitDimensions) -> Self {
        Self {
            name: fruit.name.clone(),
            length: fruit.length.to_string(),
            width: fruit.width.to_string(),
            height: fruit.height.to_string(),
            focused_field: InputField::Name,
            error: None,
        }
    }

    /// Move focus to the next field
    pub fn next_field(&mut self) {
        self.focused_field = self.focused_field.next();
    }

    /// Move focus to the previous field
    pub fn prev_field(&mut self) {
        self.focused_field = self.focused_field.prev();
    }

    /// Insert a character into the focused field
    pub fn insert_char(&mut self, c: char) {
        // Only allow valid characters for each field
        match self.focused_field {
            InputField::Name => {
                self.name.push(c);
                self.error = None;
            }
            InputField::Length | InputField::Width | InputField::Height => {
                if c.is_ascii_digit() || c == '.' {
                    match self.focused_field {
                        InputField::Length => {
                            self.length.push(c);
                        }
                        InputField::Width => {
                            self.width.push(c);
                        }
                        InputField::Height => {
                            self.height.push(c);
                        }
                        _ => {}
                    }
                    self.error = None;
                }
            }
        }
    }

    /// Remove the last character from the focused field
    pub fn backspace(&mut self) {
        match self.focused_field {
            InputField::Name => {
                self.name.pop();
            }
            InputField::Length => {
                self.length.pop();
            }
            InputField::Width => {
                self.width.pop();
            }
            InputField::Height => {
                self.height.pop();
            }
        }
    }

    /// Validate and convert to a FruitDimensions if valid
    pub fn validate_and_build(&mut self) -> Result<FruitDimensions> {
        // Validate name
        if self.name.trim().is_empty() {
            self.error = Some("Name cannot be empty".to_string());
            return Err(crate::error::AppError::Validation("Name cannot be empty".to_string()));
        }

        // Parse dimensions
        let length: f32 = self.length.parse().map_err(|_| {
            self.error = Some("Length must be a valid number".to_string());
            crate::error::AppError::Validation("Length must be a valid number".to_string())
        })?;

        let width: f32 = self.width.parse().map_err(|_| {
            self.error = Some("Width must be a valid number".to_string());
            crate::error::AppError::Validation("Width must be a valid number".to_string())
        })?;

        let height: f32 = self.height.parse().map_err(|_| {
            self.error = Some("Height must be a valid number".to_string());
            crate::error::AppError::Validation("Height must be a valid number".to_string())
        })?;

        // Validate positive values
        if length <= 0.0 || width <= 0.0 || height <= 0.0 {
            self.error = Some("All dimensions must be positive".to_string());
            return Err(crate::error::AppError::Validation(
                "All dimensions must be positive".to_string(),
            ));
        }

        Ok(FruitDimensions {
            name: self.name.trim().to_string(),
            length,
            width,
            height,
        })
    }

    /// Clear any error message
    pub fn clear_error(&mut self) {
        self.error = None;
    }
}

impl Default for ModalState {
    fn default() -> Self {
        Self::new()
    }
}
