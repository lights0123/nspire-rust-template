#![no_std]
#![no_main]
extern crate ndless_handler;

use ndless::msg::msg;
use ndless::prelude::*;

#[entry]
fn main() {
	msg("Hello", "Hello, World!");
}
