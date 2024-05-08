#![no_std]
#![no_main]
#[link_section = ".boot2"]
#[no_mangle]
#[used]
static BOOT_LOADER: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

#[cfg(feature = "rt")]
extern crate cortex_m_rt;

extern crate alloc;

pub mod hardware;
    use core::mem::MaybeUninit;
pub mod pio;
pub mod tx;
pub mod rx;
pub mod serial_logger;
pub mod state_machine;
pub mod usb_manager;

use embedded_alloc::Heap;
use hardware::Hardware;
use log::info;
use panic_reset as _;
use rp2040_hal::entry;
use serial_logger::SerialLogger;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    init_allocator();

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

fn init_allocator() {
    const HEAP_SIZE: usize = 128 * 1024;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}
