//! Handles most low level hardware abstraction

use core::cell::RefCell;

use cortex_m::delay::Delay;
use rp2040_hal::{clocks::init_clocks_and_plls, pac::{self, PIO0, PIO1}, usb::UsbBus, Clock, Sio, Watchdog};
use usb_device::class_prelude::UsbBusAllocator;

use crate::{pio::Pio, usb_manager::UsbManager};

static mut SINGLETON: Option<Hardware> = None;

#[derive(Debug)]
pub enum Error {
    AttemptToReturnExistingValue,
}

pub struct Hardware {
    delay: RefCell<Option<Delay>>,
    pins: rp2040_hal::gpio::Pins,
    pio0: RefCell<Option<Pio<PIO0>>>,
    pio1: RefCell<Option<Pio<PIO1>>>,
    usb: RefCell<Option<UsbManager>>,
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
                    delay: RefCell::new(Some(delay)),
                    pins,
                    pio0: RefCell::new(Some(pio0)),
                    pio1: RefCell::new(Some(pio1)),
                    usb: RefCell::new(None),
                    usb_bus,
                });

                usb = UsbManager::new(&SINGLETON.as_ref().unwrap().usb_bus);

                SINGLETON.as_mut().unwrap().usb = RefCell::new(Some(usb));
            }
        })
    }

    /// Get the hardware singleton
    pub fn get() -> Option<&'static mut Self> {
        unsafe { SINGLETON.as_mut() }
    }

    ////////////////////////////////////////////////////////////////////////////
    // Getters and setters
    ////////////////////////////////////////////////////////////////////////////

    pub fn get_delay_mut(&mut self) -> Option<&mut Delay> {
        self.delay.get_mut().as_mut()
    }

    pub fn take_delay(&mut self) -> Option<Delay> {
        self.delay.replace(None)
    }

    pub fn return_delay(&mut self, delay: Delay) -> Result<(), Error> {
        if already_owned(&self.delay) {
            return Err(Error::AttemptToReturnExistingValue);
        }

        self.delay.replace(Some(delay));
        Ok(())
    }

    pub fn get_pins(&mut self) -> &mut rp2040_hal::gpio::Pins {
        &mut self.pins
    }

    pub fn get_pio0_mut(&mut self) -> Option<&mut Pio<PIO0>> {
        self.pio0.get_mut().as_mut()
    }

    pub fn take_pio0(&mut self) -> Option<Pio<PIO0>> {
        self.pio0.replace(None)
    }

    pub fn return_pio0(&mut self, pio0: Pio<PIO0>) -> Result<(), Error> {
        if already_owned(&self.pio0) {
            return Err(Error::AttemptToReturnExistingValue);
        }

        self.pio0.replace(Some(pio0));
        Ok(())
    }

    pub fn get_pio1_mut(&mut self) -> Option<&mut Pio<PIO1>> {
        self.pio1.get_mut().as_mut()
    }

    pub fn take_pio1(&mut self) -> Option<Pio<PIO1>> {
        self.pio1.replace(None)
    }

    pub fn return_pio1(&mut self, pio1: Pio<PIO1>) -> Result<(), Error> {
        if already_owned(&self.pio1) {
            return Err(Error::AttemptToReturnExistingValue);
        }

        self.pio1.replace(Some(pio1));
        Ok(())
    }

    pub fn get_usb_mut(&mut self) -> Option<&mut UsbManager> {
        self.usb.get_mut().as_mut()
    }

    pub fn take_usb(&mut self) -> Option<UsbManager> {
        self.usb.replace(None)
    }

    pub fn return_usb(&mut self, usb: UsbManager) -> Result<(), Error> {
        if already_owned(&self.usb) {
            return Err(Error::AttemptToReturnExistingValue);
        }

        self.usb.replace(Some(usb));
        Ok(())
    }
}

fn already_owned<T>(data: &RefCell<Option<T>>) -> bool {
    data.borrow().as_ref().is_some()
}
