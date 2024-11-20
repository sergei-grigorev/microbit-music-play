### Microbit v2 tone music player

This is a simple tone music player for the Microbit v2. It uses the built-in speaker to play different musical notes. The project uses Embassy for asynchronous programming and the `memory.x` file for memory management. The `speaker` module contains the `play` function that plays the music. The `display` module contains the `render` function that draws a number on the display. The `controls` module contains the `get_direction` function that controls the speed of the player and `get_touch` function that detects when the usser touches the logo sensor.

### Dependencies

- Embassy
- Embassy-nrf
- defmt
- cortex-m
- panic-probe

### Compile and run

Connect microbit v2 and run `cargo run --release`. The music will play when you touch the logo sensor. To stop press the logo sensor again. The speed of the player can be controlled by pressing the A and B buttons. The number on the display will be updated.
