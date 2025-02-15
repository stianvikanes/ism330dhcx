use core::convert::{TryFrom, TryInto};
use embedded_hal::blocking::i2c::WriteRead;

use crate::{parse_accelerometer, parse_gyroscope, Register};

#[repr(u8)]
pub enum SensorTag {
    Empty,
    GyroscopeNC,
    AccelerometerNC,
    Other(u8),
}

impl TryFrom<u8> for SensorTag {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            x if x == 0x00 => Ok(SensorTag::Empty),
            x if x == 0x01 => Ok(SensorTag::GyroscopeNC),
            x if x == 0x02 => Ok(SensorTag::AccelerometerNC),
            x if x <= 0x19 => Ok(SensorTag::Other(x)),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Debug, defmt::Format)]
pub enum Value {
    Empty,
    Gyro([f64; 3]),
    Accel([f64; 3]),
    Other(u8, [u8; 6]),
}

const ADDR: u8 = 0x78;

pub struct FifoOut {
    pub address: u8,
}

impl Register for FifoOut {}

impl FifoOut {
    pub fn new(address: u8) -> Self {
        FifoOut { address }
    }

    /// Pop a value from the FIFO.
    pub fn pop<I2C>(
        &mut self,
        i2c: &mut I2C,
        gyro_scale: f64,
        accel_scale: f64,
    ) -> Result<Value, I2C::Error>
    where
        I2C: WriteRead,
    {
        let mut out = [0u8; 7];
        i2c.write_read(self.address, &[ADDR], &mut out)?;

        let (tag, out) = out.split_at(1);
        let tag = tag[0] >> 3;
        let out: &[u8; 6] = out.try_into().expect("must be 6!");

        match tag.try_into() {
            Ok(SensorTag::Empty) => Ok(Value::Empty),
            Ok(SensorTag::GyroscopeNC) => Ok(Value::Gyro(parse_gyroscope(gyro_scale, out))),
            Ok(SensorTag::AccelerometerNC) => {
                Ok(Value::Accel(parse_accelerometer(accel_scale, out)))
            }
            Ok(SensorTag::Other(u)) => Ok(Value::Other(u, *out)),
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_hal_mock::i2c::{Mock, Transaction};

    #[test]
    fn test_pop_gyro() {
        let mut i2c = Mock::new(&[Transaction::write_read(
            0x6b,
            vec![0x78],
            vec![0x01 << 3, 0, 1, 0, 2, 0, 4],
        )]);

        let mut f = FifoOut::new(crate::DEFAULT_I2C_ADDRESS);
        let v = f.pop(&mut i2c, 1., 1.).unwrap();

        assert!(matches!(v, Value::Gyro(_)));
        println!("{:?}", v);
    }
}
