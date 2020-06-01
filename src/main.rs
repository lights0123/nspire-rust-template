#![no_std]

extern crate ndless_handler;

use ndless::msg::msg;
use ndless::prelude::*;

fn main() {
	msg("Hello", "Hello, World!");
}
