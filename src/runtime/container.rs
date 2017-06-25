use path::Component as PathComponent;
use runtime::RuntimeObject;
use runtime::RuntimeNamedObject;

pub struct Container {
    content: Vec<RuntimeObject>,
    name: Option<String>,
    visits_should_be_counted: bool,
    turn_index_should_be_counted: bool,
    count_at_start_only: bool
}

impl Container {
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

    pub fn get_content_from_path_component(&self, component: &PathComponent)-> Option<&RuntimeObject> {
        match component {
            &PathComponent::Index(ref index_component) => {
                let index = index_component.index();

                if index >= 0 && index < self.content.len()  {
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
    }
}

impl RuntimeNamedObject for Container {
    fn name(&self) -> Option<&String> {
        match self.name {
            Some(ref name) => Some(name),
            _ => None
        }
    }

    fn has_valid_name(&self) -> bool {
        match self.name {
            Some(ref name) => name.is_empty(),
            _ => false
        }
    }
}