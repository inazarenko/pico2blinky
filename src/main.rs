#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

fn set_stack_limit(stack_size_max: u32) {
    unsafe extern "C" {
        static _stack_start: u32;
        static _stack_end: u32;
    }
    let stack_start = &raw const _stack_start as u32;
    let stack_end = &raw const _stack_end as u32;
    debug!("stack space: 0x{=u32:x}:{=u32:x}", stack_start, stack_end);
    let size = core::cmp::min(stack_start - stack_end, stack_size_max);
    let stack_limit = stack_start - size;
    unsafe { cortex_m::register::msplim::write(stack_limit) };
    debug!("stack limit: 0x{=u32:x} ({=u32} bytes)", stack_limit, size);
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    set_stack_limit(64 * 1024);
    let p = embassy_rp::init(Default::default());

    let mut led = Output::new(p.PIN_16, Level::Low);
    loop {
        info!("led on!");
        led.set_high();
        Timer::after_millis(250).await;

        info!("led off!");
        led.set_low();
        Timer::after_millis(250).await;
    }
}

#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"Blinky Example"),
    embassy_rp::binary_info::rp_program_description!(c"Blinks pin 16 at 2Hz"),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];
