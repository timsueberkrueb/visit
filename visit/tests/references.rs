extern crate visit;

use visit::visit;

visit! {
    #![visitor(name = "Visitor")]

    struct Foo<'a> {
        bar: &'a Bar,
        baz: &'a Baz
    }

    struct Bar;

    enum Baz { Something }
}

struct DummyVisitor;

impl Visitor for DummyVisitor {}

mod test {
    use super::*;

    /// Ensures AcceptVisitor is implemented for &Bar and &Baz
    #[test]
    fn test_reference_accept() {
        let bar = Bar {};
        let baz = Baz::Something;
        let foo = Foo {
            bar: &bar,
            baz: &baz,
        };
        let mut v = DummyVisitor {};
        foo.accept(&mut v);
    }
}
