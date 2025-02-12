use std::env;
mod interpreter;
fn main() {

    env::set_var("RUST_BACKTRACE", "1");

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        usage(&args[0]);
    }

    interpreter::run(&args[2], &args[1]);
}

pub fn usage(filename: &str) {
    println!("{} -v <simple|optiterp1|optiterp2> tests/<program_name>", filename);
    std::process::exit(1);
}