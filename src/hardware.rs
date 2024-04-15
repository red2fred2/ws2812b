//! Handles most low level hardware abstraction

use cortex_m::delay::Delay;
use rp2040_hal::{clocks::init_clocks_and_plls, pac, usb::UsbBus, Clock, Sio, Watchdog};
use usb_device::class_prelude::UsbBusAllocator;

use crate::{pio::Pio, usb_manager::UsbManager};

static mut SINGLETON: Option<Hardware> = None;

pub struct Hardware {
    delay: Delay,
    pins: rp2040_hal::gpio::Pins,
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

			let pio = Pio::new(pac.PIO0, pac.PIO1, &mut pac.RESETS);

			let program = pio_proc::pio_asm!(
				"set pindirs, 1

				pull
				mov y osr

				.wrap_target

				mov x y

				color:
				send_1:
					set pins, 1 [19]
					set pins, 0 [10]

				// send_0:
				// 	set pins, 1 [9]
				// 	set pins, 0 [20]

				jmp x-- color
				mov x y

				reset:
					set pins, 0 [31]
					set pins, 0 [31]
					jmp x-- reset

				.wrap"
			).program;

			let pin = (Some((2, 1)), None, None, None);
			let clocks = (Some((5, 1)), None, None, None);

			let (pio, (Some((_rx, mut tx)), _, _, _)) = pio.install_program_pio0(program, 1, pin, clocks).unwrap() else {
				unreachable!();
			};

			let _pio = pio.start0().unwrap();

			tx.write(24 * 300);

			// // Uninitialize the state machine again, freeing the program.
			// let (sm, installed) = sm.uninit(rx, tx);

            unsafe {
                SINGLETON = Some(Hardware {
                    delay,
                    pins,
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

    pub fn get_usb(&mut self) -> &mut UsbManager {
        // It should be impossible for this to be None, panic if it is
        self.usb.as_mut().unwrap()
    }
}