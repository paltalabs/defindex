#![no_std]

mod allowance;
mod balance;
mod contract;
mod error;
mod storage;
mod test;

pub use crate::contract::{MockToken, MockTokenClient};
