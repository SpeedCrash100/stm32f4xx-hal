//! Demonstrate the use of a blocking `Delay` using TIM5 general-purpose timer.

#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, stm32};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(_cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Mini-F4 it's connected to pin PC13.
        let gpioc = dp.GPIOC.split();
        let mut led = gpioc.pc13.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Create a delay abstraction based on general-pupose 32-bit timer TIM5
        let mut delay = hal::delay::Delay::tim5(dp.TIM5, &clocks);

        loop {
            // On for 1s, off for 1s.
            led.set_high();
            delay.delay_ms(1_000_u32);
            led.set_low();
            delay.delay_us(1_000_000_u32);
        }
    }

    loop {}
}
