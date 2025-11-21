# Rust 2D Engine

A modular 2D game engine written in Rust, leveraging **Specs (ECS)** and **SDL2** for rendering.

## Features

- **ECS Architecture**: Built with `specs` for modular entity-component management.
- **Physics System**: Gravity, velocity, drag, and AABB collision detection.
- **Particle System**: Visual effects with configurable lifetime and emission rates.
- **Gameplay Logic**: Win/Loss states, score tracking, and level resetting.
- **Input Handling**: Smooth player movement and jumping.

## Controls

- **Arrow Keys**: Move Left/Right
- **Space**: Jump
- **Goal**: Collect all 50 coins (Yellow Dots) to win!
- **Avoid**: Falling off the screen resets the level.

## Prerequisites

Before starting, ensure you have the following installed on your system:

- **Rust** (install via `rustup`)
- **SDL2** (installed via Homebrew on macOS)

### macOS Installation
```bash
brew install sdl2
```

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/LMouhssine/rust_2d_engine.git
   cd rust_2d_engine
   ```

2. Run the game:
   ```bash
   cargo run
   ```

## Architecture

- `src/main.rs`: Entry point, ECS world setup, and game loop.
- `src/components.rs`: Data components (Position, Velocity, Player, etc.).
- `src/systems/`: Logic systems (Movement, Collision, Particles, Logic).
- `src/utils/`: Helper functions for rendering and input.
