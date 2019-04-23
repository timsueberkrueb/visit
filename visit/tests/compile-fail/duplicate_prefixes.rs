extern crate visit;

use visit::visit;

visit! {
    #![visitor(name = "Visitor", enter = "bar", leave = "bar")] //~ 6:5: 6:6: Same identifier `bar` used for both leave and enter
}

fn main() {

}
