use alloc::rc::Rc;
use crate::property_ro;
use crate::systems::layout::LayoutExt;
use crate::systems::render::RenderExt;
use crate::termx::IsTermx;
use ooecs::Entity;

pub struct Decorator {
    pub(crate) child: Option<Entity>,
}

impl Decorator {
    pub fn new() -> Self {
        Decorator { child: None }
    }

    property_ro!(Termx, decorator, child, Option<Entity>);

    pub fn set_child(entity: Entity, termx: &Rc<dyn IsTermx>, value: Option<Entity>) {
        let termx = termx.termx();
        let component = termx.components().decorator;
        let mut world = termx.world.borrow_mut();
        let old_child = entity.get::<Decorator>(component, &mut world).unwrap().child;
        if let Some(child) = old_child {
            termx.systems().render.remove_visual_child(entity, child, &mut world);
        }
        entity.get_mut::<Self>(component, &mut world).unwrap().child = value;
        if let Some(child) = value {
            termx.systems().render.add_visual_child(entity, child, &mut world);
        }
        termx.systems().layout.invalidate_measure(entity, &mut world);
    }
}

/*
#[macro_export]
macro_rules! decorator_template {
    (
        $(#[$attr:meta])*
        $vis:vis struct $name:ident in $mod:ident {
            $(use $path:path as $import:ident;)*

            $($(
                $(#[$field_attr:meta])*
                pub $field_name:ident : $field_ty:ty
            ),+ $(,)?)?
        }
    ) => {
        $crate::view_template! {
            $(#[$attr])*
            $vis struct $name in $mod {
                $(use $path as $import;)*

                #[serde(default)]
                #[serde(skip_serializing_if="Option::is_none")]
                pub child: Option<Box<dyn $crate::template::Template>>,
                $($(
                    $(#[$field_attr])*
                    pub $field_name : $field_ty
                ),+)?
            }
        }
    };
}

#[macro_export]
macro_rules! decorator_apply_template {
    ($this:ident, $instance:ident, $names:ident) => {
        $crate::view_apply_template!($this, $instance, $names);
        {
            use $crate::decorator::DecoratorExt;

            let obj: $crate::alloc_rc_Rc<dyn $crate::decorator::IsDecorator>
                = $crate::dynamic_cast_dyn_cast_rc($instance.clone()).unwrap();
            $this.child.as_ref().map(|x|
                obj.set_child(Some($crate::dynamic_cast_dyn_cast_rc(x.load_content($names)).unwrap()))
            );
        }
    };
}

decorator_template! {
    #[derive(serde::Serialize, serde::Deserialize, Default, Clone)]
    #[serde(rename="Decorator@Child")]
    pub struct DecoratorTemplate in template { }
}

#[typetag::serde(name="Decorator")]
impl Template for DecoratorTemplate {
    fn is_name_scope(&self) -> bool {
        self.is_name_scope
    }

    fn name(&self) -> Option<&String> {
        Some(&self.name)
    }

    fn create_instance(&self) -> Rc<dyn IsObj> {
        Decorator::new()
    }

    fn apply(&self, instance: &Rc<dyn IsObj>, names: &mut NameResolver) {
        let this = self;
        decorator_apply_template!(this, instance, names);
    }
}
*/
