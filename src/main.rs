use clap::{Parser, Subcommand}; // Import clap for command line argument parsing.
use colored::*; // Import colored for colorful console output.
use reqwest::blocking; // Import reqwest's blocking feature for making HTTP requests synchronously.
use serde::Deserialize; // Import Deserialize to convert JSON into Rust data structures.

#[derive(Parser)] // Derive the Parser trait to handle command line input.
#[command(name = "Currency Converter")] // Set the name of the CLI application.
#[command(about = "A beautiful CLI currency converter", long_about = None)] // Description of the application.
struct Cli {
    #[command(subcommand)] // Define subcommands for the CLI tool.
    command: Commands,   
}

#[derive(Subcommand)] // Derive the Subcommand trait for handling different actions.
enum Commands {
    Convert{
        #[arg(help = "Amount to convert")] // Describe the amount to be converted.
        amount: f64,
        #[arg(help = "Base currency (e.g., USD)")] // Describe the base currency.
        base: String,
        #[arg(help = "Target currency (e.g., EUR)")] // Describe the target currency.
        target: String,
    },
}

#[derive(Deserialize)] // Derive Deserialize to parse JSON response into this struct.
struct ExchangeRateResponse{
    rates: std::collections::HashMap<String, f64> // A hashmap to hold currency rates.
}

const API_URL: &str = "https://api.exchangerate-api.com/v4/latest/"; // API URL to fetch rates.

fn main() -> Result<(), reqwest::Error>{ // Main function that returns a Result type.
    let cli: Cli = Cli::parse(); // Parse command line arguments.
    match &cli.command { // Match against the command provided.
        Commands::Convert { amount, base, target } => { // Handle the 'convert' command.
            let url: String = format!("{}{}", API_URL, base); // Construct URL with base currency.
            let response: ExchangeRateResponse = blocking::get(&url)?.json()?; // Make HTTP request and parse JSON.
            if let Some(rate) = response.rates.get(target){ // Check if target currency rate exists.
                let converted_amount: f64 = amount * rate; // Calculate converted amount.
                println!(
                    "{} {} {} = {} {}", // Print conversion result in colored format.
                    "💵".green().bold(),
                    amount.to_string().yellow().bold(),
                    base.to_uppercase().blue().bold(),
                    converted_amount.to_string().yellow().bold(),
                    target.to_uppercase().blue().bold(),
                );
            }else{
                println!("{}","Invalid target currency".red().bold()); // Handle invalid currency error.
            }       
        }
    }
    Ok(()) // Return Ok if everything executes correctly.
}
