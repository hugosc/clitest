# Initialise-catalog popup — design & implementation notes

Goal

- When `clitest` is run in a directory without `fruits.json`, show a tiny modal (popup) that asks the user whether to initialise the catalogue with defaults.
- Keep the UI and code minimal so you can learn: show the popup, accept `y/n` or let the user type a filename and press Enter to create and save the default catalogue.

Where this fits

- Current app loads data with:

```rust
let fruits: Vec<FruitDimensions> =
    load_catalogue("fruits.json").unwrap_or_else(|_| initialise_fruit_catalogue());
```

- We will change the startup flow to:
  1. Attempt `load_catalogue("fruits.json")`.
  2. If it fails, set app mode to an `InitCatalogPopup` state instead of immediately falling back.
  3. Render a centered modal that explains the situation and offers actions: `[Y]es (create fruits.json)`, `[N]o (open empty UI)`, or type a filename and press Enter to create with that name.
  4. If user accepts, call `initialise_fruit_catalogue()` then `save_catalogue(&fruits, path)` and proceed to normal UI with newly created file.

UX — what the user sees

- On startup, if `fruits.json` missing:
  - A centered popup:
    - Title: "Initialise catalogue?"
    - Body: short text explaining no `fruits.json` was found and options.
    - Input line (optional): default filename (`fruits.json`) editable.
    - Hints at bottom: `Y = create`, `N = skip`, `Enter = create with filename`, `Esc = cancel/quit`
- If user chooses to create, save file and continue to the normal list + details view.
- If user cancels or skips, continue with an in-memory default list but do NOT save a file (or optionally allow saving later).

Implementation plan (high-level)

1. Add a minimal `AppMode` enum and small `InputBuffer` struct to track the popup state.
2. Change startup to set mode to `InitCatalogPopup` when `load_catalogue` returns an error.
3. Update `render` to draw the modal when in popup mode (compose on top of existing layout).
4. Update event handling to route events to the popup when it's open:
   - Printable chars -> append to input buffer
   - Backspace -> delete last char
   - Enter -> attempt to create file using buffer (or default)
   - `Y`/`y` -> create default `fruits.json`
   - `N`/`n` -> close popup and continue with defaults (no save)
   - Esc -> cancel popup (same as `N`)
5. On creation: call `let fruits = initialise_fruit_catalogue(); save_catalogue(&fruits, &path)?;` then proceed with `fruits` in app state.

Types & state (suggested)

- Add near your current state variables:

```rust
enum AppMode {
    Normal,
    InitCatalogPopup(InputBuffer),
}

struct InputBuffer {
    buf: String,
}

impl InputBuffer {
    fn new(default: &str) -> Self {
        Self { buf: default.to_string() }
    }
    fn push(&mut self, ch: char) { self.buf.push(ch); }
    fn pop(&mut self) { self.buf.pop(); }
    fn as_str(&self) -> &str { &self.buf }
}
```

Startup flow change (pseudo)

```rust
// try to load
match load_catalogue("fruits.json") {
    Ok(vec) => {
        fruits = vec;
        mode = AppMode::Normal;
    }
    Err(_) => {
        // Don't immediately initialise. Open popup with default filename.
        mode = AppMode::InitCatalogPopup(InputBuffer::new("fruits.json"));
        // fruits can be empty or set to initialise_fruit_catalogue() depending on desired behavior.
        fruits = initialise_fruit_catalogue(); // or Vec::new()
    }
}
```

Rendering the popup (centered overlay)

- Using `ratatui`, you render the popup on top of the normal UI. Two important steps:
  1. Draw a `Clear` area to erase what's behind the modal (optional but nicer).
  2. Render a `Block` with a `Paragraph` containing the message and an input line.

Example render snippet:

