//! Example that creates a square wave (alternating high and low) using the MCP4725 driver sending
//! fast-commands. This example is written and tested on the STM32f103 on the bluepill board.

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use hal::pac;
use hal::prelude::*;
use hal::spi::{Mode, NoMiso, Phase, Polarity, Spi};
use hal::time::U32Ext;

use ad5668::*;
#[allow(unused_imports)]
use panic_semihosting;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure the clock
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    // Configure the pins for SPI2
    let spi2_mosi = gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh);
    let spi2_sck = gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh);
    let mut spi2_cs = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);

    // Initialize chip select high
    spi2_cs.set_high().ok();

    // Configure SPI
    let spi_mode = Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    };

    let spi = Spi::spi2(
        dp.SPI2,
        (spi2_sck, NoMiso, spi2_mosi),
        spi_mode,
        100.khz(),
        clocks,
        &mut rcc.apb1,
    );
    let mut dac = AD5668::new(spi, spi2_cs);

    dac.enable_internal_ref();

    loop {
        dac.write_and_update_dac_channel(Address::AllDacs, 0x0000u16)
            .unwrap();
        dac.write_and_update_dac_channel(Address::AllDacs, 0xffffu16)
            .unwrap();
    }
}
