extern crate visit;

use visit::visit;

visit! {    //~ 5:1: 7:2: proc macro panicked
    #![visitor(name = "Visitor", enter = "bar", leave = "bar")]
}

fn main() {

}
