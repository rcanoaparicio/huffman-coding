use clap::Parser;

pub enum Command {
    EncodeFile(String),
    DecodeFile(String),
    Encode(String),
    Decode(String),
    InvalidCommand,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Action to perform
    #[arg(short, long, default_value = "encode")]
    action: String,

    /// Number of times to greet
    #[arg(short, long, default_value = "")]
    file: String,

    /// Data
    #[arg(short, long, default_value = "")]
    data: String,
}

pub fn get_command() -> Command {
    let args = Args::parse();
    let action = args.action;
    let data = args.data;
    let file = args.file;

    if action == "encode" {
        if !data.is_empty() {
            return Command::Encode(data);
        }
        if !file.is_empty() {
            return Command::EncodeFile(file);
        }
    }
    if action == "decode" {
        if !data.is_empty() {
            return Command::Decode(data);
        }
        if !file.is_empty() {
            return Command::DecodeFile(file);
        }
    }

    Command::InvalidCommand
}
