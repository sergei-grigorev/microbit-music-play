use data::Direction;
use embassy_futures::select::{select, Either};
use embassy_nrf::gpio::Input;

use crate::data;

pub async fn get_direction<'a>(button1: &mut Input<'a>, button2: &mut Input<'a>) -> Direction {
    let clicks = select(button1.wait_for_low(), button2.wait_for_low()).await;
    match clicks {
        Either::First(_) => {
            button1.wait_for_high().await;
            Direction::Left
        },
        Either::Second(_) => {
            button2.wait_for_high().await;
            Direction::Right
        }
    }
}
