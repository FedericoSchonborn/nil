#[rustfmt::skip]
use std::error::Error;
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Nil;
fn nil() -> Box<Nil> {
    Box::new(Nil)
}
impl Error for Nil {}
impl std::fmt::Display for Nil {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("<nil>")
    }
}
impl PartialEq<Box<Nil>> for Box<dyn Error> {
    fn eq(&self, other: &Box<Nil>) -> bool {
        self.downcast_ref::<Nil>().unwrap() == other.as_ref()
    }
}
fn thing() -> (isize, Box<dyn Error>) {
    return (1, nil());
}
fn main() {
    let (n, err) = thing();
    if err != nil() {
        println!(":gohno:");
    }
    println!("Got {n}");
}
