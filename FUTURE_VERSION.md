# Fruit CLI - Future Version (v1.0 Preview)

This is a **polished, modular future version** of your fruit TUI. It demonstrates professional Rust patterns and architecture that scales well as you add features.

## What's New in This Version

### 1. **Modular Architecture**

The monolithic `main.rs` is now split into focused modules:

```
src/
├── main.rs              # Entry point only (~70 lines)
├── app/
│   ├── mod.rs          # Module exports
│   ├── state.rs        # AppState + mode management
│   └── events.rs       # Event handling & dispatch
├── ui/
│   ├── mod.rs          # Module exports
│   ├── render.rs       # All UI rendering logic
│   └── modal.rs        # Modal state & validation
└── error.rs            # Custom error types
```

**Benefits:**
- Each module has a single responsibility
- Easy to understand and modify
- Scales as features grow
- Clear dependencies

### 2. **Centralized State Management**

Before: Variables scattered in `run()` function  
After: Everything in `AppState` struct

```rust
pub struct AppState {
    pub fruits: Vec<FruitDimensions>,
    pub selected_index: usize,
    pub mode: AppMode,           // Normal, AddFruit, EditFruit, Filter, ConfirmDelete
    pub dirty: bool,              // Unsaved changes?
    pub filter_query: String,
    pub filtered_indices: Vec<usize>,
    pub error_message: Option<String>,
    pub modal: Option<ModalState>,
}
```

**Why this matters:**
- State is explicit and testable
- Easy to save/load or serialize
- Can add features without tangling code

### 3. **Event-Driven Architecture**

Instead of handling keypresses directly in one match statement, events are now dispatched:

```rust
pub enum AppEvent {
    KeyPress(KeyCode),
    Quit,
}

pub fn handle_event(state: &mut AppState, event: AppEvent) -> Result<bool>
```

Each mode has its own handler:
- `handle_normal_mode()` - Navigation and command entry
- `handle_filter_mode()` - Search input handling
- `handle_delete_confirm()` - Delete confirmation
- `handle_add_fruit_modal()` - Form input for adding
- `handle_edit_fruit_modal()` - Form input for editing

**Why this matters:**
- Clear separation between input handling and state updates
- Easy to add new modes (e.g., Help, Settings)
- Testable without needing the UI

### 4. **Modal System for CRUD**

A reusable `ModalState` for forms:

```rust
pub struct ModalState {
    pub name: String,
    pub length: String,
    pub width: String,
    pub height: String,
    pub focused_field: InputField,
    pub error: Option<String>,
}

impl ModalState {
    pub fn validate_and_build(&mut self) -> Result<FruitDimensions>
}
```

**Features:**
- Field-by-field input validation
- Tab/Shift+Tab navigation
- Character filtering (digits+decimals only for numbers)
- Visual error display
- Reusable for other forms (Settings, etc.)

### 5. **Better Error Handling**

Custom error types instead of just using `color_eyre`:

```rust
pub enum AppError {
    Io(#[from] std::io::Error),
    FruitData(String),
    Validation(String),
    Config(String),
    Other(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
```

**Why this matters:**
- Errors are typed and can be handled differently
- Better error messages for users
- Ready for config parsing, async I/O, etc.

---

## Current Features

### Navigation
- `↑` / `k` - Move up
- `↓` / `j` - Move down

### Search & Filter
- `/` - Enter search mode, type to filter fruits by name
- `Esc` - Clear filter and exit

### CRUD Operations
- `a` - **Add** new fruit (opens form modal)
  - Tab to next field, Shift+Tab to previous
  - Enter to save, Esc to cancel
  - Validates: non-empty name, positive dimensions

- `e` - **Edit** selected fruit (opens pre-filled form)
  - Same controls as Add modal

- `d` - **Delete** selected fruit (confirmation)
  - Press `y` to confirm, `n`/`Esc` to cancel

### Other
- `?` - Help (placeholder, ready to implement)
- `q` / `Esc` - Quit (with unsaved changes warning)

---

## Architecture Highlights

### State Management Pattern

```rust
// Instead of this (old):
fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut selected = 0;
    let mut fruits = vec![];
    // ... 200 lines of logic mixed together
}

// Now it's this (new):
fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut state = AppState::new(fruits);
    
    loop {
        terminal.draw(|frame| ui::render(frame, &state))?;
        if let Event::Key(key) = event::read()? {
            app::handle_event(&mut state, AppEvent::KeyPress(key.code))?;
        }
    }
}
```

