extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
mod macros;

mod callstack;
mod choice;
mod debug_metadata;
mod error;
mod ink_list;
mod json_parser;
mod path;
mod runtime;
mod runtime_graph;
mod story;
mod story_state;
