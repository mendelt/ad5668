#![no_std]
#[warn(missing_debug_implementations, missing_docs)]
use embedded_hal::blocking::spi::Write;

/// AD5668 DAC driver. Wraps an I2C port to send commands to an AD5668
pub struct AD5668<SPI> {
    spi: SPI,
}

impl<SPI, E> AD5668<SPI>
where
    SPI: Write<u8, Error = E>,
{
    pub fn new(spi: SPI) -> Self {
        Self { spi }
    }

    /// Write input register for the dac at address with the value, does not update dac register yet
    pub fn write_input_register(&mut self, address: Address, value: u16) -> Result<(), E> {
        self.spi.write(&encode_update_command(
            Command::WriteInputRegister,
            address,
            value,
        ))
    }

    /// Update dac register for the dac at address
    /// TODO: Check if the data is written too or if this just updates data written earlier to the
    ///       dac
    pub fn update_dac_register(&mut self, address: Address, value: u16) -> Result<(), E> {
        self.spi.write(&encode_update_command(
            Command::UpdateDacRegister,
            address,
            value,
        ))
    }

    /// Write to a single input register, then update all dac channels. This can be used as the last
    /// command when updating multiple DACs. First stage values for all DACs then update them
    /// simultaniously by performing the last write using this command
    pub fn write_input_register_update_all(
        &mut self,
        address: Address,
        value: u16,
    ) -> Result<(), E> {
        self.spi.write(&encode_update_command(
            Command::WriteInputUpdateAll,
            address,
            value,
        ))
    }

    /// Write to input register and then update the dac register in one command.
    pub fn write_and_update_dac_channel(&mut self, address: Address, value: u16) -> Result<(), E> {
        self.spi.write(&encode_update_command(
            Command::WriteUpdateDacChannel,
            address,
            value,
        ))
    }
}

/// Encodes one of the commands that updates a 16 bit value
fn encode_update_command(command: Command, address: Address, value: u16) -> [u8; 4] {
    [
        command as u8,
        ((address as u8) << 4) + (value >> 12) as u8,
        (value >> 4) as u8,
        (value << 4) as u8,
    ]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn should_encode_command() {
        assert_eq!(
            encode_update_command(Command::WriteUpdateDacChannel, Address::DacA, 0u16),
            [0b00000011, 0b00000000, 0b00000000, 0b00000000],
        )
    }

    #[test]
    pub fn should_encode_address() {
        assert_eq!(
            encode_update_command(Command::WriteInputRegister, Address::AllDacs, 0u16),
            [0b00000000, 0b11110000, 0b00000000, 0b00000000],
        )
    }

    #[test]
    pub fn should_encode_value() {
        assert_eq!(
            encode_update_command(Command::WriteInputRegister, Address::DacA, 0xffffu16),
            [0b00000000, 0b00001111, 0b11111111, 0b11110000],
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Address {
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
    WriteUpdateDacChannel = 0b0011,
}
