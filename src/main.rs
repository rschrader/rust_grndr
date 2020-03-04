//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate cortex_m_semihosting;  //  Debug console functions for ARM Cortex-M3.

use nb::block;
use embedded_graphics::{
    fonts::{Font12x16, Font24x32, Text},
    image::{Image},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyleBuilder,
};
use ssd1306::{prelude::*, Builder};
use stm32f1xx_hal as _;
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    prelude::*,
    pac,
    timer::Timer,
};
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use rt::exception;
use embedded_hal::digital::v2::OutputPin;
use numtoa::NumToA;

#[entry]
fn main() -> ! {
    hprintln!("This is the Rust grindr!").unwrap();

    let cp = cortex_m::Peripherals::take().unwrap();    // get core peripherals
    let dp = pac::Peripherals::take().unwrap();         // get peripheral access crate
    
    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOC peripheral
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, &clocks).start_count_down(2.hz());

    // show some text

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    let mut disp: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();
    disp.init().unwrap();

    let text_style = TextStyleBuilder::new(Font12x16)
        .text_color(BinaryColor::On)
        .build();

    let text_style_weight = TextStyleBuilder::new(Font24x32)
        .text_color(BinaryColor::On)
        .build();


    // show schrawerner logo
    disp.clear();
    let im: Image<BinaryColor> = 
        Image::new(include_bytes!("./splash.raw"), 64, 64).translate(Point::new(0, 0));
    im.draw(&mut disp);
    disp.flush().unwrap();

    
    block!(timer.wait()).unwrap();
    block!(timer.wait()).unwrap();
    block!(timer.wait()).unwrap();
    block!(timer.wait()).unwrap();
    block!(timer.wait()).unwrap();
    block!(timer.wait()).unwrap();
    block!(timer.wait()).unwrap();
    block!(timer.wait()).unwrap();
    block!(timer.wait()).unwrap();
    block!(timer.wait()).unwrap();


    // Wait for the timer to trigger an update and change the state of the LED
    let mut loop_counter = 0;
    let mut numbuffer = [0u8; 6];
    loop {
        block!(timer.wait()).unwrap();
        led.set_high().unwrap();
        block!(timer.wait()).unwrap();
        led.set_low().unwrap();

        disp.clear();

        Text::new(loop_counter.numtoa_str(10, &mut numbuffer), Point::new(0, 32))
            .into_styled(text_style_weight)
            .draw(&mut disp);

        Text::new("g", Point::new(104, 32))
            .into_styled(text_style_weight)
            .draw(&mut disp);

        Text::new("Grinding...", Point::zero())
            .into_styled(text_style)
            .draw(&mut disp);

        disp.flush().unwrap();

        loop_counter = loop_counter +1;
    }
}

#[exception]
fn HardFault(ef: &cortex_m_rt::ExceptionFrame) -> ! {
    // prints the exception frame as a panic message
    panic!("{:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
