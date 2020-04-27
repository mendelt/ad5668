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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
