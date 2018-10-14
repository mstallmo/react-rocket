//! Igniter is a Rocket.rs fairing for a better frontend development experience

#![feature(plugin, decl_macro)]
#![deny(missing_docs)]

extern crate rocket;
#[macro_use]
extern crate log;
pub mod engine;