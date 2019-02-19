#![cfg(test)]

use visit::visit;

visit! {
    #![visitor_trait = "Visitor"]

    struct Root {
        boxed_foo: Box<Foo>,
    }

    struct Foo {
        boxed_bar: Box<Bar>,
    }

    struct Bar;
}

struct MyVisitor {
    visit_result: Vec<&'static str>,
}

impl MyVisitor {
    fn new() -> Self {
        Self {
            visit_result: Vec::new(),
        }
    }
}

impl Visitor for MyVisitor {
    fn visit_foo(&mut self, _foo: &Foo) {
        self.visit_result.push("Foo");
    }

    fn visit_bar(&mut self, _bar: &Bar) {
        self.visit_result.push("Bar");
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_box_simple() {
        let root = Root {
            boxed_foo: Box::new(Foo {
                boxed_bar: Box::new(Bar {}),
            }),
        };

        let mut visitor = MyVisitor::new();
        root.accept(&mut visitor);

        assert_eq!(vec!["Bar", "Foo"], visitor.visit_result);
    }
}
