#![no_std]
#![no_main]

mod controls;
mod data;
mod display;
mod speaker;

use core::sync::atomic::Ordering;

use controls::{get_direction, get_touch};
use defmt::{info, warn};
use display::{Columns, DisplayPins, Rows};
use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};
use embassy_nrf::{
    gpio::{AnyPin, Flex, Input, Pin, Pull},
    peripherals::PWM0,
};
use speaker::{Speaker, SPEED_MULTIPLIER};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::task]
async fn play_song(logo: AnyPin, pwm: PWM0, ch0: AnyPin) {
    // wait button 1 click and then continue
    let mut sensor = Flex::new(logo);
    let mut speaker = Speaker::new(pwm, ch0);

    loop {
        let _ = get_touch(&mut sensor).await;

        info!("Start music");
        let _ = select(speaker.play(), get_touch(&mut sensor)).await;
        speaker.stop();
        info!("Wait for a next sensor click");
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    // wait touch sensor click and then play song
    spawner
        .spawn(play_song(p.P1_04.degrade(), p.PWM0, p.P0_00.degrade()))
        .unwrap();

    // display
    let rows = Rows {
        row1: p.P0_21.degrade(),
        row2: p.P0_22.degrade(),
        row3: p.P0_15.degrade(),
        row4: p.P0_24.degrade(),
        row5: p.P0_19.degrade(),
    };
    let columns = Columns {
        col1: p.P0_28.degrade(),
        col2: p.P0_11.degrade(),
        col3: p.P0_31.degrade(),
        col4: p.P1_05.degrade(),
        col5: p.P0_30.degrade(),
    };

    // initialize display pins
    let mut display = DisplayPins::new(columns, rows);

    // available buttons
    let mut button_a = Input::new(p.P0_14, Pull::Up);
    let mut button_b = Input::new(p.P0_23, Pull::Up);

    let mut current: u32 = 0;
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
            SPEED_MULTIPLIER.store(current, Ordering::Relaxed);
        }
    }
}
