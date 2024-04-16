use pio::Program;
use rp2040_hal::pac::{PIO0, PIO1, RESETS};
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

pub struct Pio {
    pio0: PIO<PIO0>,
    pio0sm0: StateMachine<PIO0, SM0>,
    pio0sm1: StateMachine<PIO0, SM1>,
    pio0sm2: StateMachine<PIO0, SM2>,
    pio0sm3: StateMachine<PIO0, SM3>,

    pio1: PIO<PIO1>,
    pio1sm0: StateMachine<PIO1, SM0>,
    pio1sm1: StateMachine<PIO1, SM1>,
    pio1sm2: StateMachine<PIO1, SM2>,
    pio1sm3: StateMachine<PIO1, SM3>,
}

impl Pio {
    pub fn new(pio0: PIO0, pio1: PIO1, resets: &mut RESETS) -> Pio {
        let (pio0, pio0sm0, pio0sm1, pio0sm2, pio0sm3) = pio0.split(resets);
        let (pio1, pio1sm0, pio1sm1, pio1sm2, pio1sm3) = pio1.split(resets);

        let pio0sm0 = StateMachine::new(pio0sm0);
        let pio0sm1 = StateMachine::new(pio0sm1);
        let pio0sm2 = StateMachine::new(pio0sm2);
        let pio0sm3 = StateMachine::new(pio0sm3);
        let pio1sm0 = StateMachine::new(pio1sm0);
        let pio1sm1 = StateMachine::new(pio1sm1);
        let pio1sm2 = StateMachine::new(pio1sm2);
        let pio1sm3 = StateMachine::new(pio1sm3);

        Pio {pio0, pio0sm0, pio0sm1, pio0sm2, pio0sm3, pio1, pio1sm0, pio1sm1, pio1sm2, pio1sm3}
    }

    /// Installs a program to PIO0
    ///
    /// * `program` - The program to be loaded
    /// * `num_state_machines` - The number of state machines to load to
    /// * `state_machine_pins` - A tuple containing the pins to be controlled by
    /// each state machine. It is laid out as [(SM0 base pin, SM0 range), (SM1 base pin, SM1 range), None, None]
    /// * `state_machine_clock_divisors` - A tuple containing the clock divisors
    /// for each state machine. It is laid out the same as state_machine_pins, but
    /// with (clock int, clock frac)
    ///
    /// Returns a slice with the tx and rx for each state machine.
    pub fn install_program_pio0(
        mut self, program: Program<32>,
        num_state_machines: usize,
        state_machine_pins: (PinOption, PinOption, PinOption, PinOption),
        state_machine_clock_divisors: (ClockOption, ClockOption, ClockOption, ClockOption)
    ) -> Result<(Self, (Option<RxTx<PIO0, SM0>>, Option<RxTx<PIO0, SM1>>, Option<RxTx<PIO0, SM2>>, Option<RxTx<PIO0, SM3>>)), Error> {
        if num_state_machines > 4 {
            return Err(Error::TooManyStateMachinesRequested);
        }

        // Install the program to the PIO block
        let installed = self.pio0.install(&program).unwrap();

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
            let Ok((sm, rxtx)) = self.pio0sm0.program(&installed, pins, clock_divisor) else {
                return Err(Error::BadStateMachineProgramming);
            };
            self.pio0sm0 = sm;

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
            let Ok((sm, rxtx)) = self.pio0sm1.program(&installed, pins, clock_divisor) else {
                return Err(Error::BadStateMachineProgramming);
            };
            self.pio0sm1 = sm;

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
            let Ok((sm, rxtx)) = self.pio0sm2.program(&installed, pins, clock_divisor) else {
                return Err(Error::BadStateMachineProgramming);
            };
            self.pio0sm2 = sm;

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
            let Ok((sm, rxtx)) = self.pio0sm3.program(&installed, pins, clock_divisor) else {
                return Err(Error::BadStateMachineProgramming);
            };
            self.pio0sm3 = sm;

            rxtx3 = Some(rxtx);
        }

