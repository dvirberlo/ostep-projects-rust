use std::env;
use std::fs::File;
use std::io::Read;

const ERROR_MSG: &str = "wcat: cannot open file\n";

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    for filename in args.iter().skip(1) {
        cat_file(filename)?;
    }
    Ok(())
}

fn cat_file(filename: &str) -> Result<(), std::io::Error> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => {
            print!("{}", ERROR_MSG);
            std::process::exit(1);
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    print!("{}", contents);
    Ok(())
}
