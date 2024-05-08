//! Handles most low level hardware abstraction

use cortex_m::delay::Delay;
use rp2040_hal::{clocks::init_clocks_and_plls, pac::{self, PIO0, PIO1}, usb::UsbBus, Clock, Sio, Watchdog};
use usb_device::class_prelude::UsbBusAllocator;

use crate::{pio::Pio, usb_manager::UsbManager};

static mut SINGLETON: Option<Hardware> = None;

pub struct Hardware {
    delay: Delay,
    pins: rp2040_hal::gpio::Pins,
    pio0: Pio<PIO0>,
    pio1: Pio<PIO1>,
    usb: Option<UsbManager>,
    usb_bus: UsbBusAllocator<UsbBus>,
}

impl Hardware {
    /// Initialize RP2040 hardware
    pub fn init(crystal_frequency: u32) {
        critical_section::with(|_| {
            let mut pac = pac::Peripherals::take().unwrap();
            let core = pac::CorePeripherals::take().unwrap();
            let mut watchdog = Watchdog::new(pac.WATCHDOG);
            let sio = Sio::new(pac.SIO);

            let clocks = init_clocks_and_plls(
                crystal_frequency,
                pac.XOSC,
                pac.CLOCKS,
                pac.PLL_SYS,
                pac.PLL_USB,
                &mut pac.RESETS,
                &mut watchdog,
            )
            .ok()
            .unwrap();

            let delay;
            let usb;
            let usb_bus;

            unsafe {
                delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

                usb_bus = UsbBusAllocator::new(UsbBus::new(
                    pac.USBCTRL_REGS,
                    pac.USBCTRL_DPRAM,
                    clocks.usb_clock,
                    true,
                    &mut pac.RESETS,
                ));

                // Enable the USB interrupt
                pac::NVIC::unmask(rp2040_hal::pac::Interrupt::USBCTRL_IRQ);
            };

            let pins = rp2040_hal::gpio::Pins::new(
                pac.IO_BANK0,
                pac.PADS_BANK0,
                sio.gpio_bank0,
                &mut pac.RESETS,
            );

            let pio0 = Pio::new(pac.PIO0, &mut pac.RESETS);
            let pio1 = Pio::new(pac.PIO1, &mut pac.RESETS);

            unsafe {
                SINGLETON = Some(Hardware {
                    delay,
                    pins,
                    pio0,
                    pio1,
                    usb: None,
                    usb_bus,
                });

                usb = UsbManager::new(&SINGLETON.as_ref().unwrap().usb_bus);

                SINGLETON.as_mut().unwrap().usb = Some(usb);
            }
        })
    }

    /// Get the hardware singleton
    pub fn get() -> Option<&'static mut Self> {
        unsafe { SINGLETON.as_mut() }
    }

    pub fn get_delay(&mut self) -> &mut Delay {
        &mut self.delay
    }

    pub fn get_pins(&mut self) -> &mut rp2040_hal::gpio::Pins {
        &mut self.pins
    }

    pub fn get_pio0(&mut self) -> &mut Pio<PIO0> {
        &mut self.pio0
    }

    pub fn get_pio1(&mut self) -> &mut Pio<PIO1> {
        &mut self.pio1
    }

    pub fn get_usb(&mut self) -> &mut UsbManager {
        // It should be impossible for this to be None, panic if it is
        self.usb.as_mut().unwrap()
    }
}
