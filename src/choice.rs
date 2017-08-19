use runtime::choice_point::ChoicePoint;
use callstack::Thread;
use path::Path;

pub struct Choice {
    text: Option<String>,
    choice_point: ChoicePoint,
    thread: Thread
}

impl Choice {
    pub fn from_choice_point(&self, choice_point: ChoicePoint, thread: Thread) -> Choice {
        Choice {
            text: None,
            choice_point: choice_point,
            thread: thread
        }
    }

    pub fn text(&self) -> Option<&str> {
        match self.text {
            Some(ref text) => Some(&text),
            _ => None
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = Some(text)
    }

    pub fn path_on_choice(&self) -> Option<&Path> {
        self.choice_point.path_on_choice()
    }
}