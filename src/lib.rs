#![feature(iter_array_chunks)]
#![feature(iter_map_windows)]
#![feature(iter_next_chunk)]
#![feature(extract_if)]
#![feature(slice_split_once)]
#![feature(let_chains)]
#![feature(portable_simd)]

pub mod utils;

pub use utils as aoc;

pub mod solutions;

pub use crate::solutions::year2024::*;
