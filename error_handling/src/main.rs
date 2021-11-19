use std::error::Error;
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    // If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us.
    let f = File::open("hello.txt").unwrap();
    // expect, which is similar to unwrap, lets us also choose the panic! error message
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    // Implementation witch match (not so good implementation)
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

    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    // ? OPERATOR (works as the match operator)
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let mut f = File::open("hello.txt")?;
    //     let mut s = String::new();
    //     f.read_to_string(&mut s)?;
    //     Ok(s)
    // }
}
// BETTER IMPLEMENTATION
// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let f = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {:?}", error);
//             })
//         } else {
//             panic!("Problem opening the file: {:?}", error);
//         }
//     });
// }

fn sol() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
