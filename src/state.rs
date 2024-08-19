use cortex_m::register::{psp, lr, pc, fpscr};
use cortex_m::peripheral::SYST;
use defmt::{Format, write};
use core::default::Default;
use atomic_enum::atomic_enum;
use core::sync::atomic::{AtomicU32, Ordering};

#[atomic_enum]
#[derive(Format, Default)]
pub enum ApplicationState {
    #[default]
    Reset,
    Updating,
    Initialized,
    Error,
    Panic
}

#[derive(Debug)]
pub struct AppTrace {
    state: ApplicationState,
    // all of these can be accessed both in UNPRIVILEGED and PRIVILEGED modes
    pc: AtomicU32,
    lr: AtomicU32,
    psp: AtomicU32,
    fpscr: AtomicU32,
    systick: AtomicU32
}

impl Format for AppTrace {
    fn format(&self, fmt: defmt::Formatter) {
        write!(
            fmt,
            "AppTrace {{state: {}, pc: {:#X}, lr: {:#X}, psp: {:#X}, fpscr: {:#X}, systick: {}}}",
            self.state,
            self.pc.load(Ordering::Relaxed),
            self.lr.load(Ordering::Relaxed),
            self.psp.load(Ordering::Relaxed),
            self.fpscr.load(Ordering::Relaxed),
            self.systick.load(Ordering::Relaxed)
        )
    }
}

impl AppTrace {
    pub fn new() -> Self {
        Self { 
            state: ApplicationState::Reset,
            pc: AtomicU32::from(pc::read()),
            lr: AtomicU32::from(lr::read()),
            psp: AtomicU32::from(psp::read()),
            fpscr: AtomicU32::from(fpscr::read().bits()),
            systick: AtomicU32::from(SYST::get_current())
        }
    }

    pub fn update(&mut self, state: ApplicationState) {
        self.state = state;
        self.pc = AtomicU32::from(pc::read());
        self.lr = AtomicU32::from(lr::read());
        self.psp = AtomicU32::from(psp::read());
        self.fpscr = AtomicU32::from(fpscr::read().bits());
        self.systick = AtomicU32::from(SYST::get_current());
    }
}
