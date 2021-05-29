use std::fs::File;
use std::io;
use std::io::Error;
use std::io::ErrorKind;
/// make custom error
fn devide(f: u8, s: u8) -> io::Result<f64> {
    match s {
        0 => Err(Error::new(ErrorKind::Other, "de zero")),
        _ => Ok(f as f64 / s as f64),
    }
}
/// recoverable err :try ..except
fn e() {
    let f = File::open("a.txt").unwrap_or_else(|e| {
        if e.kind() == ErrorKind::NotFound {
            File::create("a.txt").unwrap_or_else(|e| panic!("{:?}", e))
        } else {
            panic!("{:?}", e);
        }
    });
}

#[test]
fn a() {
    let a = devide(3, 2);
    println!("{:?}", a);
}
