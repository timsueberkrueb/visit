use visit::visit;

visit! {
    #![visitor(name = "Visitor")]
    #![hierarchical_visitor(name = "HierVisitor")]

    struct MyTree {
        foo: Foo,
    }

    struct Foo {
        bar: Bar,
    }

    struct Bar(Child, Child);

    struct Child;
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

struct MyHierVisitor {
    visit_result: Vec<&'static str>,
}

impl MyHierVisitor {
    fn new() -> Self {
        Self {
            visit_result: Vec::new(),
        }
    }
}

impl HierVisitor for MyHierVisitor {
    fn enter_my_tree(&mut self, _my_tree: &MyTree) {
        self.visit_result.push("enter_my_tree");
    }

    fn leave_my_tree(&mut self, _my_tree: &MyTree) {
        self.visit_result.push("leave_my_tree");
    }

    fn enter_foo(&mut self, _foo: &Foo) {
        self.visit_result.push("enter_foo");
    }

    fn leave_foo(&mut self, _foo: &Foo) {
        self.visit_result.push("leave_foo");
    }

    fn enter_bar(&mut self, _bar: &Bar) {
        self.visit_result.push("enter_bar");
    }

    fn leave_bar(&mut self, _bar: &Bar) {
        self.visit_result.push("leave_bar");
    }

    fn enter_child(&mut self, _child: &Child) {
        self.visit_result.push("enter_child");
    }

    fn leave_child(&mut self, _child: &Child) {
        self.visit_result.push("leave_child");
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_structs_simple() {
        let tree = MyTree {
            foo: Foo {
                bar: Bar(Child {}, Child {}),
            },
        };
        let mut v = MyVisitor::new();
        AcceptVisitor::accept(&tree, &mut v);
        assert_eq!(
            vec!["Child", "Child", "Bar", "Foo", "MyTree"],
            v.visit_result
        );
    }

    #[test]
    fn test_hierarchical_structs() {
        let tree = MyTree {
            foo: Foo {
                bar: Bar(Child {}, Child {}),
            },
        };
        let mut v = MyHierVisitor::new();
        AcceptHierVisitor::accept(&tree, &mut v);
        assert_eq!(
            vec![
                "enter_my_tree",
                "enter_foo",
                "enter_bar",
                "enter_child",
                "leave_child",
                "enter_child",
                "leave_child",
                "leave_bar",
                "leave_foo",
                "leave_my_tree",
            ],
            v.visit_result
        );
    }
}
