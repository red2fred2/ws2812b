//! Handles most low level hardware abstraction

use core::cell::RefCell;

use cortex_m::delay::Delay;
use rp2040_hal::{clocks::init_clocks_and_plls, gpio::{bank0::{
    Gpio0, Gpio1, Gpio10, Gpio11, Gpio12, Gpio13, Gpio14, Gpio15, Gpio16, Gpio17, Gpio18, Gpio19, Gpio2, Gpio20, Gpio21, Gpio22, Gpio23, Gpio24, Gpio25, Gpio26, Gpio27, Gpio28, Gpio29, Gpio3, Gpio4, Gpio5, Gpio6, Gpio7, Gpio8, Gpio9
}, FunctionNull, Pin, PullDown}, pac::{self, PIO0, PIO1}, usb::UsbBus, Clock, Sio, Watchdog};
use usb_device::class_prelude::UsbBusAllocator;

use crate::{pio::Pio, usb_manager::UsbManager};

static mut SINGLETON: Option<Hardware> = None;

type OptCell<T> = RefCell<Option<T>>;
type P<T> = Pin<T, FunctionNull, PullDown>;

#[derive(Debug)]
pub enum Error {
    AttemptToReturnExistingValue,
}

pub struct Hardware {
    delay: OptCell<Delay>,
    pin0: OptCell<P<Gpio0>>,
    pin1: OptCell<P<Gpio1>>,
    pin2: OptCell<P<Gpio2>>,
    pin3: OptCell<P<Gpio3>>,
    pin4: OptCell<P<Gpio4>>,
    pin5: OptCell<P<Gpio5>>,
    pin6: OptCell<P<Gpio6>>,
    pin7: OptCell<P<Gpio7>>,
    pin8: OptCell<P<Gpio8>>,
    pin9: OptCell<P<Gpio9>>,
    pin10: OptCell<P<Gpio10>>,
    pin11: OptCell<P<Gpio11>>,
    pin12: OptCell<P<Gpio12>>,
    pin13: OptCell<P<Gpio13>>,
    pin14: OptCell<P<Gpio14>>,
    pin15: OptCell<P<Gpio15>>,
    pin16: OptCell<P<Gpio16>>,
    pin17: OptCell<P<Gpio17>>,
    pin18: OptCell<P<Gpio18>>,
    pin19: OptCell<P<Gpio19>>,
    pin20: OptCell<P<Gpio20>>,
    pin21: OptCell<P<Gpio21>>,
    pin22: OptCell<P<Gpio22>>,
    pin23: OptCell<P<Gpio23>>,
    pin24: OptCell<P<Gpio24>>,
    pin25: OptCell<P<Gpio25>>,
    pin26: OptCell<P<Gpio26>>,
    pin27: OptCell<P<Gpio27>>,
    pin28: OptCell<P<Gpio28>>,
    pin29: OptCell<P<Gpio29>>,
    pio0: OptCell<Pio<PIO0>>,
    pio1: OptCell<Pio<PIO1>>,
    usb: OptCell<UsbManager>,
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
                    pin0: RefCell::new(Some(pins.gpio0)),
                    pin1: RefCell::new(Some(pins.gpio1)),
                    pin2: RefCell::new(Some(pins.gpio2)),
                    pin3: RefCell::new(Some(pins.gpio3)),
                    pin4: RefCell::new(Some(pins.gpio4)),
                    pin5: RefCell::new(Some(pins.gpio5)),
                    pin6: RefCell::new(Some(pins.gpio6)),
                    pin7: RefCell::new(Some(pins.gpio7)),
                    pin8: RefCell::new(Some(pins.gpio8)),
                    pin9: RefCell::new(Some(pins.gpio9)),
                    pin10: RefCell::new(Some(pins.gpio10)),
                    pin11: RefCell::new(Some(pins.gpio11)),
                    pin12: RefCell::new(Some(pins.gpio12)),
                    pin13: RefCell::new(Some(pins.gpio13)),
                    pin14: RefCell::new(Some(pins.gpio14)),
                    pin15: RefCell::new(Some(pins.gpio15)),
                    pin16: RefCell::new(Some(pins.gpio16)),
                    pin17: RefCell::new(Some(pins.gpio17)),
                    pin18: RefCell::new(Some(pins.gpio18)),
                    pin19: RefCell::new(Some(pins.gpio19)),
                    pin20: RefCell::new(Some(pins.gpio20)),
                    pin21: RefCell::new(Some(pins.gpio21)),
                    pin22: RefCell::new(Some(pins.gpio22)),
                    pin23: RefCell::new(Some(pins.gpio23)),
                    pin24: RefCell::new(Some(pins.gpio24)),
                    pin25: RefCell::new(Some(pins.gpio25)),
                    pin26: RefCell::new(Some(pins.gpio26)),
                    pin27: RefCell::new(Some(pins.gpio27)),
                    pin28: RefCell::new(Some(pins.gpio28)),
                    pin29: RefCell::new(Some(pins.gpio29)),
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

