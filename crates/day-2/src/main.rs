#![feature(iter_map_windows)]
pub mod day2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day2::day2_1()?;
    day2::day2_2()?;
    Ok(())
}
