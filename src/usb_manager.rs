//! Handles low level USB stuff
use rp2040_hal as hal;
use rp2040_hal::pac::interrupt;
use usb_device::{
    bus::UsbBusAllocator,
    device::{UsbDevice, UsbDeviceBuilder, UsbVidPid}
};
use usbd_serial::SerialPort;

use crate::hardware::Hardware;

/// Deals with low level USB stuff
pub struct UsbManager {
    device: UsbDevice<'static, hal::usb::UsbBus>,
    serial: SerialPort<'static, hal::usb::UsbBus>,
}

impl UsbManager {
    pub fn new(usb_bus: &'static UsbBusAllocator<hal::usb::UsbBus>) -> Self {
        let serial = usbd_serial::SerialPort::new(usb_bus);

        let device = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x0000, 0x000b))
            .device_class(2)
            .device_protocol(1)
            .build();

        UsbManager { device, serial }
    }

    /// Handles USB reads
    ///
    /// # Safety
	/// All this does is dump the data in a buffer and walk away. Once the buffer
	/// fills, it'll just miss anything new.
    pub unsafe fn interrupt(&mut self) {
        if self.device.poll(&mut [&mut self.serial]) {
            let mut data: [u8; 256] = [0x00; 256];

            let _ = self.serial.read(&mut data);
        }
    }
}

// Fmt implementation for USB writes
impl core::fmt::Write for UsbManager {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        let _ = self.serial.write(s.as_bytes());
		Ok(())
    }
}

#[allow(non_snake_case)]
#[interrupt]
unsafe fn USBCTRL_IRQ() {
    // If usb isn't enabled by the time an interrupt is attempted, just ignore
    // it and hope nothing bad happens.
    let Some(hardware) = Hardware::get() else {
        return
    };

	let Some(usb_manager) = hardware.get_usb_mut() else {
		return
	};

    usb_manager.interrupt()
}
