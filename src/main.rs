use std::{fs, io};
use std::io::{BufReader, Read};
use std::path::PathBuf;

use glob::glob;

mod r42;

fn print_help() {
    println!("r42");
}

fn handle_file(template_path: &PathBuf) {

    let out_path = template_path.with_extension("rs");

    let contents = match fs::read_to_string(template_path) {
        Ok(contents) => contents,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let code = r42::transform(&contents);
    match fs::write(&out_path, code.as_bytes()) {
        Ok(_) => { }
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    println!("'{:?}' -> '{:?}'", template_path, out_path);
}

fn use_glob(pattern: &String) {
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => handle_file(&path),
            Err(e) => println!("{:?}", e),
        }
    }
}

fn use_stdio() {
    let mut stdin_reader = Box::new(BufReader::new(io::stdin()));

    let mut buf = String::new();
    stdin_reader.read_to_string(&mut buf).unwrap();

    println!("{}", r42::transform(&buf.as_str()));
}


fn main() {
    let args: Vec<String> = ::std::env::args().collect();

    if args.len() < 2 {
        use_stdio();
    } else if args.len() == 2 {
        if args[1] == "--help" {
            print_help();
        } else {
            use_glob(&args[1]);
        }
    } else {
        print_help();
    }
}
