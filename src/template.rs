use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::termx::{Termx, IsTermx};
use dyn_clone::{DynClone, clone_trait_object};
use hashbrown::HashMap;
use ooecs::{Entity, World};
use print_no_std::eprintln;

#[derive(Clone)]
pub struct Names {
    map: HashMap<String, Entity<Termx>>,
}

impl Names {
    fn new() -> Self {
        Names { map: HashMap::new() }
    }

    fn register(&mut self, name: &str, obj: Entity<Termx>) {
        if self.map.insert(name.to_string(), obj).is_some() {
            eprintln!("Warning: conflicting names ('{name}')");
        }
    }

    pub fn find(&self, name: &str) -> Option<Entity<Termx>> {
        self.map.get(name).copied()
    }
}

pub struct NameResolver {
    names: Names,
    clients: Vec<(String, Box<dyn FnOnce(Entity<Termx>)>, Option<Box<dyn FnOnce() -> Entity<Termx>>>)>,
}

impl NameResolver {
    fn new() -> Self {
        NameResolver {
            names: Names::new(),
            clients: Vec::new(),
        }
    }

    pub fn resolve(&mut self, name: String, client: Box<dyn FnOnce(Entity<Termx>)>) {
        if !name.is_empty() {
            self.clients.push((name, client, None));
        }
    }

    pub fn resolve_or_create(
        &mut self,
        name: String,
        client: Box<dyn FnOnce(Entity<Termx>)>,
        create: Box<dyn FnOnce() -> Entity<Termx>>,
    ) {
        if !name.is_empty() {
            self.clients.push((name, client, Some(create)));
        }
    }

    fn finish(mut self) -> Names {
        for (name, client, factory) in self.clients {
            let named_obj = if let Some(&named_obj) = self.names.map.get(&name) {
                named_obj
            } else {
                if let Some(factory) = factory {
                    let named_obj = factory();
                    self.names.register(&name, named_obj);
                    named_obj
                } else {
                    eprintln!("Warning: name not found ('{name}')");
                    continue;
                }
            };
            client(named_obj)
        }
        self.names
    }
}

#[typetag::serde]
pub trait Template: DynClone {
    fn name(&self) -> Option<&String>;

    fn create_entity(&self, world: &mut World<Termx>, termx: &Rc<dyn IsTermx>) -> Entity<Termx>;

    fn apply_resources<'a>(
        &self,
        entity: Entity<Termx>,
        world: &'a mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
    ) -> Option<&'a Rc<dyn Template>>;

    fn apply(
        &self,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        names: &mut NameResolver,
    );

    fn begin_load_content_inline(
        &self,
        world: &mut World<Termx>, 
        termx: &Rc<dyn IsTermx>,
        names: &mut NameResolver,
    ) -> Entity<Termx> {
        let entity = self.create_entity(world, termx);
        if let Some(name) = self.name() && !name.is_empty() {
            names.names.register(name, entity);
        }
        entity
    }

    fn end_load_content_inline(
        &self,
        entity: Entity<Termx>,
        world: &mut World<Termx>, 
        termx: &Rc<dyn IsTermx>,
        names: &mut NameResolver,
    ) {
        if let Some(style) = self.apply_resources(entity, world, termx) {
            let mut name_resolver = NameResolver::new();
            style.clone().apply(entity, world, termx, &mut name_resolver);
            name_resolver.finish();
        }
        self.apply(entity, world, termx, names);
    }

    fn begin_load_content(
        &self,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
    ) -> (Entity<Termx>, NameResolver) {
        let mut name_resolver = NameResolver::new();
        let instance = self.begin_load_content_inline(world, termx, &mut name_resolver);
        (instance, name_resolver)
    }

    fn end_load_content(
        &self,
        entity: Entity<Termx>,
        world: &mut World<Termx>,
        termx: &Rc<dyn IsTermx>,
        mut name_resolver: NameResolver,
    ) -> Names {
        self.end_load_content_inline(entity, world, termx, &mut name_resolver);
        name_resolver.finish()
    }
}

clone_trait_object!(Template);

#[macro_export]
macro_rules! template {
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
        mod $mod {
            $(use $path as $import;)*

            $(#[$attr])*
            pub struct $name {
                $($(
                    $(#[$field_attr])*
                    pub $field_name : $field_ty
                ),+)?
            }
        }
        $vis use $mod::$name;
    };
}
