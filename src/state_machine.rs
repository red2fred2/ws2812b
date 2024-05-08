use core::cell::Cell;

use rp2040_hal::pio::{self, InstalledProgram, PIOBuilder, PIOExt, Running, Rx, StateMachineIndex, Stopped, Tx, UninitStateMachine};

#[derive(Debug)]
pub enum Error {
    ProgrammingFailed,
    FailedToStart,
    FailedToStop,
    NoProgramToUninstall,
}


enum StateMachineKind<PIO: PIOExt, SM: StateMachineIndex> {
    Running(pio::StateMachine<(PIO, SM), Running>),
    Stopped(pio::StateMachine<(PIO, SM), Stopped>),
    Uninitialized(UninitStateMachine<(PIO, SM)>),
    BeingSwapped,
}

pub struct StateMachine<PIO: PIOExt, SM: StateMachineIndex> {
    sm: Cell<StateMachineKind<PIO, SM>>,
    initialized: bool,
    running: bool,
}

impl<PIO: PIOExt, SM: StateMachineIndex> StateMachine<PIO, SM> {
    /// Create a new state machine from a pio state machine
    pub fn new(state_machine: UninitStateMachine<(PIO, SM)>) -> StateMachine<PIO, SM> {
        let sm = StateMachineKind::Uninitialized(state_machine);
        let sm = Cell::new(sm);

        StateMachine {sm, initialized: false, running: false}
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Program this state machine
    pub fn program(
        &mut self,
        installed: &InstalledProgram<PIO>,
        pins: (u8, u8),
        clock_divisor: (u16, u8)
    ) -> Result<(Rx<(PIO, SM)>, Tx<(PIO, SM)>), Error> {
        critical_section::with(|_| {
            // Make sure the machine is uninitialized before trying to program it
            let sm = self.sm.replace(StateMachineKind::BeingSwapped);
            let StateMachineKind::Uninitialized(sm) = sm else {
                return Err(Error::ProgrammingFailed)
            };

            // Get values in order
            let (base, count) = pins;
            let (int, frac) = clock_divisor;

            let program;
            unsafe {program = installed.share();}

            // Program it
            let (sm, rx, tx) = PIOBuilder
                ::from_installed_program(program)
                .set_pins(base, count)
                .clock_divisor_fixed_point(int, frac)
                .build(sm);

            // Change values
            self.sm = Cell::new(StateMachineKind::Stopped(sm));
            self.initialized = true;
            Ok((rx, tx))
        })
    }

    pub fn uninstall(&mut self, rx: Rx<(PIO, SM)>, tx: Tx<(PIO, SM)>) -> Result<(), Error> {
        critical_section::with(|_| {
            let sm = self.sm.replace(StateMachineKind::BeingSwapped);
            // Stop the machine if it's still running
            if let StateMachineKind::Running(_) = &sm {
                self.stop()?;
            }

            let StateMachineKind::Stopped(sm) = sm else {
                return Err(Error::NoProgramToUninstall)
            };

            let (sm, _) = sm.uninit(rx, tx);

            // Change values
            self.sm = Cell::new(StateMachineKind::Uninitialized(sm));
            self.initialized = false;
            Ok(())
        })
    }

    pub fn start(&mut self) -> Result<(), Error> {
        critical_section::with(|_| {
            let sm = self.sm.replace(StateMachineKind::BeingSwapped);
            let StateMachineKind::Stopped(sm) = sm else {
                return Err(Error::FailedToStart);
            };

            let sm = sm.start();
            self.sm = Cell::new(StateMachineKind::Running(sm));
            self.running = true;
            Ok(())
        })
    }

    pub fn stop(&mut self) -> Result<(), Error> {
        critical_section::with(|_| {
            let sm = self.sm.replace(StateMachineKind::BeingSwapped);
            let StateMachineKind::Running(sm) = sm else {
                return Err(Error::FailedToStop)
            };

            let sm = sm.stop();
            self.sm = Cell::new(StateMachineKind::Stopped(sm));
            self.running = false;
            Ok(())
        })
    }
}
