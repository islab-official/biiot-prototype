use std::io::{Read, Write};

pub fn get_file(path: &str) -> Result<std::fs::File, ()> {
    // Create a path to the desired file
    let path = std::path::Path::new(path);
    let display = path.display();
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match std::fs::File::open(&path) {
        Ok(_file) => {
            return Ok(_file);
        },
        Err(reason) => {
            println!("could not open {}: {}", display, reason);
            return Err(());
        }
    };
}

pub fn new_file(path: &str) -> std::fs::File {
    let path = std::path::Path::new(path);
    let display = path.display();
    let mut file = match std::fs::File::create(path) {
        Ok(f) => { return f; }
        Err(reason) => { panic!("could not create {}: {}", display, reason) }
    };
}

pub fn read_file(file: &mut std::fs::File) -> Vec<u8> {
    let mut s: Vec<u8> = vec![];
    match file.read_to_end(&mut s) {
        Ok(_) => { return s; }
        Err(reason) => { panic!("could not read.. {}", reason) }
    }
}

pub fn write_file(file: &mut std::fs::File, data: Vec<u8>) {
    match file.write_all(data.as_slice()) {
        Ok(_) => {}
        Err(reason) => { panic!("could not write.. {}", reason)}
    }
}