    pub fn take_pin0(&mut self) -> Option<P<Gpio0>> {
        self.pin0.replace(None)
    }

    pub fn return_pin0(&mut self, pin0: P<Gpio0>) -> Result<(), Error> {
        if already_owned(&self.pin0) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin0.replace(Some(pin0));
        Ok(())
    }

	pub fn take_pin1(&mut self) -> Option<P<Gpio1>> {
        self.pin1.replace(None)
    }

	pub fn return_pin1(&mut self, pin1: P<Gpio1>) -> Result<(), Error> {
        if already_owned(&self.pin1) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin1.replace(Some(pin1));
        Ok(())
    }

	pub fn take_pin2(&mut self) -> Option<P<Gpio2>> {
        self.pin2.replace(None)
    }

    pub fn return_pin2(&mut self, pin2: P<Gpio2>) -> Result<(), Error> {
        if already_owned(&self.pin2) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin2.replace(Some(pin2));
        Ok(())
    }

	pub fn take_pin3(&mut self) -> Option<P<Gpio3>> {
        self.pin3.replace(None)
    }

    pub fn return_pin3(&mut self, pin3: P<Gpio3>) -> Result<(), Error> {
        if already_owned(&self.pin3) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin3.replace(Some(pin3));
        Ok(())
    }

	pub fn take_pin4(&mut self) -> Option<P<Gpio4>> {
        self.pin4.replace(None)
    }

    pub fn return_pin4(&mut self, pin4: P<Gpio4>) -> Result<(), Error> {
        if already_owned(&self.pin4) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin4.replace(Some(pin4));
        Ok(())
    }

	pub fn take_pin5(&mut self) -> Option<P<Gpio5>> {
        self.pin5.replace(None)
    }

    pub fn return_pin5(&mut self, pin5: P<Gpio5>) -> Result<(), Error> {
        if already_owned(&self.pin5) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin5.replace(Some(pin5));
        Ok(())
    }

	pub fn take_pin6(&mut self) -> Option<P<Gpio6>> {
        self.pin6.replace(None)
    }

    pub fn return_pin6(&mut self, pin6: P<Gpio6>) -> Result<(), Error> {
        if already_owned(&self.pin6) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin6.replace(Some(pin6));
        Ok(())
    }

	pub fn take_pin7(&mut self) -> Option<P<Gpio7>> {
        self.pin7.replace(None)
    }

    pub fn return_pin7(&mut self, pin7: P<Gpio7>) -> Result<(), Error> {
        if already_owned(&self.pin7) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin7.replace(Some(pin7));
        Ok(())
    }

	pub fn take_pin8(&mut self) -> Option<P<Gpio8>> {
        self.pin8.replace(None)
    }

    pub fn return_pin8(&mut self, pin8: P<Gpio8>) -> Result<(), Error> {
        if already_owned(&self.pin8) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin8.replace(Some(pin8));
        Ok(())
    }

	pub fn take_pin9(&mut self) -> Option<P<Gpio9>> {
        self.pin9.replace(None)
    }

    pub fn return_pin9(&mut self, pin9: P<Gpio9>) -> Result<(), Error> {
        if already_owned(&self.pin9) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin9.replace(Some(pin9));
        Ok(())
    }

	pub fn take_pin10(&mut self) -> Option<P<Gpio10>> {
        self.pin10.replace(None)
    }

    pub fn return_pin10(&mut self, pin10: P<Gpio10>) -> Result<(), Error> {
        if already_owned(&self.pin10) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin10.replace(Some(pin10));
        Ok(())
    }

	pub fn take_pin11(&mut self) -> Option<P<Gpio11>> {
        self.pin11.replace(None)
    }

	pub fn return_pin11(&mut self, pin11: P<Gpio11>) -> Result<(), Error> {
        if already_owned(&self.pin11) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin11.replace(Some(pin11));
        Ok(())
    }

	pub fn take_pin12(&mut self) -> Option<P<Gpio12>> {
        self.pin12.replace(None)
    }

    pub fn return_pin12(&mut self, pin12: P<Gpio12>) -> Result<(), Error> {
        if already_owned(&self.pin12) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin12.replace(Some(pin12));
        Ok(())
    }

	pub fn take_pin13(&mut self) -> Option<P<Gpio13>> {
        self.pin13.replace(None)
    }

    pub fn return_pin13(&mut self, pin13: P<Gpio13>) -> Result<(), Error> {
        if already_owned(&self.pin13) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin13.replace(Some(pin13));
        Ok(())
    }

	pub fn take_pin14(&mut self) -> Option<P<Gpio14>> {
        self.pin14.replace(None)
    }

    pub fn return_pin14(&mut self, pin14: P<Gpio14>) -> Result<(), Error> {
        if already_owned(&self.pin14) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin14.replace(Some(pin14));
        Ok(())
    }

	pub fn take_pin15(&mut self) -> Option<P<Gpio15>> {
        self.pin15.replace(None)
    }

