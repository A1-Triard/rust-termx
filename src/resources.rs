use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::string::String;
use crate::template::Template;
use hashbrown::HashMap;

pub struct Resources {
    pub base: Option<Rc<Resources>>,
    pub map: HashMap<String, Box<dyn Template>>,
}

impl Resources {
    pub fn new() -> Self {
        Resources {
            base: None,
            map: HashMap::new(),
        }
    }

    pub fn get<'a>(&'a self, key: &str) -> Option<&'a Box<dyn Template>> {
        if let Some(res) = self.map.get(key) {
            Some(res)
        } else if let Some(base) = self.base.as_ref() {
            base.get(key)
        } else {
            None
        }
    }
}
