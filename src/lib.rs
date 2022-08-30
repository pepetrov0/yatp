#![deny(warnings, missing_docs, unsafe_code, nonstandard_style)]

//!
//! YATP (Yet Another Texture Packer)
//!
//! A small and simple CLI application to pack multiple textures/sprites into a texture atlas/sprite sheet.
//!

mod cli;
mod context;
pub mod dictionary;
mod format;
mod packer;

pub use cli::Cli;
pub use context::Context;
pub use format::DictionaryFormat;
pub use format::ImageFormat;
pub use packer::Packer;

/// Alias for euclid's Size2D of 2 unsigned 32-bit integers
pub type Size = euclid::Size2D<u32, u32>;

/// Alias for euclid's Point2D of 2 unsigned 32-bit integers
pub type Point = euclid::Point2D<u32, u32>;

/// Alias for euclid's Rect of 2 unsigned 32-bit integers
pub type Rect = euclid::Rect<u32, u32>;

/// Alias for image's RGBA color of 8-bit unsigned byte per color channel
pub type Color = image::Rgba<u8>;
