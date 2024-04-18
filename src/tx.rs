use rp2040_hal::pac::{PIO0, PIO1};
use rp2040_hal::pio::{self, SM0, SM1, SM2, SM3};

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

impl From<pio::Tx<(PIO0, SM0)>> for Tx {
    fn from(value: pio::Tx<(PIO0, SM0)>) -> Self {Tx::PIO0SM0(value)}
}
impl From<pio::Tx<(PIO0, SM1)>> for Tx {
    fn from(value: pio::Tx<(PIO0, SM1)>) -> Self {Tx::PIO0SM1(value)}
}
impl From<pio::Tx<(PIO0, SM2)>> for Tx {
    fn from(value: pio::Tx<(PIO0, SM2)>) -> Self {Tx::PIO0SM2(value)}
}
impl From<pio::Tx<(PIO0, SM3)>> for Tx {
    fn from(value: pio::Tx<(PIO0, SM3)>) -> Self {Tx::PIO0SM3(value)}
}
impl From<pio::Tx<(PIO1, SM0)>> for Tx {
    fn from(value: pio::Tx<(PIO1, SM0)>) -> Self {Tx::PIO1SM0(value)}
}
impl From<pio::Tx<(PIO1, SM1)>> for Tx {
    fn from(value: pio::Tx<(PIO1, SM1)>) -> Self {Tx::PIO1SM1(value)}
}
impl From<pio::Tx<(PIO1, SM2)>> for Tx {
    fn from(value: pio::Tx<(PIO1, SM2)>) -> Self {Tx::PIO1SM2(value)}
}
impl From<pio::Tx<(PIO1, SM3)>> for Tx {
    fn from(value: pio::Tx<(PIO1, SM3)>) -> Self {Tx::PIO1SM3(value)}
}
