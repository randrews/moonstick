#![no_std]
#![no_main]

use bsp::entry;
use defmt::*;
use defmt_rtt as _;
use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{
    clocks::init_clocks_and_plls,
    pac,
    sio::Sio,
    watchdog::Watchdog,
    gpio::{Pins}
};

use embedded_graphics::Drawable;
use embedded_graphics::geometry::Point;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::mono_font::MonoTextStyleBuilder;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::text::Text;
use fugit::RateExtU32;
use rp_pico::hal::gpio::{FunctionI2C, PullUp};
use ssd1306::I2CDisplayInterface;
use ssd1306::prelude::{DisplayConfig, DisplayRotation};
use ssd1306::size::DisplaySize128x32;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok().unwrap();
    let mut timer = rp_pico::hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let pins = Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);

    let sda = pins.gpio8.into_function::<FunctionI2C>().into_pull_type::<PullUp>();
    let scl = pins.gpio9.into_function::<FunctionI2C>().into_pull_type::<PullUp>();
    let i2c = rp_pico::hal::I2C::i2c0(pac.I2C0, sda, scl, 400.kHz(), &mut pac.RESETS, &clocks.peripheral_clock);


    let style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();


    let interface = I2CDisplayInterface::new_custom_address(i2c, 0x3c);
    let mut ssd = ssd1306::Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0).into_buffered_graphics_mode();

    ssd.init().unwrap();
    info!("Initted");

    loop {
        Text::new("Hello", Point::new(64, 16), style).draw(&mut ssd).unwrap();
        ssd.flush().unwrap();
    }
}
