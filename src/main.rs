use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage rex [input_file]");
        process::exit(1);
    }

    dbg!(&args);

    const COLS: u64 = 8;
    let input_file_path = &args[1];

    let bytes = fs::read_to_string(input_file_path)
        .unwrap_or_else(|err| {
            println!("Unable to open file : {err}");
            process::exit(1);
        })
        .into_bytes();

    let full_rows : u64 = (bytes.len() as u64)/COLS;
    let mut row_num : u64 = 0;

    for ii in 0..full_rows {
        print!("{:08X} ", row_num);

        let start = (ii*8) as usize;
        let end = (ii*8 + COLS) as usize;
        let bytes_slice = &bytes[start..end];

        for byte in bytes_slice {
            print!("{:2X} ", byte);
        }
        print!("  |"); 
        for byte in bytes_slice {
            print!("{}", *byte as char);
        }
        print!("|"); 
        print!("\n");

        row_num += 1;
    }

    let remaining = &bytes[(full_rows*COLS) as usize .. ];

    print!("{:08X} ", row_num);
    for byte in remaining {
        print!("{:02X} ", byte);
    }
    for ii in 0..(COLS - remaining.len() as u64) {
        print!("   ");
    }
    print!("  |"); 
    print!("\n");
}
