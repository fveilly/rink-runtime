extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

//mod json_parser;
mod json_parser2;
mod callstack;
mod debug_metadata;
mod ink_list;
mod path;
mod runtime;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
