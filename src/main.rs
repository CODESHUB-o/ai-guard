use clap::{Parser, Subcommand};
use std::fs;
use std::process;

mod redteam;
mod scanner;
mod utils;

#[derive(Parser)]
#[command(name = "ai_guard")]
#[command(
    version = env!("CARGO_PKG_VERSION"),
    about = "Behavioral LLM Injection & Drift Detection Engine"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Multiturn {
        file: String,

        #[arg(long, default_value_t = 70)]
        threshold: u32,

        #[arg(long)]
        json: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {

        Commands::Multiturn { file, threshold, json } => {

            let content =
                fs::read_to_string(&file)
                    .expect("Failed to read transcript file");

            let result =
                redteam::multistep::run_multiturn(&content);

            if json {
                println!(
                    "{{\"composite_risk\":{},\"risk_level\":\"{}\"}}",
                    result.composite_risk_score,
                    result.risk_level
                );
            } else {
                println!("\nAI Guard — Behavioral Risk Analysis");
                println!("------------------------------------");
                println!("User Turns           : {}", result.total_user_turns);
                println!("Peak Severity        : {}", result.peak_severity);
                println!("Final Severity       : {}", result.final_severity);
                println!("Drift Detected       : {}", result.drift_detected);
                println!("Partial Suppression  : {}", result.partial_bypass);
                println!("Oscillation Detected : {}", result.oscillation_detected);
                println!("Late Detection       : {}", result.late_detection);
                println!("Threshold Gaming     : {}", result.threshold_gaming);
                println!("Composite Risk Score : {}", result.composite_risk_score);
                println!("Risk Level           : {}", result.risk_level);
                println!("CI Threshold         : {}", threshold);
                println!();
            }

            if result.composite_risk_score >= threshold {
                process::exit(1);
            } else {
                process::exit(0);
            }
        }
    }
}