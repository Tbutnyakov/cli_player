# 🎵 Cli Audio Player

> A simple terminal‑based music player written in Rust

![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)
![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-informational)
![Rust Version](https://img.shields.io/badge/rust-%3E%3D1.56-blue)

## ✨ Features

- ✅ Play audio files from a specified directory
- ✅ Playback control (play/pause/stop)
- ✅ Navigate between tracks (next/prev)
- ✅ List all available tracks
- ✅ Select a track by number
- ✅ Adjust volume (range: 0.0–1.0)
- ✅ Minimalistic terminal interface
- ✅ Supports MP3, WAV, FLAC, and other formats via Rust crates

## 🛠️ Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.56 or higher)
- Make sure your system has basic audio support


### Build from source
```
git clone https://github.com/Tbutnyakov/cli_player
cd cli_player
```

## 🎮 Usage
```
cargo run -- --dir /path/to/music
```

### Interactive Commands

Once running, use these commands:

| Command | Description | Example |
|--------|-----------|---------|
| `play` | Start playback | `play` |
| `pause` / `unpause` | Pause or resume playback | `pause` |
| `stop` | Stop playback completely | `stop` |
| `next` | Play the next track | `next` |
| `prev` | Play the previous track | `prev` |
| `list` | Show all available tracks | `list` |
| `track <N>` | Play track number N (index starts at 0) | `track 3` |
| `volume <V>` | Set volume level (0.0 to 1.0) | `volume 0.7` |
| `help` | Display help information | `help` |

## 🔧 Dependencies

The project uses the following Rust crates:

- [`clap`](https://crates.io/crates/clap) — Command‑line argument parsing
- [`rodio`](https://crates.io/crates/rodio) — Audio playback
- [`colored`](https://crates.io/crates/colored) — Colored terminal output
