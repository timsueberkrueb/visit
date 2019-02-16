#![cfg(test)]

use visit::visit;

visit! {
    #![visitor_trait = "Visitor"]

    struct Primitives<'a> {
        test_u8: u8,
        test_u16: u16,
        test_u32: u32,
        test_u64: u64,
        test_u128: u128,
        test_i8: i8,
        test_i16: i16,
        test_i32: i32,
        test_i64: i64,
        test_i128: i128,
        test_usize: usize,
        test_isize: isize,
        test_f32: f32,
        test_f64: f64,
        test_bool: bool,
        test_str: &'a str,
        test_string: String,
        foo: Foo,
    }

    struct Foo {

    }
}

struct MyVisitor {
    visited_foo: bool,
}

impl MyVisitor {
    fn new() -> Self {
        Self { visited_foo: false }
    }
}

impl Visitor for MyVisitor {
    fn visit_foo(&mut self, _foo: &Foo) {
        self.visited_foo = true;
    }
}

mod test {
    use super::*;

    #[test]
    fn test_ignore_primitives() {
        let p = Primitives {
            test_u8: 0u8,
            test_u16: 0u16,
            test_u32: 0u32,
            test_u64: 0u64,
            test_u128: 0u128,
            test_i8: 0i8,
            test_i16: 0i16,
            test_i32: 0i32,
            test_i64: 0i64,
            test_i128: 0i128,
            test_usize: 0usize,
            test_isize: 0isize,
            test_f32: 0f32,
            test_f64: 0f64,
            test_bool: true,
            test_str: "test",
            test_string: "test".to_owned(),
            foo: Foo {},
        };

        let mut v = MyVisitor::new();
        p.accept(&mut v);

        assert!(v.visited_foo);
    }
}
