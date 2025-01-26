use std::env;
mod interpreter;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        usage(&args[0]);
    }

    interpreter::run(&args[2], &args[1]);
}

pub fn usage(filename: &str) {
    println!("{} -v <simple|optimized> tests/<program_name>", filename);
    std::process::exit(1);
}
