#![no_std]
#![no_main]
#[link_section = ".boot2"]
#[no_mangle]
#[used]
static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;

pub mod hardware;
pub mod pio;
pub mod tx;
pub mod rx;
pub mod serial_logger;
pub mod state_machine;
pub mod usb_manager;

use hardware::Hardware;
use log::info;
use panic_reset as _;
use rp2040_hal::entry;
use serial_logger::SerialLogger;

#[entry]
fn main() -> ! {
    let crystal_frequency = 12_000_000;
    Hardware::init(crystal_frequency);
    let hardware = Hardware::get().unwrap();

    SerialLogger::init(log::LevelFilter::Info);

    let mut number: u32 = 0;

    loop {
        info!("Running for {number} seconds");
        number += 1;

        hardware.get_delay().delay_ms(1000);
    }
}
