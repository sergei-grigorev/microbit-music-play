#![no_std]
#![no_main]

mod controls;
mod data;
mod display;
mod numbers;

use controls::get_direction;
use defmt::{info, warn};
use display::DisplayPins;
use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};
use embassy_nrf::gpio::{Input, Level, Output, OutputDrive, Pin, Pull};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    // display
    let row1 = p.P0_21.degrade();
    let row2 = p.P0_22.degrade();
    let row3 = p.P0_15.degrade();
    let row4 = p.P0_24.degrade();
    let row5 = p.P0_19.degrade();
    let col1 = p.P0_28.degrade();
    let col2 = p.P0_11.degrade();
    let col3 = p.P0_31.degrade();
    let col4 = p.P1_05.degrade();
    let col5 = p.P0_30.degrade();

    // initialize display pins
    let mut display = DisplayPins::new(col1, col2, col3, col4, col5, row1, row2, row3, row4, row5);

    // available buttons
    let mut button_a = Input::new(p.P0_14, Pull::Up);
    let mut button_b = Input::new(p.P0_23, Pull::Up);

    let mut current: usize = 0;
    loop {
        let actions = select(
            get_direction(&mut button_a, &mut button_b),
            display.render(current),
        )
        .await;

        if let Either::First(direction) = actions {
            match direction {
                data::Direction::Left if current > 0 => current -= 1,
                data::Direction::Right if current < 9 => current += 1,
                _ => warn!("Invalid direction"),
            }

            info!("Direction: {:?}, new value: {}", direction, current);
        }
    }
}
