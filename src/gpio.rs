pub mod led {
    use stm32h7::stm32h743v::{Peripherals, GPIOC};

    // LedUserPin = hal::gpio::gpioc::PC7<hal::gpio::Analog>; // LED_USER
    fn setup_leds(per: &GPIOC) {
        // user led: PC7
        per.moder.write(|w| w.moder7().output());
        per.otyper.write(|w| w.ot7().push_pull());
        per.ospeedr.write(|w| w.ospeedr7().low_speed());
        per.pupdr.write(|w| w.pupdr7().floating());
    }

    pub fn set_led(per: &GPIOC) {
        per.bsrr.write(|w| w.bs7().set_bit());
    }

    pub fn reset_led(per: &GPIOC) {
        per.bsrr.write(|w| w.br7().set_bit());
    }

    pub fn toggle_led(per: &GPIOC) {
        let status = per.odr.read().odr7().bit();
        match status {
            false => set_led(per),
            true => reset_led(per),
        }
    }

    pub fn setup_gpios(per: &Peripherals) {
        setup_leds(&per.GPIOC);
    }
}

use core::marker::PhantomData;
use defmt::Format;
use stm32h7::stm32h743v::{
    GPIOA, GPIOB, GPIOC, GPIOD, GPIOE, GPIOF, GPIOG, GPIOH, GPIOI, GPIOJ, GPIOK,
};

pub trait GpioBank {}

impl GpioBank for GPIOA {}
impl GpioBank for GPIOB {}
impl GpioBank for GPIOC {}
impl GpioBank for GPIOD {}
impl GpioBank for GPIOE {}
impl GpioBank for GPIOF {}
impl GpioBank for GPIOG {}
impl GpioBank for GPIOH {}
impl GpioBank for GPIOI {}
impl GpioBank for GPIOJ {}
impl GpioBank for GPIOK {}

// This struct exists to ensure the pin value is contained in the 0-15 range
// as it's the maximum value allowed. This may still go over the packaging
// of the MCU, but it's already plenty to avoid some errors at compile time
// struct PinNumber {
//     pin_number: u8,
// }
//
// // FIXME: transfor this into some generic type parameters so it's can become
// // a typestate pattern later
// impl PinNumber {
//     const PIN_NUMBER_UPPER_BOUND: u8 = 15; // following datasheet
//     const fn new<const num: u8>() -> Self {
//         let n: u8 = const {
//             if num <= Self::PIN_NUMBER_UPPER_BOUND {
//                 num
//             } else {
//                 panic!("Invalid pin number");
//             }
//         };
//
//         PinNumber { pin_number: n }
//     }
// }

// impl UpperBounded for PinNumber {
//     const fn upper_bound() -> PinNumber {
//         15
//     }
// }

// #[derive(Debug, Format, Default)]
// pub enum GpioSpeed {
//     #[default]
//     Low,
//     Medium,
//     High,
//     VeryHigh,
// }
pub trait GpioSpeed {}
pub struct Low {}
pub struct Medium {}
pub struct High {}
pub struct VeryHigh {}
impl GpioSpeed for Low {}
impl GpioSpeed for Medium {}
impl GpioSpeed for High {}
impl GpioSpeed for VeryHigh {}

pub trait PinNumber {}
impl PinNumber for u8 {}

pub trait GpioMode {}
pub struct Output {}
pub struct Input {}
pub struct Alternate {}
impl GpioMode for Output {}
impl GpioMode for Input {}
impl GpioMode for Alternate {}

pub trait GpioPull {}
pub struct PullUp {}
pub struct PullDown {}
pub struct OpenDrain {}
impl GpioPull for PullUp {}
impl GpioPull for PullDown {}
impl GpioPull for OpenDrain {}

#[derive(Debug, Format)]
pub struct GpioPin<'a, B, N, S, M, P>
where
    B: GpioBank,
    N: PinNumber,
    S: GpioSpeed,
    M: GpioMode,
    P: GpioPull,
{
    bank: &'a B,
    pin: u8,
    _pin_number: PhantomData<N>,
    _pin_speed: PhantomData<S>,
    _pin_mode: PhantomData<M>,
    _pin_pull: PhantomData<P>,
}

impl<'a, B, N, S, M, P> GpioPin<'a, B, N, S, M, P>
where
    B: GpioBank,
    N: PinNumber,
    S: GpioSpeed,
    M: GpioMode,
    P: GpioPull,
{
    pub fn new(bank: &'a B, pin: u8) -> Self {
        Self {
            bank,
            pin,
            _pin_number: PhantomData,
            _pin_mode: PhantomData,
            _pin_speed: PhantomData,
            _pin_pull: PhantomData,
        }
    }
}
