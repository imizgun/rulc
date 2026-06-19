use crate::view::viewable::Viewable;

pub struct TuiView;

impl Viewable for TuiView {
    fn run(&self) {
        println!("TUI");
    }
}