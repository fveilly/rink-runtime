use runtime::RuntimeObject;
use runtime::container::Container;
use runtime::divert::PushPopType;
use macros;

use std::rc::Rc;

#[derive(Clone)]
struct Element {
    container: Rc<Container>,
    index: usize
}

impl Element {
    pub fn new(container: Rc<Container>) -> Element {
        Element {
            container: container,
            index: 0
        }
    }

    pub fn get(&self) -> Option<&RuntimeObject> {
        self.container.get(self.index)
    }

    pub fn get_container(&self) -> &Rc<Container> {
        &self.container
    }

    pub fn next(&mut self) -> Option<&RuntimeObject> {
        if self.index + 1 >= self.container.len() {
            return None;
        }

        self.index += 1;
        self.container.get(self.index)
    }

    pub fn move_to(&mut self, index: usize) -> bool {
        if index >= self.container.len() {
            return false;
        }

        self.index = index;
        true
    }
}

#[derive(Clone)]
pub struct RuntimeContext {
    stack: Vec<Element>,
    in_expression_evaluation: bool,
    stack_push_type: PushPopType
}

/// Depth-first search (pre-order) of the runtime graph implemented as a LIFO stack.
impl RuntimeContext {
    pub fn new(container: &Rc<Container>) -> RuntimeContext {
        RuntimeContext {
            stack: vec![Element::new(container.clone())],
            in_expression_evaluation: false,
            stack_push_type: PushPopType::Tunnel
        }
    }

    pub fn with_capacity(capacity: usize, container: &Rc<Container>) -> RuntimeContext {
        let mut stack = Vec::with_capacity(capacity);
        stack.push(Element::new(container.clone()));

        RuntimeContext {
            stack: stack,
            in_expression_evaluation: false,
            stack_push_type: PushPopType::Tunnel
        }
    }

    pub fn depth(&self) -> usize {
        self.stack.len()
    }

    pub fn in_expression_evaluation(&self) -> bool {
        self.in_expression_evaluation
    }

    pub fn set_in_expression_evaluation(&mut self, in_expression_evaluation: bool) {
        self.in_expression_evaluation = in_expression_evaluation;
    }

    pub fn stack_push_type(&self) -> PushPopType {
        self.stack_push_type
    }

    pub fn set_stack_push_type(&mut self, stack_push_type: PushPopType) {
        self.stack_push_type = stack_push_type;
    }

    pub fn reset(&mut self, container: &Rc<Container>, index: usize) {
        let mut element = Element::new(container.clone());

        if index != 0 {
            element.move_to(index);
        }

        self.stack.clear();
        self.stack.push(element);
    }

    pub fn get(&self) -> Option<&RuntimeObject> {
        try_opt!(self.stack.last()).get()
    }

    pub fn get_container(&self) -> Option<&Rc<Container>> {
        Some(try_opt!(self.stack.last()).get_container())
    }

    pub fn next(&mut self) -> Option<&RuntimeObject> {
        if !self.do_next() {
            return None;
        }

        self.get()
    }

    fn do_next(&mut self) -> bool {
        // Need to do this because at the moment rust does not support Non-lexical borrow scopes
        // See https://github.com/rust-lang/rfcs/issues/811
        let mut next_container: Option<Rc<Container>> = None;

        if let Some(ref mut element) = self.stack.last_mut() {
            match element.next() {
                Some(&RuntimeObject::Container(ref container)) => {
                    next_container = Some(container.clone());
                },
                Some(runtime_object) => return true,
                None => {}
            }
        }
        else {
            return false;
        }

        match next_container {
            Some(container) => {
                self.stack.push(Element::new(container));
                true
            },
            _ => {
                if self.stack.len() == 1 {
                    return false;
                }

                self.stack.pop();
                self.do_next()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_context_test() {
        use runtime::value::Value;

        // [42, [10, "value1", [3.14]], "value2"]
        let mut root_container = Rc::new(Container::new());

        Rc::get_mut(&mut root_container).unwrap().add_child(RuntimeObject::Value(Value::Int(42)));

        let mut sub_container = Rc::new(Container::new());
        Rc::get_mut(&mut sub_container).unwrap().add_child(RuntimeObject::Value(Value::Int(10)));
        Rc::get_mut(&mut sub_container).unwrap().add_child(RuntimeObject::Value(Value::String("value1".to_owned())));

        let mut sub_sub_container = Rc::new(Container::new());
        Rc::get_mut(&mut sub_sub_container).unwrap().add_child(RuntimeObject::Value(Value::Float(3.14)));
        Rc::get_mut(&mut sub_container).unwrap().add_child(RuntimeObject::Container(sub_sub_container));

        Rc::get_mut(&mut root_container).unwrap().add_child(RuntimeObject::Container(sub_container));
        Rc::get_mut(&mut root_container).unwrap().add_child(RuntimeObject::Value(Value::String("value2".to_owned())));

        let mut runtime_context = RuntimeContext::new(&root_container);

        assert_eq!(runtime_context.get().unwrap().as_value().unwrap().as_int().unwrap(), 42);
        assert_eq!(runtime_context.depth(), 1);

        assert_eq!(runtime_context.next().unwrap().as_value().unwrap().as_int().unwrap(), 10);
        assert_eq!(runtime_context.depth(), 2);

        assert_eq!(runtime_context.next().unwrap().as_value().unwrap().as_string().unwrap(), "value1");
        assert_eq!(runtime_context.depth(), 2);

        assert_eq!(runtime_context.next().unwrap().as_value().unwrap().as_float().unwrap(), 3.14);
        assert_eq!(runtime_context.depth(), 3);

        assert_eq!(runtime_context.next().unwrap().as_value().unwrap().as_string().unwrap(), "value2");
        assert_eq!(runtime_context.depth(), 1);

        assert!(runtime_context.next().is_none());
        assert_eq!(runtime_context.get().unwrap().as_value().unwrap().as_string().unwrap(), "value2");
    }

    #[test]
    fn empty_container_test() {
        use runtime::value::Value;

        let mut root_container = Rc::new(Container::new());
        let mut runtime_context = RuntimeContext::new(&root_container);

        assert!(runtime_context.get().is_none());
        assert!(runtime_context.next().is_none());
        assert_eq!(runtime_context.depth(), 1);
    }
}