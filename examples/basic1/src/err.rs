
use std::error;
use std::fs::File;
use std::io;
use std::io::Error;
use std::io::ErrorKind;
use std::num::ParseIntError;
use failure::Fail;
/// make custom error
fn devide(f: u8, s: u8) -> BoxResult<f64> {
    match s {
        0 =>Err( Box::new( Error::new(ErrorKind::Other, "de zero"))),
        _ => Ok(f as f64 / s as f64),
    }
}
/// err into box for std error and custom error by user
type  BoxResult<T>=std::result::Result<T,Box<dyn error::Error>>;
fn r<'a>()->BoxResult<()> {
    let s="3".parse::<i32>()?;
    let f=File::open("path")?;
    Ok(())
}

/// wrap error into your custom error type
type WrapResult<T,E=Er>=std::result::Result<T,E>;
/// replace of impl fmt::Display for E 
#[derive(Debug,Fail,Clone)]
enum Er {
    #[fail(display = "empty vec")]
    EmpVec,
    #[fail(display = "parseint error")]
    Parse(ParseIntError),
    #[fail(display = "I/O error: {}", info)]
    IOError { info: String },
}
impl From<ParseIntError> for Er {
    fn from(err: ParseIntError) -> Self {
        Er::Parse(err)
    }
}
impl From<io::Error> for Er {
    fn from(err: io::Error) -> Self {
        Er::IOError{info:format!("{:?}",err)}
    }
}

fn s(v:Vec<&str>)-> WrapResult<i32>{
  let f=  v.first().ok_or(Er::EmpVec)?.to_owned().to_owned();
let p=f.parse::<i32>()?;
let fd=File::open("path")?;
    Ok(p)
}

#[test]
fn test_boxerr()->BoxResult<()> {
    let a = devide(3, 2)?;
    println!("{:?}", a);
let v:Vec<&str>=vec![];
s(v).unwrap();
   Ok(())
}
