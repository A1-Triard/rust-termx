use std::rc::Rc;
use termx_screen_ncurses::{self};
use termx::base::{MonoClock, World};
use termx::components::static_text::StaticText;
use termx::components::t_button::TButton;
use termx::systems::input::InputExt;
use termx::template::Template;
use termx::termx::{Termx, TermxExt};
use termx::xaml::{self};

fn main() {
    let clock = unsafe { MonoClock::new() };
    let mut screen = unsafe { termx_screen_ncurses::init(None, None).unwrap() };
    let world = &mut World::new();
    let termx = Termx::new(world);
    let xaml = include_str!("ui.xaml");
    let ui: Box<dyn Template> = xaml::from_str(xaml).unwrap();
    let (root, names) = ui.load_content(world, &termx);
    let text = names.find("Text").unwrap();
    let ok = names.find("Ok").unwrap();
    let cancel = names.find("Cancel").unwrap();
    termx.termx().systems().input.focus(Some(ok), world);
    let termx_ref_1 = Rc::downgrade(&termx);
    let ok_str = Rc::new("OK".to_string());
    let cancel_str = Rc::new("Cancel".to_string());
    TButton::on_click(ok, world, &termx, Some(Box::new(move |world| {
        let termx = termx_ref_1.upgrade().unwrap();
        StaticText::set_text(text, world, &termx, ok_str.clone());
    })));
    let termx_ref_2 = Rc::downgrade(&termx);
    TButton::on_click(cancel, world, &termx, Some(Box::new(move |world| {
        let termx = termx_ref_2.upgrade().unwrap();
        StaticText::set_text(text, world, &termx, cancel_str.clone());
    })));
    termx.run(root, screen.as_mut(), world, &clock).unwrap();
}
