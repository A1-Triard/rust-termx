use alloc::rc::{self, Rc};
use basic_oop::{Vtable, import, class_unsafe};
use crate::base::{ViewVAlign, TextWrapping};
use crate::components::adorners_panel::AdornersPanel;
use crate::components::background::Background;
use crate::components::border::Border;
use crate::components::content_presenter::ContentPresenter;
use crate::components::decorator::Decorator;
use crate::components::focus_scope::FocusScope;
use crate::components::layout_view::LayoutView;
use crate::components::input_element::InputElement;
use crate::components::panel::Panel;
use crate::systems::layout::LayoutExt;
use crate::systems::render::RenderExt;
use crate::termx::IsTermx;
use int_vec_2d::Thickness;

import! { pub init:
    use [obj basic_oop::obj];
    use crate::termx::Termx;
    use ooecs::{Entity, World};
}

#[class_unsafe(inherits_Obj)]
pub struct Init {
    termx: rc::Weak<dyn IsTermx>,
    #[virt]
    init_t_button: fn(entity: Entity<Termx>, world: &mut World<Termx>),
    #[virt]
    init_group_box: fn(entity: Entity<Termx>, world: &mut World<Termx>),
    #[non_virt]
    init_control: fn(entity: Entity<Termx>, world: &mut World<Termx>, visual_tree: Entity<Termx>),
}

impl Init {
    pub fn new(termx: &Rc<dyn IsTermx>) -> Rc<dyn IsInit> {
        Rc::new(unsafe { Self::new_raw(termx, INIT_VTABLE.as_ptr()) })
    }

    pub unsafe fn new_raw(termx: &Rc<dyn IsTermx>, vtable: Vtable) -> Self {
        Init {
            obj: unsafe { Obj::new_raw(vtable) },
            termx: Rc::downgrade(termx),
        }
    }

    pub fn init_t_button_impl(this: &Rc<dyn IsInit>, entity: Entity<Termx>, world: &mut World<Termx>) {
        let init = this.init();
        let termx = init.termx.upgrade().unwrap();
        let s = termx.termx().systems();
        s.render.set_shadow(entity, world, Thickness::new(0, 0, 1, 1));
    }

    pub fn init_group_box_impl(this: &Rc<dyn IsInit>, entity: Entity<Termx>, world: &mut World<Termx>) {
        let init = this.init();
        let termx = init.termx.upgrade().unwrap();
        let c = termx.termx().components();
        let group_box = entity.get(c.group_box, world).unwrap();
        let double = group_box.double();
        let color = group_box.color();
        let header_align = group_box.header_align();
        let text = group_box.text().clone();
        let text_color = group_box.text_color();
        let header_text = group_box.header_text().clone();

        let adorners_panel = AdornersPanel::new_entity(world, &termx);

        let part_border = Border::new_entity(world, &termx);
        FocusScope::set_tab_index(part_border, world, &termx, 1);
        Border::set_double(part_border, world, &termx, double);
        Border::set_color(part_border, world, &termx, color);

        let part_content_presenter = ContentPresenter::new_entity(world, &termx);
        ContentPresenter::set_text_wrapping(part_content_presenter, world, &termx, TextWrapping::Wrap);
        ContentPresenter::set_text(part_content_presenter, world, &termx, text);
        ContentPresenter::set_text_color(part_content_presenter, world, &termx, text_color);
        Decorator::set_child(part_border, world, &termx, Some(part_content_presenter));

        let part_header_background = Background::new_entity(world, &termx);
        FocusScope::set_tab_index(part_header_background, world, &termx, 0);
        Background::set_fit_to_content(part_header_background, world, &termx, true);
        Background::set_color(part_header_background, world, &termx, color);
        LayoutView::set_margin(part_header_background, world, &termx, Thickness::new(1, 0, 1, 0));
        LayoutView::set_h_align(part_header_background, world, &termx, header_align);

        let part_header_presenter = ContentPresenter::new_entity(world, &termx);
        LayoutView::set_margin(part_header_presenter, world, &termx, Thickness::new(1, 0, 1, 0));
        LayoutView::set_v_align(part_header_presenter, world, &termx, ViewVAlign::Top);
        ContentPresenter::set_text(part_header_presenter, world, &termx, header_text);
        ContentPresenter::set_text_color(part_header_presenter, world, &termx, color);
        Decorator::set_child(part_header_background, world, &termx, Some(part_header_presenter));

        Panel::get_children_mut(adorners_panel, world, &termx, |children| {
            children.push(part_border);
            children.push(part_header_background);
        });

        let group_box = entity.get_mut(c.group_box, world).unwrap();
        group_box.part_border = Some(part_border);
        group_box.part_header_background = Some(part_header_background);
        group_box.part_content_presenter = Some(part_content_presenter);
        group_box.part_header_presenter = Some(part_header_presenter);

        this.init_control(entity, world, adorners_panel);
    }

    pub fn init_control_impl(
        this: &Rc<dyn IsInit>,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        visual_tree: Entity<Termx>,
    ) {
        let init = this.init();
        let termx = init.termx.upgrade().unwrap();
        let c = termx.termx().components();
        let control = entity.get_mut(c.control, world).unwrap();
        assert!(control.visual_tree.is_none());
        control.visual_tree = Some(visual_tree);
        let s = termx.termx().systems();
        s.render.add_visual_child(entity, visual_tree, world);
        s.layout.invalidate_measure(entity, world);
        InputElement::set_focusable(entity, world, &termx, false);
    }
}
