use pio::{ArrayVec, Program};
use rp2040_hal::pac::RESETS;
use rp2040_hal::pio::{PIOExt, PIO, SM0, SM1, SM2, SM3};

use crate::rx::Rx;
use crate::state_machine::{self, StateMachine};
use crate::tx::Tx;

#[derive(Debug)]
pub enum Error {
    /// There is no free PIO block to install the program to
    NoFreePIO,
    /// Requesting more state machines than are available
    TooManyStateMachinesRequested,
    /// Bad State machine programming input
    BadStateMachineProgramming,
}

pub struct Pio<P: PIOExt> {
    pio: PIO<P>,
    sm0: StateMachine<P, SM0>,
    sm1: StateMachine<P, SM1>,
    sm2: StateMachine<P, SM2>,
    sm3: StateMachine<P, SM3>,
    in_use: bool,
}

impl<P: PIOExt + 'static> Pio<P> {
    pub fn new(pio: P, resets: &mut RESETS) -> Pio<P> {
        let (pio, sm0, sm1, sm2, sm3) = pio.split(resets);

        let sm0 = StateMachine::new(sm0);
        let sm1 = StateMachine::new(sm1);
        let sm2 = StateMachine::new(sm2);
        let sm3 = StateMachine::new(sm3);

        Pio {pio, sm0, sm1, sm2, sm3, in_use: false}
    }

    /// Installs a program
    ///
    /// Returns a tuple with the tx and rx for each state machine.
    pub fn install_program<const NUM: usize>(
        &mut self, program: Program<32>,
        pins: [(u8, u8); NUM],
        clock_divisors: [(u16, u8); NUM],
    ) -> Result<ArrayVec<(Rx, Tx), NUM>, Error> {
        if NUM > 4 {
            return Err(Error::TooManyStateMachinesRequested)
        }

        // Install the program to the PIO block
        let installed = self.pio.install(&program).unwrap();

        let mut rxtxs: ArrayVec<_, NUM> = ArrayVec::new();

        if NUM >= 1 {
            // Install to state machine 0
            let pins = pins[0];
            let clock_divisor = clock_divisors[0];

            let Ok((rx, tx)) = self.sm0.program(&installed, pins, clock_divisor) else {
                return Err(Error::BadStateMachineProgramming)
            };

            rxtxs.push((rx, tx));
        }

        if NUM >= 2 {
            // Install to state machine 1
            let pins = pins[1];
            let clock_divisor = clock_divisors[1];

            let Ok((rx, tx)) = self.sm1.program(&installed, pins, clock_divisor) else {
                return Err(Error::BadStateMachineProgramming)
            };

            rxtxs.push((rx, tx));
        }

        if NUM >= 3 {
            // Install to state machine 2
            let pins = pins[2];
            let clock_divisor = clock_divisors[2];

            let Ok((rx, tx)) = self.sm2.program(&installed, pins, clock_divisor) else {
                return Err(Error::BadStateMachineProgramming)
            };

            rxtxs.push((rx, tx));
        }

        if NUM >= 4 {
            // Install to state machine 3
            let pins = pins[3];
            let clock_divisor = clock_divisors[3];

            let Ok((rx, tx)) = self.sm3.program(&installed, pins, clock_divisor) else {
                return Err(Error::BadStateMachineProgramming)
            };

            rxtxs.push((rx, tx));
        }

        self.in_use = true;
        Ok(rxtxs)
    }

    // /// Uninstalls a program
    // ///
    // /// * `rxtx` - The same rx and tx channels returned by install_program
    // ///
    // /// Returns a tuple with the tx and rx for each state machine.
    // pub fn unininstall_program<const NUM: usize>(
    //     &mut self,
    //     rxtxs: ArrayVec<(Rx, Tx), NUM>
    // ) -> Result<(), state_machine::Error> {
	// 	if NUM > 0 {
	// 		let (rx, tx) = rxtxs[0];
	// 		self.sm0.uninstall(rx, tx);
	// 	}
	// 	if NUM > 1 {
	// 		let (rx, tx) = rxtxs[1];
	// 		self.sm1.uninstall(rx, tx);
	// 	}
	// 	if NUM > 2 {
	// 		let (rx, tx) = rxtxs[2];
	// 		self.sm2.uninstall(rx, tx);
	// 	}
	// 	if NUM > 3 {
	// 		let (rx, tx) = rxtxs[3];
	// 		self.sm3.uninstall(rx, tx);
	// 	}

    //     self.in_use = false;
    //     Ok(())
    // }

    /// Returns whether or not this pio is in use
    pub fn in_use(&self) -> bool {
        self.in_use
    }

    /// Starts the state machines
    pub fn start(&mut self) -> Result<(), state_machine::Error> {
        if self.sm0.is_initialized() {
            self.sm0.start()?;
        }
        if self.sm1.is_initialized() {
            self.sm1.start()?;
        }
        if self.sm2.is_initialized() {
            self.sm2.start()?;
        }
        if self.sm3.is_initialized() {
            self.sm3.start()?;
        }

        Ok(())
    }

    /// Stops the state machines
    pub fn stop(&mut self) -> Result<(), state_machine::Error> {
        if self.sm0.is_initialized() {
            self.sm0.stop()?;
        }
        if self.sm1.is_initialized() {
            self.sm1.stop()?;
        }
        if self.sm2.is_initialized() {
            self.sm2.stop()?;
        }
        if self.sm3.is_initialized() {
            self.sm3.stop()?;
        }

        Ok(())
    }
}
