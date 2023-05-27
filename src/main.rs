#![no_std]
#![no_main]

use vex_rt::prelude::*;

struct PissBot;

impl Robot for PissBot {
    fn new(_peripherals: Peripherals) -> Self {
        println!("Hello, pissbot");
        Self
    }
}

entry!(PissBot);
