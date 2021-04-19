//! x86 Settings.

use crate::settings::{self, detail, Builder};
use core::fmt;

// Include code generated by `cranelift-codegen/meta/src/gen_settings.rs:`. This file contains a
// public `Flags` struct with an impl for all of the settings defined in
// `cranelift-codegen/meta/src/isa/x86/settings.rs`.
include!(concat!(env!("OUT_DIR"), "/settings-x86.rs"));
