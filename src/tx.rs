//! This shit show exists because rp2040_hal uses a limited set of marker
//! types to specify which pio and state machine is being referenced. This is not
//! something the average user needs to worry about anyway. I have no idea why they
//! wouldn't just include ID in the struct's data.

use core::any::TypeId;
use core::ptr::{addr_of, read};

use rp2040_hal::pac::{PIO0, PIO1};
use rp2040_hal::pio::{self, PIOExt, StateMachineIndex, SM0, SM1, SM2, SM3};

/// This entire thing exists to unfuck the rp2040_hal library design for Tx
pub enum Tx {
    PIO0SM0(pio::Tx<(PIO0, SM0)>),
    PIO0SM1(pio::Tx<(PIO0, SM1)>),
    PIO0SM2(pio::Tx<(PIO0, SM2)>),
    PIO0SM3(pio::Tx<(PIO0, SM3)>),
    PIO1SM0(pio::Tx<(PIO1, SM0)>),
    PIO1SM1(pio::Tx<(PIO1, SM1)>),
    PIO1SM2(pio::Tx<(PIO1, SM2)>),
    PIO1SM3(pio::Tx<(PIO1, SM3)>),
}

impl Tx {
    /// Clears the `tx_stalled` flag.
    pub fn clear_stalled_flag(&self) {
        match self {
            Tx::PIO0SM0(tx) => tx.clear_stalled_flag(),
            Tx::PIO0SM1(tx) => tx.clear_stalled_flag(),
            Tx::PIO0SM2(tx) => tx.clear_stalled_flag(),
            Tx::PIO0SM3(tx) => tx.clear_stalled_flag(),
            Tx::PIO1SM0(tx) => tx.clear_stalled_flag(),
            Tx::PIO1SM1(tx) => tx.clear_stalled_flag(),
            Tx::PIO1SM2(tx) => tx.clear_stalled_flag(),
            Tx::PIO1SM3(tx) => tx.clear_stalled_flag(),
        }
    }

    /// Checks if the state machine has stalled on empty TX FIFO during a blocking PULL, or an OUT
    /// with autopull enabled.
    ///
    /// **Note this is a sticky flag and may not reflect the current state of the machine.**
    pub fn has_stalled(&self) -> bool {
        match self {
            Tx::PIO0SM0(tx) => tx.has_stalled(),
            Tx::PIO0SM1(tx) => tx.has_stalled(),
            Tx::PIO0SM2(tx) => tx.has_stalled(),
            Tx::PIO0SM3(tx) => tx.has_stalled(),
            Tx::PIO1SM0(tx) => tx.has_stalled(),
            Tx::PIO1SM1(tx) => tx.has_stalled(),
            Tx::PIO1SM2(tx) => tx.has_stalled(),
            Tx::PIO1SM3(tx) => tx.has_stalled(),
        }
    }

    /// Indicate if the tx FIFO is empty
    pub fn is_empty(&self) -> bool {
        match self {
            Tx::PIO0SM0(tx) => tx.is_empty(),
            Tx::PIO0SM1(tx) => tx.is_empty(),
            Tx::PIO0SM2(tx) => tx.is_empty(),
            Tx::PIO0SM3(tx) => tx.is_empty(),
            Tx::PIO1SM0(tx) => tx.is_empty(),
            Tx::PIO1SM1(tx) => tx.is_empty(),
            Tx::PIO1SM2(tx) => tx.is_empty(),
            Tx::PIO1SM3(tx) => tx.is_empty(),
        }
    }

     /// Indicate if the tx FIFO is full
     pub fn is_full(&self) -> bool {
        match self {
            Tx::PIO0SM0(tx) => tx.is_full(),
            Tx::PIO0SM1(tx) => tx.is_full(),
            Tx::PIO0SM2(tx) => tx.is_full(),
            Tx::PIO0SM3(tx) => tx.is_full(),
            Tx::PIO1SM0(tx) => tx.is_full(),
            Tx::PIO1SM1(tx) => tx.is_full(),
            Tx::PIO1SM2(tx) => tx.is_full(),
            Tx::PIO1SM3(tx) => tx.is_full(),
        }
    }

