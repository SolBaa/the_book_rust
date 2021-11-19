use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let res = File::open("hello.txt");
    let _res = match res {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(err) => panic!("Error creating file: {:?}", err),
            },
            other_error => {
                panic!("Error opening file: {:?}", other_error)
            }
        },
    };
    println!("Hello, world!");
}
