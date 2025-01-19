use console::Term;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let file = match File::open(&args[0]) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}: Cannot open file", e);
            std::process::exit(1);
        }
    };

    let term = Term::stdout();
    let buf_reader = BufReader::new(file);
    let mut contents: Vec<char> = Vec::new();

    for line in buf_reader.lines() {
        match line {
            Ok(l) => {
                for c in l.chars() {
                    contents.push(c);
                }
            }
            Err(e) => {
                eprintln!("{}: Cannot read line", e);
                std::process::exit(1);
            }
        }
    }

    let mut ptr: i32 = 0;
    let mut arr: [u8; 30000] = [0; 30000];

    // println!("{:?}", contents);

    let mut i = 0;
    while i < contents.len() {
        match contents[i] {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => arr[ptr as usize] += 1,
            '-' => arr[ptr as usize] -= 1,
            '.' => {
                print!("{}", arr[ptr as usize] as char);
            }
            ',' => {
                arr[ptr as usize] = match Term::read_char(&term) {
                    Ok(tc) => tc as u8,
                    Err(e) => {
                        eprintln!("{}: Cannot read character from standard input", e);
                        std::process::exit(1);
                    }
                }
            }
            '[' => {
                if arr[ptr as usize] == 0 {
                    let mut stack = 1;
                    while stack != 0 {
                        if stack < 0 {
                            eprintln!("Missing a matching bracket");
                            std::process::exit(1);
                        }

                        i += 1;

                        if contents[i] == '[' {
                            stack += 1;
                        }

                        if contents[i] == ']' {
                            stack -= 1;
                        }
                    }
                }
            }
            ']' => {
                if arr[ptr as usize] != 0 {
                    let mut stack = 1;
                    while stack != 0 {
                        i -= 1;

                        if stack < 0 {
                            eprintln!("Missing a matching bracket");
                            std::process::exit(1);
                        }

                        if contents[i] == '[' {
                            stack -= 1;
                        }

                        if contents[i] == ']' {
                            stack += 1;
                        }
                    }
                }
            }
            _ => eprintln!("Cannot recognize character"),
        }
        i += 1;
    }
    println!("");
}
