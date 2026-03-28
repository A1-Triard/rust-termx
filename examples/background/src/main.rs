use basic_oop::obj::IsObj;
use std::rc::Rc;
use termx_screen_ncurses::{self};
use termx::template::Template;
use termx::termx::{Termx, TermxExt};
use termx::xaml::{self};

fn main() {
    let mut screen = unsafe { termx_screen_ncurses::init(None, None).unwrap() };
    let termx = Termx::new();
    let termx_as_obj: Rc<dyn IsObj> = termx.clone();
    let xaml = include_str!("ui.xaml");
    let ui: Box<dyn Template> = xaml::from_str(xaml).unwrap();
    let (root, _) = ui.load_content(&termx_as_obj);
    termx.run(root, screen.as_mut()).unwrap();
}
