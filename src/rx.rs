use rp2040_hal::pac::{PIO0, PIO1};
use rp2040_hal::pio::{self, SM0, SM1, SM2, SM3};

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

impl From<pio::Rx<(PIO0, SM0)>> for Rx {
    fn from(value: pio::Rx<(PIO0, SM0)>) -> Self {Rx::PIO0SM0(value)}
}
impl From<pio::Rx<(PIO0, SM1)>> for Rx {
    fn from(value: pio::Rx<(PIO0, SM1)>) -> Self {Rx::PIO0SM1(value)}
}
impl From<pio::Rx<(PIO0, SM2)>> for Rx {
    fn from(value: pio::Rx<(PIO0, SM2)>) -> Self {Rx::PIO0SM2(value)}
}
impl From<pio::Rx<(PIO0, SM3)>> for Rx {
    fn from(value: pio::Rx<(PIO0, SM3)>) -> Self {Rx::PIO0SM3(value)}
}
impl From<pio::Rx<(PIO1, SM0)>> for Rx {
    fn from(value: pio::Rx<(PIO1, SM0)>) -> Self {Rx::PIO1SM0(value)}
}
impl From<pio::Rx<(PIO1, SM1)>> for Rx {
    fn from(value: pio::Rx<(PIO1, SM1)>) -> Self {Rx::PIO1SM1(value)}
}
impl From<pio::Rx<(PIO1, SM2)>> for Rx {
    fn from(value: pio::Rx<(PIO1, SM2)>) -> Self {Rx::PIO1SM2(value)}
}
impl From<pio::Rx<(PIO1, SM3)>> for Rx {
    fn from(value: pio::Rx<(PIO1, SM3)>) -> Self {Rx::PIO1SM3(value)}
}
