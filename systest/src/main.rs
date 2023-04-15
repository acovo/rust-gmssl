#![allow(bad_style, deprecated, clippy::all)]

use libc::*;
use gmssl_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
