# FRUITDATA Integration — Learning Notes (for `fruitcli` branch)

This document explains, at a very small and focused learning level, how the `fruitdata` crate is used inside the `clitest` TUI app (branch `fruitcli`). It describes:

- which `fruitdata` APIs the TUI uses,
- how the TUI maps key bindings to behavior,
- how fruit information is displayed,
- where data is loaded from and how to experiment or extend the demo.

Keep this intentionally simple — the goal is to help you understand how a library crate is consumed by a TUI.

---

Contents
- Quick summary
- Where to look in the code
- How the `fruitdata` functionality is used
- Keybindings and input handling
- How information is displayed
- Running the demo and `fruits.json`
- Small extension ideas (next steps)

---

Quick summary
- The TUI app uses `fruitdata` as a library dependency (pulled from GitHub branch `fruitcrate`).
- The TUI calls `fruitdata::load_catalogue("fruits.json")` and falls back to `fruitdata::initialise_fruit_catalogue()` if the file isn't present.
- UI shows a selectable list (left) and a details pane (right). Up/Down change selection. Esc or `q` quits.
- The app does not (yet) call `fruitdata::save_catalogue()` — it is a read-only UI demo for learning.

---

Where to look in the repo
- TUI app (consumer): `clitest/src/main.rs`
  - This file contains the TUI implementation and the event loop.
- Project dependency: `clitest/Cargo.toml`
  - Contains the Git dependency line:
    - `fruitdata = { git = "https://github.com/hugosc/fruitdata.git", branch = "fruitcrate" }`
- The `fruitdata` crate (library + binary) lives in the GitHub repo `hugosc/fruitdata` on branch `fruitcrate`.
  - The library re-exports functions like `load_catalogue`, `initialise_fruit_catalogue`, `save_catalogue`, and the `FruitDimensions` type.

---

How `fruitdata` functionality is used in the TUI
- The TUI imports library items:
  ```rust
  use fruitdata::{FruitDimensions, initialise_fruit_catalogue, load_catalogue};
  ```
- Loading data:
  ```rust
  load_catalogue("fruits.json")  // attempted first
  initialise_fruit_catalogue()   // fallback if file missing/invalid
  ```
  - If `load_catalogue("fruits.json")` returns an `Err` (file missing or invalid), the TUI calls `initialise_fruit_catalogue()` to get a small default list.
  - This makes the demo work without requiring a `fruits.json` file, while also allowing you to provide one for custom data.
- Data model:
  - The TUI treats each item as a `FruitDimensions` instance (fields: `name`, `length`, `width`, `height`) and uses the `volume()` helper when showing derived info.
- Note: The `fruitdata` crate also contains the logic for CLI commands (`list`, `get`, `add`, `remove`) in its original binary, but in the demo we only use the library API:
  ```rust
  // Library functions used:
  load_catalogue("fruits.json")
  initialise_fruit_catalogue()
  save_catalogue(&[FruitDimensions], path)  // available but not used in current demo
  ```
  This shows how the same core logic can be used both by a CLI binary and by another program as a library.

---

Keybindings — what each key does
- Up Arrow
  - Moves the selection up one item in the list (if not already at the top).
- Down Arrow
  - Moves the selection down one item in the list (if not already at the bottom).
- Enter
  - For this simple demo: does nothing special. The details pane always shows info for the currently selected item.
  - Practical extension: map Enter to open a modal with more actions, or trigger `save_catalogue` after editing.
- Esc or `q`
  - Quit the TUI and restore the terminal.
- Notes on behavior:
  - The app keeps a `selected: usize` index and a `ListState` that tells the `ratatui::List` which item is highlighted:
  ```rust
  let mut selected: usize = 0;
  let mut list_state = ratatui::widgets::ListState::default();
  list_state.select(Some(selected));
  ```
  - Navigation updates both the `selected` index and the `ListState`.

---

How the info is displayed (UI layout)
- Layout
  - The TUI uses a horizontal split layout with two panes:
    - Left pane (≈60% width): selectable `List` of fruit names.
    - Right pane (≈40% width): `Paragraph` showing detailed information for the selected fruit.
  - This layout is created with `ratatui::layout::Layout` and a `.split(frame.area())` call.
