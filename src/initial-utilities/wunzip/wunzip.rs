use std::fs::File;
use std::io::Read;
use std::iter::Iterator;

const HELP_MSG: &str = "wunzip: file1 [file2 ...]\n";
const ERROR_MSG: &str = "wunzip: cannot open file\n";

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    let args_len = args.len();
    if args_len < 2 {
        print!("{}", HELP_MSG);
        std::process::exit(1);
    } else {
        for filename in args.iter().skip(1) {
            wunzip(filename)?;
        }
    }
    Ok(())
}

fn wunzip(filename: &str) -> Result<(), std::io::Error> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => {
            print!("{}", ERROR_MSG);
            std::process::exit(1);
        }
    };
    let mut num_buf = [0; 4];
    let mut char_buf = [0; 1];
    let mut handle;
    let mut dup: u32;
    let mut size: usize;
    loop {
        handle = file.take(4);
        handle.read(&mut num_buf)?;
        file = handle.into_inner();
        dup = as_u32(&num_buf);
        handle = file.take(1);
        size = handle.read(&mut char_buf)?;
        file = handle.into_inner();
        // ignore new-line
        // if char_buf[0] == 10 {
        //     continue;
        // }
        if size == 0 {
            break;
        }
        wextract(char_buf[0] as char, dup);
    }
    Ok(())
}

fn wextract(c: char, dup: u32) {
    for _ in 0..dup {
        print!("{}", c);
    }
}

fn as_u32(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 0)
        + ((array[1] as u32) << 8)
        + ((array[2] as u32) << 16)
        + ((array[3] as u32) << 24)
}
