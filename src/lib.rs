#![no_std]

#![no_std]
#[warn(missing_debug_implementations, missing_docs)]
use embedded_hal::blocking::spi::Write;

/// MCP4725 DAC driver. Wraps an I2C port to send commands to an MCP4725
pub struct AD5668<SPI>
{
    spi: SPI,
}

impl<SPI, E> AD5668<SPI> 
where SPI: Write<u16, Error = E>
{
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }



    // TODO send stuff to the DAC
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
enum Address {
    DacA = 0b0000,
    DacB = 0b0001,
    DacC = 0b0010,
    DacD = 0b0011,
    DacE = 0b0100,
    DacF = 0b0101,
    DacG = 0b0110,
    DacH = 0b0111,
    AllDacs = 0b1111,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
enum Command {
    WriteInputRegister = 0b0000,
    UpdateDacRegister = 0b0001,
    WriteInputUpdateAll = 0b0010,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
