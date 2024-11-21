use defmt::info;
pub(crate) use direction::Direction;
use embassy_futures::select::{select, Either};
use embassy_nrf::gpio::{Flex, Input, OutputDrive, Pull};
use embassy_time::Timer;

mod direction;

/// Minimum charging time in microseconds to regard as
/// "touched".
const TOUCH_THRESHOLD: u64 = 100;

/// Time in milliseconds to discharge the touchpad before
/// testing.
const DISCHARGE_TIME: u64 = 100;

pub async fn get_direction<'a>(button1: &mut Input<'a>, button2: &mut Input<'a>) -> Direction {
    let clicks = select(
        button1.wait_for_falling_edge(),
        button2.wait_for_falling_edge(),
    )
    .await;
    match clicks {
        Either::First(_) => Direction::Left,
        Either::Second(_) => Direction::Right,
    }
}

pub async fn get_touch<'a>(sensor: &mut Flex<'a>) {
    sensor.set_as_input_output(Pull::None, OutputDrive::Standard0Disconnect1);
    sensor.set_low();

    // true - touch has been detected
    let mut state = false;

    Timer::after_millis(DISCHARGE_TIME).await;
    loop {
        sensor.set_high();

        let mut new_state = true;
        for _ in 0..TOUCH_THRESHOLD {
            if sensor.is_high() {
                new_state = false;
                break;
            }
            Timer::after_nanos(100).await;
        }

        if new_state != state {
            match new_state {
                true => info!("Touch detected"),
                false => return,
            }
        }

        state = new_state;
        // Pull the touchpad to ground to discharge any accumulated
        // voltage. Allow time to settle.
        sensor.set_low();
        Timer::after_millis(DISCHARGE_TIME).await;
    }
}
