#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::binary_info;
use embassy_rp::pwm::{Pwm, SetDutyCycle};
use embassy_time::Timer;
use panic_probe as _;

#[unsafe(link_section = ".bi_entries")]
#[used]
pub static METADATA: [binary_info::EntryAddr; 4] = [
    binary_info::rp_program_name!(c"Dimming External LED"),
    binary_info::rp_program_description!(c"Dimming External LED"),
    binary_info::rp_cargo_version!(),
    binary_info::rp_program_build_attribute!(),
];

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut pwm = Pwm::new_output_b(p.PWM_SLICE6, p.PIN_13, Default::default());

    loop {
        for i in 0..=100 {
            Timer::after_millis(8).await;
            let _ = pwm.set_duty_cycle(i);
        }
        for i in (0..=100).rev() {
            Timer::after_millis(8).await;
            let _ = pwm.set_duty_cycle(i);
        }
        Timer::after_millis(500).await;
    }
}
