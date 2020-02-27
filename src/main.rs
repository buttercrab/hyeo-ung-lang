use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use hyeo_ung_lang::big_number::Num;
use hyeo_ung_lang::parse::Command;

fn main() {
    let path = Path::new("hello.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                           why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    let a = Command::parse(s);
    match a {
        Ok(t) => {
            println!("{}", t.len());
        },
        Err(e) => {
            eprintln!("{}", e.get_error());
        }
    }
}
