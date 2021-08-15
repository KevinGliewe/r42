use std::ffi::OsStr;
use std::{fs, io};
use std::io::{BufReader, Read};
use std::path::PathBuf;

use glob::glob;

mod langs;

fn print_help() {
    println!("r42");
    println!("Usage: r42 [Language/Glob pattern]");
    println!("  Language: (using stdio)");
    for l in langs::LENGUAGES {
        println!("    r42 {:?}", l.name);
    }
    println!("  Glob pattern: (using filesystem, files like 'file.rs.r42')");
    println!("    directory/*.r42");
}

fn handle_file(template_path: &PathBuf) {

    let template_extension = match template_path.extension().and_then(OsStr::to_str) {
        Some(ext) => ext,
        None => {
            println!("No extension found in {:?}", template_path);
            return;
        },
    };

    if template_extension != "r42" {
        return;
    }

    let out_path = template_path.with_extension("");

    let extension = match out_path.extension().and_then(OsStr::to_str) {
        Some(ext) => ext,
        None => {
            println!("No language extension found in {:?}", out_path);
            return;
        },
    };

    let lang = match langs::LENGUAGES.iter().filter(|l| l.extension == extension.to_string()).nth(0) {
        Some(l) => l,
        None => {
            println!("No language found for extension {:?}", extension);
            return;
        },
    };

    let contents = match fs::read_to_string(template_path) {
        Ok(contents) => contents,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let code = langs::r42t::transform(&contents, &lang.template_writer, &lang.expression_writer);
    match fs::write(&out_path, code.as_bytes()) {
        Ok(_) => { }
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    println!("[{}] '{:?}' -> '{:?}'", lang.name, template_path, out_path);
}

fn use_glob(pattern: &String) {
    println!("cwd: {:?}", std::env::current_dir().unwrap());
    println!("glob: {}", pattern);
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => handle_file(&path),
            Err(e) => println!("{:?}", e),
        }
    }
}

fn use_stdio(lang: &langs::Language) {
    let mut stdin_reader = Box::new(BufReader::new(io::stdin()));

    let mut buf = String::new();
    stdin_reader.read_to_string(&mut buf).unwrap();

    println!("{}", langs::r42t::transform(&buf.as_str(), &lang.template_writer, &lang.expression_writer));
}


fn main() {

    let args: Vec<String> = ::std::env::args().collect();

    let lang = &langs::LENGUAGES[0];

    if args.len() < 2 {
        use_stdio(&lang);
    } else if args.len() == 2 {
        if args[1] == "--help" {
            print_help();
        } else {
            match langs::LENGUAGES.iter().filter(|l| l.name == &args[1]).nth(0) {
                Some(l) => { use_stdio(&l) },
                None => { use_glob(&args[1]);},
            }
        }
    } else {
        print_help();
    }
}
