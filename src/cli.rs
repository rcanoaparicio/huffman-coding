use std::process::Output;

use clap::Parser;

pub enum Command {
    EncodeFile(String, String),
    DecodeFile(String, String),
    Encode(String, String),
    Decode(String, String),
    InvalidCommand,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Action to perform
    #[arg(short, long, default_value = "encode")]
    action: String,

    /// Input
    #[arg(short, long, default_value = "")]
    input: String,

    /// Output
    #[arg(short, long)]
    output: String,

    /// Data
    #[arg(short, long, default_value = "")]
    data: String,
}

pub fn get_command() -> Command {
    let args = Args::parse();
    let action = args.action;
    let data = args.data;
    let input = args.input;
    let output = args.output;

    if action == "encode" {
        if !data.is_empty() {
            return Command::Encode(data, output);
        }
        if !input.is_empty() {
            return Command::EncodeFile(input, output);
        }
    }
    if action == "decode" {
        if !data.is_empty() {
            return Command::Decode(data, output);
        }
        if !input.is_empty() {
            return Command::DecodeFile(input, output);
        }
    }

    Command::InvalidCommand
}
