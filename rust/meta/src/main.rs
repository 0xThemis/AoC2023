use std::path::PathBuf;

use aoc_traits::AdventOfCodeSolutions;
use clap::Parser;
use color_eyre::Result;

#[derive(Parser)]
struct AoCRunner {
    #[clap(short, long)]
    day: usize,
    #[clap(short, long)]
    input: PathBuf,
}

fn main() -> Result<()> {
    let args = AoCRunner::parse();

    let input = std::fs::read_to_string(&args.input)?;

    meta::AoC2023::solve_day(args.day, &input).map_err(|e| color_eyre::eyre::eyre!(e))?;

    Ok(())
}
