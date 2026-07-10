# Timer

> [!note]
> This project is inspired by [studytimer](https://github.com/MohssineX/studytimer.git)

A simple **command-line countdown timer** written in Rust. Enter a duration in minutes, watch the live `HH:MM:SS` countdown, and get alerted with an alarm sound when time's up.

## Features

- **Minute-based input** — enter any duration in minutes
- **Live countdown display** — real-time `HH:MM:SS` output updating every second
- **Alarm sound** — plays `alarm.mp3` when the timer reaches zero
- **Input validation** — rejects non-numeric input gracefully
- **Day limit check** — warns if the duration exceeds 24 hours and restarts
- **Custom alarm** — replace `alarm.mp3` with any MP3 file of your choice to change the sound

## Tech Stack

| Language | Audio Library |
|----------|---------------|
| Rust (edition 2024) | `rusty_audio` v1.4.1 |

## Getting Started

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (rustc + cargo)
- Linux: ALSA development libraries (alsa-lib, pkg-config)

### Installation

```bash
# Clone the repo
git clone https://github.com/rida-hdj/timer.git
cd timer

# Build (release)
cargo build --release
```
## Usage

```bash
# Run the timer if you build it
cargo run --release
# or directly
./timer
```

Then enter the number of minutes when prompted. The timer will count down and play an alarm when finished.

### Changing the alarm sound

Replace `alarm.mp3` in the project root with your own MP3 file (keep the same filename).

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## License

This project is under MIT License
