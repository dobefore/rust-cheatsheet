/// create and deference raw pointer 
fn raw_pointer() {

    let mut num=5;
    // create raw pointer
    let r1=&num as *const i32;
    let r2=&mut num as *mut i32;
    // plus 1 through raw pointer
unsafe {
    *r2+=1;
}
// not exist possiblly
    let addr=0x012345usize;
    let r=addr as *const i32;
unsafe {
    println!("{}{}",*r1,*r2);
}
   
}
/// create unsafe funtion 
fn unsafe_fn(){
    unsafe fn danger() {} 
    unsafe {
        danger(); }
}
/// static value
fn static_val(){
    static mut C:u8=3;
    unsafe {
        C+=1;
    }
unsafe {
    println!("{}",C);
}
} 
/// create unsafe trait
unsafe trait AA {}
unsafe impl AA for i32 {}


#[test]
fn test_create_pointer(){
    raw_pointer();
}