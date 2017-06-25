use std::fmt;

pub struct DebugMetadata {
    start_line_number: u32,
    end_line_number: u32,
    file_name: Option<String>,
    source_name: Option<String>
}

impl DebugMetadata {
    pub fn new() -> DebugMetadata {
        DebugMetadata {
            start_line_number: 0,
            end_line_number: 0,
            file_name: None,
            source_name: None
        }
    }

    pub fn from_metadata(start_line_number: u32, end_line_number: u32, file_name: String, source_name: String) -> DebugMetadata {
        DebugMetadata {
            start_line_number: start_line_number,
            end_line_number: end_line_number,
            file_name: Some(file_name),
            source_name: Some(source_name)
        }
    }

    pub fn start_line_number(&self) -> u32 {
        self.start_line_number
    }

    pub fn end_line_number(&self) -> u32 {
        self.end_line_number
    }

    pub fn file_name(&self) -> Option<&String> {
        match self.file_name {
            Some(ref file_name) => Some(file_name),
            _ => None
        }
    }

    pub fn source_name(&self) -> Option<&String> {
        match self.source_name {
            Some(ref source_name) => Some(source_name),
            _ => None
        }
    }
}

impl fmt::Display for DebugMetadata {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.file_name {
            Some(ref file_name) => write!(f, "line {} of {}", self.start_line_number, file_name),
            _ => write!(f, "line {}", self.start_line_number)
        }
    }
}