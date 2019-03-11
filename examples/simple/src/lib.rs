use visit::visit;

visit! {
    #![visitor(name = "Visitor", public = true)]

    pub struct Bar {
        pub a: Child,
        pub b: Child,
    }

    pub struct Child {}
}

pub struct MyVisitor;

impl Visitor for MyVisitor {
    fn visit_child(&mut self, _child: &Child) {
        // Do something cool
    }
}
