#![no_std]
#![no_main]

use arduino_nano_connect as bsp;
use bsp::entry;
use bsp::hal::prelude::*;
use bsp::{hal, hal::pac};

use embedded_hal::digital::{OutputPin,PinState};

use panic_halt as _;

#[entry]
fn main() -> ! {

    let mut peri = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let sio = hal::Sio::new(peri.SIO);
    //pins
    let pins = bsp::Pins::new(
        peri.IO_BANK0,
        peri.PADS_BANK0,
        sio.gpio_bank0,
        &mut peri.RESETS
    );

    let mut watchdog = hal::Watchdog::new(peri.WATCHDOG);
    let clocks = hal::clocks::init_clocks_and_plls(
        bsp::XOSC_CRYSTAL_FREQ,
        peri.XOSC,
        peri.CLOCKS,
        peri.PLL_SYS,
        peri.PLL_USB,
        &mut peri.RESETS,
        &mut watchdog
    ).ok()
    .unwrap();
    
    let mut delay = cortex_m::delay::Delay::new(core.SYST,clocks.system_clock.freq().to_Hz());
    
    let mut led = pins.d4.into_push_pull_output_in_state(PinState::Low);
    loop {
        delay.delay_ms(500);
        led.set_high().unwrap();
        delay.delay_ms(500);
        led.set_low().unwrap();
      }
}
