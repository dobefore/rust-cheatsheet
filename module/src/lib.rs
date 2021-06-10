/// # modules ,paths, workspace
/// a crate is a binary or library
/// the crate root is src file (src/main.rs ..that is crate/)
/// a package : cargo new pack (contain cargo.toml)
mod a {
    pub mod b {
        pub fn q() {}
    }
    pub mod c {
        pub fn w() {}
    }
}
///  path
/// use : bring module into scoops use..as
/// pub use: re-exporting names
/// mod :separate module into multi-files
mod q;
use self::a::b;
pub use self::a::c;
pub fn aa() {
    // absolute path
    crate::a::b::q();
    // rel path
    a::b::q();

    // use
    b::q();
    q::c::d();
}
mod d {
    fn bb() {
        super::a::b::q();
    }
}
/// # workspace
/// mkdir workspace
/// cd workspace

/// cargo new a
/// cargo build

/// .workspace/cargo.toml
/// vi cargo.toml  

/// [workspace]
/// members=["a",
/// "b","c"]

/// cargo new b --lib
/// cargo new c --lib

fn main() {}
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
