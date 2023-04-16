use super::*;

pub struct Void;

impl Widget for Void {
    fn calc_constraints(&mut self, _children: &ConstraintsContext) -> Constraints {
        Constraints::default()
    }
}
