#![cfg(test)]

use visit::visit;

visit! {
    #![visitor_trait = "EnumVisitor"]

    struct Tree {
        foo1: Foo,
        foo2: Foo,
    }

    enum Foo {
        Bar { bar: BarItem },
        Baz { baz: BazItem },
    }

    enum BiologicalTree {
        Oak(Roots, Trunk, Vec<Leaf>),
        #[allow(dead_code)]
        Birch,
        #[allow(dead_code)]
        Spruce,
    }

    struct Roots {}
    struct Trunk {}
    struct Leaf {}
    struct AST {}

    struct BarItem {}
    struct BazItem {}
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

    fn visit_biological_tree(&mut self, _tree: &BiologicalTree) {
        self.visit_result.push("BiologicalTree");
    }

    fn visit_roots(&mut self, _roots: &Roots) {
        self.visit_result.push("Roots");
    }

    fn visit_trunk(&mut self, _trunk: &Trunk) {
        self.visit_result.push("Trunk");
    }

    fn visit_leaf(&mut self, _leaf: &Leaf) {
        self.visit_result.push("Leaf");
    }
}

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

    #[test]
    fn test_unit_and_unnamed_values() {
        let tree = BiologicalTree::Oak(Roots {}, Trunk {}, vec![Leaf {}, Leaf {}, Leaf {}]);
        let mut visitor = MyVisitor::new();
        tree.accept(&mut visitor);
        assert_eq!(
            vec!["Roots", "Trunk", "Leaf", "Leaf", "Leaf", "BiologicalTree"],
            visitor.visit_result
        );
    }
}
