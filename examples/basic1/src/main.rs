mod closure_iter;
mod concurrency;
mod err;
mod smart_pointers;
mod trait_;
mod trait_object;
/// # struct enum and pattern match(match and if let)
/// associated funtioons--constructor
#[derive(Debug)]
struct R {
    w: u8,
    h: u8,
}
impl R {
    fn squire(size: u8) -> R {
        R { w: size, h: size }
    }
}

///  # enum and Option\<T>
enum Q {
    Quit,
    Point(u8, u8),
    Val(u8),
}
/// # pattern match
/// _ placeholder () unit value
/// @ and if usage in match pattern
/// 1 | 2: 1 or 2
fn pattern(en: Q) {
    match en {
        Q::Quit => println!("Quit"),
        Q::Val(x @ 5..=10) => println!("[5,10]"),
        Q::Val(x) if x >= 11 => println!("x >= 11"),
        _ => (),
    }
}
/// if let block
fn iflet() {
    let p = Q::Quit;
    if let Q::Quit = p {
        println!("Quit");
    }
}
///

fn main() {
    // associated funtioons
    let s = R::squire(3);
    // enum
    let q = Q::Point(2, 3);
}
