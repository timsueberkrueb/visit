# visit

Automatically generate code for the [Visitor pattern](https://en.wikipedia.org/wiki/Visitor_pattern).

## Usage

```rust
visit! {
    // Automatically generates `Visitor` and `AcceptVisitor` traits
    // The `Visitor` trait will contain `visit_<child>` functions for all items inside of the macro call.
    // The `AcceptVisitor` will be implemented automatically for all items.
    #![visitor(name = "Visitor")]

    struct Bar {
        a: Child,
        b: Child,
    }

    struct Child {}
}

struct MyVisitor;

impl Visitor for MyVisitor {
    fn visit_child(&mut self, _child: &Child) {
        // Do something cool
    }
}
```

## License

visit is licensed under either of the following licenses, at your option:

* [Apache License Version 2.0](LICENSE-APACHE)
* [MIT License](LICENSE-MIT)
