use std::collections::HashSet;

use visit::visit;

visit! {
    #![visitor_trait = "Visitor"]

    struct FooVec {
        bars_vec: Vec<Bar>,
    }

    struct FooSet {
        bazs_set: HashSet<Baz>,
    }

    struct FooArray {
        bars_array: [Bar; 2],
    }

    struct Bar {
        id: usize,
    }

    #[derive(Hash, Eq, PartialEq)]
    struct Baz {
        id: usize,
    }
}

#[allow(dead_code)]
struct MyVisitor {
    visit_result: Vec<String>,
}

impl MyVisitor {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            visit_result: Vec::new(),
        }
    }
}

impl Visitor for MyVisitor {
    fn visit_foo_set(&mut self, _foo: &FooSet) {
        self.visit_result.push("FooSet".to_owned());
    }

    fn visit_foo_vec(&mut self, _foo: &FooVec) {
        self.visit_result.push("FooVec".to_owned());
    }

    fn visit_foo_array(&mut self, _foo: &FooArray) {
        self.visit_result.push("FooArray".to_owned());
    }

    fn visit_bar(&mut self, bar: &Bar) {
        self.visit_result.push(format!("Bar{}", bar.id));
    }

    fn visit_baz(&mut self, _baz: &Baz) {
        self.visit_result.push("Baz".to_owned());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_simple() {
        let tree = FooVec {
            bars_vec: vec![Bar { id: 0 }, Bar { id: 1 }],
        };
        let mut v = MyVisitor::new();
        tree.accept(&mut v);
        assert_eq!(vec!["Bar0", "Bar1", "FooVec"], v.visit_result);
    }

    #[test]
    fn test_hash_set_simple() {
        let mut bazs_set = HashSet::new();
        bazs_set.insert(Baz { id: 0 });
        bazs_set.insert(Baz { id: 1 });
        let tree = FooSet { bazs_set };
        let mut v = MyVisitor::new();
        tree.accept(&mut v);
        assert_eq!(vec!["Baz", "Baz", "FooSet"], v.visit_result);
    }

    #[test]
    fn test_array_simple() {
        let tree = FooArray {
            bars_array: [Bar { id: 0 }, Bar { id: 1 }],
        };
        let mut v = MyVisitor::new();
        tree.accept(&mut v);
        assert_eq!(vec!["Bar0", "Bar1", "FooArray"], v.visit_result);
    }
}
