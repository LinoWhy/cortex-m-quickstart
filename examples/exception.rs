//! Overriding an exception handler
//!
//! You can override an exception handler using the [`exception!`][1] macro.
//!
//! [1]: https://docs.rs/cortex-m-rt/0.5.0/cortex_m_rt/macro.exception.html
//!
//! ---

#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_abort;

use core::fmt::Write;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::{asm, Peripherals};
use rt::ExceptionFrame;
use sh::hio::{self, HStdout};

main!(main);

fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let mut syst = p.SYST;

    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000); // 1s
    syst.enable_counter();
    syst.enable_interrupt();

    loop {}
}

// try commenting out this line: you'll end in `deh` instead of in `sys_tick`
exception!(SysTick, sys_tick, state: Option<HStdout> = None);

fn sys_tick(state: &mut Option<HStdout>) {
    if state.is_none() {
        *state = Some(hio::hstdout().unwrap());
    }

    if let Some(hstdout) = state.as_mut() {
        hstdout.write_str(".").unwrap();
    }
}

exception!(DefaultHandler, deh);

#[inline(always)]
fn deh(_nr: u8) {
    asm::bkpt();
}

exception!(HardFault, hf);

#[inline(always)]
fn hf(_ef: &ExceptionFrame) -> ! {
    asm::bkpt();

    loop {}
}

interrupts!(DefaultHandler);
