# clitest — Fruit CLI demo (branch: fruitcli)

This branch (`fruitcli`) is a small learning demo that integrates the `fruitdata` library
(<https://github.com/hugosc/fruitdata>, branch `fruitcrate`) into a simple
ratatui TUI so you can practice using an external crate as a data provider.

What it shows

- Left pane: selectable list of fruit names.
- Right pane: details for the selected fruit.
- Navigation: Up / Down keys to move selection.
- Quit: Esc or `q`.

How the integration works (very simple)

- `clitest/Cargo.toml` depends on the Git repo and branch:
  `fruitdata = { git = "https://github.com/hugosc/fruitdata.git", branch = "fruitcrate" }`
- `src/main.rs` calls:
  - `fruitdata::load_catalogue("fruits.json")` to load a JSON file, or
  - `fruitdata::initialise_fruit_catalogue()` to get defaults if the file is missing.
- The code uses `FruitDimensions` from `fruitdata` to render names and details.

Run locally

1. Switch to the branch:
   - `git checkout fruitcli`
2. Build and run:
   - `cargo run`
   - The app looks for `fruits.json` in the current working directory.
     - If not found, it will show a small default catalogue.

Sample `fruits.json` (optional)
Save a file named `fruits.json` next to where you run the binary:

```
[
  {"name":"Apple","length":4.0,"width":2.5,"height":1.5},
  {"name":"Banana","length":6.0,"width":3.5,"height":2.5}
]
```

Notes / next steps (ideas for learning)

- Make Enter open a modal with more actions.
- Add a way to add/remove fruits from the UI and save back to `fruits.json`.
- Replace the Git branch with a pinned `rev` for reproducible builds if needed.

That's it — intentionally tiny so you can read the code and experiment.

