use pio::Program;
use rp2040_hal::pac::RESETS;
use rp2040_hal::pio::{PIOExt, Rx, Tx, PIO, SM0, SM1, SM2, SM3};

use crate::state_machine::{self, StateMachine};

type ClockOption = Option<(u16, u8)>;
type PinOption = Option<(u8, u8)>;
type RxTx<PIO, SM> = (Rx<(PIO, SM)>, Tx<(PIO, SM)>);

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

impl<P: PIOExt> Pio<P> {
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
    pub fn install_program(
        &mut self, program: Program<32>,
        num_state_machines: usize,
        state_machine_pins: (PinOption, PinOption, PinOption, PinOption),
        state_machine_clock_divisors: (ClockOption, ClockOption, ClockOption, ClockOption)
    ) -> Result<(Option<RxTx<P, SM0>>, Option<RxTx<P, SM1>>, Option<RxTx<P, SM2>>, Option<RxTx<P, SM3>>), Error> {
        if num_state_machines > 4 {
            return Err(Error::TooManyStateMachinesRequested);
        }

        // Install the program to the PIO block
        let installed = self.pio.install(&program).unwrap();

        let mut rxtx0 = None;
        let mut rxtx1 = None;
        let mut rxtx2 = None;
        let mut rxtx3 = None;

        if num_state_machines >= 1 {
            // Install to state machine 0
            let (Some(pins), _, _, _) = state_machine_pins else {
                return Err(Error::BadStateMachineProgramming);
            };
            let (Some(clock_divisor), _, _, _) = state_machine_clock_divisors else {
                return Err(Error::BadStateMachineProgramming);
            };
            let Ok(rxtx) = self.sm0.program(&installed, pins, clock_divisor) else {
                return Err(Error::BadStateMachineProgramming);
            };

            rxtx0 = Some(rxtx);
        }

        if num_state_machines >= 2 {
            // Install to state machine 1
            let (_, Some(pins), _, _) = state_machine_pins else {
                return Err(Error::BadStateMachineProgramming);
            };
            let (_, Some(clock_divisor), _, _) = state_machine_clock_divisors else {
                return Err(Error::BadStateMachineProgramming);
            };
            let Ok(rxtx) = self.sm1.program(&installed, pins, clock_divisor) else {
                return Err(Error::BadStateMachineProgramming);
            };

            rxtx1 = Some(rxtx);
        }

        if num_state_machines >= 3 {
            // Install to state machine 2
            let (_, _, Some(pins), _) = state_machine_pins else {
                return Err(Error::BadStateMachineProgramming);
            };
            let (_, _, Some(clock_divisor), _) = state_machine_clock_divisors else {
                return Err(Error::BadStateMachineProgramming);
            };
            let Ok(rxtx) = self.sm2.program(&installed, pins, clock_divisor) else {
                return Err(Error::BadStateMachineProgramming);
            };

            rxtx2 = Some(rxtx);
        }

        if num_state_machines >= 4 {
            // Install to state machine 3
            let (_, _, _, Some(pins)) = state_machine_pins else {
                return Err(Error::BadStateMachineProgramming);
            };
            let (_, _, _, Some(clock_divisor)) = state_machine_clock_divisors else {
                return Err(Error::BadStateMachineProgramming);
            };
            let Ok(rxtx) = self.sm3.program(&installed, pins, clock_divisor) else {
                return Err(Error::BadStateMachineProgramming);
            };

            rxtx3 = Some(rxtx);
        }

		self.in_use = true;
        return Ok((rxtx0, rxtx1, rxtx2, rxtx3));
    }

    /// Uninstalls a program
    ///
    /// * `rxtx` - The same rx and tx channels returned by install_program
    ///
    /// Returns a tuple with the tx and rx for each state machine.
    pub fn unininstall_program(
        &mut self,
        rxtx: (Option<RxTx<P, SM0>>, Option<RxTx<P, SM1>>, Option<RxTx<P, SM2>>, Option<RxTx<P, SM3>>)
    ) -> Result<(), state_machine::Error> {
		if let (Some((rx, tx)), _, _, _) = rxtx {
			self.sm0.uninstall(rx, tx)?;
		}
		if let (_, Some((rx, tx)), _, _) = rxtx {
			self.sm1.uninstall(rx, tx)?;
		}
		if let (_, _, Some((rx, tx)), _) = rxtx {
			self.sm2.uninstall(rx, tx)?;
		}
		if let (_, _, _, Some((rx, tx))) = rxtx {
			self.sm3.uninstall(rx, tx)?;
		}

		self.in_use = false;
		return Ok(());
    }

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
