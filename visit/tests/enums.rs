use visit::visit;

visit! {
    #![visitor(name = "EnumVisitor")]
    #![visitor(name = "HierVisitor", enter = "enter", leave = "leave")]

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
    fn enter_tree(&mut self, _tree: &Tree) {
        self.visit_result.push("enter_tree");
    }

    fn leave_tree(&mut self, _tree: &Tree) {
        self.visit_result.push("leave_tree");
    }

    fn enter_foo(&mut self, _foo: &Foo) {
        self.visit_result.push("enter_foo");
    }

    fn leave_foo(&mut self, _foo: &Foo) {
        self.visit_result.push("leave_foo");
    }

    fn enter_bar_item(&mut self, _bar_item: &BarItem) {
        self.visit_result.push("enter_bar_item");
    }

    fn leave_bar_item(&mut self, _bar_item: &BarItem) {
        self.visit_result.push("leave_bar_item");
    }

    fn enter_baz_item(&mut self, _baz_item: &BazItem) {
        self.visit_result.push("enter_baz_item");
    }

    fn leave_baz_item(&mut self, _baz_item: &BazItem) {
        self.visit_result.push("leave_baz_item");
    }

    fn enter_biological_tree(&mut self, _tree: &BiologicalTree) {
        self.visit_result.push("enter_biological_tree");
    }

    fn leave_biological_tree(&mut self, _tree: &BiologicalTree) {
        self.visit_result.push("leave_biological_tree");
    }

    fn enter_roots(&mut self, _roots: &Roots) {
        self.visit_result.push("enter_roots");
    }

    fn leave_roots(&mut self, _roots: &Roots) {
        self.visit_result.push("leave_roots");
    }

    fn enter_trunk(&mut self, _trunk: &Trunk) {
        self.visit_result.push("enter_trunk");
    }

    fn leave_trunk(&mut self, _trunk: &Trunk) {
        self.visit_result.push("leave_trunk");
    }

    fn enter_leaf(&mut self, _leaf: &Leaf) {
        self.visit_result.push("enter_leaf");
    }

    fn leave_leaf(&mut self, _leaf: &Leaf) {
        self.visit_result.push("leave_leaf");
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
        AcceptEnumVisitor::accept(&tree, &mut visitor);
        assert_eq!(
            vec!["BarItem", "Foo", "BazItem", "Foo", "Tree"],
            visitor.visit_result
        );
    }

    #[test]
    fn test_enums_named_values_hierarchical() {
        let tree = Tree {
            foo1: Foo::Bar { bar: BarItem {} },
            foo2: Foo::Baz { baz: BazItem {} },
        };
        let mut visitor = MyHierVisitor::new();
        AcceptHierVisitor::accept(&tree, &mut visitor);
        assert_eq!(
            vec![
                "enter_tree",
                "enter_foo",
                "enter_bar_item",
                "leave_bar_item",
                "leave_foo",
                "enter_foo",
                "enter_baz_item",
                "leave_baz_item",
                "leave_foo",
                "leave_tree",
            ],
            visitor.visit_result
        );
    }

    #[test]
    fn test_unit_and_unnamed_values() {
        let tree = BiologicalTree::Oak(Roots {}, Trunk {}, vec![Leaf {}, Leaf {}, Leaf {}]);
        let mut visitor = MyVisitor::new();
        AcceptEnumVisitor::accept(&tree, &mut visitor);
        assert_eq!(
            vec!["Roots", "Trunk", "Leaf", "Leaf", "Leaf", "BiologicalTree"],
            visitor.visit_result
        );
    }

    #[test]
    fn test_unit_and_unnamed_values_hierarchical() {
        let tree = BiologicalTree::Oak(Roots {}, Trunk {}, vec![Leaf {}, Leaf {}, Leaf {}]);
        let mut visitor = MyHierVisitor::new();
        AcceptHierVisitor::accept(&tree, &mut visitor);
        assert_eq!(
            vec![
                "enter_biological_tree",
                "enter_roots",
                "leave_roots",
                "enter_trunk",
                "leave_trunk",
                "enter_leaf",
                "leave_leaf",
                "enter_leaf",
                "leave_leaf",
                "enter_leaf",
                "leave_leaf",
                "leave_biological_tree",
            ],
            visitor.visit_result
        );
    }
}
