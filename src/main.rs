mod cli;
mod decoder;
mod encoder;

use std::fs;

fn read_file_contents(path: &str) -> Vec<u8> {
    let data = fs::read(path).expect("Error while reading the file");
    data
}

fn main() {
    let command = cli::get_command();
    match command {
        cli::Command::Encode(contents) => {
            let data = contents.as_bytes();
            let encoded_contents = encoder::encode_data(&data.to_vec());
            let result_string = encoded_contents
                .iter()
                .map(|&c| c as char)
                .collect::<String>();
            println!("{result_string}");
        }
        cli::Command::EncodeFile(file) => {
            let data = read_file_contents(&file);
            let encoded_contents = encoder::encode_data(&data.to_vec());
            let result_string = encoded_contents
                .iter()
                .map(|&c| c as char)
                .collect::<String>();
            println!("{result_string}");
        }
        cli::Command::Decode(contents) => {
            let data = contents.as_bytes();
            let decoded_contents = decoder::decode_data(&data.to_vec());
            let result_string = decoded_contents
                .iter()
                .map(|&c| c as char)
                .collect::<String>();
            println!("{result_string}");
        }
        cli::Command::DecodeFile(file) => {
            let data = read_file_contents(&file);
            let decoded_contents = decoder::decode_data(&data.to_vec());
            let result_string = decoded_contents
                .iter()
                .map(|&c| c as char)
                .collect::<String>();
            println!("{result_string}");
        }
        _ => panic!("Invalid args"),
    }
}