- Left pane (List)
  - Each fruit's `name` is used to create a `ListItem`.
  - The `List` is rendered statefully with `frame.render_stateful_widget(list, left_chunk, &mut list_state)`.
  - The widget shows a highlight symbol (e.g., `>> `) next to the currently selected entry.
- Right pane (Details)
  - Shows a formatted block like:
    ```
    Name: Apple

    Dimensions:
      Length: 4.0
      Width : 2.5
      Height: 1.5

    Volume: 15.00
    ```
  - `volume()` is computed from `length * width * height` and shown with two decimal places:
  ```rust
  // In FruitDimensions:
  fn volume(&self) -> f64 {
      self.length * self.width * self.height
  }
  ```
- Empty state
  - If there are no fruits, the details pane shows "No fruits available" and the list is empty.

---

Where the data comes from
- Default: `initialise_fruit_catalogue()` returns an in-memory list of sample fruits. This is used if `fruits.json` can't be loaded.
- Optional JSON file: If you create a `fruits.json` next to where the binary runs, `load_catalogue("fruits.json")` will parse that file into `Vec<FruitDimensions>`:
  ```rust
  let fruits: Vec<FruitDimensions> = load_catalogue("fruits.json")?;
  ```
  - Example `fruits.json`:
    ```json
    [
      {"name":"Apple","length":4.0,"width":2.5,"height":1.5},
      {"name":"Banana","length":6.0,"width":3.5,"height":2.5}
    ]
    ```
- Persisting changes (not currently implemented in TUI):
  ```rust
  save_catalogue(&[FruitDimensions], path)  // available but not used in current demo
  ```
  - The `fruitdata` library exposes `save_catalogue(&[FruitDimensions], path)` — you can call this from the TUI to write edits back to disk.
  - For exercises, try wiring `save_catalogue` to a keybinding or to a simple "add fruit" flow.

---

Running the demo
1. Ensure you are on the `fruitcli` branch:
   - `git checkout fruitcli`
2. Build and run from project root:
   - `cargo run`
3. Behavior:
   - The app will fetch `fruitdata` from GitHub (branch `fruitcrate`) the first time you build.
   - It will try to load `fruits.json` from the current working directory. If absent, the app uses the built-in defaults.

---

Small extension ideas (learning exercises)
- Add persistence:
  - Implement a key (e.g., `a`) to add a new fruit (prompt in TUI), then call `save_catalogue` to persist to `fruits.json`.
- Toggle mode on Enter:
  - Make Enter open a modal where you can edit dimensions and then save.
- Implement deletion:
  - Add a key (e.g., `d`) to delete the selected fruit and `save_catalogue`.
- Paginate / search:
  - Add filtering by typing a query and showing only fruits matching the name.
- Replace Git branch pin with a commit SHA:
  - In `clitest/Cargo.toml` you can replace `branch = "fruitcrate"` with `rev = "<commit_sha>"` for reproducible builds:
  ```toml
  fruitdata = { git = "https://github.com/hugosc/fruitdata.git", rev = "abc123def456" }
  ```

---

FAQ / quick answers
Q: Does this TUI change the `fruitdata` repo?
A: No. The demo uses `fruitdata` as a library dependency. It only reads data from `fruits.json` (if present) in the consumer's runtime directory. No changes were pushed to the `fruitdata` repo.

Q: Can I reuse the same logic from the `fruitdata` CLI commands?
A: Yes. The `fruitdata` crate was refactored to expose core functions (load/save/initialise and `FruitDimensions`) in `src/lib.rs`. That makes it easy to call the same logic from both a CLI binary and a TUI consumer.

Q: Where to look if something fails to compile?
A: Check:
- `clitest/Cargo.toml` to ensure the `fruitdata` git dependency is reachable.
- Network access if Cargo must fetch from GitHub.
- The `fruitdata` repo branch: `https://github.com/hugosc/fruitdata` (branch `fruitcrate`).

---

If you want, I can:
- Add a small `fruits.json` example file into `clitest/` (so the demo shows your data by default), or
- Implement a simple "Add fruit" flow and call `save_catalogue` to make the TUI editable.

Pick one and I'll add it as a tiny follow-up change.