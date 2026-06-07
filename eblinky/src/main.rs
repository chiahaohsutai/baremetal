#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::binary_info;
use embassy_rp::block::ImageDef;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[unsafe(link_section = ".start_block")]
#[used]
pub static IMGDEF: ImageDef = ImageDef::secure_exe();

#[unsafe(link_section = ".bi_entries")]
#[used]
pub static METADATA: [binary_info::EntryAddr; 4] = [
    binary_info::rp_program_name!(c"eBlinky"),
    binary_info::rp_program_description!(c"Toggle external LED on and off"),
    binary_info::rp_cargo_version!(),
    binary_info::rp_program_build_attribute!(),
];

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_13, Level::Low);

    loop {
        led.set_high();
        Timer::after_millis(500).await;
        led.set_low();
        Timer::after_millis(500).await;
    }
}
