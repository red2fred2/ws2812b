use rp2040_hal::pio::{self, InstalledProgram, PIOBuilder, PIOExt, Running, Rx, StateMachineIndex, Stopped, Tx, UninitStateMachine};

enum StateMachineKind<PIO: PIOExt, SM: StateMachineIndex> {
    Running(pio::StateMachine<(PIO, SM), Running>),
    Stopped(pio::StateMachine<(PIO, SM), Stopped>),
    Uninitialized(UninitStateMachine<(PIO, SM)>),
}

pub struct StateMachine<PIO: PIOExt, SM: StateMachineIndex> {
    sm: StateMachineKind<PIO, SM>
}

impl<PIO: PIOExt, SM: StateMachineIndex> StateMachine<PIO, SM> {
    /// Create a new state machine from a pio state machine
    pub fn new(state_machine: UninitStateMachine<(PIO, SM)>) -> StateMachine<PIO, SM> {
        let sm = StateMachineKind::Uninitialized(state_machine);

        StateMachine {sm}
    }

    pub fn is_initialized(&self) -> bool {
        match self.sm {
            StateMachineKind::Uninitialized(_) => false,
            _ => true
        }
    }

    /// Program this state machine
    pub fn program(
        mut self,
        installed: &InstalledProgram<PIO>,
        pins: (u8, u8),
        clock_divisor: (u16, u8)
    ) -> Result<(Self, (Rx<(PIO, SM)>, Tx<(PIO, SM)>)), Error> {
        // Make sure the machine is uninitialized before trying to program it
        let StateMachineKind::Uninitialized(sm) = self.sm  else {
            return Err(Error::ProgrammingFailed)
        };

        let (base, count) = pins;
        let (int, frac) = clock_divisor;

        let program;
        unsafe {program = installed.share();}

        let (sm, rx, tx) = PIOBuilder
            ::from_installed_program(program)
            .set_pins(base, count)
            .clock_divisor_fixed_point(int, frac)
            .build(sm);

        self.sm = StateMachineKind::Stopped(sm);

        return Ok((self, (rx, tx)))
    }

    pub fn start(mut self) -> Result<Self, Error> {
        let StateMachineKind::Stopped(sm) = self.sm else {
            return Err(Error::FailedToStart);
        };

        let sm = sm.start();
        self.sm = StateMachineKind::Running(sm);
        return Ok(self);
    }

    pub fn stop(mut self) -> Result<Self, Error> {
        let StateMachineKind::Running(sm) = self.sm else {
            return Err(Error::FailedToStop);
        };

        let sm = sm.stop();
        self.sm = StateMachineKind::Stopped(sm);
        return Ok(self);
    }
}

#[derive(Debug)]
pub enum Error {
    ProgrammingFailed,
    FailedToStart,
    FailedToStop,
}
