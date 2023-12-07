// so what is this
//
// I think their lots of steps keep a fixed rule in general.
// now I figure out the rule by this abstract layer.

use super::common::Step;

pub struct Picture {
    steps: Vec<Box<dyn Step>>,
}
