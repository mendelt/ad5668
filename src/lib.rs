//! *Analog Devices AD5668 DAC Driver for Rust Embedded HAL*
//!
//! This is a driver crate for embedded Rust. It's built on top of the Rust
//! [embedded HAL](https://github.com/rust-embedded/embedded-hal)
//! It supports sending commands to a AD5668 DAC over SPI.
//!
//! First you create an instance of the driver wrapping the SPI port the DAC is connected to;
//! ```
//! # use embedded_hal_mock::{spi, pin};
//! # use ad5668::*;
//! # let mut spi = spi::Mock::new(&[]);
//! # let mut chip_select = pin::Mock::new(&[pin::Transaction::set(pin::State::High)]);
//! let mut dac = AD5668::new(spi, chip_select);
//! ```
//!
//! Now commands can be sent to the DAC, for example to set all outputs high
//! ```
//! # use embedded_hal_mock::{spi, pin};
//! # use ad5668::*;
//! # let mut spi = spi::Mock::new(&[spi::Transaction::write(vec![0x02, 0xff, 0xff, 0xf0]),]);
//! # let mut chip_select = pin::Mock::new(&[
//! #     pin::Transaction::set(pin::State::High),
//! #     pin::Transaction::set(pin::State::Low),
//! #     pin::Transaction::set(pin::State::High),
//! # ]);
//! # let mut dac = AD5668::new(spi, chip_select);
//! dac.write_input_register_update_all(Address::AllDacs, 0xffff);
//! ```
//!
//! ## More information
//! - [AD5668 datasheet](https://www.analog.com/media/en/technical-documentation/data-sheets/AD5628_5648_5668.pdf)
//! - [API documentation](https://docs.rs/ad5668)
//! - [Github repository](https://github.com/mendelt/ad5668)
//! - [Crates.io](https://crates.io/crates/ad5668)

#![no_std]
#[warn(missing_debug_implementations, missing_docs)]
use embedded_hal::{blocking::spi::Write, digital::v2::OutputPin};

/// AD5668 DAC driver. Wraps an I2C port to send commands to an AD5668
pub struct AD5668<SPI, CS> {
    spi: SPI,
    chip_select: CS,
}

impl<SPI, CS, E> AD5668<SPI, CS>
where
    SPI: Write<u8, Error = E>,
    CS: OutputPin,
{
    /// Construct a new AD5668 driver
    pub fn new(spi: SPI, mut chip_select: CS) -> Self {
        // Init chip select high
        chip_select.set_high().ok();

        Self { spi, chip_select }
    }

    /// Helper function that handles writing to the SPI bus while toggeling chip select
    fn write_spi(&mut self, data: &[u8]) -> Result<(), E> {
        self.chip_select.set_low().ok();
        let result = self.spi.write(data);
        self.chip_select.set_high().ok();
        result
    }

    /// Write input register for the dac at address with the value, does not update dac register yet
    pub fn write_input_register(&mut self, address: Address, value: u16) -> Result<(), E> {
        self.write_spi(&encode_update_command(
            Command::WriteInputRegister,
            address,
            value,
        ))
    }

    /// Update dac register for the dac at address
    /// TODO: Check if the data is written too or if this just updates data written earlier to the
    ///       dac
    pub fn update_dac_register(&mut self, address: Address, value: u16) -> Result<(), E> {
        self.write_spi(&encode_update_command(
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
        self.write_spi(&encode_update_command(
            Command::WriteInputUpdateAll,
            address,
            value,
        ))
    }

    /// Write to input register and then update the dac register in one command.
    pub fn write_and_update_dac_channel(&mut self, address: Address, value: u16) -> Result<(), E> {
        self.write_spi(&encode_update_command(
            Command::WriteUpdateDacChannel,
            address,
            value,
        ))
    }

    /// Enable the internal reference
    pub fn enable_internal_ref(&mut self) -> Result<(), E> {
        self.write_spi(&[
            Command::SetInternalRefRegister as u8,
            0x00u8,
            0x00u8,
            InternalRef::Enabled as u8,
        ])
    }

    /// Disable the internal reference
    pub fn disable_internal_ref(&mut self) -> Result<(), E> {
        self.write_spi(&[
            Command::SetInternalRefRegister as u8,
            0x00u8,
            0x00u8,
            InternalRef::Disabled as u8,
        ])
    }

    /// Reset the DAC
    pub fn reset(&mut self) -> Result<(), E> {
        self.write_spi(&[Command::Reset as u8, 0x00u8, 0x00u8, 0x00u8])
    }

    /// Destroy the driver and return the wrapped SPI driver to be re-used
    pub fn destroy(self) -> (SPI, CS) {
        (self.spi, self.chip_select)
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
pub enum InternalRef {
    Disabled = 0x00u8,
    Enabled = 0x01u8,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
enum Command {
    WriteInputRegister = 0b0000,
    UpdateDacRegister = 0b0001,
    WriteInputUpdateAll = 0b0010,
    WriteUpdateDacChannel = 0b0011,
    PowerDACUpDown = 0b0100,
    LoadClearCodeRegister = 0b0101,
    LoadLDACRegister = 0b0110,
    Reset = 0b0111,
    SetInternalRefRegister = 0b1000,
}

#[cfg(test)]
mod test {
    use super::*;
    use embedded_hal_mock::{pin, spi};

    extern crate std;
    use std::vec;

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

    fn setup_mocks() -> (spi::Mock, pin::Mock) {
        let spi = spi::Mock::new(&[]);

        // Default cs expectations, new sets high, sending command toggles low, then high
        let chip_select = pin::Mock::new(&[
            pin::Transaction::set(pin::State::High),
            pin::Transaction::set(pin::State::Low),
            pin::Transaction::set(pin::State::High),
        ]);

        (spi, chip_select)
    }

    #[test]
    pub fn should_init_chip_select_high() {
        let (spi, mut chip_select) = setup_mocks();

        chip_select.expect(&[pin::Transaction::set(pin::State::High)]);

        let _dac = AD5668::new(spi, chip_select);
    }

    #[test]
    pub fn should_enable_internal_ref() {
        let (mut spi, chip_select) = setup_mocks();

        spi.expect(&[spi::Transaction::write(vec![
            0x08u8, 0x00u8, 0x00u8, 0x01u8,
        ])]);

        let mut dac = AD5668::new(spi, chip_select);

        dac.enable_internal_ref().unwrap();
    }

    #[test]
    pub fn should_disable_internal_ref() {
        let (mut spi, chip_select) = setup_mocks();

        spi.expect(&[spi::Transaction::write(vec![
            0x08u8, 0x00u8, 0x00u8, 0x00u8,
        ])]);

        let mut dac = AD5668::new(spi, chip_select);

        dac.disable_internal_ref().unwrap();
    }

    #[test]
    pub fn should_send_reset_command() {
        let (mut spi, chip_select) = setup_mocks();

        spi.expect(&[spi::Transaction::write(vec![
            0x07u8, 0x00u8, 0x00u8, 0x00u8,
        ])]);

        let mut dac = AD5668::new(spi, chip_select);

        dac.reset().unwrap();
    }
}
