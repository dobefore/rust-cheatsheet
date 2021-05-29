/// type alias
use std::io::Error;
fn type_alias(){
    type t=Box<dyn Fn()+Send+'static>;
type Result<T>=std::result::Result<T,std::io::Error>;
}