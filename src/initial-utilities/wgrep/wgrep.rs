use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;

const HELP_MSG: &str = "wgrep: searchterm [file ...]\n";
const ERROR_MSG: &str = "wgrep: cannot open file\n";

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();
    let args_len = args.len();
    if args_len < 2 {
        print!("{}", HELP_MSG);
        std::process::exit(1);
    } else if args_len == 2 {
        return grep_stdin(&args[1]);
    } else {
        return grep_file(&args[1], &args[2]);
    }
}

fn grep_stdin(search_term: &str) -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    return grep_lines(search_term, &mut lines);
}

fn grep_file(search_term: &str, filename: &str) -> Result<(), std::io::Error> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => {
            print!("{}", ERROR_MSG);
            std::process::exit(1);
        }
    };
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    return grep_lines(search_term, &mut lines);
}

fn grep_lines<T: BufRead>(search_term: &str, lines: &mut Lines<T>) -> Result<(), std::io::Error>{
    for line in lines {
        match line {
            Ok(line) => {
                if line.contains(search_term) {
                    print!("{}\n", line);
                }
            }
            Err(_) => {}
        }
    }
    Ok(())
}
