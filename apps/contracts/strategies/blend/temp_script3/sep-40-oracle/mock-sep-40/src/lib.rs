#![no_std]

#[cfg(any(test, feature = "testutils"))]
extern crate std;

mod error;
mod mock_sep_40;
mod storage;
mod test;

pub use mock_sep_40::*;
