use std::cell::RefCell;
use std::rc::Rc;
use std::{ops::Deref, rc::Weak};
/// smart  pointers usually are structs,differ in
/// what they impl traits Deref and Drop
/// box ,Rc<T>,RefCell<T>
///create custom smart pointer
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
/// impl drop trait
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {}
}
#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

/// deref coercion
fn hello(name: &str) {
    println!("hello {}", name);
}
fn deref_coercion() {
    let mb = MyBox::new(String::from("Peter"));
    hello(&mb);
}

/// Rc<T>: immutable reference counting :multi-ownership
/// in single-threaded app
///  &Rc<T> -> Rc<T>
struct A(i32);
struct B(Rc<A>);
fn reference_count() {
    let a = Rc::new(A(1));
    let r1 = Rc::clone(&a);
    let r2 = Rc::clone(&a);
    let b1 = B(r1);
    let b2 = B(r2);
    println!("{}", Rc::strong_count(&a));
}

/// RefCell<T> :interior mutalbility :all mut/inmut borrow
/// check borrow rules at run time
/// single-threaded app

fn refcell() {
    let v = RefCell::new(vec![2]);
    v.borrow_mut().push(3);

    println!("{:?}", *v.borrow());
}

/// conbine Rc with RefCell
/// child don't own parent
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    child: RefCell<Vec<Rc<Node>>>,
}
fn rc_refcell() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![]),
    });
    println!("leaf parent {:?}", leaf.parent.borrow().upgrade());
    let branch = Rc::new(Node {
        value: 4,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent {:?}", leaf.parent.borrow().upgrade());
}
/// trait objects: own a value and care only that
/// it's a type that impl a paticular trait rather
/// than being of a specific type
#[test]
fn test_deref_coercion() {
    deref_coercion();
}
#[test]
fn test_rc() {
    reference_count();
}

#[test]
fn test_refcell() {
    refcell();
}
#[test]
fn test_rc_refcell() {
    rc_refcell();
}
