use core::fmt;
use embedded_hal::blocking::i2c::Write;

use crate::Register;

// Description

pub struct freq_fine {
    pub address: u8,
    value: u8,
}

impl fmt::Display for freq_fine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in self.value.iter() {
            write!(f, "{}", r)?;
        }

        Ok(())
    }
}

impl fmt::Binary for freq_fine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in self.value.iter() {
            write!(f, "{:b}", r)?;
        }

        Ok(())
    }
}

impl fmt::LowerHex for freq_fine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in self.value.iter() {
            fmt::LowerHex::fmt(&r, f)?;
        }

        Ok(())
    }
}


/// Sub-address of the register.
pub const ADDR: u8 = 0x63u8;


impl Register for freq_fine {}

impl freq_fine {
    pub fn new(value: u8, address: u8) -> Self {
        freq_fine { address, value }
    }


    pub fn read_freq<I2C>(&mut self, i2c: &mut I2C) -> Result<(), I2C::Error>
    where
        I2C: Write,
    {
        const RESET: u8 = 0b00000000;
        self.value &= !RESET;
        self.write(i2c, self.address, ADDR, self.value)
    }
}