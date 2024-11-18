pub mod year2015;
pub mod year2016;
pub mod year2017;
pub mod year2018;
pub mod year2019;
pub mod year2020;
pub mod year2021;
pub mod year2022;
pub mod year2023;
pub mod year2024;

type Solutions = Vec<[[Box<dyn Fn(&str) -> String>; 2]; 25]>;

pub fn get_solutions() -> Solutions {
    Vec::from([
        year2015::get_solutions_per_year(),
        year2016::get_solutions_per_year(),
        year2017::get_solutions_per_year(),
        year2018::get_solutions_per_year(),
        year2019::get_solutions_per_year(),
        year2020::get_solutions_per_year(),
        year2021::get_solutions_per_year(),
        year2022::get_solutions_per_year(),
        year2023::get_solutions_per_year(),
        year2024::get_solutions_per_year(),
    ])
}
