use runtime::container::Container;
use runtime::divert::PushPopType;
use runtime::RuntimeObject;

use std::collections::HashMap;

#[derive(Clone)]
pub struct Element<'ru> {
    container: &'ru Container,
    index: usize,
    in_expression_evaluation: bool,
    /*temporary_variables: HashMap<String, RuntimeObject>,*/
    stack_push_type: PushPopType
}

impl<'ru> Element<'ru> {
    pub fn new(stack_push_type: PushPopType, container: &'ru Container, index: usize) -> Element<'ru> {
        Element {
            container: container,
            index: index,
            stack_push_type: stack_push_type,
            in_expression_evaluation: false
        }
    }

    pub fn current_runtime_object(&self) -> Option<&RuntimeObject> {
        self.container.content().get(self.index)
    }

    pub fn stack_push_type(&self) -> PushPopType {
        self.stack_push_type
    }

    pub fn in_expression_evaluation(&self) -> bool {
        self.in_expression_evaluation
    }

    pub fn set_container(&mut self, container: &'ru Container) {
        self.container = container;
    }

    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    pub fn set_runtime_object(&mut self, container: &'ru Container, index: usize) {
        self.container = container;
        self.index = index;
    }

    pub fn set_in_expression_evaluation(&mut self, in_expression_evaluation: bool) {
        self.in_expression_evaluation = in_expression_evaluation;
    }
}

#[derive(Clone)]
pub struct Thread<'ru> {
    stack: Vec<Element<'ru>>
}

impl<'ru> Thread<'ru> {
    pub fn new() -> Thread<'ru> {
        Thread {
            stack: Vec::new()
        }
    }

    pub fn stack(&self) -> &Vec<Element<'ru>> {
        &self.stack
    }

    pub fn push(&mut self, element: Element<'ru>) {
        self.stack.push(element);
    }

    pub fn pop(&mut self) -> Option<Element<'ru>> {
        self.stack.pop()
    }

    pub fn pop_if<F>(&mut self, f: F) -> Option<Element<'ru>>
        where F: FnOnce(&Element<'ru>) -> bool {
        let mut should_pop = match self.stack.last() {
            Some(element) => f(element),
            _ => false
        };

        // Need to do this because at the moment rust does not support Non-lexical borrow scopes
        // See https://github.com/rust-lang/rfcs/issues/811
        if should_pop {
            return self.stack.pop();
        }

        None
    }

}

#[derive(Clone)]
pub struct CallStack<'ru> {
    threads: Vec<Thread<'ru>>
}

impl<'ru> CallStack<'ru> {
    pub fn new(root_container: &'ru Container) -> CallStack<'ru> {
        let mut threads = Vec::new();
        let mut thread = Thread::new();

        thread.push(Element::new(PushPopType::Tunnel, root_container, 0));
        threads.push(thread);

        CallStack {
            threads: threads
        }
    }

    pub fn current_thread(&self) -> Option<&Thread<'ru>> {
        self.threads.last()
    }

    pub fn current_stack(&self) -> Option<&Vec<Element<'ru>>> {
        self.current_thread().map(|thread| thread.stack())
    }

    pub fn current_element(&self) -> Option<&Element<'ru>> {
        self.current_stack().and_then(|stack| stack.last())
    }

    pub fn depth(&self) -> usize {
        match self.current_stack() {
            Some(stack) => stack.len(),
            _ => 0
        }
    }

    pub fn get_thread(&self, index: usize) -> Option<&Thread<'ru>> {
        self.threads.get(index)
    }

    pub fn push_thread(&mut self) -> bool {
        if let Some(thread) = self.threads.last().map(|thread| thread.clone()) {
            self.threads.push(thread);
            return true;
        }

        false
    }

    pub fn pop_thread(&mut self) -> bool {
        match self.threads.pop() {
            Some(_) => true,
            _ => false
        }
    }
}