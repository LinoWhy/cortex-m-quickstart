#![no_std]
#![no_main]

use cortex_m_semihosting::{debug, hprintln};
// pick a panicking behavior
#[cfg(debug_assertions)]
use panic_semihosting as _;

use cortex_m::peripheral::{syst, Peripherals};
use cortex_m_rt::entry;
// use tm4c123x;
use tm4c123x_hal as hal;
use tm4c123x_hal::prelude::*;
use tm4c123x_hal::serial::{NewlineMode, Serial};
use tm4c123x_hal::sysctl;

#[entry]
fn main() -> ! {
    {
        let peripherals = Peripherals::take().unwrap();
        let mut systick = peripherals.SYST;
        systick.set_clock_source(syst::SystClkSource::Core);
        systick.set_reload(1_000);
        systick.clear_current();
        systick.enable_counter();
        while !systick.has_wrapped() {
            // unimplemented!();
        }
        hprintln!("sys timer elapsed!");
    }

    {
        let p = tm4c123x::Peripherals::take().unwrap();
        let pwm = p.PWM0;
        pwm.ctl.write(|w| w.globalsync0().clear_bit());
        // Mode = 1 => Count up/down mode
        pwm._2_ctl.write(|w| w.enable().set_bit().mode().set_bit());
        pwm._2_gena.write(|w| w.actcmpau().zero().actcmpad().one());
        // 528 cycles (264 up and down) = 4 loops per video line (2112 cycles)
        pwm._2_load.write(|w| unsafe { w.load().bits(263) });
        pwm._2_cmpa.write(|w| unsafe { w.compa().bits(64) });
        pwm.enable.write(|w| w.pwm4en().set_bit());

        if pwm.ctl.read().globalsync0().bit_is_clear() {
            hprintln!("pwm0 globalsync0 is cleared!");
        }
    }

    // {
    //     let p = hal::Peripherals::take().unwrap();
    //     let _cp = hal::CorePeripherals::take().unwrap();

    //     let mut sc = p.SYSCTL.constrain();
    //     sc.clock_setup.oscillator = sysctl::Oscillator::Main(
    //         sysctl::CrystalFrequency::_16mhz,
    //         sysctl::SystemClock::UsePll(sysctl::PllOutputFrequency::_80_00mhz),
    //     );
    //     let clocks = sc.clock_setup.freeze();

    //     let mut porta = p.GPIO_PORTA.split(&sc.power_control);

    //     let _uart = Serial::uart0(
    //         p.UART0,
    //         porta
    //             .pa1
    //             .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
    //         porta
    //             .pa0
    //             .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
    //         (),
    //         (),
    //         115200_u32.bps(),
    //         NewlineMode::SwapLFtoCRLF,
    //         &clocks,
    //         &sc.power_control,
    //     );
    // }

    let xs = [0, 1, 2];
    let i = xs.len() + 1;
    let _y = xs[i];

    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}
