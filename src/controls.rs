use core::cell::RefCell;

use cortex_m::interrupt::{free, Mutex};
use data::Direction;
use microbit::{
    board::Buttons,
    hal::gpiote::Gpiote,
    pac::{self, interrupt},
};

use crate::{data, DURATION};

static GPIO: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));
static DIRECTION: Mutex<RefCell<Direction>> = Mutex::new(RefCell::new(Direction::None));

pub(crate) fn init_buttons(board_gpiote: pac::GPIOTE, board_buttons: Buttons) {
    let gpiote = Gpiote::new(board_gpiote);

    let channel0 = gpiote.channel0();
    channel0
        .input_pin(&board_buttons.button_a.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel0.reset_events();

    let channel1 = gpiote.channel1();
    channel1
        .input_pin(&board_buttons.button_b.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel1.reset_events();

    free(move |cs| {
        *GPIO.borrow(cs).borrow_mut() = Some(gpiote);

        unsafe {
            pac::NVIC::unmask(pac::Interrupt::GPIOTE);
        }

        pac::NVIC::unpend(pac::Interrupt::GPIOTE);
    });
}

pub(crate) fn get_direction() -> Direction {
    free(|cs| *DIRECTION.borrow(cs).borrow())
}

pub(crate) fn clear_direction() {
    free(|cs| DIRECTION.borrow(cs).replace(Direction::None));
}

#[interrupt]
fn GPIOTE() {
    free(|cs| {
        if let Some(gpiote) = GPIO.borrow(cs).borrow().as_ref() {
            let left_pressed = gpiote.channel0().is_event_triggered();
            let right_pressed = gpiote.channel1().is_event_triggered();

            let turn = match (left_pressed, right_pressed) {
                (true, false) => Direction::Left,
                (false, true) => Direction::Right,
                _ => Direction::None,
            };

            gpiote.channel0().reset_events();
            gpiote.channel1().reset_events();

            *DIRECTION.borrow(cs).borrow_mut() = turn;
        }
    });
}
