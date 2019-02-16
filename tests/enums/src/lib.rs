use visit::visit;

visit! {
    #![visitor_trait = "EnumVisitor"]

    struct Tree {
        foo1: Foo,
        foo2: Foo,
    }

    #[allow(dead_code)]
    enum Foo {
        Bar { bar: BarItem },
        Baz { baz: BazItem },
    }

    struct BarItem {}
    struct BazItem {}
}

#[allow(dead_code)]
struct MyVisitor {
    visit_result: Vec<&'static str>,
}

impl MyVisitor {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            visit_result: Vec::new(),
        }
    }
}

impl EnumVisitor for MyVisitor {
    fn visit_tree(&mut self, _tree: &Tree) {
        self.visit_result.push("Tree");
    }

    fn visit_foo(&mut self, _foo: &Foo) {
        self.visit_result.push("Foo");
    }

    fn visit_bar_item(&mut self, _bar_item: &BarItem) {
        self.visit_result.push("BarItem");
    }

    fn visit_baz_item(&mut self, _baz_item: &BazItem) {
        self.visit_result.push("BazItem");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enums_named_values() {
        let tree = Tree {
            foo1: Foo::Bar { bar: BarItem {} },
            foo2: Foo::Baz { baz: BazItem {} },
        };
        let mut visitor = MyVisitor::new();
        tree.accept(&mut visitor);
        assert_eq!(
            vec!["BarItem", "Foo", "BazItem", "Foo", "Tree"],
            visitor.visit_result
        );
    }
}
