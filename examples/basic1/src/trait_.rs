/// # generics
struct B<T> {
    x: T,
}
enum Q<T> {
    Generic(T),
}
impl<T> B<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

/// # trait
/// trait: shared behavior
pub trait A {
    fn make_sound(&self);
}
struct Dog;
impl A for Dog {
    fn make_sound(&self) {
        println!("汪汪汪~")
    }
}
struct Neko;
impl A for Neko {
    fn make_sound(&self) {
        println!("喵喵喵~")
    }
}

/// trait bound : parameters(item) can use trait methods(make_sound())
pub fn bark(item: &impl A) {
    item.make_sound();
}

pub fn bark_<T: A>(item: &T) {
    item.make_sound();
}
pub fn bark__<T>(item: &T)
where
    T: A,
{
    item.make_sound();
}
/// return impl trait : after executing fn ,can use trit methoda
fn coe() -> impl A {
    Neko
}
use std::{fmt::write, io::LineWriter, ops::Add};
/// Rhs=Self : default type patameters
/// Rhs : right hand side ,default to Self  
/// associated type Output  
trait ADD<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

struct M(u32);
struct N(u32);
impl Add<N> for M {
    type Output = M;
    fn add(self, rhs: N) -> M {
        M(self.0 + rhs.0 * 10)
    }
}
/// <type as trait>::fn() : fully qulified syntax  
/// want to use trait fn baby_name() ,for associated fn
trait Animal {
    fn baby_name() -> String;
}
struct Cat;
impl Cat {
    fn baby_name() -> String {
        String::from("impl_二花")
    }
}
impl Animal for Cat {
    fn baby_name() -> String {
        String::from("trait_二花")
    }
}
fn use_trait() {
    let name = <Cat as Animal>::baby_name();
    println!("{}", name);
}
/// supertraits
use std::fmt;
trait Print_: fmt::Display {
    fn print_(&self) {
        let s = self.to_string();
        println!("{}", s);
    }
}
struct Point {
    x: i32,
    y: i32,
}
impl Print_ for Point {}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
fn print_point() {
    let p = Point { x: 3, y: 5 };
    p.print_();
}
///  
#[test]
fn test_() {
    let d = Dog;
    bark(&d);
    coe().make_sound();
}
#[test]
fn test_qulified() {
    use_trait();
}
#[test]
fn test_suprttrait() {
    print_point();
}
