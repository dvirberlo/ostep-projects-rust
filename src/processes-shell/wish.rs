use io::BufRead;
use io::BufReader;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::vec;

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
    // Note: concurrent use of path command may cause unpredictable results
    // TODO: concurrency?
    for proc in procs {
        match wish_cmd(&proc, paths) {
            Ok(_) => {}
            Err(_) => {
                _error();
                break;
            }
        };
    }
}

enum Mode {
    Simple,
    Write,
}
fn wish_cmd(cmd: &str, paths: &mut Vec<String>) -> Result<Vec<u8>, std::io::Error> {
    let mut line: &str = cmd.clone().trim_start();
    if line.len() == 0 {
        _error_exit();
    }
    let mut first: &str;
    let mut args: Vec<&str> = vec![];
    let mut arg: &str = "";
    let mut output: Vec<u8> = vec![];
    let mut mode: Mode = Mode::Simple;
    while line.len() > 0 {
        first = "";
        args.clear();
        if !matches!(mode, Mode::Write) {
            first = get_token(&mut line);
            line = line[first.len()..].trim_start();
        }
        while line.len() > 0 {
            arg = get_token(&mut line);
            line = line[arg.len()..].trim_start();
            if arg == "|" || arg == ">" {
                break;
            } else {
                args.push(arg);
            }
        }
        match first {
            "exit" => _exit(&args),
            "cd" => _cd(&args),
            "path" => _path(&args, paths),
            &_ => {
                if matches!(mode, Mode::Write) {
                    output = _write(&args, output.clone());
                } else {
                    match wish_search(first, paths) {
                        Ok(path) => match _exec(&args, path, first, &output) {
                            Ok(out) => {
                                output = out;
                            }
                            Err(_) => {
                                _error();
                            }
                        },
                        Err(_) => {
                            _error();
                        }
                    };
                }
            }
        }
        mode = match arg {
            ">" => {
                // multipile redirercions required to be unsupported
                if matches!(mode, Mode::Write) {
                    break;
                }
                Mode::Write
            }
            &_ => Mode::Simple,
        }
    }
    if matches!(mode, Mode::Simple) {
        print!("{}", String::from_utf8(output).unwrap());
    } else {
        _error();
    }
    Ok((&[]).to_vec())
}

fn get_token(line: &str) -> &str {
    let mut i: usize = 0;
    for c in line.chars() {
        if c == ' ' || c == '>' || c == '|' {
            if i == 0 {
                i = 1;
            }
            break;
        }
        i += 1;
    }
    &line[0..i]
}

fn _write(args: &[&str], output: Vec<u8>) -> Vec<u8> {
    if args.len() != 1 || args[0].len() == 0 {
        _error();
        return vec![];
    }
    match std::fs::write(args[0], output) {
        Ok(_) => {
            return vec![];
        }
        Err(_) => {
            _error();
            return vec![];
        }
    }
}

fn _exit(args: &Vec<&str>) {
    if args.len() == 0 {
        std::process::exit(0);
    } else {
        _error();
    }
}
fn _cd(args: &Vec<&str>) {
    if args.len() != 1 {
        _error();
        return;
    }
    match std::env::set_current_dir(args[0]) {
        Ok(_) => {}
        Err(_) => {
            _error();
        }
    }
}
fn _path(args: &Vec<&str>, paths: &mut Vec<String>) {
    paths.clear();
    for word in args.iter() {
        paths.push(word.to_string());
    }
}

fn wish_search(cmd: &str, paths: &Vec<String>) -> Result<PathBuf, std::io::Error> {
    let mut file_path: &Path;
    for path in paths {
        file_path = Path::new(path);
        if file_path.join(cmd).exists() {
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
    // but for some reason it doesn't work when the path is relative, so here it is:
    let current: PathBuf = std::env::current_dir()?;
    let mut cmd: Command = match file_dir.is_absolute() {
        true => {
            std::env::set_current_dir(&file_dir.clone()).unwrap();
            Command::new(cmd)
        }
        false => Command::new(file_dir.join(cmd).to_str().unwrap()),
    };
    cmd.args(words.iter());

    let mut child: std::process::Child = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .current_dir(current.clone())
        .spawn()?;

    let in_bytes: Vec<u8> = in_bytes.to_vec();
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(&in_bytes)
            .expect("Failed to write to stdin");
    });

    let output: std::process::Output = child.wait_with_output()?;
    std::env::set_current_dir(current)?;
    Ok(output.stdout.to_vec())
}

fn _error() -> Vec<u8> {
    eprint!("{}", SHELL_ERROR);
    SHELL_ERROR.as_bytes().to_vec()
}
fn _error_exit() -> ! {
    eprint!("{}", ERROR_MSG);
    std::process::exit(1);
}
