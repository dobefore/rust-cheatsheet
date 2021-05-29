pub trait Sound {
    fn make_sound(&self);
}
/// trait object: duck typing in python
/// dynamic dispach : compiler  can't tell at compile time which method you're calling 
struct Animal {
    spieces: Vec<Box<dyn Sound>>,
}
impl Animal {
    fn start(&self) {
        for spiece in self.spieces.iter() {
            spiece.make_sound();
        }
    }
}
struct Dog<'a> {
    name: &'a str,
}
impl<'a> Sound for Dog<'a> {
    fn make_sound(&self) {
        println!("{} 汪汪汪~", self.name);
    }
}
struct Neko<'a> {
    name: &'a str,
}
impl<'a> Sound for Neko<'a> {
    fn make_sound(&self) {
        println!("{} 喵喵喵~", self.name);
    }
}
fn trait_obj() {
    let a = Animal {
        spieces: vec![
            Box::new(Dog { name: "大吉" }),
            Box::new(Neko { name: "二花" }),
        ],
    };
    a.start();
}

#[test]
fn test_trait_obj() {
    trait_obj();
}
