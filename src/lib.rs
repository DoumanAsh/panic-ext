//!Extension library to panic facilities to make it more usable
//!
//!- `alloc` - Enables `String` usage via `alloc`. This is useful until [message](https://doc.rust-lang.org/std/panic/struct.PanicInfo.html#method.message) is stable

#![no_std]
#![warn(missing_docs)]
#![allow(clippy::style)]

#[cfg(feature = "alloc")]
extern crate alloc;

use core::fmt;
use core::panic::{Location, PanicInfo};

///Panic's message definition
pub trait Message: fmt::Display + fmt::Debug {}

impl<T: fmt::Display + fmt::Debug> Message for T {}

#[inline(always)]
///Retrieves panic message from the dynamic payload
///
///Note that while it accepts any type, it is designed to work with panic's payload
pub fn downcast_payload<'a>(payload: &'a (dyn core::any::Any + Send + 'static)) -> &'a dyn Message {
    const DEFAULT_MESSAGE: &'static str = "panic occurred";
    match payload.downcast_ref::<&'static str>() {
        Some(message) => message,
        #[cfg(feature = "alloc")]
        None => match payload.downcast_ref::<alloc::string::String>() {
            Some(message) => message,
            None => &DEFAULT_MESSAGE,
        },
        #[cfg(not(feature = "alloc"))]
        None => &DEFAULT_MESSAGE,
    }
}

#[inline(always)]
///Retrieves panic message from the panic info
pub fn get_message<'a>(panic: &'a PanicInfo<'a>) -> &'a dyn Message {
    downcast_payload(panic.payload())
}

#[derive(Clone, Copy, Debug)]
///Panic details
pub struct PanicDetails<'a> {
    ///Panic location
    ///
    ///Location is optional in PanicInfo.
    ///
    ///Therefore if for some reason standard library shall remove it,
    ///instead it will be initialized with location of where message function is called
    pub location: &'a Location<'a>,
    ///Panic message which can string or
    ///[core::fmt::Arguments](https://doc.rust-lang.org/core/fmt/struct.Arguments.html)
    pub message: &'a dyn Message,
}

///Extension trait to provide better API for PanicInfo
pub trait PanicInfoExt<'a> {
    ///Access uniform details of panic
    fn panic_details(&'a self) -> PanicDetails<'a>;
}

impl<'a> PanicInfoExt<'a> for PanicInfo<'a> {
    #[track_caller]
    #[inline(always)]
    fn panic_details(&'a self) -> PanicDetails<'a> {
        let location = match self.location() {
            Some(location) => location,
            None => Location::caller(),
        };
        PanicDetails {
            location,
            message: get_message(self),
        }
    }
}

impl<'a> PanicInfoExt<'a> for &'a (dyn core::any::Any + Send + 'static) {
    #[track_caller]
    #[inline(always)]
    ///Retrieves panic details from the raw payload
    ///As raw payload doesn't have location information
    ///It shall be initialized from where this function is called
    fn panic_details(&'a self) -> PanicDetails<'a> {
        PanicDetails {
            location: Location::caller(),
            message: downcast_payload(*self),
        }
    }
}

#[cfg(feature = "alloc")]
impl<'a> PanicInfoExt<'a> for alloc::boxed::Box<dyn core::any::Any + Send + 'static> {
    #[track_caller]
    #[inline(always)]
    ///Retrieves panic details from the raw payload
    ///As raw payload doesn't have location information
    ///It shall be initialized from where this function is called
    fn panic_details(&'a self) -> PanicDetails<'a> {
        PanicDetails {
            location: Location::caller(),
            message: downcast_payload(self),
        }
    }
}

impl fmt::Display for PanicDetails<'_> {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.location, fmt)?;
        fmt.write_str(": ")?;
        fmt::Display::fmt(self.message, fmt)
    }
}
