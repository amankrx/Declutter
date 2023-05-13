//! # Declutter
#![allow(clippy::new_without_default)]
#![allow(clippy::type_complexity)]
#![warn(clippy::await_holding_refcell_ref)]
#![warn(clippy::cast_lossless)]
#![warn(clippy::comparison_to_empty)]
#![warn(clippy::manual_find_map)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::redundant_closure_for_method_calls)]
#![warn(clippy::struct_excessive_bools)]
#![warn(clippy::unnecessary_unwrap)]
#![warn(clippy::wildcard_imports)]
#![warn(clippy::trivially_copy_pass_by_ref)]
pub mod config;
pub mod core;
pub mod models;
pub mod schema;
// pub mod views;
// pub mod widgets;
pub mod windows;

pub mod prelude {
    // use anyhow::Result;
    pub use gtk::{gdk, gio, glib};
    // use std::{future::Future, pin::Pin};
}
