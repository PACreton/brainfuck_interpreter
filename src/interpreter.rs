use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

pub fn run(filename: &str, v: &str) {
    let file = File::open(filename).expect("Cannot open file");

    let buf_reader = BufReader::new(file);
    let mut contents: Vec<char> = Vec::new();

    for line in buf_reader.lines() {
        match line {
            Ok(l) => {
                for c in l.chars() {
                    if c == '<'
                        || c == '>'
                        || c == '+'
                        || c == '-'
                        || c == '.'
                        || c == ','
                        || c == '['
                        || c == ']'
                    {
                        contents.push(c);
                    }
                }
            }
            Err(e) => {
                eprintln!("{}: Cannot read line", e);
                std::process::exit(1);
            }
        }
    }

    if v == "simple" {
        simpleinterp(&contents);
    } else if v == "optimized" {
        optinterp(&contents);
    }
}

fn simpleinterp(contents: &Vec<char>) {
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
            '.' => print!("{}", arr[ptr as usize] as char),
            ',' => {
                let mut input: [u8; 1] = [0; 1];
                io::stdin()
                    .read_exact(&mut input)
                    .expect("Cannot read input");
                arr[ptr as usize] = input[0];
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
            _ => print!(""),
        }
        i += 1;
    }
    println!("");
}

fn optinterp(contents: &Vec<char>) {
    let mut ptr: i32 = 0;
    let mut arr: [u8; 30000] = [0; 30000];

    // println!("{:?}", contents);

    let bracket_jmptable = bracket_jumptable(contents);

    let mut i = 0;
    while i < contents.len() {
        match contents[i] {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => arr[ptr as usize] += 1,
            '-' => arr[ptr as usize] -= 1,
            '.' => print!("{}", arr[ptr as usize] as char),
            ',' => {
                let mut input: [u8; 1] = [0; 1];
                io::stdin()
                    .read_exact(&mut input)
                    .expect("Cannot read input");
                arr[ptr as usize] = input[0];
            }
            '[' => {
                if arr[ptr as usize] == 0 {
                    i = bracket_jmptable[i];
                }
            }
            ']' => {
                if arr[ptr as usize] != 0 {
                    i = bracket_jmptable[i];
                }
            }
            _ => print!(""),
        }
        i += 1;
    }
    println!("");
}

fn bracket_jumptable(contents: &Vec<char>) -> Vec<usize> {
    let mut jmptable: Vec<usize> = vec![0; contents.len()];
    let mut pc = 0;

    while pc < contents.len() {
        if contents[pc] == '[' {
            let mut nesting = 1;
            let mut matching = pc + 1;

            while nesting > 0 {
                if matching >= contents.len() {
                    eprintln!("Missing a matching bracket");
                    std::process::exit(1);
                }

                if contents[matching] == '[' {
                    nesting += 1;
                }

                if contents[matching] == ']' {
                    nesting -= 1;
                }

                if nesting > 0 {
                    matching += 1;
                }
            }

            if nesting == 0 {
                jmptable[pc] = matching;
                jmptable[matching] = pc;
            } else if nesting < 0 {
                eprintln!("Missing a matching bracket");
                std::process::exit(1);
            }
        }
        pc += 1;
    }

    jmptable
}
