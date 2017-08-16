use path::{Path, Fragment};
use runtime::RuntimeObject;

pub struct Container {
    content: Vec<RuntimeObject>,
    name: Option<String>,
    visits_should_be_counted: bool,
    turn_index_should_be_counted: bool,
    count_at_start_only: bool
}

impl Container {
    pub fn new() -> Container {
        Container {
            content: Vec::new(),
            name: None,
            visits_should_be_counted: false,
            turn_index_should_be_counted: false,
            count_at_start_only: false
        }
    }

    pub fn from_runtime_object_vec(content: Vec<RuntimeObject>) -> Container {
        Container {
            content: content,
            name: None,
            visits_should_be_counted: false,
            turn_index_should_be_counted: false,
            count_at_start_only: false
        }
    }

    pub fn content(&self) -> &Vec<RuntimeObject> {
        &self.content
    }

    pub fn visits_should_be_counted(&self) -> bool {
        self.visits_should_be_counted
    }

    pub fn set_visits_should_be_counted(&mut self, flag: bool) {
        self.visits_should_be_counted = flag;
    }

    pub fn turn_index_should_be_counted(&self) -> bool {
        self.turn_index_should_be_counted
    }

    pub fn set_turn_index_should_be_counted(&mut self, flag: bool) {
        self.turn_index_should_be_counted = flag;
    }

    pub fn count_at_start_only(&self) -> bool {
        self.count_at_start_only
    }

    pub fn set_count_at_start_only(&mut self, flag: bool) {
        self.count_at_start_only = flag;
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|x| x.as_ref())
    }

    pub fn set_name(&mut self, name: String)
    {
        self.name = Some(name);
    }

    pub fn count_flags(&self) -> u8 {
        let mut count_flags: u8 = 0;

        if self.visits_should_be_counted {
            count_flags &= 0x1;
        }

        if self.turn_index_should_be_counted {
            count_flags &= 0x2;
        }

        if self.count_at_start_only {
            count_flags &= 0x4;
        }

        if count_flags == 0x4 {
            0
        } else {
            count_flags
        }
    }

    pub fn set_count_flags(&mut self, count_flags: u8) {
        if count_flags &  0x1 > 0 {
            self.visits_should_be_counted = true;
        }

        if count_flags &  0x2 > 0 {
            self.turn_index_should_be_counted = true;
        }

        if count_flags &  0x4 > 0 {
            self.count_at_start_only = true;
        }
    }

    pub fn add_child(&mut self, obj: RuntimeObject) {
        self.content.push(obj);
    }

    pub fn append(&mut self, mut objects: Vec<RuntimeObject>) {
        self.content.append(&mut objects);
    }

    pub fn prepend(&mut self, mut objects: Vec<RuntimeObject>) {
        objects.append(&mut self.content);
        self.content = objects;
    }

    /*pub fn get_content_from_path_component(&self, component: &PathComponent)-> Option<&RuntimeObject> {
        match component {
            &PathComponent::Index(ref index_component) => {
                let index = index_component.index();

                if index < self.content.len()  {
                    self.content.get(index)
                } else {
                    None
                }
            },
            &PathComponent::Named(ref named_component) => {
                if named_component.is_parent() {
                    // self.parent()
                    None
                }
                else {
                    // TODO
                    None
                }
            }
        }
    }*/

    pub fn search_by_name(&self, name: &str) -> Option<&RuntimeObject> {
        for runtime_object in &self.content {
            if let Some(other_name) = runtime_object.name() {
                if name == other_name {
                    return Some(runtime_object)
                }
            }
        }

        None
    }
}
