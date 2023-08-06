//! A fast and simple, zero-dependency 3D renderer library that runs on the CPU.
//!
//! Can be compiled for any platform that supports the rust std Library.
//! 
//! See a demo here:
//! 
//! - [`rusterer`]
//!
//! [`rusterer`]: https://paulbryden.github.io/rusterer



pub mod draw;
pub mod framebuffer;
pub mod geometry;
pub mod renderer;
pub mod texture;

#[cfg(feature = "loader_helper")]
pub mod texture_helper;