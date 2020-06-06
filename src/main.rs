use std::env;
use std::fs::File;
use std::io;
use std::process;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 3 || args.len() < 2 {
        eprintln!("Usage {} path filename|stdin", args[0]);
        process::exit(1);
    }
    let mut reader: Box<dyn io::BufRead> = match args.len() == 3 {
        false => Box::new(io::BufReader::new(io::stdin())),
        true => Box::new(io::BufReader::new(File::open(&args[2]).unwrap())),
    };

    let mut buf = String::new();
    let _ = reader.read_to_string(&mut buf);

    let res = utils::traverse(&buf, &args[1]);
    if res.is_err() {
        eprintln!("Invalid path or JSON");
        process::exit(1);
    }

    for v in utils::traverse(&buf, &args[1]).unwrap() {
        println!("{}", v);
    }
}
