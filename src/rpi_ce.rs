use std::io;

use rppal::gpio::{Level, Gpio, OutputPin};

pub struct CEPin {
    pin: OutputPin,
    value: Level,
    saved_value: Level,
}

impl CEPin {
    pub fn new(pin_num: u64) -> io::Result<CEPin> {
        let gpio = Gpio::new().unwrap();
        let pin = gpio.get(pin_num as u8).unwrap().into_output();
        Ok(CEPin {
            pin,
            value: Level::Low,
            saved_value: Level::Low,
        })
    }

    pub fn up(&mut self) -> io::Result<()> {
        self.pin.write(Level::High);
        self.value = Level::High;
        Ok(())
    }

    pub fn down(&mut self) -> io::Result<()> {
        self.pin.write(Level::Low);
        self.value = Level::Low;
        Ok(())
    }

    pub fn save_state(&mut self) -> () {
        self.saved_value = self.value;
    }

    pub fn restore_state(&mut self) -> io::Result<()> {
        self.pin.write(self.saved_value);
        self.value = self.saved_value;
        Ok(())
    }
}
