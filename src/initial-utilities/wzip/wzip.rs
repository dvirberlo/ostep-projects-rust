use std::fs::File;
use std::io::Read;
use std::io::{self, StdoutLock, Write};
use std::iter::Iterator;

const HELP_MSG: &str = "wzip: file1 [file2 ...]\n";
const ERROR_MSG: &str = "wzip: cannot open file\n";

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    let args_len = args.len();
    if args_len < 2 {
        print!("{}", HELP_MSG);
        std::process::exit(1);
    } else {
        return wzip(&args[1..]);
    }
}

fn wzip(filenames: &[String]) -> Result<(), std::io::Error> {
    let mut current_char: char;
    let mut pre_char: char = '\0';
    let mut counter: u32 = 0;

    let mut file;
    let mut handle;
    let mut char_buf = [0; 1];
    let mut size;

    let mut stdout = io::stdout().lock();
    for filename in filenames {
        file = match File::open(&filename) {
            Ok(file) => file,
            Err(_) => {
                print!("{}", ERROR_MSG);
                std::process::exit(1);
            }
        };
        loop {
            handle = file.take(1);
            size = handle.read(&mut char_buf)?;
            file = handle.into_inner();
            current_char = char_buf[0] as char;
            if size == 0 {
                break;
            }
            if pre_char == '\0' {
                pre_char = current_char;
            } else if pre_char != current_char {
                write_out(&mut stdout, counter, pre_char)?;
                pre_char = current_char;
                counter = 0;
            }
            counter += 1;
        }
    }
    write_out(&mut stdout, counter, pre_char)?;
    Ok(())
}

fn write_out(out: &mut StdoutLock<'static>, counter: u32, c: char) -> Result<(), std::io::Error> {
    out.write_all(&counter.to_le_bytes())?;
    print!("{}", c);
    Ok(())
}
