extern crate visit;

use visit::visit;

visit! {    //~ 5:1: 8:2: proc macro panicked
    #![visitor(name = "Visitor", public = false)]
    #![visitor(name = "Visitor", public = true)]
}

fn main() {

}
