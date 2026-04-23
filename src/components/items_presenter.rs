use alloc::rc::Rc;
use alloc::string::ToString;
use alloc::vec::Vec;
use crate::property;
use crate::components::stack_panel::StackPanelTemplate;
use crate::components::static_text::StaticTextTemplate;
use crate::components::panel::Panel;
use crate::components::focus_scope::FocusScope;
use crate::components::layout_view::*;
use crate::components::view::*;
use crate::systems::layout::LayoutExt;
use crate::systems::render::RenderExt;
use crate::systems::init::InitExt;
use crate::termx::{Termx, IsTermx};
use crate::template::Template;
use ooecs::{Entity, World};

pub struct ItemsPresenter {
    panel_template: Rc<dyn Template>,
    item_template: Rc<dyn Template>,
    items_count: usize,
    panel: Option<Entity<Termx>>,
}

impl ItemsPresenter {
    pub fn new() -> Self {
        ItemsPresenter {
            panel_template: Rc::new(StackPanelTemplate::default()),
            item_template: Rc::new(StaticTextTemplate {
                text: Some("ITEM".to_string()),
                .. Default::default()
            }),
            items_count: 0,
            panel: None,
        }
    }

    pub fn new_entity(world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx> {
        let c = termx.termx().components();
        let s = termx.termx().systems();
        let e = Entity::new(c.items_presenter, world);
        e.add(c.view, world, View::new(TREE_ITEMS_PRESENTER, RENDER_NONE));
        e.add(c.layout_view, world, LayoutView::new(LAYOUT_ITEMS_PRESENTER));
        e.add(c.focus_scope, world, FocusScope::new());
        e.add(c.items_presenter, world, ItemsPresenter::new());
        s.init.init_items_presenter(e, world);
        e
    }

    property!(Termx, items_presenter, panel_template, ref Rc<dyn Template>, update_panel);

    pub(crate) fn update_panel(entity: Entity<Termx>, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) {
        let c = termx.termx().components();
        let s = termx.termx().systems();
        let items_presenter = entity.get(c.items_presenter, world).unwrap();
        let panel_template = items_presenter.panel_template.clone();
        if let Some(old_panel) = items_presenter.panel {
            s.render.remove_visual_child(entity, old_panel, world);
        }
        let (panel, names) = panel_template.begin_load_content(world, termx);
        entity.get_mut(c.items_presenter, world).unwrap().panel = Some(panel);
        s.render.add_visual_child(entity, panel, world);
        s.layout.invalidate_measure(entity, world);
        panel_template.end_load_content(panel, world, termx, names);
        Self::update_items(entity, world, termx);
    }

    property!(Termx, items_presenter, item_template, ref Rc<dyn Template>, update_items);

    fn update_items(entity: Entity<Termx>, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) {
        let c = termx.termx().components();
        let items_presenter = entity.get(c.items_presenter, world).unwrap();
        let panel = items_presenter.panel.unwrap();
        let item_template = items_presenter.item_template.clone();
        let items_count = items_presenter.items_count;
        let items: Vec<_> = (0 .. items_count).into_iter()
            .map(|_| item_template.begin_load_content(world, termx))
            .collect()
        ;
        Panel::set_children(panel, world, termx, &items.iter().map(|x| x.0).collect());
        for (item, names) in items {
            item_template.end_load_content(item, world, termx, names);
        }
    }
}