```rust
// inside terminal.draw(|frame| { ... })
if let AppMode::InitCatalogPopup(input) = &app.mode {
    // Make a centered rect
    let area = frame.size();
    let popup_area = ratatui::layout::Rect {
        x: area.width / 6,
        y: area.height / 4,
        width: area.width * 2 / 3,
        height: area.height / 2,
    };

    // Optionally clear the area behind the popup
    // frame.render_widget(Clear, popup_area);

    let title = "Initialise catalogue?";
    let message = format!(
        "No `fruits.json` was found.\n\nPress Y to create `{}` with default data.\nPress N to continue without creating a file.\nOr edit the filename below and press Enter to create it.",
        input.as_str()
    );

    // Body paragraph + input line
    let body = format!("{}\n\nFilename: {}", message, input.as_str());
    let paragraph = Paragraph::new(body)
        .block(Block::default().title(title).borders(Borders::ALL));
    frame.render_widget(paragraph, popup_area);
}
```

Notes:

- `Clear` is a widget available in `ratatui::widgets::Clear` — it erases the area so the popup looks modal.
- Using a separate `Rect` computed from `frame.size()` centers and sizes the popup.

Event handling for the popup

- In your main event loop, when you receive a key event, first check `AppMode`.
- If `AppMode::InitCatalogPopup(input)`:
  - match keys:
    - `KeyCode::Char(c)`:
      - if `c == 'y' || c == 'Y'`: treat like Enter with filename = `"fruits.json"`
      - if `c == 'n' || c == 'N'`: close popup (mode = Normal) and continue (do not save)
      - else: append `c` to `input.buf` (unless it's Enter/Backspace)
    - `KeyCode::Backspace` -> `input.pop()`
    - `KeyCode::Enter` -> attempt create: call `let path = input.as_str().to_string(); let new = initialise_fruit_catalogue(); save_catalogue(&new, &path)?; fruits = new; mode = Normal;`
    - `KeyCode::Esc` -> mode = Normal (cancel)
- Example event fragment:

```rust
if let Event::Key(key) = event::read()? {
    match &mut app.mode {
        AppMode::InitCatalogPopup(input) => {
            match key.code {
                KeyCode::Char(c) => {
                    match c {
                        'y' | 'Y' => {
                            let path = input.as_str().to_string();
                            let new = initialise_fruit_catalogue();
                            save_catalogue(&new, &path)?;
                            app.fruits = new;
                            app.mode = AppMode::Normal;
                        }
                        'n' | 'N' => {
                            app.mode = AppMode::Normal;
                        }
                        _ => input.push(c),
                    }
                }
                KeyCode::Backspace => { input.pop(); }
                KeyCode::Enter => {
                    let path = input.as_str().to_string();
                    let new = initialise_fruit_catalogue();
                    save_catalogue(&new, &path)?;
                    app.fruits = new;
                    app.mode = AppMode::Normal;
                }
                KeyCode::Esc => { app.mode = AppMode::Normal; }
                _ => {}
            }
            continue; // don't process normal-mode keys
        }

        AppMode::Normal => {
            // existing key handling (Up/Down/Esc/etc.)
        }
    }
}
```

Error handling

- `save_catalogue` returns `Result<(), Box<dyn Error>>`. If saving fails (permission, disk full), show a transient error message in the popup instead of closing it.
- Add a small `Option<String>` in the popup state to hold an error message and render it inside the modal when set.
- Example: if `save_catalogue` returns Err(e)`, set `popup_error = Some(format!("Save failed: {}", e))` and keep the popup open.

Testing & suggested exercises

- Run the app from an empty temp directory:
  - Expect popup to appear and allow creation.
- Try typing a custom filename like `myfruits.json` and press Enter — file should be created in the working directory with the JSON from `initialise_fruit_catalogue()`.
- Try file permission failure (create directory with no write) and confirm the error message appears.
- Extend: after successful create, show a confirmation message for a second, then close popup and proceed.

Minimal safety note

- When saving to disk, if you care about atomic writes, write to a temp file (`path.tmp`) then `fs::rename` to the final `fruits.json`. For this learning demo the simple `save_catalogue` call is fine.

Conclusion — small checklist for the code changes

- [ ] Add `AppMode` and `InputBuffer` types.
- [ ] Change startup load logic to set `InitCatalogPopup` when file missing.
- [ ] Add popup rendering code on top of existing UI.
- [ ] Add event routing so popup consumes key events while open.
- [ ] Call `initialise_fruit_catalogue()` + `save_catalogue()` when user confirms.
- [ ] Add error display in popup for save failures.

