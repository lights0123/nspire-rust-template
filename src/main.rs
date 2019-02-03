#![no_std]
#![no_main]
extern crate ndless_handler;

use nspire::prelude::*;
use nspire::msg::msg;

#[no_mangle]
fn main() {
	msg("Hello", "Hello, World!");
}
