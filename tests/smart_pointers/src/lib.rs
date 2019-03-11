#![cfg(test)]

use std::rc::Rc;
use std::sync::Arc;

use visit::visit;

visit! {
    #![visitor_trait = "Visitor"]

    struct BoxRoot {
        foo: Box<BoxFoo>,
    }

    struct BoxFoo {
        bar: Box<Bar>,
    }

    struct RcRoot {
        foo: Rc<RcFoo>,
    }

    struct RcFoo {
        bar: Rc<Bar>,
    }

    struct ArcRoot {
        foo: Arc<ArcFoo>,
    }

    struct ArcFoo {
        bar: Arc<Bar>,
    }

    struct Bar;

    struct NestedBoxes {
        name: &'static str,
        children: Vec<Box<NestedBoxes>>,
    }

    struct NestedRcs {
        name: &'static str,
        children: Vec<Rc<NestedRcs>>,
    }

    struct NestedArcs {
        name: &'static str,
        children: Vec<Arc<NestedArcs>>,
    }
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
    fn visit_box_foo(&mut self, _foo: &BoxFoo) {
        self.visit_result.push("BoxFoo");
    }

    fn visit_rc_foo(&mut self, _foo: &RcFoo) {
        self.visit_result.push("RcFoo");
    }

    fn visit_arc_foo(&mut self, _foo: &ArcFoo) {
        self.visit_result.push("ArcFoo");
    }

    fn visit_bar(&mut self, _bar: &Bar) {
        self.visit_result.push("Bar");
    }

    fn visit_nested_boxes(&mut self, nested_node: &NestedBoxes) {
        self.visit_result.push(nested_node.name);
    }

    fn visit_nested_rcs(&mut self, nested_node: &NestedRcs) {
        self.visit_result.push(nested_node.name);
    }

    fn visit_nested_arcs(&mut self, nested_node: &NestedArcs) {
        self.visit_result.push(nested_node.name);
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_box_simple() {
        let root = BoxRoot {
            foo: Box::new(BoxFoo {
                bar: Box::new(Bar {}),
            }),
        };

        let mut visitor = MyVisitor::new();
        root.accept(&mut visitor);

        assert_eq!(vec!["Bar", "BoxFoo"], visitor.visit_result);
    }

    #[test]
    fn test_rc_simple() {
        let root = RcRoot {
            foo: Rc::new(RcFoo {
                bar: Rc::new(Bar {}),
            }),
        };

        let mut visitor = MyVisitor::new();
        root.accept(&mut visitor);

        assert_eq!(vec!["Bar", "RcFoo"], visitor.visit_result);
    }

    #[test]
    fn test_arc_simple() {
        let root = ArcRoot {
            foo: Arc::new(ArcFoo {
                bar: Arc::new(Bar {}),
            }),
        };

        let mut visitor = MyVisitor::new();
        root.accept(&mut visitor);

        assert_eq!(vec!["Bar", "ArcFoo"], visitor.visit_result);
    }

    #[test]
    fn test_box_nested() {
        let root = NestedBoxes {
            name: "root",
            children: vec![Box::new(NestedBoxes {
                name: "A",
                children: vec![
                    Box::new(NestedBoxes {
                        name: "B",
                        children: Vec::new(),
                    }),
                    Box::new(NestedBoxes {
                        name: "C",
                        children: Vec::new(),
                    }),
                ],
            })],
        };

        let mut visitor = MyVisitor::new();
        root.accept(&mut visitor);

        assert_eq!(vec!["B", "C", "A", "root"], visitor.visit_result);
    }

    #[test]
    fn test_rc_nested() {
        let root = NestedRcs {
            name: "root",
            children: vec![Rc::new(NestedRcs {
                name: "A",
                children: vec![
                    Rc::new(NestedRcs {
                        name: "B",
                        children: Vec::new(),
                    }),
                    Rc::new(NestedRcs {
                        name: "C",
                        children: Vec::new(),
                    }),
                ],
            })],
        };

        let mut visitor = MyVisitor::new();
        root.accept(&mut visitor);

        assert_eq!(vec!["B", "C", "A", "root"], visitor.visit_result);
    }

    #[test]
    fn test_arc_nested() {
        let root = NestedArcs {
            name: "root",
            children: vec![Arc::new(NestedArcs {
                name: "A",
                children: vec![
                    Arc::new(NestedArcs {
                        name: "B",
                        children: Vec::new(),
                    }),
                    Arc::new(NestedArcs {
                        name: "C",
                        children: Vec::new(),
                    }),
                ],
            })],
        };

        let mut visitor = MyVisitor::new();
        root.accept(&mut visitor);

        assert_eq!(vec!["B", "C", "A", "root"], visitor.visit_result);
    }
}
