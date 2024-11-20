use embassy_nrf::gpio::{AnyPin, Flex, Level, Output, OutputDrive, Pull};
use embassy_time::Timer;

mod numbers;
use numbers::*;

pub struct DisplayPins<'a> {
    cols: [Flex<'a>; 5],
    rows: [Output<'a>; 5],
}

pub struct Columns {
    pub col1: AnyPin,
    pub col2: AnyPin,
    pub col3: AnyPin,
    pub col4: AnyPin,
    pub col5: AnyPin,
}

pub struct Rows {
    pub row1: AnyPin,
    pub row2: AnyPin,
    pub row3: AnyPin,
    pub row4: AnyPin,
    pub row5: AnyPin,
}

impl DisplayPins<'_> {
    pub fn new<'a>(cols: Columns, rows: Rows) -> DisplayPins<'a> {
        let mut cols = [
            Flex::new(cols.col1),
            Flex::new(cols.col2),
            Flex::new(cols.col3),
            Flex::new(cols.col4),
            Flex::new(cols.col5),
        ];

        cols.iter_mut()
            .for_each(|c| c.set_as_input_output(Pull::Up, OutputDrive::Standard0Disconnect1));

        let rows = [
            Output::new(rows.row1, Level::Low, OutputDrive::Standard),
            Output::new(rows.row2, Level::Low, OutputDrive::Standard),
            Output::new(rows.row3, Level::Low, OutputDrive::Standard),
            Output::new(rows.row4, Level::Low, OutputDrive::Standard),
            Output::new(rows.row5, Level::Low, OutputDrive::Standard),
        ];

        DisplayPins { cols, rows }
    }

    pub async fn render(&mut self, number: u32) {
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
