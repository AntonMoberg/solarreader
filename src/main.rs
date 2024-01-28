#![no_std]
#![no_main]

use core::fmt::Write;

use bsp::entry;
use defmt_serial as _;
use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    uart::*,
    Watchdog,
};

use fugit::RateExtU32;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // uart declaration
    let mut uart = bsp::hal::uart::UartPeripheral::new(
        // using the first UART channel (pins 0 and 1)
        pac.UART0,
        // pins allocation for UART
        (pins.gpio0.into_function(), pins.gpio1.into_function()),
        &mut pac.RESETS,
    )
    .enable(
        // these configs we'll be using on the serial receiver.
        UartConfig::new(9600.Hz(), DataBits::Eight, None, StopBits::One),
        clocks.peripheral_clock.freq(),
    )
    .unwrap();

    // MAIN PROGRAM
    let mut seconds = 0;
    uart.write_raw(b"Hello World!\n").unwrap();
    loop {
        uart.write_fmt(format_args!("Seconds since start: {}s\n", seconds))
            .unwrap();
        delay.delay_ms(1000);
        seconds += 1;
    }
}
