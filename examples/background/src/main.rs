use termx_screen_ncurses::{self};
use termx::base::MonoClock;
use termx::template::Template;
use termx::termx::{Termx, TermxExt};
use termx::xaml::{self};

fn main() {
    let clock = unsafe { MonoClock::new() };
    let mut screen = unsafe { termx_screen_ncurses::init(None, None).unwrap() };
    let termx = Termx::new();
    let xaml = include_str!("ui.xaml");
    let ui: Box<dyn Template> = xaml::from_str(xaml).unwrap();
    let (root, _) = ui.load_content(&termx);
    termx.run(root, screen.as_mut(), &clock).unwrap();
}
