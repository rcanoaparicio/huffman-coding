mod cli;
mod decoder;
mod encoder;

use std::fs;

fn read_file_contents(path: &str) -> Vec<u8> {
    let data = fs::read(path).expect("Error while reading the file");
    data
}

fn write_contents_to_file(path: &str, contents: &Vec<u8>) {
    let _ = fs::write(path, contents);
}

fn main() {
    let command = cli::get_command();
    match command {
        cli::Command::Encode(contents, output) => {
            let data = contents.as_bytes();
            let encoded_contents = encoder::encode_data(&data.to_vec());
            write_contents_to_file(&output, &encoded_contents);
        }
        cli::Command::EncodeFile(file, output) => {
            let data = read_file_contents(&file);
            let encoded_contents = encoder::encode_data(&data.to_vec());
            write_contents_to_file(&output, &encoded_contents);
        }
        cli::Command::Decode(contents, output) => {
            let data = contents.as_bytes();
            let decoded_contents = decoder::decode_data(&data.to_vec());
            write_contents_to_file(&output, &decoded_contents);
        }
        cli::Command::DecodeFile(file, output) => {
            let data = read_file_contents(&file);
            let decoded_contents = decoder::decode_data(&data.to_vec());
            write_contents_to_file(&output, &decoded_contents);
        }
        _ => panic!("Invalid args"),
    }
}
