use embassy_nrf::gpio::{AnyPin, Flex, Level, Output, OutputDrive, Pull};
use embassy_time::Timer;

use crate::numbers::*;

pub struct DisplayPins<'a> {
    cols: [Flex<'a>; 5],
    rows: [Output<'a>; 5],
}

impl DisplayPins<'_> {
    pub fn new<'a>(
        col1: AnyPin,
        col2: AnyPin,
        col3: AnyPin,
        col4: AnyPin,
        col5: AnyPin,
        row1: AnyPin,
        row2: AnyPin,
        row3: AnyPin,
        row4: AnyPin,
        row5: AnyPin,
    ) -> DisplayPins<'a> {
        let mut cols = [
            Flex::new(col1),
            Flex::new(col2),
            Flex::new(col3),
            Flex::new(col4),
            Flex::new(col5),
        ];

        cols.iter_mut()
            .for_each(|c| c.set_as_input_output(Pull::Up, OutputDrive::Standard0Disconnect1));

        let rows = [
            Output::new(row1, Level::Low, OutputDrive::Standard),
            Output::new(row2, Level::Low, OutputDrive::Standard),
            Output::new(row3, Level::Low, OutputDrive::Standard),
            Output::new(row4, Level::Low, OutputDrive::Standard),
            Output::new(row5, Level::Low, OutputDrive::Standard),
        ];

        DisplayPins { cols, rows }
    }

    pub async fn render(&mut self, number: usize) {
        let frame = match number {
            0 => LED0,
            1 => LED1,
            2 => LED2,
            3 => LED3,
            4 => LED4,
            5 => LED5,
            6 => LED6,
            7 => LED7,
            8 => LED8,
            9 => LED9,
            _ => LED9,
        };

        for _ in 0..(100 / self.rows.len()) {
            for (row, values) in self.rows.iter_mut().zip(frame.iter()) {
                row.set_high();
                // open columns
                for (col, status) in self.cols.iter_mut().zip(values.iter()) {
                    if *status == 1 {
                        col.set_low();
                    } else {
                        col.set_high();
                    }
                }

                // activate
                Timer::after_millis(1).await;

                // deactivate columns and line
                self.cols.iter_mut().for_each(|c| c.set_high());
                row.set_low();
            }
        }
    }
}
