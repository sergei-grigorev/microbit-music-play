use core::sync::atomic::{AtomicU32, Ordering};

use defmt::info;
use embassy_nrf::{gpio::AnyPin, pwm, Peripheral};
use embassy_time::Timer;
use music::NOTES;

mod music;

pub static SPEED_MULTIPLIER: AtomicU32 = AtomicU32::new(0_u32);
const TEMPO: u32 = 50u32;

pub struct Speaker<'d, T: pwm::Instance> {
    pwm: pwm::SimplePwm<'d, T>,
}

impl<'d, T: pwm::Instance> Speaker<'d, T> {
    pub fn new(pwm: impl Peripheral<P = T> + 'd, ch0: AnyPin) -> Self {
        // init pwm
        let pwm = pwm::SimplePwm::new_1ch(pwm, ch0);
        pwm.set_prescaler(pwm::Prescaler::Div16);

        info!("Speaker initialized, max duty: {}", pwm.max_duty());

        Speaker { pwm }
    }

    pub async fn play(&mut self) {
        // get the next note
        for (tone, delay) in NOTES {
            let speed = SPEED_MULTIPLIER.load(Ordering::Relaxed) as f32;
            let timer_change: f32 = (speed / 5f32) + 1f32;
            let tempo = (TEMPO as f32 / timer_change) as u64;
            let delay = (delay as f32 / timer_change) as u64;

            info!(
                "tone: {} Hz, delay: {}, timer_change: {}",
                tone, delay, timer_change
            );

            if tone > 0 {
                // start playing
                self.pwm.set_period(tone);
                self.pwm.enable();

                let duty = self.pwm.max_duty();
                // not so loud
                self.pwm.set_duty(0, duty / 4);

                // Keep the output on for as long as required
                Timer::after_millis(delay).await;
            } else {
                // if the note is 0, silence the output, do nothing
                Timer::after_millis(delay).await;
            }

            // Silence between notes
            self.pwm.disable();

            // 4.2 Keep the output off for half a beat between notes
            Timer::after_millis(tempo).await;
            // 5. Go back to 1.
        }
    }

    pub fn stop(&mut self) {
        self.pwm.disable();
    }
}

// Make sure to stop the speaker when the program exits
impl<'d, T: pwm::Instance> Drop for Speaker<'d, T> {
    fn drop(&mut self) {
        self.stop()
    }
}
