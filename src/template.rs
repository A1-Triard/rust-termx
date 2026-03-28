use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use basic_oop::obj::IsObj;
use dyn_clone::{DynClone, clone_trait_object};
use hashbrown::HashMap;
use ooecs::Entity;
use print_no_std::eprintln;

#[derive(Clone)]
pub struct Names {
    map: HashMap<String, Entity>,
}

impl Names {
    fn new() -> Self {
        Names { map: HashMap::new() }
    }

    fn register(&mut self, name: &str, obj: Entity) {
        if self.map.insert(name.to_string(), obj).is_some() {
            eprintln!("Warning: conflicting names ('{name}')");
        }
    }

    pub fn find(&self, name: &str) -> Option<Entity> {
        self.map.get(name).copied()
    }
}

pub struct NameResolver {
    names: Names,
    clients: Vec<(String, Box<dyn FnOnce(Entity)>, Option<Box<dyn FnOnce() -> Entity>>)>,
}

impl NameResolver {
    fn new() -> Self {
        NameResolver {
            names: Names::new(),
            clients: Vec::new(),
        }
    }

    pub fn resolve(&mut self, name: String, client: Box<dyn FnOnce(Entity)>) {
        if !name.is_empty() {
            self.clients.push((name, client, None));
        }
    }

    pub fn resolve_or_create(
        &mut self,
        name: String,
        client: Box<dyn FnOnce(Entity)>,
        create: Box<dyn FnOnce() -> Entity>,
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
    fn name(&self) -> Option<&String> {
        None
    }

    fn create_instance(&self, world_owner: &Rc<dyn IsObj>) -> Entity;

    fn apply(&self, instance: Entity, world_owner: &Rc<dyn IsObj>, names: &mut NameResolver);

    fn load_content(&self, world_owner: &Rc<dyn IsObj>) -> (Entity, Names) {
        let mut name_resolver = NameResolver::new();
        let instance = self.create_instance(world_owner);
        if let Some(name) = self.name() && !name.is_empty() {
            name_resolver.names.register(name, instance);
        }
        self.apply(instance, world_owner, &mut name_resolver);
        let names = name_resolver.finish();
        (instance, names)
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
