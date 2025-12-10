#![feature(iter_array_chunks)]
#![feature(ascii_char)]
#![feature(unchecked_shifts)]
#![feature(iter_map_windows)]
#![feature(iter_next_chunk)]
#![feature(binary_heap_into_iter_sorted)]
#![feature(slice_split_once)]
#![feature(portable_simd)]
#![feature(if_let_guard)]
#![feature(core_intrinsics)]
#![feature(slice_ptr_get)]
#![allow(dead_code)]
#![allow(unused_attributes)]
#![allow(internal_features)]
#![allow(clippy::type_complexity)]
#![allow(static_mut_refs)]
use std::{
    env::args,
    time::{Duration, Instant},
};

mod utils;

mod solutions;

pub use crate::solutions::year2025::*;

pub fn main() {
    let mut args = args();
    let _binary_name = args.next().unwrap();
    let year: usize = args.next().unwrap().parse().unwrap();
    let day: usize = args.next().unwrap().parse().unwrap();
    let part: usize = args.next().unwrap().parse().unwrap();
    let bench = args.any(|arg| arg == "--bench");

    let mut input = std::fs::read_to_string(format!("inputs/{}/day{}.txt", year, day)).unwrap();
    if !input.ends_with('\n') {
        input.push('\n');
    }

    let solution = if year < 2025 {
        &solutions::get_solutions()[year - 2015][day - 1][part - 1]
    } else {
        &solutions::get_solutions_12d()[year - 2025][day - 1][part - 1]
    };

    if bench {
        let mut total = Duration::ZERO;
        const WARMUP_MS: u64 = 500;
        const MS: u64 = 2000;
        let duration = Duration::from_secs(MS);
        let warmup_duration = Duration::from_millis(WARMUP_MS);
        let first_start = Instant::now();
        while first_start.elapsed() < warmup_duration {
            let _ = solution(&input);
        }
        let first_start = Instant::now();
        let mut runs = 0;
        while first_start.elapsed() < duration {
            let start = Instant::now();
            let _ = solution(&input);
            total += start.elapsed();
            runs += 1;
        }
        println!(
            "Average time with warmup runs over {WARMUP_MS} seconds and running over {MS} seconds ({runs} runs): {:?}",
            total / runs
        );
        return;
    }

    let start = std::time::Instant::now();
    let result = solution(&input);
    println!("Duration: {:?}", start.elapsed());
    println!("Result: {result:?}");
}
