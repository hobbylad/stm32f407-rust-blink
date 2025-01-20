#![no_std]  // No standard library
#![no_main] // No entry point from the standard library

extern crate panic_halt; // Panic handler that stops the program

use stm32f4xx_hal::{
    pac,
    prelude::*,
    rcc::RccExt,
    delay::Delay
}; // STM32F4 HAL crate

use cortex_m_rt::entry;
#[entry]  // Entry point of our application
fn main() -> ! {

    // Try consume device and core peripherals
    if let (Some(peripheral), Some(cortex_peripheral)) = (pac::Peripherals::take(), cortex_m::Peripherals::take()) {

        // Splot GPIOA into individual GPIO lines
        let gpioa = peripheral.GPIOA.split();

        // Initialize on-board LEDs
        let mut led1 = gpioa.pa6.into_push_pull_output();
        let mut led2 = gpioa.pa7.into_push_pull_output();

        // LED1 off and LED2 on (Inverted logic)
        led1.set_high();
        led2.set_low();

        // Constrain clock registers to make start in an know operating mode
        let rcc = peripheral.RCC.constrain();

        // Configure clock to 168 MHz
        let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

        // Get delay object
        let mut delay = Delay::new(cortex_peripheral.SYST, &clocks);

        loop {
            // LED1 off and LED2 on
            led1.set_high();
            led2.set_low();
            delay.delay_ms(200u16);

            // LED1 on and LED2 off
            led1.set_low();
            led2.set_high();
            delay.delay_ms(200u16);
        }
    }

    // Wait forever if error obtaining peripherals
    loop {
        continue;
    }
}
