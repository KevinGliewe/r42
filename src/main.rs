use std::ffi::OsStr;
use std::{fs, io};
use std::io::{BufReader, Read};
use std::path::PathBuf;

use glob::glob;

mod langs;

mod build_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}


/// Print the command line help message with build information.
fn print_help() {

    // Display version and build time information first

    println!("r42 {} ({})",build_info::PKG_VERSION, build_info::BUILT_TIME_UTC);

    match build_info::GIT_COMMIT_HASH {
        Some(hash) => println!("git commit hash: {}", hash),
        None =>  { }
    }

    println!("Usage: r42 [Language/Glob]");
    println!("  Language: (using stdio)");
    for l in langs::LANGUAGES.iter() {
        println!("    r42 {:?}", l.name);
    }
    println!("  Glob: (using filesystem, files like 'file.rs.r42')");
    println!("    r42 \"directory/*.r42\"");
}

/// Convert a single template file into its output language.
fn handle_file(template_path: &PathBuf) {

    // Extract the final `.r42` extension used to identify templates
    let template_extension = match template_path.extension().and_then(OsStr::to_str) {
        Some(ext) => ext,
        None => {
            println!("No extension found in {:?}", template_path);
            return;
        },
    };

    // Skip files that aren't templates
    if template_extension != "r42" {
        return;
    }

    // Remove the `.r42` suffix to obtain the real output path
    let out_path = template_path.with_extension("");

    // Determine the language from the file extension of the output path
    let extension = match out_path.extension().and_then(OsStr::to_str) {
        Some(ext) => ext,
        None => {
            println!("No language extension found in {:?}", out_path);
            return;
        },
    };

    // Look up the language record to know how to generate code
    let lang = match langs::LANGUAGES.iter().filter(|l| l.extension == extension.to_string()).nth(0) {
        Some(l) => l,
        None => {
            println!("No language found for extension {:?}", extension);
            return;
        },
    };

    // Load the template source from disk
    let contents = match fs::read_to_string(template_path) {
        Ok(contents) => contents,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // Convert the template into executable code for the target language
    let code = langs::r42t::transform(&contents, &lang.template_writer, &lang.expression_writer);
    // Write the generated source to disk
    match fs::write(&out_path, code.as_bytes()) {
        Ok(_) => { }
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // Inform the user about the conversion result
    println!("[{}] '{:?}' -> '{:?}'", lang.name, template_path, out_path);
}

/// Expand a glob pattern and handle each discovered template file.
fn use_glob(pattern: &String) {
    // Report the working directory and provided pattern for transparency
    println!("cwd: {:?}", std::env::current_dir().unwrap());
    println!("glob: {}", pattern);
    // Iterate over all matching files and convert them individually
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => handle_file(&path),
            Err(e) => println!("{:?}", e),
        }
    }
}

/// Read a template from STDIN and emit the generated code to STDOUT.
fn use_stdio(lang: &langs::Language) {
    // Collect all input from STDIN first
    let mut stdin_reader = Box::new(BufReader::new(io::stdin()));

    let mut buf = String::new();
    stdin_reader.read_to_string(&mut buf).unwrap();

    // Convert the contents and print the generated code to STDOUT
    println!("{}", langs::r42t::transform(&buf.as_str(), &lang.template_writer, &lang.expression_writer));
}


/// Entry point of the `r42` command line application.
fn main() {

    // Collect command line arguments
    let args: Vec<String> = ::std::env::args().collect();

    // Default to the first language when reading from STDIN
    let lang = &langs::LANGUAGES[0];

    // Branch depending on whether a pattern or language name was provided
    if args.len() < 2 {
        use_stdio(&lang);
    } else if args.len() == 2 {
        if args[1] == "--help" {
            print_help();
        } else {
            match langs::LANGUAGES.iter().filter(|l| l.name == &args[1]).nth(0) {
                Some(l) => { use_stdio(&l) },
                None => { use_glob(&args[1]); },
            }
        }
    } else {
        print_help();
    }
}
