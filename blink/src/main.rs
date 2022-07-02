#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

const LED_PIN: u32 = 1 << 7;

#[entry]
fn main() -> ! {
    let core = cortex_m::peripheral::Peripherals::take().unwrap();
    let peripherals = atsaml10e16a::Peripherals::take().unwrap();

    // Clock should be enabled by default.

    // Set pin 1 to be an output.
    unsafe {
        peripherals.PORT.group[0].dir.write(|w| w.bits(LED_PIN));
    }

    let mut delay = cortex_m::delay::Delay::new(core.SYST, 4_000_000);

    loop {
        unsafe {
            peripherals.PORT.group[0].outtgl.write(|w| w.bits(LED_PIN));
        }
        delay.delay_ms(1000);
    }
}
