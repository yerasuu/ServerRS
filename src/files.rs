use std::path::Path;
use std::fs;
use std::io::Read;

pub fn file() {
    // Create a path to Serve Directory
    let path: &Path = Path::new("./public_http");

    if path.exists() && path.is_dir() {
        let paths = fs::read_dir(path).unwrap();
        for route in paths {
            println!("{}",route.unwrap().path().display());
        }
    }

    if path.is_file() {
        let display = path.display();
        // Open the path in read-only mode, returns `io::Result<File>`
        let mut file = match fs::File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, why),
            Ok(file) => file,
        };
// Read the file contents into a string, returns `io::Result<usize>`
        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("couldn't read {}: {}", display, why),
            Ok(_) => print!("{} contains:\n{}", display, s),
        }
    }
}

