use std::fs;
use std::process;

use rex::{Cli, ClapParser};

fn main() {
    let cli = Cli::parse();

    println!("File name: {}", cli.file_name);
    println!("Cols : {}", cli.cols);

    let cols = cli.cols;

    let bytes = fs::read_to_string(cli.file_name)
        .unwrap_or_else(|err| {
            println!("Unable to open file : {err}");
            process::exit(1);
        })
        .into_bytes();

    let mut byte_iter = bytes.chunks(cols).enumerate();
    while let Some((idx, chunk)) = byte_iter.next() {
        rex::print_line(chunk, idx, cols);
    }
}