    pub fn return_pin15(&mut self, pin15: P<Gpio15>) -> Result<(), Error> {
        if already_owned(&self.pin15) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin15.replace(Some(pin15));
        Ok(())
    }

	pub fn take_pin16(&mut self) -> Option<P<Gpio16>> {
        self.pin16.replace(None)
    }

    pub fn return_pin16(&mut self, pin16: P<Gpio16>) -> Result<(), Error> {
        if already_owned(&self.pin16) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin16.replace(Some(pin16));
        Ok(())
    }

	pub fn take_pin17(&mut self) -> Option<P<Gpio17>> {
        self.pin17.replace(None)
    }

    pub fn return_pin17(&mut self, pin17: P<Gpio17>) -> Result<(), Error> {
        if already_owned(&self.pin17) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin17.replace(Some(pin17));
        Ok(())
    }

	pub fn take_pin18(&mut self) -> Option<P<Gpio18>> {
        self.pin18.replace(None)
    }

    pub fn return_pin18(&mut self, pin18: P<Gpio18>) -> Result<(), Error> {
        if already_owned(&self.pin18) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin18.replace(Some(pin18));
        Ok(())
    }

	pub fn take_pin19(&mut self) -> Option<P<Gpio19>> {
        self.pin19.replace(None)
    }

    pub fn return_pin19(&mut self, pin19: P<Gpio19>) -> Result<(), Error> {
        if already_owned(&self.pin19) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin19.replace(Some(pin19));
        Ok(())
    }


	pub fn take_pin20(&mut self) -> Option<P<Gpio20>> {
        self.pin20.replace(None)
    }

    pub fn return_pin20(&mut self, pin20: P<Gpio20>) -> Result<(), Error> {
        if already_owned(&self.pin20) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin20.replace(Some(pin20));
        Ok(())
    }

	pub fn take_pin21(&mut self) -> Option<P<Gpio21>> {
        self.pin21.replace(None)
    }

	pub fn return_pin21(&mut self, pin21: P<Gpio21>) -> Result<(), Error> {
        if already_owned(&self.pin21) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin21.replace(Some(pin21));
        Ok(())
    }

	pub fn take_pin22(&mut self) -> Option<P<Gpio22>> {
        self.pin22.replace(None)
    }

    pub fn return_pin22(&mut self, pin22: P<Gpio22>) -> Result<(), Error> {
        if already_owned(&self.pin22) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin22.replace(Some(pin22));
        Ok(())
    }

	pub fn take_pin23(&mut self) -> Option<P<Gpio23>> {
        self.pin23.replace(None)
    }

    pub fn return_pin23(&mut self, pin23: P<Gpio23>) -> Result<(), Error> {
        if already_owned(&self.pin23) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin23.replace(Some(pin23));
        Ok(())
    }

	pub fn take_pin24(&mut self) -> Option<P<Gpio24>> {
        self.pin24.replace(None)
    }

    pub fn return_pin24(&mut self, pin24: P<Gpio24>) -> Result<(), Error> {
        if already_owned(&self.pin24) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin24.replace(Some(pin24));
        Ok(())
    }

	pub fn take_pin25(&mut self) -> Option<P<Gpio25>> {
        self.pin25.replace(None)
    }

    pub fn return_pin25(&mut self, pin25: P<Gpio25>) -> Result<(), Error> {
        if already_owned(&self.pin25) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin25.replace(Some(pin25));
        Ok(())
    }

	pub fn take_pin26(&mut self) -> Option<P<Gpio26>> {
        self.pin26.replace(None)
    }

    pub fn return_pin26(&mut self, pin26: P<Gpio26>) -> Result<(), Error> {
        if already_owned(&self.pin26) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin26.replace(Some(pin26));
        Ok(())
    }

	pub fn take_pin27(&mut self) -> Option<P<Gpio27>> {
        self.pin27.replace(None)
    }

    pub fn return_pin27(&mut self, pin27: P<Gpio27>) -> Result<(), Error> {
        if already_owned(&self.pin27) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin27.replace(Some(pin27));
        Ok(())
    }

	pub fn take_pin28(&mut self) -> Option<P<Gpio28>> {
        self.pin28.replace(None)
    }

    pub fn return_pin28(&mut self, pin28: P<Gpio28>) -> Result<(), Error> {
        if already_owned(&self.pin28) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin28.replace(Some(pin28));
        Ok(())
    }

	pub fn take_pin29(&mut self) -> Option<P<Gpio29>> {
        self.pin29.replace(None)
    }

    pub fn return_pin29(&mut self, pin29: P<Gpio29>) -> Result<(), Error> {
        if already_owned(&self.pin29) {
            return Err(Error::AttemptToReturnExistingValue);
        }

		self.pin29.replace(Some(pin29));
        Ok(())
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

fn already_owned<T>(data: &OptCell<T>) -> bool {
    let result = data.try_borrow();
    !matches!(result, Ok(v) if v.as_ref().is_none())
}
