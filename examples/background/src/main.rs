use termx_screen_ncurses::{self};
use termx::termx::{Termx, TermxExt};

fn main() {
    let mut screen = unsafe { termx_screen_ncurses::init(None, None).unwrap() };
    let termx = Termx::new();
    let root = termx.new_background();
    termx.run(root, screen.as_mut()).unwrap();
}
