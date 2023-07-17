#![feature(iter_array_chunks)]
mod part_a;
mod part_b;
use std::fs;

fn main() {
    part_a::run();
    part_b::run();
}
