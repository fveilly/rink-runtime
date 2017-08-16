extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

//mod json_parser;
mod json_parser;
mod callstack;
mod debug_metadata;
mod error;
mod ink_list;
mod path;
mod runtime;
mod runtime_graph;
