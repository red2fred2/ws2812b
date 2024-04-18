//! This shit show exists because rp2040_hal uses a limited set of marker
//! types to specify which pio and state machine is being referenced. This is not
//! something the average user needs to worry about anyway. I have no idea why they
//! wouldn't just include ID in the struct's data.

use core::any::TypeId;
use core::ptr::{addr_of, read};

use rp2040_hal::pac::{PIO0, PIO1};
use rp2040_hal::pio::{self, PIOExt, StateMachineIndex, SM0, SM1, SM2, SM3};

/// This entire thing exists to unfuck the rp2040_hal library design for Rx
pub enum Rx {
    PIO0SM0(pio::Rx<(PIO0, SM0)>),
    PIO0SM1(pio::Rx<(PIO0, SM1)>),
    PIO0SM2(pio::Rx<(PIO0, SM2)>),
    PIO0SM3(pio::Rx<(PIO0, SM3)>),
    PIO1SM0(pio::Rx<(PIO1, SM0)>),
    PIO1SM1(pio::Rx<(PIO1, SM1)>),
    PIO1SM2(pio::Rx<(PIO1, SM2)>),
    PIO1SM3(pio::Rx<(PIO1, SM3)>),
}

impl Rx {
    /// Indicate if the rx FIFO is empty
    pub fn is_empty(&self) -> bool {
        match self {
            Rx::PIO0SM0(rx) => rx.is_empty(),
            Rx::PIO0SM1(rx) => rx.is_empty(),
            Rx::PIO0SM2(rx) => rx.is_empty(),
            Rx::PIO0SM3(rx) => rx.is_empty(),
            Rx::PIO1SM0(rx) => rx.is_empty(),
            Rx::PIO1SM1(rx) => rx.is_empty(),
            Rx::PIO1SM2(rx) => rx.is_empty(),
            Rx::PIO1SM3(rx) => rx.is_empty(),
        }
    }

     /// Indicate if the rx FIFO is full
     pub fn is_full(&self) -> bool {
        match self {
            Rx::PIO0SM0(rx) => rx.is_full(),
            Rx::PIO0SM1(rx) => rx.is_full(),
            Rx::PIO0SM2(rx) => rx.is_full(),
            Rx::PIO0SM3(rx) => rx.is_full(),
            Rx::PIO1SM0(rx) => rx.is_full(),
            Rx::PIO1SM1(rx) => rx.is_full(),
            Rx::PIO1SM2(rx) => rx.is_full(),
            Rx::PIO1SM3(rx) => rx.is_full(),
        }
    }

    /// Get the next element from RX FIFO.
    ///
    /// Returns `None` if the FIFO is empty.
    pub fn read(&mut self) -> Option<u32> {
        match self {
            Rx::PIO0SM0(rx) => rx.read(),
            Rx::PIO0SM1(rx) => rx.read(),
            Rx::PIO0SM2(rx) => rx.read(),
            Rx::PIO0SM3(rx) => rx.read(),
            Rx::PIO1SM0(rx) => rx.read(),
            Rx::PIO1SM1(rx) => rx.read(),
            Rx::PIO1SM2(rx) => rx.read(),
            Rx::PIO1SM3(rx) => rx.read(),
        }
    }
}

// This whole thing just tells rust that yes I do know what type it is. No you
// don't need to worry about it.
impl<PIO: PIOExt + 'static, SM: StateMachineIndex + 'static> From<pio::Rx<(PIO, SM)>> for Rx {
    #[inline(always)]
    fn from(value: pio::Rx<(PIO, SM)>) -> Self {
        let pio = TypeId::of::<PIO>();
        let sm = TypeId::of::<SM>();

        let pio0 = TypeId::of::<PIO0>();
        let pio1 = TypeId::of::<PIO1>();

        let sm0 = TypeId::of::<SM0>();
        let sm1 = TypeId::of::<SM1>();
        let sm2 = TypeId::of::<SM2>();
        let sm3 = TypeId::of::<SM3>();

        match (pio, sm) {
            _ if pio == pio0 && sm == sm0 => unsafe {
                let value = read(addr_of!(value).cast());
                Rx::PIO0SM0(value)
            },
            _ if pio == pio0 && sm == sm1 => unsafe {
                let value = read(addr_of!(value).cast());
                Rx::PIO0SM1(value)
            },
            _ if pio == pio0 && sm == sm2 => unsafe {
                let value = read(addr_of!(value).cast());
                Rx::PIO0SM2(value)
            },
            _ if pio == pio0 && sm == sm3 => unsafe {
                let value = read(addr_of!(value).cast());
                Rx::PIO0SM3(value)
            },
            _ if pio == pio1 && sm == sm0 => unsafe {
                let value = read(addr_of!(value).cast());
                Rx::PIO1SM0(value)
            },
            _ if pio == pio1 && sm == sm1 => unsafe {
                let value = read(addr_of!(value).cast());
                Rx::PIO1SM1(value)
            },
            _ if pio == pio1 && sm == sm2 => unsafe {
                let value = read(addr_of!(value).cast());
                Rx::PIO1SM2(value)
            },
            _ if pio == pio1 && sm == sm3 => unsafe {
                let value = read(addr_of!(value).cast());
                Rx::PIO1SM3(value)
            },
            _ => unreachable!(),
        }
    }
}

pub struct WrongDeviceError;

impl<PIO: PIOExt + 'static, SM: StateMachineIndex + 'static> TryInto<pio::Rx<(PIO, SM)>> for Rx {
    type Error = WrongDeviceError;

    #[inline(always)]
    fn try_into(self) -> Result<pio::Rx<(PIO, SM)>, Self::Error> {
        let pio = TypeId::of::<PIO>();
        let sm = TypeId::of::<SM>();

        let pio0 = TypeId::of::<PIO0>();
        let pio1 = TypeId::of::<PIO1>();

        let sm0 = TypeId::of::<SM0>();
        let sm1 = TypeId::of::<SM1>();
        let sm2 = TypeId::of::<SM2>();
        let sm3 = TypeId::of::<SM3>();

        match self {
            Rx::PIO0SM0(value) if pio == pio0 && sm == sm0 => unsafe {
                Ok(read(addr_of!(value).cast()))
            },
            Rx::PIO0SM1(value) if pio == pio0 && sm == sm1 => unsafe {
                Ok(read(addr_of!(value).cast()))
            },
            Rx::PIO0SM2(value) if pio == pio0 && sm == sm2 => unsafe {
                Ok(read(addr_of!(value).cast()))
            },
            Rx::PIO0SM3(value) if pio == pio0 && sm == sm3 => unsafe {
                Ok(read(addr_of!(value).cast()))
            },
            Rx::PIO1SM0(value) if pio == pio1 && sm == sm0 => unsafe {
                Ok(read(addr_of!(value).cast()))
            },
            Rx::PIO1SM1(value) if pio == pio1 && sm == sm1 => unsafe {
                Ok(read(addr_of!(value).cast()))
            },
            Rx::PIO1SM2(value) if pio == pio1 && sm == sm2 => unsafe {
                Ok(read(addr_of!(value).cast()))
            },
            Rx::PIO1SM3(value) if pio == pio1 && sm == sm3 => unsafe {
                Ok(read(addr_of!(value).cast()))
            },
            _ => Err(WrongDeviceError)
        }
    }
}
