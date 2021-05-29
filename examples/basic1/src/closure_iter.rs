/// # closure
fn closure() {
    let clo = |x: u8| x;
}

/// T:Fn(u32)->u32 parameter include this is a closure
/// Fn/FnMut/Fnonce trait  
fn cl<T>(par: T) -> T
where
    T: Fn(u32) -> u32,
{
    par
}
///return closure
fn return_closure()->Box<dyn Fn(u32)->u32>{
Box::new(|x|x+1)
} 
/// fn as a parameter
fn receive(f:fn(u32)->u32,val:u32){
    f(val)+f(val);
} 
/// # iteratr
/// custom impl iterator for struct  
#[derive(Debug, Clone)]
struct I {
    v: Vec<u8>,
}
impl Iterator for I {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        match self.v.iter().next() {
            Some(x) => Some(x.to_owned()),
            None => None,
        }
    }
}
fn assert_iter() {
    let i = I { v: vec![1, 2, 3] };
    let mut it = i.into_iter();
    assert_eq!(Some(1), it.next());
    assert_eq!(Some(1), it.next());
}
#[test]
fn test_closure() {
    let s = cl(|a| a)(3);
}

#[test]
fn test_return_closure(){
    let x=3;
   let s= return_closure()(2);
 
}