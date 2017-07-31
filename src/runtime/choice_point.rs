use std::fmt;

use path::Path;

pub struct ChoicePoint {
    has_condition: bool,
    has_start_content: bool,
    has_choice_only_content: bool,
    is_invisible_default: bool,
    once_only: bool,
    path_on_choice: Option<Path>
}

impl ChoicePoint {
    pub fn new() -> ChoicePoint {
        ChoicePoint {
            has_condition: false,
            has_start_content: false,
            has_choice_only_content: false,
            is_invisible_default: false,
            once_only: false,
            path_on_choice: None
        }
    }

    pub fn flags(&self) -> u8 {
        let mut flags: u8 = 0;

        if self.has_condition {
            flags |= 0x1;
        }

        if self.has_start_content {
            flags |= 0x2;
        }

        if self.has_choice_only_content {
            flags |= 0x4;
        }

        if self.is_invisible_default {
            flags |= 0x8;
        }

        if self.once_only {
            flags |= 0x10;
        }

        flags
    }

    pub fn set_flags(&mut self, flags: u8) {
        self.has_condition = flags & 0x1 > 0;
        self.has_start_content = flags & 0x2 > 0;
        self.has_choice_only_content = flags & 0x4 > 0;
        self.is_invisible_default = flags & 0x8 > 0;
        self.once_only = flags & 0x10 > 0;
    }

    pub fn has_condition(&self) -> bool {
        self.has_condition
    }

    pub fn has_start_content(&self) -> bool {
        self.has_start_content
    }

    pub fn has_choice_only_content(&self) -> bool {
        self.has_choice_only_content
    }

    pub fn is_invisible_default(&self) -> bool {
        self.is_invisible_default
    }

    pub fn once_only(&self) -> bool {
        self.once_only
    }

    pub fn path_on_choice(&self) -> Option<&Path> {
        match self.path_on_choice {
            Some(ref path) => {
                // TODO
                Some(path)
            },
            _ => None
        }
    }

    pub fn set_path_on_choice(&mut self, path: Path) {
        self.path_on_choice = Some(path);
    }
}

impl fmt::Display for ChoicePoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO
        write!(f, "")
    }
}