//! 🦀🚀🔥 A blazingly fast and memory-efficient implementation of `if err != nil` 🔥🚀🦀

use std::error::Error;

pub mod prelude {
    pub use crate::{error, nil};
}

/// An alias for Box<dyn Error> that even the noobiest of googlers can understand.
#[allow(non_camel_case_types)]
pub type error = Box<dyn Error>;

#[doc(hidden)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nil;

/// `Nil` isn't a real type.
/// `Nil` isn't a real type.
/// `Nil` isn't a real type.
/// `Nil` isn't a real type.
/// `Nil` isn't a real type.
/// `Nil` isn't a real type.
/// `Nil` isn't a real type.
/// `Nil` isn't a real type.
/// `Nil` isn't a real type.
/// `Nil` isn't a real type.
/// `Nil` isn't a real type.
/// `Nil` isn't a real type.
/// `Nil` isn't a real type.
/// `Nil` isn't a real type.
/// `Nil` isn't a real type.
pub fn nil() -> Box<Nil> {
    Box::new(Nil)
}

impl Error for Nil {}

impl std::fmt::Display for Nil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<nil>")
    }
}

impl PartialEq<Nil> for Box<dyn Error> {
    fn eq(&self, other: &Nil) -> bool {
        if let Some(_nil) = self.downcast_ref::<Nil>() {
            _nil == other
        } else {
            false
        }
    }
}

impl PartialEq<Box<Nil>> for Box<dyn Error> {
    fn eq(&self, other: &Box<Nil>) -> bool {
        if let Some(_nil) = self.downcast_ref::<Nil>() {
            _nil == other.as_ref()
        } else {
            false
        }
    }
}
