// Reponsibilities:
// - setup clocks
// - setup gpios
// - setup reset vectors and shit

use stm32h7::stm32h743v::Peripherals;
use thiserror::Error;
use defmt::Format;

pub struct Mcu {
    pub per: Peripherals
}

#[derive(Error, Debug, Format)]
pub enum InitiatizationError {
    //while self.per.PWR.d3cr.read().vosrdy().bit_is_clear() {
    #[error("VOSRDY bit doesn't get set")]
    VosRdyError,

    #[error("External XTAL didn't converge")]
    HseRdyError,

    #[error("PLL won't lock on late freq!")]
    PllEarlyConvergenceError,

    #[error("PLL won't lock on late freq!")]
    PllLateConvergenceError
}

impl Mcu {
    pub fn new(per: Peripherals) -> Self {
         Self{ per }
    }

    const INIT_TIMEOUT: u32 = 60000;
    pub fn system_init(&self) -> Result<(), InitiatizationError> {
        let mut timeout_counter: u32 = 0;
        // power
        self.per.PWR.cr3.write(|w| {
            w
            .scuen().clear_bit()
            .ldoen().clear_bit()
            .bypass().clear_bit()
            .ldoen().set_bit()
        });

        // enable syscfg to allow for VOS0 to be triggered
        self.per.RCC.apb4enr.write(|w| {
            w.syscfgen().set_bit()
        });

        // voltage scaling 1
        self.per.PWR.d3cr.write(|w| {
            unsafe {
                w.vos().bits(0b11)
            }
        });

        // will be needed to enable power overdrive
        self.per.SYSCFG.pwrcr.write(|w| {
            w.oden().set_bit()
        });

        while self.per.PWR.d3cr.read().vosrdy().bit_is_clear() {
            timeout_counter += 1;
        }
        if timeout_counter >= Self::INIT_TIMEOUT {
            return Err(InitiatizationError::VosRdyError);
        }
        timeout_counter = 0;


        // clock setup :)
        // going overdrive cause hell yeah! 480MHz on this b
        // Enable HSE
        self.per.RCC.cr.write(|w| {
            w
            .hseon().set_bit()
        });
        
        while self.per.RCC.cr.read().hserdy().is_not_ready() {
            timeout_counter += 1;
        }
        if timeout_counter >= Self::INIT_TIMEOUT {
            return Err(InitiatizationError::HseRdyError);
        }
        timeout_counter = 0;

        // switch to hse temporarily
        self.per.RCC.cfgr.write(|w| {
            w.sw().hse()
        });

        // disable hsi
        self.per.RCC.cr.write(|w| {
            w
            .hsion().clear_bit()
        });

        // disable PLL
        self.per.RCC.cr.write(|w| {
            w
            .pll1on().clear_bit()
        });

        // wait until pll is completely disabled
        while self.per.RCC.cr.read().pll1rdy().is_ready() {
            timeout_counter += 1;
        }
        if timeout_counter >= Self::INIT_TIMEOUT {
            return Err(InitiatizationError::PllEarlyConvergenceError);
        }
        timeout_counter = 0;


        // setup PLLs

        self.per.RCC.pllckselr.write(|w| {
            w
            // setup the divider before the prescaler
            .pllsrc().hse() // hse as pll src
            .divm1().bits(0u8)
            .divm1().bits(2u8) // bypass
        });

        // select PLL1 input reference frequency range
        // NOTE: HSE is 16MHz
        self.per.RCC.pllcfgr.write(|w| {
            w
            .pll1rge().range8()
            // select PLL1 output frequency range (bit = 0 -> wide range (192-960MHz))
            .pll1vcosel().clear_bit()
            // disable fractional mode
            .pll1fracen().clear_bit()
        });

        self.per.RCC.pll1divr.write(|w| {
            unsafe {
                w
                .divn1().bits((60 - 1) as u16) // 60-1
                .divp1().bits((2 - 1) as u8)
                // not used but configured cause why not
                .divq1().bits((2 - 1) as u8)
                .divr1().bits((2 - 1) as u8)
            }
        });

        self.per.RCC.pllcfgr.write(|w| {
            w
            .divp1en().set_bit()
            .divq1en().set_bit()
            .divr1en().set_bit()
        });

        // finally (god forbid) enable the PLL
        self.per.RCC.cr.write(|w| {
            w.pll1on().on()
        });

        // let pll_ready = self.per.RCC.cr.read().pll1rdy().bit_is_set();
        while self.per.RCC.cr.read().pll1rdy().is_not_ready() {
            timeout_counter += 1;
        }
        if timeout_counter >= Self::INIT_TIMEOUT {
            return Err(InitiatizationError::PllLateConvergenceError);
        }

        // initialize clocks on the AHB
        self.per.RCC.d1cfgr.write(|w| {
            w
            .d1cpre().div1()
            .hpre().div2()
            .d1ppre().div2()
        });

        self.per.RCC.cfgr.write(|w| {
            w
            .sw().pll1()
        });

        self.per.RCC.d2cfgr.write(|w| {
            w
            .d2ppre1().div2()
            .d2ppre2().div2()
        });

        self.per.RCC.d3cfgr.write(|w| {
            w
            .d3ppre().div2()
        });


        self.per.RCC.ahb4enr.write(|w| {
            w.gpiocen().set_bit()
        });

        Ok(())
    }


}
