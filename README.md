# awqat

A Rust terminal application for prayer times (`awqat` = "times"), with local data storage and a TUI interface.

## Features

- Terminal UI powered by [`ratatui`](https://github.com/ratatui/ratatui)
- Prayer time calculations via [`salah`](https://crates.io/crates/salah)
- Timezone handling with `chrono`, `chrono-tz`, and `tz-search`
- Local SQLite database (`rusqlite` with bundled SQLite)
- JSON serialization support for data/config
- Error handling using `color-eyre` and `thiserror`

## Tech Stack

- **Language:** Rust (Edition 2024)
- **UI:** `ratatui`, `crossterm`, `tui-big-text`
- **Data:** `rusqlite` (bundled), `serde`, `serde_json`
- **Date/Time:** `chrono`, `chrono-tz`, `icu_calendar`, `tz-search`
- **Domain logic:** `salah`

## Project Structure

- `src/` — main source code
- `core/` — core logic and types
- `tui/` — terminal interface app code
- `tray/` — tray-related components
- `data/` — dataset files
- `assets/` — static assets
- `docs/` — documentation
- `exmamples/` — examples (note: directory name is currently `exmamples`)
- `todo.md` — development notes and roadmap

## Getting Started

### Prerequisites

- Rust toolchain (stable) installed via [rustup](https://rustup.rs)

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

## Development Notes

Current progress and planned improvements are tracked in [`todo.md`](./todo.md), including:

- Data quality improvements for city-level precision
- Better nearest-city heuristics in search results
- Improved issue templates for reporting data inaccuracies
- Future OS-adaptive color palette support

## Status

This project is actively evolving. Interfaces, data handling, and behavior may change between versions while core features are refined.
