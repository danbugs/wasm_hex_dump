use clap::Parser;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

/// CLI tool to convert a Wasm file to a hex string and write to an output file.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the input Wasm file
    #[arg(short, long, value_name = "FILE")]
    input: PathBuf,

    /// Path to the output file
    #[arg(short, long, value_name = "FILE", default_value = "output.txt")]
    output: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // Read the input Wasm file
    let mut file = File::open(&args.input)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Convert binary data to hex string
    let hex_string = hex::encode(buffer);

    // Determine the output filename
    let output_path = args.output.to_string_lossy().into_owned();

    // Write the hex string to the output file
    let mut output_file = File::create(&output_path)?;
    output_file.write_all(hex_string.as_bytes())?;

    println!("Hex dump written to {}", output_path);

    Ok(())
}
