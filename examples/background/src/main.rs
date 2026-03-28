use termx_screen_ncurses::{self};
use termx::components::background::Background;
use termx::termx::{Termx, TermxExt};

fn main() {
    let mut screen = unsafe { termx_screen_ncurses::init(None, None).unwrap() };
    let termx = Termx::new();
    let root = Background::new_entity(&termx);
    termx.run(root, screen.as_mut()).unwrap();
}
