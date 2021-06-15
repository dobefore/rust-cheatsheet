/// type alias
use std::{io::Error, num::ParseIntError, str::FromStr};
fn type_alias() {
    type t = Box<dyn Fn() + Send + 'static>;
    type Result<T> = std::result::Result<T, std::io::Error>;
}

/// type conversion 
/// From and Into 
#[derive(Debug)]
struct A{
a:i32
} 
impl From<i32> for A {
    fn from(item:i32) -> Self {
        A{a:item}
    }
}

/// ToString and FromStr
impl ToString for A {
    fn to_string(&self) -> String {
        format!("A.a=={:?}",self.a)
    }
} 

impl FromStr for A {
    type Err=ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let m=s.parse::<i32>()?;
        Ok(A{a:m})
    }
}

#[test]
fn test_from_into() {
    let n=3i32;
    let n1=A::from(n);
    let s:A=n.into();
    println!("{}",s.to_string())
}

#[test]
fn test_to_string_fromstr() {
    let n:A=4i32.into();
let s= n.to_string();
let st="12";
let a= st.parse::<A>();
}