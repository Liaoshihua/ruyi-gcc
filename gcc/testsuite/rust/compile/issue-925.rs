#[lang = "sized"]
pub trait Sized {}

struct S;
trait A {
    fn foo(&self);
}

trait B: A {
    fn foo(&self);
}

impl A for S {
    fn foo(&self) {}
}

impl B for S {
    fn foo(&self) {}
}

fn test() {
    let a = S;
    a.foo();
    // { dg-error "multiple applicable items in scope for method .foo." "" { target *-*-* } .-1 }
}