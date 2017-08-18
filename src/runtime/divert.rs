use path::Path;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PushPopType {
    Tunnel,
    Function,
    None
}

#[derive(PartialEq)]
pub enum TargetType {
    Name(String),
    Path(Path)
}

pub struct Divert {
    target: Option<TargetType>,
    stack_push_type: PushPopType,
    pushes_to_stack: bool,
    external_args: Option<u32>,
    is_external: bool,
    is_conditional: bool
}

impl Divert {
    pub fn new() -> Divert {
        Divert {
            target: None,
            stack_push_type: PushPopType::None,
            pushes_to_stack: false,
            external_args: None,
            is_external: false,
            is_conditional: false
        }
    }

    pub fn new_function() -> Divert {
        Divert {
            target: None,
            stack_push_type: PushPopType::Function,
            pushes_to_stack:true,
            external_args: None,
            is_external: false,
            is_conditional: false
        }
    }

    pub fn new_tunnel() -> Divert {
        Divert {
            target: None,
            stack_push_type: PushPopType::Tunnel,
            pushes_to_stack:true,
            external_args: None,
            is_external: false,
            is_conditional: false
        }
    }

    pub fn new_external_function() -> Divert {
        Divert {
            target: None,
            stack_push_type: PushPopType::Function,
            pushes_to_stack:false,
            external_args: None,
            is_external: true,
            is_conditional: false
        }
    }

    pub fn stack_push_type(&self) -> &PushPopType {
        &self.stack_push_type
    }

    pub fn pushes_to_stack(&self) -> bool {
        self.pushes_to_stack
    }

    pub fn is_external(&self) -> bool {
        self.is_external
    }

    pub fn is_conditional(&self) -> bool {
        self.is_conditional
    }

    pub fn target(&self) -> Option<&TargetType> {
        self.target.as_ref()
    }

    pub fn external_args(&self) -> Option<u32> {
        self.external_args
    }

    pub fn set_target(&mut self, target: TargetType) {
        self.target = Some(target);
    }

    pub fn set_is_conditional(&mut self, is_conditional: bool) {
        self.is_conditional = is_conditional;
    }

    pub fn set_external_args(&mut self, external_args: u32) {
        self.external_args = Some(external_args);
    }
}