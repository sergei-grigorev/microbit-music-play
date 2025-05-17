# Micro:bit v2 Interactive Music Player

An embedded Rust application for the BBC micro:bit v2 that plays music through the built-in speaker with interactive speed control using the buttons and touch sensor.

## Features

- **Touch-Activated Playback**: Start and stop music playback by touching the micro:bit's logo
- **Variable Playback Speed**: Adjust music tempo using the A (slower) and B (faster) buttons
- **Visual Feedback**: Current speed level (0-9) is displayed on the LED matrix
- **Asynchronous Operation**: Built using Embassy for efficient async/await programming
- **Power Efficient**: Automatically stops PWM when not playing

## Hardware Requirements

- BBC micro:bit v2
- USB cable for programming
- Optional: External speaker (connects to pin P0.00 and GND)

## How It Works

1. **Touch Control**:
   - Touch the micro:bit's logo to start playing music
   - Touch it again to stop playback

2. **Speed Control**:
   - Press button A to decrease playback speed
   - Press button B to increase playback speed
   - Current speed level (0-9) is shown on the LED display
   - Speed affects both tempo and note duration

3. **Technical Implementation**:
   - Uses PWM (Pulse Width Modulation) on pin P0.00 for audio output
   - Implements touch sensing on the logo (P1.04)
   - Uses button A (P0.14) and B (P0.23) for speed control
   - LED matrix is driven using row/column multiplexing

## Dependencies

- `embassy-executor`: Async/await executor for embedded systems
- `embassy-nrf`: Hardware abstraction for nRF52 series (micro:bit v2)
- `defmt`: Efficient logging framework for embedded systems
- `cortex-m`: Low-level ARM Cortex-M support
- `panic-probe`: Panic handler for embedded systems

## Building and Flashing

### Prerequisites

1. Install Rust:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Add the ARM Cortex-M target:
   ```bash
   rustup target add thumbv7em-none-eabihf
   ```

3. Install the `probe-run` tool for flashing:
   ```bash
   cargo install probe-run
   ```

### Compiling and Running

1. Connect your micro:bit v2 via USB
2. Build and run the application:
   ```bash
   cargo run --release
   ```

   This will:
   - Compile the application in release mode
   - Automatically flash it to the connected micro:bit
   - Start the application

## Usage

1. **Start/Stop Music**:
   - Touch the micro:bit's logo to start playback
   - Touch it again to stop

2. **Adjust Speed**:
   - Press button A to slow down
   - Press button B to speed up
   - The current speed level (0-9) is shown on the display

## Project Structure

- `src/main.rs`: Main application logic
- `src/speaker/`: Audio playback and music generation
  - `speaker.rs`: PWM-based tone generation
  - `music.rs`: Music notes and sequences
- `src/display/`: LED matrix control
  - `display.rs`: LED matrix driver
  - `numbers.rs`: Number patterns for the display
- `src/controls/`: Input handling
  - `controls.rs`: Button and touch input processing
  - `direction.rs`: Direction enum for button presses

## Troubleshooting

- If you get permission errors when flashing, ensure your user has access to the USB device
- If the music sounds distorted, check your speaker connections
- Make sure you're using micro:bit v2 (nRF52833), as this code is not compatible with v1

## License

This project is licensed under the terms of the MIT license. See the [LICENSE](LICENSE) file for details.