    /// Write a u32 value to TX FIFO.
    ///
    /// Returns `true` if the value was written to FIFO, `false` otherwise.
    pub fn write(&mut self, value: u32) -> bool {
        match self {
            Tx::PIO0SM0(tx) => tx.write(value),
            Tx::PIO0SM1(tx) => tx.write(value),
            Tx::PIO0SM2(tx) => tx.write(value),
            Tx::PIO0SM3(tx) => tx.write(value),
            Tx::PIO1SM0(tx) => tx.write(value),
            Tx::PIO1SM1(tx) => tx.write(value),
            Tx::PIO1SM2(tx) => tx.write(value),
            Tx::PIO1SM3(tx) => tx.write(value),
        }
    }
}

// This whole thing just tells rust that yes I do know what type it is. No you
// don't need to worry about it.
impl<PIO: PIOExt + 'static, SM: StateMachineIndex + 'static> From<pio::Tx<(PIO, SM)>> for Tx {
    #[inline(always)]
    fn from(value: pio::Tx<(PIO, SM)>) -> Self {
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
                Tx::PIO0SM0(value)
            },
            _ if pio == pio0 && sm == sm1 => unsafe {
                let value = read(addr_of!(value).cast());
                Tx::PIO0SM1(value)
            },
            _ if pio == pio0 && sm == sm2 => unsafe {
                let value = read(addr_of!(value).cast());
                Tx::PIO0SM2(value)
            },
            _ if pio == pio0 && sm == sm3 => unsafe {
                let value = read(addr_of!(value).cast());
                Tx::PIO0SM3(value)
            },
            _ if pio == pio1 && sm == sm0 => unsafe {
                let value = read(addr_of!(value).cast());
                Tx::PIO1SM0(value)
            },
            _ if pio == pio1 && sm == sm1 => unsafe {
                let value = read(addr_of!(value).cast());
                Tx::PIO1SM1(value)
            },
            _ if pio == pio1 && sm == sm2 => unsafe {
                let value = read(addr_of!(value).cast());
                Tx::PIO1SM2(value)
            },
            _ if pio == pio1 && sm == sm3 => unsafe {
                let value = read(addr_of!(value).cast());
                Tx::PIO1SM3(value)
            },
            _ => unreachable!(),
        }
    }
}


pub struct WrongDeviceError;

impl<PIO: PIOExt + 'static, SM: StateMachineIndex + 'static> TryInto<pio::Tx<(PIO, SM)>> for Tx {
    type Error = WrongDeviceError;

    #[inline(always)]
    fn try_into(self) -> Result<pio::Tx<(PIO, SM)>, Self::Error> {
        let pio = TypeId::of::<PIO>();
        let sm = TypeId::of::<SM>();

        let pio0 = TypeId::of::<PIO0>();
        let pio1 = TypeId::of::<PIO1>();

        let sm0 = TypeId::of::<SM0>();
        let sm1 = TypeId::of::<SM1>();
        let sm2 = TypeId::of::<SM2>();
        let sm3 = TypeId::of::<SM3>();

        match self {
            Tx::PIO0SM0(value) if pio == pio0 && sm == sm0 => unsafe {
                Ok(read(addr_of!(value).cast()))
            },
            Tx::PIO0SM1(value) if pio == pio0 && sm == sm1 => unsafe {
                Ok(read(addr_of!(value).cast()))
            },
            Tx::PIO0SM2(value) if pio == pio0 && sm == sm2 => unsafe {
                Ok(read(addr_of!(value).cast()))
            },
            Tx::PIO0SM3(value) if pio == pio0 && sm == sm3 => unsafe {
                Ok(read(addr_of!(value).cast()))
            },
            Tx::PIO1SM0(value) if pio == pio1 && sm == sm0 => unsafe {
                Ok(read(addr_of!(value).cast()))
            },
            Tx::PIO1SM1(value) if pio == pio1 && sm == sm1 => unsafe {
                Ok(read(addr_of!(value).cast()))
            },
            Tx::PIO1SM2(value) if pio == pio1 && sm == sm2 => unsafe {
                Ok(read(addr_of!(value).cast()))
            },
            Tx::PIO1SM3(value) if pio == pio1 && sm == sm3 => unsafe {
                Ok(read(addr_of!(value).cast()))
            },
            _ => Err(WrongDeviceError)
        }
    }
}
