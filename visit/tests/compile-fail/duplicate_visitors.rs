extern crate visit;

use visit::visit;

visit! {
    #![visitor(name = "Visitor", public = false)]
    #![visitor(name = "Visitor", public = true)] //~ 7:5: 7:6: Visitor `Visitor` defined more than once
}

fn main() {

}
