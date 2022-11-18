use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

pub mod grasp;
pub mod karmarkar_karp;

#[derive(Parser, Debug)]
#[command(author, version = "1.0", about = "Solver for the multiway number partition problem", long_about = None)]
struct Args {
    #[arg(short, long)]
    input_file: String,
    #[arg(long)]
    k: u64,
}

fn main() {
    let args = Args::parse();
    let file = File::open(args.input_file).expect("Failed to open input file");

    let input: Vec<u64> = BufReader::new(file)
        .lines()
        .flat_map(|line| {
            line.expect("Error reading line")
                .split_ascii_whitespace()
                .map(|num| num.parse().expect("Expected numeric input"))
                .collect::<Vec<_>>()
        })
        .collect();

    let kk_start = Instant::now();
    let kk_result = karmarkar_karp::karmarkar_karp(&input, args.k);
    let kk_end = Instant::now();
    let kk_elapsed = kk_end - kk_start;

    let grasp_start = Instant::now();
    let grasp_result = grasp::grasp(&input, args.k, 100);
    let grasp_end = Instant::now();
    let grasp_elapsed = grasp_end - grasp_start;

    print!(
        "Heuristic,SolutionQuality,TimeInMillis\nGRASP,{},{}\nKarmarkar-Karp,{},{}\n",
        grasp_result.maximum_sum,
        grasp_elapsed.as_millis(),
        kk_result.maximum_sum,
        kk_elapsed.as_millis()
    );
}
