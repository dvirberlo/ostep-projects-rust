use io::BufRead;
use io::BufReader;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, Stdio};

const ERROR_MSG: &str = "wish: cannot open file\n";
const SHELL_START: &str = "wish> ";
const SHELL_ERROR: &str = "An error has occurred\n";

fn main() -> Result<(), std::io::Error> {
    let mut paths: Vec<String> = Vec::new();
    paths.push("/bin".to_string());
    paths.push("/usr/bin".to_string());

    let args: Vec<String> = std::env::args().collect();
    let args_len: usize = args.len();

    if args_len < 2 {
        interactive(&mut paths)?;
    } else {
        from_file(&args[1..], &mut paths);
    }
    Ok(())
}

fn interactive(paths: &mut Vec<String>) -> Result<(), std::io::Error> {
    let mut line_buffer: String;
    loop {
        print!("{}", SHELL_START);
        io::stdout().flush()?;
        line_buffer = String::new();
        io::stdin().read_line(&mut line_buffer)?;
        wish_line(line_buffer.trim_end().to_string(), paths);
    }
}

fn from_file(filenames: &[String], paths: &mut Vec<String>) {
    let mut file: File;
    let mut reader: BufReader<File>;
    for filename in filenames {
        file = match File::open(&filename) {
            Ok(file) => file,
            Err(_) => {
                print!("{}", ERROR_MSG);
                std::process::exit(1);
            }
        };
        reader = BufReader::new(file);
        for line in reader.lines() {
            match line {
                Ok(line) => wish_line(line, paths),
                Err(_) => {}
            }
        }
    }
}

fn wish_line(line: String, paths: &mut Vec<String>) {
    let procs: Vec<&str> = line
        .split('&')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.trim())
        .collect();
    let mut words: Vec<&str>;
    let mut output: Vec<u8>;
    // concurrent use of path command may cause unpredictable results
    for proc in procs {
        words = proc.split(' ').collect();
        output = match wish_cmd(&words, paths, &[]) {
            Ok(output) => output,
            Err(_) => {
                _error();
                break;
            }
        };
        print!("{}", std::str::from_utf8(&output).unwrap());
    }
}

fn wish_cmd(
    words: &Vec<&str>,
    paths: &mut Vec<String>,
    in_bytes: &[u8],
) -> Result<Vec<u8>, std::io::Error> {
    let first: &str = words[0];
    if first == "" {
        _error_exit();
    }
    match first {
        "exit" => _exit(&words),
        "cd" => _cd(&words),
        "path" => _path(&words, paths),
        &_ => match wish_search(&words, paths) {
            Ok(path) => {
                return _exec(&words, path, words[0], &in_bytes);
            }
            Err(_) => _error(),
        },
    }
    Ok((&[]).to_vec())
}
fn _exit(_word: &Vec<&str>) {
    if _word.len() == 1 {
        std::process::exit(0);
    } else {
        _error();
    }
}
fn _cd(words: &Vec<&str>) {
    if words.len() != 2 {
        return _error();
    }
    match std::env::set_current_dir(words[1]) {
        Ok(_) => {}
        Err(_) => _error(),
    }
}
fn _path(words: &Vec<&str>, paths: &mut Vec<String>) {
    paths.clear();
    for word in words.iter().skip(1) {
        paths.push(word.to_string());
    }
}

fn wish_search(words: &Vec<&str>, paths: &Vec<String>) -> Result<PathBuf, std::io::Error> {
    let mut file_path: &Path;
    for path in paths {
        file_path = Path::new(path);
        if file_path.join(words[0]).exists() {
            return Ok(file_path.to_path_buf());
        }
    }
    Err((std::io::ErrorKind::NotFound).into())
}

fn _exec(
    words: &Vec<&str>,
    file_dir: PathBuf,
    cmd: &str,
    in_bytes: &[u8],
) -> Result<Vec<u8>, std::io::Error> {
    // this overhead because the required error message is (for $ls non-existent) "ls: ..." (while if direct path is given, it is "/bin/ls: ...")
    let current: PathBuf = std::env::current_dir()?;
    std::env::set_current_dir(file_dir)?;
    let mut cmd: Command = Command::new(Path::new(cmd).to_path_buf());
    std::env::set_current_dir(current)?;
    for word in words.iter().skip(1) {
        cmd.arg(word);
    }
    let mut child: std::process::Child =
        cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).spawn()?;

    let in_bytes: Vec<u8> = in_bytes.to_vec();
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(&in_bytes[..])
            .expect("Failed to write to stdin");
    });

    let output: std::process::Output = child.wait_with_output()?;
    Ok(output.stdout.to_vec())
}

fn _error() {
    eprint!("{}", SHELL_ERROR);
}
fn _error_exit() -> ! {
    eprint!("{}", ERROR_MSG);
    std::process::exit(1);
}
