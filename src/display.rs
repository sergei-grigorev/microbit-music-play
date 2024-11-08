use core::cell::RefCell;
use cortex_m::interrupt::{free, Mutex};
use microbit::pac;
use microbit::pac::interrupt;
use microbit::{display::nonblocking::Display, gpio::DisplayPins, pac::TIMER1};
use tiny_led_matrix::Render;

static DISPLAY: Mutex<RefCell<Option<Display<TIMER1>>>> = Mutex::new(RefCell::new(None));

pub(crate) fn init_display(board_timer: TIMER1, board_display: DisplayPins) {
    let display = Display::new(board_timer, board_display);

    free(move |cs| *DISPLAY.borrow(cs).borrow_mut() = Some(display));

    // enble timer interrupt
    unsafe {
        pac::NVIC::unmask(pac::Interrupt::TIMER1);
    }
}

pub(crate) fn display_image(image: &impl Render) {
    free(|cs| {
        if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.show(image);
        }
    });
}

pub(crate) fn clear_display() {
    free(|cs| {
        if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.clear();
        }
    })
}

#[interrupt]
fn TIMER1() {
    free(
        |cs| {
            if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
                display.handle_display_event();
            }
        },
    );
}
