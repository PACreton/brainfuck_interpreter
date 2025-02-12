use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::u8;
#[derive(Debug)]
enum BFOpsType {
    NullOp,    // Invalid operation
    IncrPtr,   // Increment the pointer
    DecrPtr,   // Decrement the pointer
    IncrByte,  // Increment the byte at the cell pointed to by the pointer
    DecrByte,  // Decrement the byte at the cell pointed to by the pointer
    WriteChar, // Output the byte at the cell pointed to by the pointer
    ReadChar,  // Input a byte into the cell pointed to by the pointer
    BeginLoop, // Start of loop
    EndLoop,   // End of loop
}

#[derive(Debug)]
struct BFOp {
    bftype: BFOpsType,
    n: u32, // Nombre de fois que l'opération est exécutée
}

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

    let program = translate_program(&contents);

    match v {
        "simple" => simpleinterp(&contents),
        "optiterp1" => optinterp1(&contents),
        "optiterp2" => println!("{:?}", program),
        _ => println!("Not implemented / Does not exist"),
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
            '+' => arr[ptr as usize] = arr[ptr as usize].checked_add(1).unwrap_or(u8::MIN),
            '-' => arr[ptr as usize] = arr[ptr as usize].checked_sub(1).unwrap_or(u8::MAX),
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

fn optinterp1(contents: &Vec<char>) {
    let mut ptr: i32 = 0;
    let mut arr: [u8; 30000] = [0; 30000];

    // println!("{:?}", contents);

    let bracket_jmptable = bracket_jumptable(contents);

    let mut i = 0;
    while i < contents.len() {
        match contents[i] {
            '>' => ptr += 1,
            '<' => ptr -= 1,
            '+' => arr[ptr as usize] = arr[ptr as usize].checked_add(1).unwrap_or(u8::MIN),
            '-' => arr[ptr as usize] = arr[ptr as usize].checked_sub(1).unwrap_or(u8::MAX),
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

fn count_iteration(contents: &Vec<char>, character: &char, start: &usize) -> u32 {
    let mut n = 0;
    let mut idx = *start;

    while idx < contents.len() && contents[idx] == *character {
        n += 1;
        idx += 1;
    }

    n
}

fn translate_program(contents: &Vec<char>) -> Vec<BFOp> {
    let mut pc: usize = 0;
    let bracket_jmptable = bracket_jumptable(contents);
    let mut program: Vec<BFOp> = Vec::new();

    while pc < contents.len() {
        match contents[pc] {
            '>' => {
                let it = count_iteration(&contents, &contents[pc], &pc);
                program.push(BFOp {
                    bftype: BFOpsType::IncrPtr,
                    n: it,
                });
                pc += it as usize;
            }
            '<' => {
                let it = count_iteration(&contents, &contents[pc], &pc);
                program.push(BFOp {
                    bftype: BFOpsType::DecrPtr,
                    n: it,
                });
                pc += it as usize;
            }
            '+' => {
                let it = count_iteration(&contents, &contents[pc], &pc);
                program.push(BFOp {
                    bftype: BFOpsType::IncrByte,
                    n: it,
                });
                pc += it as usize;
            }
            '-' => {
                let it = count_iteration(&contents, &contents[pc], &pc);
                program.push(BFOp {
                    bftype: BFOpsType::DecrByte,
                    n: it,
                });
                pc += it as usize;
            }
            '.' => {
                let it = count_iteration(&contents, &contents[pc], &pc);
                program.push(BFOp {
                    bftype: BFOpsType::ReadChar,
                    n: it,
                });
                pc += it as usize;
            }
            ',' => {
                let it = count_iteration(&contents, &contents[pc], &pc);
                program.push(BFOp {
                    bftype: BFOpsType::WriteChar,
                    n: it,
                });
                pc += it as usize;
            }
            '[' => {
                program.push(BFOp {
                    bftype: BFOpsType::BeginLoop,
                    n: bracket_jmptable[pc] as u32,
                });
                pc += 1;
            }
            ']' => {
                program.push(BFOp {
                    bftype: BFOpsType::EndLoop,
                    n: bracket_jmptable[pc] as u32,
                });
                pc += 1;
            }
            _ => program.push(BFOp {
                bftype: BFOpsType::NullOp,
                n: 0,
            }),
        }
    }

    program
}
