//!Extension library to panic facilities to make it more usable
//!
//!Requires Rust 1.81
//!
//!## Features
//!
//!- `alloc` - Enables usage of `alloc` types
//!- `std` - Enables `std::error::Error` impl on panic details. Implies `alloc`

#![no_std]
#![warn(missing_docs)]
#![allow(clippy::style)]

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use core::fmt;
use core::panic::{Location, PanicInfo};

///Panic's message definition
pub trait Message: fmt::Display + fmt::Debug {}

impl<T: fmt::Display + fmt::Debug> Message for T {}

#[track_caller]
#[inline(always)]
///Retrieves panic details
pub fn panic_details<'a>(payload: &'a impl PanicInfoExt<'a>) -> PanicDetails<'a, impl Message + 'a> {
    payload.panic_details()
}

#[inline(always)]
///Retrieves panic message
pub fn panic_message<'a>(payload: &'a impl PanicInfoExt<'a>) -> impl Message + 'a {
    payload.panic_message()
}

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

#[derive(Clone, Copy, Debug)]
///Panic details
pub struct PanicDetails<'a, M: 'a> {
    ///Panic location
    ///
    ///Location is optional in PanicInfo.
    ///
    ///Therefore if for some reason standard library shall remove it,
    ///instead it will be initialized with location of where message function is called
    pub location: &'a Location<'a>,
    ///Panic message which can string or
    ///[core::fmt::Arguments](https://doc.rust-lang.org/core/fmt/struct.Arguments.html)
    pub message: M,
}

impl<M: Message> core::error::Error for PanicDetails<'_, M> {
}

impl<M: Message> fmt::Display for PanicDetails<'_, M> {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.location, fmt)?;
        fmt.write_str(": ")?;
        fmt::Display::fmt(&self.message, fmt)
    }
}

///Extension trait to provide better API for PanicInfo
pub trait PanicInfoExt<'a> {
    ///Retrieves underlying panic message
    fn panic_message(&'a self) -> impl Message + 'a;

    #[track_caller]
    #[inline(always)]
    ///Access uniform details of panic
    ///
    ///Default implementation uses location of this function call.
    ///When panic location is known, it is overridden with specialized version
    fn panic_details(&'a self) -> PanicDetails<'a, impl Message + 'a> {
        PanicDetails {
            location: Location::caller(),
            message: self.panic_message(),
        }
    }
}

impl<'a> PanicInfoExt<'a> for PanicInfo<'a> {
    #[inline(always)]
    fn panic_message(&'a self) -> impl Message + 'a {
        self.message()
    }

    #[track_caller]
    #[inline(always)]
    fn panic_details(&'a self) -> PanicDetails<'a, impl Message + 'a> {
        let location = match self.location() {
            Some(location) => location,
            None => Location::caller(),
        };
        PanicDetails {
            location,
            message: self.panic_message()
        }
    }
}

impl<'a> PanicInfoExt<'a> for &'a (dyn core::any::Any + Send + 'static) {
    #[inline(always)]
    fn panic_message(&'a self) -> impl Message + 'a {
        downcast_payload(*self)
    }
}

#[cfg(feature = "alloc")]
impl<'a> PanicInfoExt<'a> for alloc::boxed::Box<dyn core::any::Any + Send + 'static> {
    #[inline(always)]
    fn panic_message(&'a self) -> impl Message + 'a {
        downcast_payload(self)
    }
}

#[cfg(feature = "std")]
impl<'a> PanicInfoExt<'a> for std::panic::PanicHookInfo<'a> {
    #[inline(always)]
    fn panic_message(&'a self) -> impl Message + 'a {
        downcast_payload(self.payload())
    }

    #[track_caller]
    #[inline(always)]
    fn panic_details(&'a self) -> PanicDetails<'a, impl Message + 'a> {
        let location = match self.location() {
            Some(location) => location,
            None => Location::caller(),
        };
        PanicDetails {
            location,
            message: self.panic_message()
        }
    }
}
