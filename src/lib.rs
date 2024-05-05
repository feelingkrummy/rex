
pub use clap::Parser as ClapParser;

#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Name of file to dump
    pub file_name: String, 

    #[arg(short, long, default_value_t = 8)]
    /// Number of Columns to Print
    pub cols: usize,

    #[arg(short, long, default_value_t = 4)]
    /// Number of Columns to Print
    pub word_width: usize,
}

pub fn print_line(bytes: &[u8], row_num: usize, cols: usize) {
    let remaning = cols - bytes.len();

    print!("{:08X} ", row_num*cols);

    for byte in bytes {
        print!("{:02X} ", byte);
    }
    for _ in 0..remaning {
        print!("   ");
    }
    print!("  "); 
    for byte in bytes {
        match *byte as char {
            '\n' => print!("."),
            _ => print!("{}", *byte as char),
        };
    }
    print!("\n");
}
