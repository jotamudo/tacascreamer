#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(alloc_error_handler)]
#![feature(error_in_core)]

mod gpio;
mod state;
mod alloc;
mod mcu;

use gpio::led;
use alloc as _;
use mcu::Mcu;
use state::AppTrace;

use cortex_m_rt::entry;
use cortex_m;
use stm32h7::stm32h743v::Peripherals;
use panic_halt as _;
// you can put a breakpoint on `rust_begin_unwind` to catch panics
use defmt_rtt as _; // defmt transport layer
use defmt::{info, debug};


#[entry]
fn main() -> ! {
    let mut trace = AppTrace::new();
    let per = Peripherals::take().unwrap();
    // let cortex = cortex_m::Peripherals::take().unwrap();

    // a simple hello blinky to be sure nothing is exploding
    let mcu = Mcu::new(per);
    match mcu.system_init() {
        Ok(_) => {trace.update(state::ApplicationState::Initialized)},
        Err(v) => {
            trace.update(state::ApplicationState::Panic);
            info!("{}", v)
        }
    }

    led::setup_gpios(&mcu.per);
    let mut counter = 0;
    loop {
        led::toggle_led(&mcu.per.GPIOC);
        info!("Hey :D {}", counter);
        debug!("{}", trace);
        trace.update(state::ApplicationState::Updating);
        counter += 1;
        for _ in 1..60000 {
            cortex_m::asm::nop();
        }
    }
}
