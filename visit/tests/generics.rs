use visit::visit;

visit! {
    #![visitor(name = "Visitor")]
    #![hierarchical_visitor(name = "HierVisitor")]

    struct GenericTest<A, B>
    where
        A: Copy + AcceptVisitor + AcceptHierVisitor,
        B: Default + AcceptVisitor + AcceptHierVisitor,
    {
        foo: A,
        bar: B,
    }

    enum GenericEnum<A, B>
    where
        A: Copy + AcceptVisitor + AcceptHierVisitor,
        B: Default + AcceptVisitor + AcceptHierVisitor,
    {
        Foo { a: A },
        Bar { b: B },
    }

    struct LifetimeTest<'a> {
        s: &'a str,
    }

    struct OptionTest {
        maybe_foo1: Option<Foo>,
        maybe_foo2: Option<Foo>,
    }

    struct Foo;
}

struct MyVisitor {
    visited_generic_test: bool,
    visited_lifetime_test: bool,
    visited_generic_enum: bool,
    visited_option_count: usize,
}

impl MyVisitor {
    fn new() -> Self {
        Self {
            visited_generic_test: false,
            visited_lifetime_test: false,
            visited_generic_enum: false,
            visited_option_count: 0,
        }
    }
}

impl Visitor for MyVisitor {
    fn visit_generic_test<A, B>(&mut self, _test: &GenericTest<A, B>)
    where
        A: Copy + AcceptVisitor + AcceptHierVisitor,
        B: Default + AcceptVisitor + AcceptHierVisitor,
    {
        self.visited_generic_test = true;
    }

    fn visit_generic_enum<A, B>(&mut self, _test: &GenericEnum<A, B>)
    where
        A: Copy + AcceptVisitor + AcceptHierVisitor,
        B: Default + AcceptVisitor + AcceptHierVisitor,
    {
        self.visited_generic_enum = true;
    }

    fn visit_lifetime_test<'a>(&mut self, _test: &LifetimeTest<'a>) {
        self.visited_lifetime_test = true;
    }

    fn visit_foo(&mut self, _foo: &Foo) {
        self.visited_option_count += 1;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_generic_struct_simple() {
        let test = GenericTest {
            foo: 0usize,
            bar: String::new(),
        };
        let mut v = MyVisitor::new();
        AcceptVisitor::accept(&test, &mut v);
        assert!(v.visited_generic_test);
    }

    #[test]
    fn test_generic_enum_simple_a() {
        let test: GenericEnum<usize, String> = GenericEnum::Foo { a: 0usize };
        let mut v = MyVisitor::new();
        AcceptVisitor::accept(&test, &mut v);
        assert!(v.visited_generic_enum);
    }

    #[test]
    fn test_generic_enum_simple_b() {
        let test: GenericEnum<usize, String> = GenericEnum::Bar { b: String::new() };
        let mut v = MyVisitor::new();
        AcceptVisitor::accept(&test, &mut v);
        assert!(v.visited_generic_enum);
    }

    #[test]
    fn test_lifetime_simple() {
        let test_string = "Borrow me!".to_owned();
        let test = LifetimeTest { s: &test_string };
        let mut v = MyVisitor::new();
        AcceptVisitor::accept(&test, &mut v);
        assert!(v.visited_lifetime_test);
    }

    #[test]
    fn test_option_simple() {
        let test = OptionTest {
            maybe_foo1: Some(Foo {}),
            maybe_foo2: None,
        };
        let mut v = MyVisitor::new();
        AcceptVisitor::accept(&test, &mut v);
        assert_eq!(1, v.visited_option_count);
    }
}
