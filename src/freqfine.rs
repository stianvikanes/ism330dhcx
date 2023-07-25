use core::fmt;
use embedded_hal::blocking::i2c::{Write, WriteRead};

use crate::Register;

// Description

pub struct FreqFine {
    pub address: u8,
    value: u8,
}

impl fmt::Display for FreqFine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Binary for FreqFine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}", self.value)
    }
}

impl fmt::LowerHex for FreqFine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.value, f)
    }
}


/// Sub-address of the register.
pub const ADDR: u8 = 0x63u8;


impl Register for FreqFine {}

impl FreqFine {
    pub fn new(value: u8, address: u8) -> Self {
        FreqFine { address, value }
    }


    pub fn read_freq<I2C>(&mut self, i2c: &mut I2C) -> Result<u8, I2C::Error>
    where
        I2C: WriteRead,
    {
        const RESET: u8 = 0b00000000;
        self.value &= !RESET;
        let actual: u8 = self.read(i2c, self.address, ADDR)?;
        Ok(actual)
    }
}