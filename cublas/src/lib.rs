#![allow(dead_code)]

extern crate cublas_sys as ffi;

#[cfg(test)]
extern crate collenchyma as co;

pub use api::Context;
pub use error::Error;

#[derive(Debug, Copy, Clone)]
/// Defines the cuBLAS API.
pub struct API;

pub mod api;
pub mod error;
