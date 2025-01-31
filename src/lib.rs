#![feature(abi_msp430_interrupt)]
#![doc = "Peripheral access API for MSP430G2231 microcontrollers (generated using svd2rust v0.28.0 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next]
svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.28.0/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(private_in_public)]
#![deny(unconditional_recursion)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
use core::marker::PhantomData;
use core::ops::Deref;
#[allow(unused_imports)]
use generic::*;
#[cfg(feature = "rt")]
pub use msp430_rt::interrupt;
#[doc = "Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "msp430-interrupt" {
    fn PORT1();
    fn PORT2();
    fn USI();
    fn ADC10();
    fn TIMERA1();
    fn TIMERA0();
    fn WDT();
    fn NMI();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "msp430-interrupt" fn(),
    _reserved: u16,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static __INTERRUPTS: [Vector; 15] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: PORT1 },
    Vector { _handler: PORT2 },
    Vector { _handler: USI },
    Vector { _handler: ADC10 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: TIMERA1 },
    Vector { _handler: TIMERA0 },
    Vector { _handler: WDT },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: NMI },
];
#[doc = r"Enumeration of all the interrupts. This enum is seldom used in application or library crates. It is present primarily for documenting the device's implemented interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "2 - 0xFFE4 Port 1"]
    PORT1 = 2,
    #[doc = "3 - 0xFFE6 Port 2"]
    PORT2 = 3,
    #[doc = "4 - 0xFFE8 USI"]
    USI = 4,
    #[doc = "5 - 0xFFEA ADC10"]
    ADC10 = 5,
    #[doc = "8 - 0xFFF0 Timer A CC1, TA"]
    TIMERA1 = 8,
    #[doc = "9 - 0xFFF2 Timer A CC0"]
    TIMERA0 = 9,
    #[doc = "10 - 0xFFF4 Watchdog Timer"]
    WDT = 10,
    #[doc = "14 - 0xFFFC Non-maskable"]
    NMI = 14,
}
#[doc = "Special Function"]
pub struct SPECIAL_FUNCTION {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPECIAL_FUNCTION {}
impl SPECIAL_FUNCTION {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const special_function::RegisterBlock = 0 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const special_function::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SPECIAL_FUNCTION {
    type Target = special_function::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPECIAL_FUNCTION {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPECIAL_FUNCTION").finish()
    }
}
#[doc = "Special Function"]
pub mod special_function;
#[doc = "Port 1/2"]
pub struct PORT_1_2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORT_1_2 {}
impl PORT_1_2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const port_1_2::RegisterBlock = 0x20 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const port_1_2::RegisterBlock {
        Self::PTR
    }
}
impl Deref for PORT_1_2 {
    type Target = port_1_2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for PORT_1_2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PORT_1_2").finish()
    }
}
#[doc = "Port 1/2"]
pub mod port_1_2;
#[doc = "ADC10"]
pub struct ADC10 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC10 {}
impl ADC10 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const adc10::RegisterBlock = 0x48 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc10::RegisterBlock {
        Self::PTR
    }
}
impl Deref for ADC10 {
    type Target = adc10::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for ADC10 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC10").finish()
    }
}
#[doc = "ADC10"]
pub mod adc10;
#[doc = "System Clock"]
pub struct SYSTEM_CLOCK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTEM_CLOCK {}
impl SYSTEM_CLOCK {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const system_clock::RegisterBlock = 0x52 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const system_clock::RegisterBlock {
        Self::PTR
    }
}
impl Deref for SYSTEM_CLOCK {
    type Target = system_clock::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SYSTEM_CLOCK {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYSTEM_CLOCK").finish()
    }
}
#[doc = "System Clock"]
pub mod system_clock;
#[doc = "USI"]
pub struct USI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USI {}
impl USI {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const usi::RegisterBlock = 0x78 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usi::RegisterBlock {
        Self::PTR
    }
}
impl Deref for USI {
    type Target = usi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for USI {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USI").finish()
    }
}
#[doc = "USI"]
pub mod usi;
#[doc = "Calibration Data"]
pub struct CALIBRATION_DATA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CALIBRATION_DATA {}
impl CALIBRATION_DATA {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const calibration_data::RegisterBlock = 0x10fe as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const calibration_data::RegisterBlock {
        Self::PTR
    }
}
impl Deref for CALIBRATION_DATA {
    type Target = calibration_data::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for CALIBRATION_DATA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALIBRATION_DATA").finish()
    }
}
#[doc = "Calibration Data"]
pub mod calibration_data;
#[doc = "Watchdog Timer"]
pub struct WATCHDOG_TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WATCHDOG_TIMER {}
impl WATCHDOG_TIMER {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const watchdog_timer::RegisterBlock = 0x0120 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const watchdog_timer::RegisterBlock {
        Self::PTR
    }
}
impl Deref for WATCHDOG_TIMER {
    type Target = watchdog_timer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for WATCHDOG_TIMER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WATCHDOG_TIMER").finish()
    }
}
#[doc = "Watchdog Timer"]
pub mod watchdog_timer;
#[doc = "Flash"]
pub struct FLASH {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH {}
impl FLASH {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const flash::RegisterBlock = 0x0128 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash::RegisterBlock {
        Self::PTR
    }
}
impl Deref for FLASH {
    type Target = flash::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for FLASH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH").finish()
    }
}
#[doc = "Flash"]
pub mod flash;
#[doc = "Timer A2"]
pub struct TIMER_A2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER_A2 {}
impl TIMER_A2 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer_a2::RegisterBlock = 0x012e as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer_a2::RegisterBlock {
        Self::PTR
    }
}
impl Deref for TIMER_A2 {
    type Target = timer_a2::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMER_A2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_A2").finish()
    }
}
#[doc = "Timer A2"]
pub mod timer_a2;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals."]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "SPECIAL_FUNCTION"]
    pub SPECIAL_FUNCTION: SPECIAL_FUNCTION,
    #[doc = "PORT_1_2"]
    pub PORT_1_2: PORT_1_2,
    #[doc = "ADC10"]
    pub ADC10: ADC10,
    #[doc = "SYSTEM_CLOCK"]
    pub SYSTEM_CLOCK: SYSTEM_CLOCK,
    #[doc = "USI"]
    pub USI: USI,
    #[doc = "CALIBRATION_DATA"]
    pub CALIBRATION_DATA: CALIBRATION_DATA,
    #[doc = "WATCHDOG_TIMER"]
    pub WATCHDOG_TIMER: WATCHDOG_TIMER,
    #[doc = "FLASH"]
    pub FLASH: FLASH,
    #[doc = "TIMER_A2"]
    pub TIMER_A2: TIMER_A2,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*."]
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Each of the returned peripherals must be used at most once."]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            SPECIAL_FUNCTION: SPECIAL_FUNCTION {
                _marker: PhantomData,
            },
            PORT_1_2: PORT_1_2 {
                _marker: PhantomData,
            },
            ADC10: ADC10 {
                _marker: PhantomData,
            },
            SYSTEM_CLOCK: SYSTEM_CLOCK {
                _marker: PhantomData,
            },
            USI: USI {
                _marker: PhantomData,
            },
            CALIBRATION_DATA: CALIBRATION_DATA {
                _marker: PhantomData,
            },
            WATCHDOG_TIMER: WATCHDOG_TIMER {
                _marker: PhantomData,
            },
            FLASH: FLASH {
                _marker: PhantomData,
            },
            TIMER_A2: TIMER_A2 {
                _marker: PhantomData,
            },
        }
    }
}
