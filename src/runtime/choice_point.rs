use std::fmt;

use path::Path;

pub struct ChoicePoint {
    has_condition: bool,
    has_start_content: bool,
    has_choice_only_content: bool,
    once_only: bool,
    is_invisible_default: bool,
    path_on_choice: Option<Path>
}

impl ChoicePoint {
    pub fn new(once_only: bool) -> ChoicePoint {
        ChoicePoint {
            has_condition: false,
            has_start_content: false,
            has_choice_only_content: false,
            once_only: once_only,
            is_invisible_default: false,
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

        if self.once_only {
            flags |= 0x8;
        }

        if self.is_invisible_default {
            flags |= 0x10;
        }

        flags
    }

    pub fn set_flags(&mut self, flags: u8) {
        self.has_condition = flags & 0x1 > 0;
        self.has_start_content = flags & 0x2 > 0;
        self.has_choice_only_content = flags & 0x4 > 0;
        self.has_condition = flags & 0x8 > 0;
        self.has_condition = flags & 0x10 > 0;
    }
}

impl fmt::Display for ChoicePoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO
        write!(f, "")
    }
}