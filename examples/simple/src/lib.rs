use visit::visit;

visit! {
    #![visitor_trait_pub = "Visitor"]

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
