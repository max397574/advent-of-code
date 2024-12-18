#![feature(iter_array_chunks)]
#![feature(iter_map_windows)]
#![feature(iter_next_chunk)]
#![feature(extract_if)]
#![feature(slice_split_once)]
#![feature(let_chains)]
#![feature(portable_simd)]
#![feature(if_let_guard)]
#![feature(core_intrinsics)]
#![feature(slice_ptr_get)]
#![allow(internal_features)]
#![allow(clippy::type_complexity)]
#![allow(static_mut_refs)]

pub mod utils;

pub use utils as aoc;

pub mod solutions;

pub use crate::solutions::year2024::*;
