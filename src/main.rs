// #![deny(unsafe_code)]
#![no_main]
#![no_std]

mod controls;
mod data;
mod display;

use controls::clear_direction;
use cortex_m_rt::entry;
use data::Direction;
use display::{clear_display, display_image};
use microbit::board::Board;
use microbit::display::nonblocking::BitImage;
use microbit::hal::Timer;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

const MAX_X: usize = 4;
const MAX_Y: usize = 4;
const DURATION: u32 = 200;

const LED1: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 1, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
];

const LED2: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 1, 1, 0],
];

const LED3: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [0, 0, 0, 1, 0],
    [0, 1, 1, 1, 0],
    [0, 0, 0, 1, 0],
    [0, 1, 1, 1, 0],
];

const LED4: [[u8; 5]; 5] = [
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 1, 0],
    [0, 0, 0, 1, 0],
    [0, 0, 1, 1, 0],
];

const LED5: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 1, 1, 0],
    [0, 0, 0, 1, 0],
    [0, 1, 1, 1, 0],
];

const LED6: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 1, 0],
];

const LED7: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [0, 0, 0, 1, 0],
    [0, 0, 1, 0, 0],
    [0, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
];

const LED8: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 1, 0],
];

const LED9: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 1, 0],
    [0, 0, 0, 1, 0],
    [0, 0, 0, 1, 0],
];

const LED0: [[u8; 5]; 5] = [
    [0, 1, 1, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 1, 1, 1, 0],
];

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0).into_periodic();
    // let mut rng = Rng::new(board.RNG);

    controls::init_buttons(board.GPIOTE, board.buttons);
    display::init_display(board.TIMER1, board.display_pins);

    let all = [LED0, LED1, LED2, LED3, LED4, LED5, LED6, LED7, LED8, LED9];
    let mut current: usize = 0;

    loop {
        if let Some(leds) = all.get(current) {
            let image = BitImage::new(&leds);
            display_image(&image);
        } else {
            rprintln!("Element is not found, replace with the first one");
            current = 0;
        }

        timer.delay(100_000u32);

        clear_display();
        let direction = controls::get_direction();
        if direction != Direction::None {
            clear_direction();

            match direction {
                Direction::Left if current > 0 => current -= 1,
                Direction::Right if current < (all.len() - 1) => current += 1,
                _ => {}
            }

            rprintln!("direction: {:?}, new current: {}", direction, current);
        }
    }
}
