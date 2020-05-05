//! Example that creates a square wave (alternating high and low) using the MCP4725 driver sending
//! fast-commands. This example is written and tested on the STM32f103 on the bluepill board.

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use hal::spi::{Mode, Phase, Polarity, Spi};
use hal::pac;
use hal::prelude::*;
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
    // let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    // Configure the pins for SPI2
    let pins = (
        gpiob.pb13.into_alternate_push_pull(&mut gpiob.crh),
        gpiob.pb14.into_floating_input(&mut gpiob.crh),
        gpiob.pb15.into_alternate_push_pull(&mut gpiob.crh),
    );

    // Configure SPI
    let spi_mode = Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    };

    let spi = Spi::spi2(dp.SPI2, pins, spi_mode, 100.khz(), clocks, &mut rcc.apb1);

    let dac = AD5668::new(spi);

    loop {
        // dac.send_stuff()
    }
}