### Clean Event Dispatch

```rust
match state.mode {
    AppMode::Normal => handle_normal_mode(&mut state, key)?,
    AppMode::Filter => handle_filter_mode(&mut state, key)?,
    AppMode::ConfirmDelete => handle_delete_confirm(&mut state, key)?,
    AppMode::AddFruit => handle_add_fruit_modal(&mut state, key)?,
    AppMode::EditFruit => handle_edit_fruit_modal(&mut state, key)?,
}
```

Each handler is focused and testable.

### Reusable Modal Component

The `ModalState` is generic enough to use for:
- Adding/editing fruits (current)
- Settings form (future)
- Search/replace (future)
- Command palette (future)

---

## What's Ready to Add Next

### Short-term (Easy)
1. **Help Screen** (`?` key)
   - Keybinding reference in a modal
   - Just add `render_help_modal()` in `ui/render.rs`

2. **Persistence** (Save to file)
   - Add `Ctrl+S` handler that calls `fruitdata::save_catalogue()`
   - Use the `dirty` flag to warn about unsaved changes

3. **Copy to Clipboard**
   - `Ctrl+C` to copy selected fruit details
   - Need `clipboard` crate

### Medium-term (Moderate)
4. **Sort Options** (`o` key)
   - Cycle through sort orders (A-Z, Volume, etc.)
   - Already has the `filtered_indices` system to support this

5. **Statistics View** (new tab)
   - Min/max/average dimensions
   - Volume distribution
   - Top N by volume

6. **Configuration File**
   - `~/.config/fruitcli/config.toml`
   - Custom keybindings
   - Default sort order

### Long-term (Advanced)
7. **Multi-View Tabs**
   - Catalogue (current list view)
   - Comparison (side-by-side 2–3 fruits)
   - Statistics dashboard

8. **Async I/O**
   - Use Tokio for non-blocking file operations
   - Auto-save in background

9. **Testing**
   - State transitions (verify that keys change state correctly)
   - Validation (test form input validation)
   - Filtering (test search results)

---

## How to Use This Version

### Build and Run
```bash
cargo build
cargo run
```

### Keybinding Cheat Sheet

| Key | Action |
|-----|--------|
| `↑` / `k` | Move up |
| `↓` / `j` | Move down |
| `/` | Start search |
| `a` | Add fruit |
| `e` | Edit fruit |
| `d` | Delete fruit |
| `q` / `Esc` | Quit |

### Try the Full Workflow
1. Start the app
2. Press `a` to add a fruit (e.g., "Mango" with dimensions 10.0, 8.0, 7.0)
3. Tab through fields, press Enter to save
4. Press `/` and type "mango" to filter
5. Press `e` to edit, change the name, press Enter
6. Press `d` to delete, then `y` to confirm
7. Press `q` to quit (should warn if unsaved)

---

## Code Quality

### What's Good
✅ Modular structure that scales  
✅ Clear separation of concerns  
✅ Custom error types ready for expansion  
✅ Modal system reusable for any form  
✅ State is centralized and testable  
✅ Event handling is predictable  

### What's Not (Ready for Future)
⚠️ No persistence yet (add `Ctrl+S` + file saving)  
⚠️ No config file support yet  
⚠️ No async I/O (still blocking on file reads)  
⚠️ Limited visual polish (colors, formatting ready though)  
⚠️ No tests yet (but structure is test-friendly)  

---

## Learning Points

This version demonstrates several **professional Rust patterns**:

1. **State Machines** - `AppMode` enum for clear mode transitions
2. **Result Types** - Custom `AppError` for better error handling
3. **Module Organization** - Clear responsibilities and public API
4. **Separation of Concerns** - State, events, and rendering are independent
5. **Validation** - Modal validates before persisting
6. **Reusable Components** - `ModalState` can be used for many forms

These patterns scale well as projects grow from 500 lines → 5,000+ lines.

---

## Next Steps for You

Pick one:

1. **Add Persistence** - Make `Ctrl+S` actually save to disk (30 min)
2. **Add Help** - Implement the Help modal when user presses `?` (20 min)
3. **Add Stats** - Create a Statistics view tab (1–2 hours)
4. **Write Tests** - Test state transitions and validation (1 hour)
5. **Add Config** - Parse a TOML config file for keybindings (1–2 hours)

All of these are straightforward given the current architecture!

---

**Created:** This version represents what your TUI could aspire to become—clean, modular, and ready to scale.

Enjoy exploring! 🚀
