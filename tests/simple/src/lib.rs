#![cfg(test)]

use visit::visit;

visit! {
    #![visitor_trait = "Visitor"]

    struct MyTree {
        foo: Foo,
    }

    struct Foo {
        bar: Bar,
    }

    struct Bar {
        a: Child,
        b: Child,
    }

    struct Child {}
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
    fn visit_my_tree(&mut self, _my_tree: &MyTree) {
        self.visit_result.push("MyTree");
    }

    fn visit_foo(&mut self, _foo: &Foo) {
        self.visit_result.push("Foo");
    }

    fn visit_bar(&mut self, _bar: &Bar) {
        self.visit_result.push("Bar");
    }

    fn visit_child(&mut self, _child: &Child) {
        self.visit_result.push("Child");
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        let tree = MyTree {
            foo: Foo {
                bar: Bar {
                    a: Child {},
                    b: Child {},
                },
            },
        };
        let mut v = MyVisitor::new();
        tree.accept(&mut v);
        assert_eq!(
            vec!["Child", "Child", "Bar", "Foo", "MyTree"],
            v.visit_result
        );
    }
}
