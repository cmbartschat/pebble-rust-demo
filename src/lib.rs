#![no_main]
#![no_std]

extern crate alloc;

mod app;
mod bitmaps;
mod content_indicator;
mod draw_commands;
mod flash;
mod heap;
mod move_box;
mod scroll;
mod spin;
mod time;
mod unsafe_content_indicator;

use crate::app::run_app;
use pebble_rust_2026 as _;

#[unsafe(no_mangle)]
fn main() -> i32 {
    run_app();
    0
}