        return Ok((self, (rxtx0, rxtx1, rxtx2, rxtx3)));
    }

    /// Installs a program to PIO0
    ///
    /// * `program` - The program to be loaded
    /// * `num_state_machines` - The number of state machines to load to
    /// * `state_machine_pins` - A tuple containing the pins to be controlled by
    /// each state machine. It is laid out as [(SM0 base pin, SM0 range), (SM1 base pin, SM1 range), None, None]
    /// * `state_machine_clock_divisors` - A tuple containing the clock divisors
    /// for each state machine. It is laid out the same as state_machine_pins, but
    /// with (clock int, clock frac)
    ///
    /// Returns a slice with the tx and rx for each state machine.
    pub fn install_program_pio1(
        mut self, program: Program<32>,
        num_state_machines: usize,
        state_machine_pins: (PinOption, PinOption, PinOption, PinOption),
        state_machine_clock_divisors: (ClockOption, ClockOption, ClockOption, ClockOption)
    ) -> Result<(Self, (Option<RxTx<PIO1, SM0>>, Option<RxTx<PIO1, SM1>>, Option<RxTx<PIO1, SM2>>, Option<RxTx<PIO1, SM3>>)), Error> {
        if num_state_machines > 4 {
            return Err(Error::TooManyStateMachinesRequested);
        }

        // Install the program to the PIO block
        let installed = self.pio1.install(&program).unwrap();

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
            let Ok((sm, rxtx)) = self.pio1sm0.program(&installed, pins, clock_divisor) else {
                return Err(Error::BadStateMachineProgramming);
            };
            self.pio1sm0 = sm;

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
            let Ok((sm, rxtx)) = self.pio1sm1.program(&installed, pins, clock_divisor) else {
                return Err(Error::BadStateMachineProgramming);
            };
            self.pio1sm1 = sm;

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
            let Ok((sm, rxtx)) = self.pio1sm2.program(&installed, pins, clock_divisor) else {
                return Err(Error::BadStateMachineProgramming);
            };
            self.pio1sm2 = sm;

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
            let Ok((sm, rxtx)) = self.pio1sm3.program(&installed, pins, clock_divisor) else {
                return Err(Error::BadStateMachineProgramming);
            };
            self.pio1sm3 = sm;

            rxtx3 = Some(rxtx);
        }

        return Ok((self, (rxtx0, rxtx1, rxtx2, rxtx3)));
    }

    /// Uninstalls a program from PIO0
    ///
    /// * `rxtx` - The same rx and tx channels returned by install_program
    ///
    /// Returns a slice with the tx and rx for each state machine.
    pub fn unininstall_program_pio0(
        mut self,
        rxtx: (Option<RxTx<PIO0, SM0>>, Option<RxTx<PIO0, SM1>>, Option<RxTx<PIO0, SM2>>, Option<RxTx<PIO0, SM3>>)
    ) -> Result<Self, state_machine::Error> {
		if let (Some((rx, tx)), _, _, _) = rxtx {
			self.pio0sm0 = self.pio0sm0.uninstall(rx, tx)?;
		}
		if let (_, Some((rx, tx)), _, _) = rxtx {
			self.pio0sm1 = self.pio0sm1.uninstall(rx, tx)?;
		}
		if let (_, _, Some((rx, tx)), _) = rxtx {
			self.pio0sm2 = self.pio0sm2.uninstall(rx, tx)?;
		}
		if let (_, _, _, Some((rx, tx))) = rxtx {
			self.pio0sm3 = self.pio0sm3.uninstall(rx, tx)?;
		}

		return Ok(self);
    }

	/// Uninstalls a program from PIO1
    ///
    /// * `rxtx` - The same rx and tx channels returned by install_program
    ///
    /// Returns a slice with the tx and rx for each state machine.
    pub fn unininstall_program_pio1(
        mut self,
        rxtx: (Option<RxTx<PIO1, SM0>>, Option<RxTx<PIO1, SM1>>, Option<RxTx<PIO1, SM2>>, Option<RxTx<PIO1, SM3>>)
    ) -> Result<Self, state_machine::Error> {
		if let (Some((rx, tx)), _, _, _) = rxtx {
			self.pio1sm0 = self.pio1sm0.uninstall(rx, tx)?;
		}
		if let (_, Some((rx, tx)), _, _) = rxtx {
			self.pio1sm1 = self.pio1sm1.uninstall(rx, tx)?;
		}
		if let (_, _, Some((rx, tx)), _) = rxtx {
			self.pio1sm2 = self.pio1sm2.uninstall(rx, tx)?;
		}
		if let (_, _, _, Some((rx, tx))) = rxtx {
			self.pio1sm3 = self.pio1sm3.uninstall(rx, tx)?;
		}

		return Ok(self);
    }

    /// Returns whether or not PIO block 0 is in use
    pub fn pio0_in_use(&self) -> bool {
        self.pio0sm0.is_initialized() |
        self.pio0sm1.is_initialized() |
        self.pio0sm2.is_initialized() |
        self.pio0sm3.is_initialized()
    }

    /// Returns whether or not PIO block 1 is in use
    pub fn pio1_in_use(&self) -> bool {
        self.pio1sm0.is_initialized() |
        self.pio1sm1.is_initialized() |
        self.pio1sm2.is_initialized() |
        self.pio1sm3.is_initialized()
    }

    /// Starts the state machines in PIO0
    pub fn start0(mut self) -> Result<Self, state_machine::Error> {
        if self.pio0sm0.is_initialized() {
            let sm = self.pio0sm0.start()?;
            self.pio0sm0 = sm;
        }
        if self.pio0sm1.is_initialized() {
            let sm = self.pio0sm1.start()?;
            self.pio0sm1 = sm;
        }
        if self.pio0sm2.is_initialized() {
            let sm = self.pio0sm2.start()?;
            self.pio0sm2 = sm;
        }
        if self.pio0sm3.is_initialized() {
            let sm = self.pio0sm3.start()?;
            self.pio0sm3 = sm;
        }

        Ok(self)
    }

    /// Starts the state machines in PIO1
    pub fn start1(mut self) -> Result<Self, state_machine::Error> {
        if self.pio1sm0.is_initialized() {
            let sm = self.pio1sm0.start()?;
            self.pio1sm0 = sm;
        }
        if self.pio1sm1.is_initialized() {
            let sm = self.pio1sm1.start()?;
            self.pio1sm1 = sm;
        }
        if self.pio1sm2.is_initialized() {
            let sm = self.pio1sm2.start()?;
            self.pio1sm2 = sm;
        }
        if self.pio1sm3.is_initialized() {
            let sm = self.pio1sm3.start()?;
            self.pio1sm3 = sm;
        }

        Ok(self)
    }

    /// Starts the state machines in PIO0
    pub fn stop0(mut self) -> Result<Self, state_machine::Error> {
        if self.pio0sm0.is_initialized() {
            let sm = self.pio0sm0.stop()?;
            self.pio0sm0 = sm;
        }
        if self.pio0sm1.is_initialized() {
            let sm = self.pio0sm1.stop()?;
            self.pio0sm1 = sm;
        }
        if self.pio0sm2.is_initialized() {
            let sm = self.pio0sm2.stop()?;
            self.pio0sm2 = sm;
        }
        if self.pio0sm3.is_initialized() {
            let sm = self.pio0sm3.stop()?;
            self.pio0sm3 = sm;
        }

        Ok(self)
    }

    /// Starts the state machines in PIO1
    pub fn stop1(mut self) -> Result<Self, state_machine::Error> {
        if self.pio1sm0.is_initialized() {
            let sm = self.pio1sm0.stop()?;
            self.pio1sm0 = sm;
        }
        if self.pio1sm1.is_initialized() {
            let sm = self.pio1sm1.stop()?;
            self.pio1sm1 = sm;
        }
        if self.pio1sm2.is_initialized() {
            let sm = self.pio1sm2.stop()?;
            self.pio1sm2 = sm;
        }
        if self.pio1sm3.is_initialized() {
            let sm = self.pio1sm3.stop()?;
            self.pio1sm3 = sm;
        }

        Ok(self)
    }
}
